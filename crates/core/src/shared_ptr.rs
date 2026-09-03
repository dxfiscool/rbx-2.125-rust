//! boost::detail shared-ownership control blocks backing `SharedPtr<T> = Arc<T>`.
//! Grounded in IDA decompile/disasm of the instantiations wrapped by the
//! `stub_*` fns in `boost_core_b/c/f`: shared_ptr ctor 0x463ce8/0x4fe078,
//! shared_count 0x4fe14c, dispose 0x463dc8/0x4fec90/0x491ac0, D0 thunk
//! 0x4fec8c/0x491abc, D1 0x463dc0/0x4fec88/0x491ab8, get_deleter
//! 0x4fed34/0x491ae0, get_untyped_deleter 0x463e70/0x4fed38/0x491af8.
//! was: boost::detail::sp_counted_impl_p<T>, sp_counted_impl_pd<P,D>,
//!      shared_count, sp_counted_base → owned boxes + Arc (AGENTS.md §4).

use super::SharedPtr;

/// was: `boost::detail::sp_counted_impl_p<T>` — 0x10-byte block.
/// IDA 0x4fe14c: `new 0x10; use_count = 1; weak_count = 1; vtable set; px = p`.
pub struct ControlBlockP<T> {
    ptr: Option<Box<T>>,
    use_count: usize,
    weak_count: usize,
}

/// was: `boost::detail::sp_counted_impl_pd<P, RBX::Creatable<RBX::Instance>::Deleter>`
/// — `sp_counted_impl_p` plus deleter at +16 (IDA 0x491ae0/0x491af8: `this+16`).
pub struct ControlBlockPd<T, D> {
    ptr: Option<Box<T>>,
    use_count: usize,
    weak_count: usize,
    deleter: D,
}

/// was: `RBX::Creatable<RBX::Instance>::Deleter` — empty tag stored at block+16.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CreatableInstanceDeleter;

/// type_info name compared at IDA 0x491af2.
pub const CREATABLE_INSTANCE_DELETER_TYPE_NAME: &str =
    "N3RBX9CreatableINS_8InstanceEE7DeleterE";

/// IDA 0x463ce8/0x4fe078 `shared_ptr<Y>(Y* p)`: `px = p`,
/// `pi = new sp_counted_impl_p(p)` (checked_delete(p) on throw).
/// Box→Arc is the same single-owner adoption: one owner, freed on release.
pub fn shared_ptr_from_raw<T>(px: Box<T>) -> SharedPtr<T> {
    SharedPtr::from(px)
}

impl<T> ControlBlockP<T> {
    /// IDA 0x4fe14c `shared_count<Y>(Y* p)`: fresh block, both counts 1.
    pub fn new(px: Box<T>) -> Self {
        Self {
            ptr: Some(px),
            use_count: 1,
            weak_count: 1,
        }
    }

    pub fn use_count(&self) -> usize {
        self.use_count
    }

    pub fn weak_count(&self) -> usize {
        self.weak_count
    }

    pub fn get(&self) -> Option<&T> {
        self.ptr.as_deref()
    }

    /// IDA 0x463dc8/0x4fec90: `px = this+12; if (px) { T::~T(px); operator delete(px); }`.
    /// `Option::take` + drop is exactly dtor-then-free, skipped when null.
    pub fn dispose(&mut self) {
        self.ptr.take();
    }

    /// IDA 0x4fed34: `return 0` — a `_p` block never carries a deleter.
    pub fn get_deleter(&self) -> Option<CreatableInstanceDeleter> {
        None
    }

    /// IDA 0x463e70/0x4fed38: `return 0`.
    pub fn get_untyped_deleter(&self) -> Option<CreatableInstanceDeleter> {
        None
    }
}

impl<T, D> ControlBlockPd<T, D> {
    pub fn new(px: Box<T>, deleter: D) -> Self {
        Self {
            ptr: Some(px),
            use_count: 1,
            weak_count: 1,
            deleter,
        }
    }

    pub fn use_count(&self) -> usize {
        self.use_count
    }

    pub fn get(&self) -> Option<&T> {
        self.ptr.as_deref()
    }

    /// IDA 0x491ac0: `v2 = px; predelete(v2); if (v2) virtual-delete(v2)`.
    /// `predelete` is the `RBX::Instance::predelete` hook (datamodel-owned,
    /// passed in); the trailing deleter-driven delete is drop-after-hook.
    pub fn dispose_with(&mut self, predelete: impl FnOnce(Option<&T>)) {
        predelete(self.ptr.as_deref());
        self.ptr.take();
    }
}

impl<T> ControlBlockPd<T, CreatableInstanceDeleter> {
    /// IDA 0x491ae0: `if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE")
    /// return 0; return this+16;`.
    pub fn get_deleter(&self, type_name: &str) -> Option<CreatableInstanceDeleter> {
        if type_name == CREATABLE_INSTANCE_DELETER_TYPE_NAME {
            Some(self.deleter)
        } else {
            None
        }
    }

    /// IDA 0x491af8: `return this+16` — unconditionally the stored deleter.
    pub fn get_untyped_deleter(&self) -> CreatableInstanceDeleter {
        self.deleter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctor_adopts_single_owner_with_unit_counts() {
        let b = ControlBlockP::new(Box::new(7u32));
        assert_eq!((b.use_count(), b.weak_count()), (1, 1));
        assert_eq!(b.get(), Some(&7));
        let s: SharedPtr<u32> = shared_ptr_from_raw(Box::new(7u32));
        assert_eq!(*s, 7);
    }

    #[test]
    fn dispose_runs_dtor_once_then_noops() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct Bomb(Arc<AtomicUsize>);
        impl Drop for Bomb {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let n = Arc::new(AtomicUsize::new(0));
        let mut b = ControlBlockP::new(Box::new(Bomb(Arc::clone(&n))));
        b.dispose();
        assert_eq!(n.load(Ordering::SeqCst), 1);
        assert!(b.get().is_none());
        b.dispose();
        assert_eq!(n.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn plain_block_never_has_a_deleter() {
        let b = ControlBlockP::new(Box::new(0u8));
        assert_eq!(b.get_deleter(), None);
        assert_eq!(b.get_untyped_deleter(), None);
    }

    #[test]
    fn pd_get_deleter_matches_type_name_only() {
        let b =
            ControlBlockPd::new(Box::new(1u8), CreatableInstanceDeleter);
        assert_eq!(
            b.get_deleter(CREATABLE_INSTANCE_DELETER_TYPE_NAME),
            Some(CreatableInstanceDeleter)
        );
        assert_eq!(b.get_deleter("i"), None);
        assert_eq!(b.get_untyped_deleter(), CreatableInstanceDeleter);
    }

    #[test]
    fn pd_dispose_calls_predelete_before_free() {
        use std::cell::Cell;
        let seen_null = Cell::new(false);
        let mut b: ControlBlockPd<u8, CreatableInstanceDeleter> =
            ControlBlockPd::new(Box::new(9u8), CreatableInstanceDeleter);
        b.dispose_with(|px| assert_eq!(px, Some(&9)));
        assert!(b.get().is_none());
        // Original calls predelete even when px is null (IDA 0x491ac8 before 0x491ace).
        b.dispose_with(|px| seen_null.set(px.is_none()));
        assert!(seen_null.get());
    }
}
