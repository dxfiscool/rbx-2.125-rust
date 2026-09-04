//! `rbx::intrusive_ptr_target` ref counts — intrusive strong/weak pair.
//! was: `rbx::intrusive_ptr_target<T, int, 0, 0>` + `boost::intrusive_ptr`
//!      → atomics here; ownership itself is `Arc<T>` (`crate::SharedPtr`).

use std::sync::atomic::{AtomicI32, Ordering};

/// was: `rbx::intrusive_ptr_target<..., int, 0, 0>::counts` — the
/// `{strong, weak}` header stored 8 bytes before the object
/// (IDA 0x3c068/0x3bd16 address the counts at `obj − 8` / `obj − 4`).
#[doc(alias = "rbx::intrusive_ptr_target::counts")]
pub struct IntrusiveCounts {
    strong: AtomicI32,
    weak: AtomicI32,
}

impl IntrusiveCounts {
    /// IDA 0x3d240: alignment asserts, then `*(u64 *)c = 0x100000000` —
    /// strong = 0 (no owner yet; the adopting `shared_ptr` addrefs), weak = 1
    /// (the block's self reference, freed last).
    // 0x3d240 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev
    #[doc(alias = "rbx::intrusive_ptr_target::counts::counts")]
    pub fn new() -> Self {
        Self {
            strong: AtomicI32::new(0),
            weak: AtomicI32::new(1),
        }
    }

    pub fn strong(&self) -> i32 {
        self.strong.load(Ordering::Acquire)
    }

    pub fn weak(&self) -> i32 {
        self.weak.load(Ordering::Acquire)
    }

    /// was: `boost::intrusive_ptr_add_ref` — strong + 1.
    pub fn add_ref(&self) {
        self.strong.fetch_add(1, Ordering::Relaxed);
    }

    /// was: `boost::intrusive_ptr_release` — strong − 1; true when the last
    /// strong ref is gone and the caller must destroy the object (the weak
    /// self-ref keeps the block alive until `release_weak`).
    pub fn release(&self) -> bool {
        self.strong.fetch_sub(1, Ordering::Release) == 1
    }

    /// IDA 0x3c010: assert `strong > 0` (intrusive_ptr_target.h:214), then
    /// `OSAtomicAdd32(1, &weak)`; assert `weak < max − 10`.
    // 0x3c010 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
    #[doc(alias = "boost::intrusive_ptr_add_weak_ref")]
    pub fn add_weak_ref(&self) {
        debug_assert!(self.strong.load(Ordering::Relaxed) > 0, "c->strong > 0");
        let w = self.weak.fetch_add(1, Ordering::Relaxed) + 1;
        debug_assert!(w < i32::MAX - 10, "c->weak < max - 10");
    }

    /// Weak − 1; true when the self-ref is gone and the caller must free
    /// the block (mirrors the `weak == 1` precondition of `operator delete`).
    pub fn release_weak(&self) -> bool {
        self.weak.fetch_sub(1, Ordering::Release) == 1
    }

    /// IDA 0x3bcb8: assert `strong == 0` (:133) and `weak == 1` (:134),
    /// then `free(block)`. Call before freeing; the free itself is the
    /// caller's `drop` (Rust has no placement `operator delete`).
    // 0x3bcb8 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv
    #[doc(alias = "rbx::intrusive_ptr_target::operator delete")]
    pub fn check_free_preconditions(&self) {
        debug_assert_eq!(self.strong.load(Ordering::Relaxed), 0, "c->strong == 0");
        debug_assert_eq!(self.weak.load(Ordering::Relaxed), 1, "c->weak == 1");
    }
}

impl Default for IntrusiveCounts {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_counts_are_strong0_weak1() {
        // IDA 0x3d2e8: `*(u64 *)c = 0x100000000`.
        let c = IntrusiveCounts::new();
        assert_eq!(c.strong(), 0);
        assert_eq!(c.weak(), 1);
    }

    #[test]
    fn weak_lifecycle_needs_a_live_strong() {
        // IDA 0x3c010 path: strong > 0, weak 1 → 2.
        let c = IntrusiveCounts::new();
        c.add_ref();
        c.add_weak_ref();
        assert_eq!((c.strong(), c.weak()), (1, 2));
        // Object dies at strong 0 …
        assert!(c.release());
        // … block frees when the last weak goes (2 → 1 → free check ok).
        assert!(!c.release_weak());
        c.check_free_preconditions();
        assert!(c.release_weak());
    }
}
