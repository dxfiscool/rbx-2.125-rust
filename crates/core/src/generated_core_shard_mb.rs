//! core shard mb — 150 core stubs EA-sorted, next uncovered fallback gap filler (lowest unstubbed EA first).
//! Source: ida/export.json (85545 funcs) global EA asc not yet stubbed in any crate — next 150 uncovered sorted asc (0x3eb460..0x4136b0).
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "global constructor keyed to_a_169")]
// 0x3eb460 — __GLOBAL__I_a_169
pub fn stub_0x3eb460() {
    // IDA 0x3eb460: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_170")]
// 0x3f0a94 — __GLOBAL__I_a_170
pub fn stub_0x3f0a94() {
    // IDA 0x3f0a94: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ClickDetector> RBX::shared_from<RBX::ClickDetector>(RBX::ClickDetector*)")]
// 0x3f1984 — __ZN3RBX11shared_fromINS_13ClickDetectorEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
pub fn stub_0x3f1984() {
    // IDA 0x3f1984: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_171")]
// 0x3f2ab4 — __GLOBAL__I_a_171
pub fn stub_0x3f2ab4() {
    // IDA 0x3f2ab4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x408264 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x408264() {
    // IDA 0x408264: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FillTool>::shared_ptr<RBX::FillTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x408990 — __ZN5boost10shared_ptrIN3RBX8FillToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x408990() {
    // IDA 0x408990: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::FillTool,RBX::FillTool>(rbx_core::SharedPtr<RBX::FillTool> const*,RBX::FillTool *)const")]
// 0x408a58 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8FillToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x408a58() {
    // IDA 0x408a58: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x408b3c — __ZN5boost6detail12shared_countC2IPN3RBX8FillToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x408b3c() {
    // IDA 0x408b3c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x408c34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x408c34() {
    // IDA 0x408c34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x408c38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x408c38() {
    // IDA 0x408c38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x408c3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x408c3c() {
    // IDA 0x408c3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x408c4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x408c4c() {
    // IDA 0x408c4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x408c64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x408c64() {
    // IDA 0x408c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool> RBX::shared_from<RBX::LockTool>(RBX::LockTool*)")]
// 0x409374 — __ZN3RBX11shared_fromINS_8LockToolEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
pub fn stub_0x409374() {
    // IDA 0x409374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool>::shared_ptr<RBX::LockTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x4094dc — __ZN5boost10shared_ptrIN3RBX8LockToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x4094dc() {
    // IDA 0x4094dc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LockTool,RBX::LockTool>(rbx_core::SharedPtr<RBX::LockTool> const*,RBX::LockTool *)const")]
// 0x4095a4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8LockToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x4095a4() {
    // IDA 0x4095a4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x409688 — __ZN5boost6detail12shared_countC2IPN3RBX8LockToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x409688() {
    // IDA 0x409688: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x409780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x409780() {
    // IDA 0x409780: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x409784 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x409784() {
    // IDA 0x409784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x409788 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x409788() {
    // IDA 0x409788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x409798 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x409798() {
    // IDA 0x409798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x4097b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x4097b0() {
    // IDA 0x4097b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)")]
// 0x409ec8 — __ZN3RBX11shared_fromINS_10AnchorToolEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
pub fn stub_0x409ec8() {
    // IDA 0x409ec8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40a030 — __ZN5boost10shared_ptrIN3RBX10AnchorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40a030() {
    // IDA 0x40a030: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AnchorTool,RBX::AnchorTool>(rbx_core::SharedPtr<RBX::AnchorTool> const*,RBX::AnchorTool *)const")]
// 0x40a0f8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10AnchorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40a0f8() {
    // IDA 0x40a0f8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40a1dc — __ZN5boost6detail12shared_countC2IPN3RBX10AnchorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40a1dc() {
    // IDA 0x40a1dc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40a2d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40a2d4() {
    // IDA 0x40a2d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40a2d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40a2d8() {
    // IDA 0x40a2d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40a2dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40a2dc() {
    // IDA 0x40a2dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40a2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40a2ec() {
    // IDA 0x40a2ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40a304 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40a304() {
    // IDA 0x40a304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40aa30 — __ZN5boost10shared_ptrIN3RBX20SmoothNoOutlinesToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40aa30() {
    // IDA 0x40aa30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::SmoothNoOutlinesTool,RBX::SmoothNoOutlinesTool>(rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> const*,RBX::SmoothNoOutlinesTool *)const")]
// 0x40aaf8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_20SmoothNoOutlinesToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40aaf8() {
    // IDA 0x40aaf8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40abdc — __ZN5boost6detail12shared_countC2IPN3RBX20SmoothNoOutlinesToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40abdc() {
    // IDA 0x40abdc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40acd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40acd4() {
    // IDA 0x40acd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40acd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40acd8() {
    // IDA 0x40acd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40acdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40acdc() {
    // IDA 0x40acdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40acec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40acec() {
    // IDA 0x40acec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40ad04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40ad04() {
    // IDA 0x40ad04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool>::shared_ptr<RBX::OscillateMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40b368 — __ZN5boost10shared_ptrIN3RBX18OscillateMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40b368() {
    // IDA 0x40b368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::OscillateMotorTool,RBX::OscillateMotorTool>(rbx_core::SharedPtr<RBX::OscillateMotorTool> const*,RBX::OscillateMotorTool *)const")]
// 0x40b430 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18OscillateMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40b430() {
    // IDA 0x40b430: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40b514 — __ZN5boost6detail12shared_countC2IPN3RBX18OscillateMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40b514() {
    // IDA 0x40b514: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40b60c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40b60c() {
    // IDA 0x40b60c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40b610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40b610() {
    // IDA 0x40b610: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40b614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40b614() {
    // IDA 0x40b614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40b624 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40b624() {
    // IDA 0x40b624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40b63c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40b63c() {
    // IDA 0x40b63c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool>::shared_ptr<RBX::LeftMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40bca0 — __ZN5boost10shared_ptrIN3RBX13LeftMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40bca0() {
    // IDA 0x40bca0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LeftMotorTool,RBX::LeftMotorTool>(rbx_core::SharedPtr<RBX::LeftMotorTool> const*,RBX::LeftMotorTool *)const")]
// 0x40bd68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13LeftMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40bd68() {
    // IDA 0x40bd68: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40be4c — __ZN5boost6detail12shared_countC2IPN3RBX13LeftMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40be4c() {
    // IDA 0x40be4c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40bf44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40bf44() {
    // IDA 0x40bf44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40bf48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40bf48() {
    // IDA 0x40bf48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40bf4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40bf4c() {
    // IDA 0x40bf4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40bf5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40bf5c() {
    // IDA 0x40bf5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40bf74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40bf74() {
    // IDA 0x40bf74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool>::shared_ptr<RBX::RightMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40c6a0 — __ZN5boost10shared_ptrIN3RBX14RightMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40c6a0() {
    // IDA 0x40c6a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::RightMotorTool,RBX::RightMotorTool>(rbx_core::SharedPtr<RBX::RightMotorTool> const*,RBX::RightMotorTool *)const")]
// 0x40c768 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14RightMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40c768() {
    // IDA 0x40c768: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40c84c — __ZN5boost6detail12shared_countC2IPN3RBX14RightMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40c84c() {
    // IDA 0x40c84c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40c944 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40c944() {
    // IDA 0x40c944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40c948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40c948() {
    // IDA 0x40c948: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40c94c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40c94c() {
    // IDA 0x40c94c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40c95c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40c95c() {
    // IDA 0x40c95c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40c974 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40c974() {
    // IDA 0x40c974: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HingeTool>::shared_ptr<RBX::HingeTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40d0a0 — __ZN5boost10shared_ptrIN3RBX9HingeToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40d0a0() {
    // IDA 0x40d0a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::HingeTool,RBX::HingeTool>(rbx_core::SharedPtr<RBX::HingeTool> const*,RBX::HingeTool *)const")]
// 0x40d168 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9HingeToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40d168() {
    // IDA 0x40d168: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40d24c — __ZN5boost6detail12shared_countC2IPN3RBX9HingeToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40d24c() {
    // IDA 0x40d24c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40d344 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40d344() {
    // IDA 0x40d344: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40d348 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40d348() {
    // IDA 0x40d348: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40d34c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40d34c() {
    // IDA 0x40d34c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40d35c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40d35c() {
    // IDA 0x40d35c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40d374 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40d374() {
    // IDA 0x40d374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UniversalTool>::shared_ptr<RBX::UniversalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40daa0 — __ZN5boost10shared_ptrIN3RBX13UniversalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40daa0() {
    // IDA 0x40daa0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::UniversalTool,RBX::UniversalTool>(rbx_core::SharedPtr<RBX::UniversalTool> const*,RBX::UniversalTool *)const")]
// 0x40db68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13UniversalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40db68() {
    // IDA 0x40db68: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40dc4c — __ZN5boost6detail12shared_countC2IPN3RBX13UniversalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40dc4c() {
    // IDA 0x40dc4c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40dd44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40dd44() {
    // IDA 0x40dd44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40dd48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40dd48() {
    // IDA 0x40dd48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40dd4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40dd4c() {
    // IDA 0x40dd4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40dd5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40dd5c() {
    // IDA 0x40dd5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40dd74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40dd74() {
    // IDA 0x40dd74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv")]
// 0x40e3b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv
// type: int()
pub fn stub_0x40e3b8() {
    // IDA 0x40e3b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InletTool>::shared_ptr<RBX::InletTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40e4a0 — __ZN5boost10shared_ptrIN3RBX9InletToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40e4a0() {
    // IDA 0x40e4a0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::InletTool,RBX::InletTool>(rbx_core::SharedPtr<RBX::InletTool> const*,RBX::InletTool *)const")]
// 0x40e568 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9InletToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40e568() {
    // IDA 0x40e568: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40e64c — __ZN5boost6detail12shared_countC2IPN3RBX9InletToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40e64c() {
    // IDA 0x40e64c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40e744 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40e744() {
    // IDA 0x40e744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40e748 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40e748() {
    // IDA 0x40e748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40e74c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40e74c() {
    // IDA 0x40e74c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40e75c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40e75c() {
    // IDA 0x40e75c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40e774 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40e774() {
    // IDA 0x40e774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v")]
// 0x40e778 — __ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v
// type: int(void)
pub fn stub_0x40e778() {
    // IDA 0x40e778: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv")]
// 0x40e7bc — __ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv
pub fn stub_0x40e7bc() {
    // IDA 0x40e7bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v")]
// 0x40e7c0 — __ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v
// type: int()
pub fn stub_0x40e7c0() {
    // IDA 0x40e7c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv")]
// 0x40edb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv
// type: int()
pub fn stub_0x40edb8() {
    // IDA 0x40edb8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StudsTool>::shared_ptr<RBX::StudsTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40eea0 — __ZN5boost10shared_ptrIN3RBX9StudsToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40eea0() {
    // IDA 0x40eea0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::StudsTool,RBX::StudsTool>(rbx_core::SharedPtr<RBX::StudsTool> const*,RBX::StudsTool *)const")]
// 0x40ef68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9StudsToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40ef68() {
    // IDA 0x40ef68: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40f04c — __ZN5boost6detail12shared_countC2IPN3RBX9StudsToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40f04c() {
    // IDA 0x40f04c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40f144 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40f144() {
    // IDA 0x40f144: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40f148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40f148() {
    // IDA 0x40f148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40f14c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40f14c() {
    // IDA 0x40f14c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40f15c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40f15c() {
    // IDA 0x40f15c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40f174 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40f174() {
    // IDA 0x40f174: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v")]
// 0x40f178 — __ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v
// type: int(void)
pub fn stub_0x40f178() {
    // IDA 0x40f178: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv")]
// 0x40f1bc — __ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv
pub fn stub_0x40f1bc() {
    // IDA 0x40f1bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v")]
// 0x40f1c0 — __ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v
// type: int()
pub fn stub_0x40f1c0() {
    // IDA 0x40f1c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv")]
// 0x40f7b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv
// type: int()
pub fn stub_0x40f7b8() {
    // IDA 0x40f7b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::WeldTool>::shared_ptr<RBX::WeldTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40f8a0 — __ZN5boost10shared_ptrIN3RBX8WeldToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x40f8a0() {
    // IDA 0x40f8a0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::WeldTool,RBX::WeldTool>(rbx_core::SharedPtr<RBX::WeldTool> const*,RBX::WeldTool *)const")]
// 0x40f968 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8WeldToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x40f968() {
    // IDA 0x40f968: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x40fa4c — __ZN5boost6detail12shared_countC2IPN3RBX8WeldToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x40fa4c() {
    // IDA 0x40fa4c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40fb44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x40fb44() {
    // IDA 0x40fb44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x40fb48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40fb48() {
    // IDA 0x40fb48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x40fb4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x40fb4c() {
    // IDA 0x40fb4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x40fb5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x40fb5c() {
    // IDA 0x40fb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x40fb74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x40fb74() {
    // IDA 0x40fb74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v")]
// 0x40fb78 — __ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v
// type: int(void)
pub fn stub_0x40fb78() {
    // IDA 0x40fb78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv")]
// 0x40fbbc — __ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv
pub fn stub_0x40fbbc() {
    // IDA 0x40fbbc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v")]
// 0x40fbc0 — __ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v
// type: int()
pub fn stub_0x40fbc0() {
    // IDA 0x40fbc0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv")]
// 0x4101b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv
// type: int()
pub fn stub_0x4101b8() {
    // IDA 0x4101b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlueTool>::shared_ptr<RBX::GlueTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x4102a0 — __ZN5boost10shared_ptrIN3RBX8GlueToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x4102a0() {
    // IDA 0x4102a0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::GlueTool,RBX::GlueTool>(rbx_core::SharedPtr<RBX::GlueTool> const*,RBX::GlueTool *)const")]
// 0x410368 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GlueToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x410368() {
    // IDA 0x410368: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x41044c — __ZN5boost6detail12shared_countC2IPN3RBX8GlueToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x41044c() {
    // IDA 0x41044c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x410544 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x410544() {
    // IDA 0x410544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x410548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x410548() {
    // IDA 0x410548: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x41054c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x41054c() {
    // IDA 0x41054c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x41055c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x41055c() {
    // IDA 0x41055c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x410574 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x410574() {
    // IDA 0x410574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v")]
// 0x410578 — __ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v
// type: int(void)
pub fn stub_0x410578() {
    // IDA 0x410578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv")]
// 0x4105bc — __ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv
pub fn stub_0x4105bc() {
    // IDA 0x4105bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v")]
// 0x4105c0 — __ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v
// type: int()
pub fn stub_0x4105c0() {
    // IDA 0x4105c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv")]
// 0x410bb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv
// type: int()
pub fn stub_0x410bb8() {
    // IDA 0x410bb8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlatTool>::shared_ptr<RBX::FlatTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x410ca0 — __ZN5boost10shared_ptrIN3RBX8FlatToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x410ca0() {
    // IDA 0x410ca0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::FlatTool,RBX::FlatTool>(rbx_core::SharedPtr<RBX::FlatTool> const*,RBX::FlatTool *)const")]
// 0x410d68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8FlatToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x410d68() {
    // IDA 0x410d68: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x410e4c — __ZN5boost6detail12shared_countC2IPN3RBX8FlatToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x410e4c() {
    // IDA 0x410e4c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x410f44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
pub fn stub_0x410f44() {
    // IDA 0x410f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x410f48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_0x410f48() {
    // IDA 0x410f48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x410f4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x410f4c() {
    // IDA 0x410f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x410f5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x410f5c() {
    // IDA 0x410f5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x410f74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x410f74() {
    // IDA 0x410f74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v")]
// 0x410f78 — __ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v
// type: int(void)
pub fn stub_0x410f78() {
    // IDA 0x410f78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv")]
// 0x410fbc — __ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv
pub fn stub_0x410fbc() {
    // IDA 0x410fbc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v")]
// 0x410fc0 — __ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v
// type: int()
pub fn stub_0x410fc0() {
    // IDA 0x410fc0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv")]
// 0x4119fc — __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv
// type: int()
pub fn stub_0x4119fc() {
    // IDA 0x4119fc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v")]
// 0x41208c — __ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v
// type: int(void)
pub fn stub_0x41208c() {
    // IDA 0x41208c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv")]
// 0x4120d0 — __ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv
pub fn stub_0x4120d0() {
    // IDA 0x4120d0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v")]
// 0x4120d4 — __ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v
// type: int()
pub fn stub_0x4120d4() {
    // IDA 0x4120d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv")]
// 0x4126e4 — __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv
// type: int()
pub fn stub_0x4126e4() {
    // IDA 0x4126e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v")]
// 0x412b58 — __ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v
// type: int(void)
pub fn stub_0x412b58() {
    // IDA 0x412b58: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv")]
// 0x412b9c — __ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv
pub fn stub_0x412b9c() {
    // IDA 0x412b9c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v")]
// 0x412ba0 — __ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v
// type: int()
pub fn stub_0x412ba0() {
    // IDA 0x412ba0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12AdvArrowToolELZNS_19sMoveResizeJoinToolEEE7getNameEv")]
// 0x4132b0 — __ZNK3RBX5NamedINS_12AdvArrowToolELZNS_19sMoveResizeJoinToolEEE7getNameEv
// type: int()
pub fn stub_0x4132b0() {
    // IDA 0x4132b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
// 0x41366c — __ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v
// type: int(void)
pub fn stub_0x41366c() {
    // IDA 0x41366c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sMoveResizeJoinToolEEEEvv")]
// 0x4136b0 — __ZN3RBX4Name13callDoDeclareILZNS_19sMoveResizeJoinToolEEEEvv
pub fn stub_0x4136b0() {
    // IDA 0x4136b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}
