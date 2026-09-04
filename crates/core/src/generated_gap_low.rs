//! core — generated_gap_low —  11 stubs EA-sorted asc global gap filler 0x84e0..0xee84 (lowest uncovered, 81902 gaps before, 81891 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/ — next 150 uncovered sorted asc, partitioned by demangled namespace
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x84e0 — start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
#[doc(alias = "start")]
pub fn stub_0x84e0() {
    // IDA 0x84e0: process entry point (crt0 start, __noreturn argc/argv). Rust std owns process entry -- carrier no-op.
}

// 0x9b2c — __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
pub fn stub_0x9b2c() {
    // IDA 0x9b2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_0x9b30() {
    // IDA 0x9b30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_0x9b44() {
    // IDA 0x9b44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb8d0() {
    // IDA 0xb8d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb900() {
    // IDA 0xb900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0xb930() {
    // IDA 0xb930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "exception_detail::refcount_ptr<exception_detail::error_info_container>::~refcount_ptr()")]
pub fn stub_0xcb94() {
    // IDA 0xcb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xeccc() {
    // IDA 0xeccc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xedfc() {
    // IDA 0xedfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_0xee84() {
    // IDA 0xee84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
