//! datamodel — generated_gap_low —  7 stubs EA-sorted asc global gap filler 0xb474..0xb8b0 (lowest uncovered, 81902 gaps before, 81895 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/ — next 150 uncovered sorted asc, partitioned by demangled namespace
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
pub fn stub_0xb474(this: *const u8) -> i32 {
    // IDA 0xb474..0xb476: `return *((_DWORD *)this + 7)` — dword at byte
    // offset 28 (0x1C), the `auto_quality_level` slot (item +0x7C).
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { (this.add(28) as *const i32).read_unaligned() }
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
pub fn stub_0xb49c(this: *const u8) -> i32 {
    // IDA 0xb49c..0xb4a0: `return *((unsigned __int8 *)this + 41)` — byte at
    // offset 41 (0x29), the `enable_frm` slot (item +0x89), zero-extended.
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { this.add(41).read_unaligned() as i32 }
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
pub fn stub_0xb4a4(this: *const u8) -> i32 {
    // IDA 0xb4a4..0xb4a6: `return *((_DWORD *)this + 6)` — dword at byte
    // offset 24 (0x18), the `resolution_preset` slot (item +0x78).
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { (this.add(24) as *const i32).read_unaligned() }
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
pub fn stub_0xb4cc(this: *const u8) -> i32 {
    // IDA 0xb4cc..0xb4ce: `return *((_DWORD *)this + 8)` — dword at byte
    // offset 32 (0x20), the `max_quality_level` slot (item +0x80).
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { (this.add(32) as *const i32).read_unaligned() }
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
pub fn stub_0xb4f4(this: *const u8) -> i32 {
    // IDA 0xb4f4..0xb4f6: `return *((_DWORD *)this + 16)` — dword at byte
    // offset 64 (0x40), the `texture_cache_size` slot (item +0xA0).
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { (this.add(64) as *const i32).read_unaligned() }
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
pub fn stub_0xb4f8(this: *const u8) -> i32 {
    // IDA 0xb4f8..0xb4fa: `return *((_DWORD *)this + 17)` — dword at byte
    // offset 68 (0x44), the `mesh_cache_size` slot (item +0xA4).
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { (this.add(68) as *const i32).read_unaligned() }
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
pub fn stub_0xb8b0(this: *const u8) -> i32 {
    // IDA 0xb8b0..0xb8b4: `return *((unsigned __int8 *)this + 61)` — byte at
    // offset 61 (0x3D), the `eager_bulk_execution` slot (item +0x9D),
    // zero-extended.
    // SAFETY: `this` must point to a valid `RBX::CRenderSettings`.
    unsafe { this.add(61).read_unaligned() as i32 }
}

#[cfg(test)]
mod gap_low_tests {
    use super::*;
    #[test]
    fn getters_read_their_ida_slots() {
        let mut raw = [0u8; 72];
        raw[28..32].copy_from_slice(&7i32.to_ne_bytes());
        raw[41] = 1;
        raw[24..28].copy_from_slice(&3i32.to_ne_bytes());
        raw[32..36].copy_from_slice(&21i32.to_ne_bytes());
        raw[64..68].copy_from_slice(&0x2000000i32.to_ne_bytes());
        raw[68..72].copy_from_slice(&0x100000i32.to_ne_bytes());
        raw[61] = 1;
        let base = raw.as_ptr();
        assert_eq!(stub_0xb474(base), 7);
        assert_eq!(stub_0xb49c(base), 1);
        assert_eq!(stub_0xb4a4(base), 3);
        assert_eq!(stub_0xb4cc(base), 21);
        assert_eq!(stub_0xb4f4(base), 0x2000000);
        assert_eq!(stub_0xb4f8(base), 0x100000);
        assert_eq!(stub_0xb8b0(base), 1);
    }
}
