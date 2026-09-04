// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|Workspace (ongoing) — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x28ce40..0x45b538 | datamodel distinct 15589->15709 global uncovered 69956->69836, lowest EA-sorted asc next uncovered
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: watchdog_A EA-sorted ascending next uncovered gap (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x28ce40 — __ZN3RBX10BaseScript19computeNewWorkspaceEv
// type: _DWORD __fastcall(RBX::BaseScript *__hidden this)
#[doc(alias = "RBX::BaseScript::computeNewWorkspace(void)")]
pub fn stub_0x28ce40() -> ! {
    todo!("0x28ce40 RBX::BaseScript::computeNewWorkspace(void)")
}

// 0x38fb1c — __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
pub fn stub_0x38fb1c() -> ! {
    todo!("0x38fb1c RBX::Accoutrement::upTo_InWorkspace(void)")
}

// 0x455890 — __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x455890 as stub_0x455890;

// 0x455998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x455998() -> ! {
    todo!("0x455998 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x45599c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x45599c() -> ! {
    todo!("0x45599c boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4559a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4559a0() -> ! {
    todo!("0x4559a0 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4559c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x4559c0 as stub_0x4559c0;

// 0x4559d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x4559d8 as stub_0x4559d8;

// 0x455ea4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const&)
pub fn stub_0x455ea4() -> ! {
    todo!("0x455ea4 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")
}

// 0x456090 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")]
// was: boost::shared_ptr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)
pub use crate::instance::stub_0x456090 as stub_0x456090;

// 0x456140 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const&)
pub fn stub_0x456140() -> ! {
    todo!("0x456140 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")
}

// 0x456174 — __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x456174 as stub_0x456174;

// 0x456328 — __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x456328 as stub_0x456328;

// 0x456430 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x456430() -> ! {
    todo!("0x456430 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x456434() -> ! {
    todo!("0x456434 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x456438() -> ! {
    todo!("0x456438 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x456458 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x456458 as stub_0x456458;

// 0x456470 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x456470 as stub_0x456470;

// 0x4565e8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")]
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)
pub use crate::instance::stub_0x4565e8 as stub_0x4565e8;

// 0x456698 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const&)
pub fn stub_0x456698() -> ! {
    todo!("0x456698 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")
}

// 0x4567f4 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x4567f4 as stub_0x4567f4;

// 0x4569a8 — __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x4569a8 as stub_0x4569a8;

// 0x456ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x456ab0() -> ! {
    todo!("0x456ab0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x456ab4() -> ! {
    todo!("0x456ab4 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x456ab8() -> ! {
    todo!("0x456ab8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x456ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x456ad8 as stub_0x456ad8;

// 0x456af0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x456af0 as stub_0x456af0;

// 0x456d08 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")]
// was: boost::shared_ptr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)
pub use crate::instance::stub_0x456d08 as stub_0x456d08;

// 0x456db8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&)
pub fn stub_0x456db8() -> ! {
    todo!("0x456db8 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")
}

// 0x456ff4 — __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x456ff4 as stub_0x456ff4;

// 0x4571a8 — __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x4571a8 as stub_0x4571a8;

// 0x4572b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4572b0() -> ! {
    todo!("0x4572b0 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4572b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4572b4() -> ! {
    todo!("0x4572b4 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4572b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4572b8() -> ! {
    todo!("0x4572b8 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4572d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x4572d8 as stub_0x4572d8;

// 0x4572f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x4572f0 as stub_0x4572f0;

// 0x457c98 — __ZN5boost10shared_ptrIN3RBX7GuiRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x457c98 as stub_0x457c98;

// 0x457e4c — __ZN5boost6detail12shared_countC2IPN3RBX7GuiRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x457e4c as stub_0x457e4c;

// 0x457f54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x457f54() -> ! {
    todo!("0x457f54 boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x457f58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x457f58() -> ! {
    todo!("0x457f58 boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x457f5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x457f5c() -> ! {
    todo!("0x457f5c boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x457f7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x457f7c as stub_0x457f7c;

// 0x457f94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x457f94 as stub_0x457f94;

// 0x457f98 — __ZN5boost10shared_ptrIN3RBX9WorkspaceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x457f98 as stub_0x457f98;

// 0x458060 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9WorkspaceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(rbx_core::SharedPtr<RBX::Workspace> const*,RBX::Workspace *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(boost::shared_ptr<RBX::Workspace> const*,RBX::Workspace *)const
pub fn stub_0x458060() -> ! {
    todo!("0x458060 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(rbx_core::SharedPtr<RBX::Workspace> const*,RBX::Workspace *)const")
}

// 0x45814c — __ZN5boost6detail12shared_countC2IPN3RBX9WorkspaceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x45814c as stub_0x45814c;

// 0x458254 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x458254() -> ! {
    todo!("0x458254 boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x458258 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x458258() -> ! {
    todo!("0x458258 boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x45825c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x45825c() -> ! {
    todo!("0x45825c boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x45827c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x45827c as stub_0x45827c;

// 0x458294 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x458294 as stub_0x458294;

// 0x45847c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ChangeHistoryServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)")]
// was: boost::shared_ptr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)
pub use crate::instance::stub_0x45847c as stub_0x45847c;

// 0x45852c — __ZN5boost10shared_ptrIN3RBX20ChangeHistoryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x45852c as stub_0x45852c;

// 0x4586e0 — __ZN5boost6detail12shared_countC2IPN3RBX20ChangeHistoryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x4586e0 as stub_0x4586e0;

// 0x4587e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4587e8() -> ! {
    todo!("0x4587e8 boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4587f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4587f0() -> ! {
    todo!("0x4587f0 boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x458810 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x458810 as stub_0x458810;

// 0x458828 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x458828 as stub_0x458828;

// 0x458f10 — __ZN5boost10shared_ptrIN3RBX9DataModelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x458f10 as stub_0x458f10;

// 0x458fd8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9DataModelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const*,RBX::DataModel *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const*,RBX::DataModel *)const
pub fn stub_0x458fd8() -> ! {
    todo!("0x458fd8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const*,RBX::DataModel *)const")
}

// 0x4590c4 — __ZN5boost6detail12shared_countC2IPN3RBX9DataModelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x4590c4 as stub_0x4590c4;

// 0x4591cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4591cc() -> ! {
    todo!("0x4591cc boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4591d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4591d0() -> ! {
    todo!("0x4591d0 boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4591d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4591d4() -> ! {
    todo!("0x4591d4 boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4591f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x4591f4 as stub_0x4591f4;

// 0x45920c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x45920c as stub_0x45920c;

// 0x459210 — __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEC2IS3_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
// was: boost::shared_ptr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)
pub use crate::instance::stub_0x459210 as stub_0x459210;

// 0x4592f8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_9DataModel10GenericJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const")]
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(boost::shared_ptr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const
pub fn stub_0x4592f8() -> ! {
    todo!("0x4592f8 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const")
}

// 0x4593dc — __ZN5boost6detail12shared_countC2IN3RBX9DataModel10GenericJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
pub use crate::instance::stub_0x4593dc as stub_0x4593dc;

// 0x4594d4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
pub fn stub_0x4594d4() -> ! {
    todo!("0x4594d4 boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")
}

// 0x4594d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
pub fn stub_0x4594d8() -> ! {
    todo!("0x4594d8 boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")
}

// 0x4594dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)")]
pub fn stub_0x4594dc() -> ! {
    todo!("0x4594dc boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)")
}

// 0x4594ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x4594ec as stub_0x4594ec;

// 0x4594f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x4594f0 as stub_0x4594f0;

// 0x459b9c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5VisitEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)")]
// was: boost::shared_ptr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)
pub use crate::instance::stub_0x459b9c as stub_0x459b9c;

// 0x459c4c — __ZN5boost10shared_ptrIN3RBX5VisitEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x459c4c as stub_0x459c4c;

// 0x459e00 — __ZN5boost6detail12shared_countC2IPN3RBX5VisitENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x459e00 as stub_0x459e00;

// 0x459f08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x459f08() -> ! {
    todo!("0x459f08 boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x459f0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x459f0c() -> ! {
    todo!("0x459f0c boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x459f10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x459f10() -> ! {
    todo!("0x459f10 boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x459f30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x459f30 as stub_0x459f30;

// 0x459f48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x459f48 as stub_0x459f48;

// 0x45a174 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Instance10SaveFilterEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::initSingleton(void)")]
pub fn stub_0x45a174() -> ! {
    todo!("0x45a174 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::initSingleton(void)")
}

// 0x45a178 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Instance10SaveFilterEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::doGetSingleton(void)")]
pub use crate::instance::stub_0x45a178 as stub_0x45a178;

// 0x45a268 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel8GearTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::initSingleton(void)")]
pub fn stub_0x45a268() -> ! {
    todo!("0x45a268 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::initSingleton(void)")
}

// 0x45a26c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel8GearTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::doGetSingleton(void)")]
pub use crate::instance::stub_0x45a26c as stub_0x45a26c;

// 0x45a35c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel16GearGenreSettingEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> const>::initSingleton(void)")]
pub fn stub_0x45a35c() -> ! {
    todo!("0x45a35c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> const>::initSingleton(void)")
}

// 0x45a360 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel16GearGenreSettingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> const>::doGetSingleton(void)")]
pub use crate::instance::stub_0x45a360 as stub_0x45a360;

// 0x45a450 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel5GenreEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::initSingleton(void)")]
pub fn stub_0x45a450() -> ! {
    todo!("0x45a450 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::initSingleton(void)")
}

// 0x45a454 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel5GenreEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::doGetSingleton(void)")]
pub use crate::instance::stub_0x45a454 as stub_0x45a454;

// 0x45a544 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel11CreatorTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::initSingleton(void)")]
pub fn stub_0x45a544() -> ! {
    todo!("0x45a544 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::initSingleton(void)")
}

// 0x45a548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9DataModel11CreatorTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::doGetSingleton(void)")]
pub use crate::instance::stub_0x45a548 as stub_0x45a548;

// 0x45a638 — __ZN3rbx8any_castIN3RBX8Instance10SaveFilterENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Instance::SaveFilter * rbx::any_cast<RBX::Instance::SaveFilter,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45a638() -> ! {
    todo!("0x45a638 RBX::Instance::SaveFilter * rbx::any_cast<RBX::Instance::SaveFilter,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x45a78c — __ZN3rbx8any_castIRN3RBX8Instance10SaveFilterENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Instance::SaveFilter & rbx::any_cast<RBX::Instance::SaveFilter &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45a78c() -> ! {
    todo!("0x45a78c RBX::Instance::SaveFilter & rbx::any_cast<RBX::Instance::SaveFilter &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x45a880 — __ZNSt6vectorIN3RBX8Instance10SaveFilterESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::resize(unsigned long,RBX::Instance::SaveFilter)")]
pub fn stub_0x45a880() -> ! {
    todo!("0x45a880 std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::resize(unsigned long,RBX::Instance::SaveFilter)")
}

// 0x45a8b8 — __ZNSt6vectorIN3RBX8Instance10SaveFilterESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::push_back(RBX::Instance::SaveFilter const&)")]
pub fn stub_0x45a8b8() -> ! {
    todo!("0x45a8b8 std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::push_back(RBX::Instance::SaveFilter const&)")
}

// 0x45a8e4 — __ZNSt3mapIPKN3RBX4NameENS0_8Instance10SaveFilterESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::Instance::SaveFilter,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::operator[](RBX::Name const* const&)")]
pub use crate::instance::stub_0x45a8e4 as stub_0x45a8e4;

// 0x45a93c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")]
pub use crate::instance::stub_0x45a93c as stub_0x45a93c;

// 0x45a9f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")]
pub fn stub_0x45a9f0() -> ! {
    todo!("0x45a9f0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")
}

// 0x45aa48 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")]
pub use crate::instance::stub_0x45aa48 as stub_0x45aa48;

// 0x45aab4 — __ZNSt6vectorIN3RBX8Instance10SaveFilterESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Instance::SaveFilter*,std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>>,RBX::Instance::SaveFilter const&)")]
pub fn stub_0x45aab4() -> ! {
    todo!("0x45aab4 std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Instance::SaveFilter*,std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>>,RBX::Instance::SaveFilter const&)")
}

// 0x45ab98 — __ZNSt12_Vector_baseIN3RBX8Instance10SaveFilterESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Vector_base<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_allocate(unsigned long)")]
pub use crate::instance::stub_0x45ab98 as stub_0x45ab98;

// 0x45abb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Instance10SaveFilterES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Instance::SaveFilter * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *>(RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *)")]
pub fn stub_0x45abb0() -> ! {
    todo!("0x45abb0 RBX::Instance::SaveFilter * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *>(RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *)")
}

// 0x45abf0 — __ZNSt6vectorIN3RBX8Instance10SaveFilterESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Instance::SaveFilter*,std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>>,unsigned long,RBX::Instance::SaveFilter const&)")]
pub fn stub_0x45abf0() -> ! {
    todo!("0x45abf0 std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Instance::SaveFilter*,std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>>,unsigned long,RBX::Instance::SaveFilter const&)")
}

// 0x45ad84 — __ZN3rbx8any_castIN3RBX9DataModel8GearTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::GearType * rbx::any_cast<RBX::DataModel::GearType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45ad84() -> ! {
    todo!("0x45ad84 RBX::DataModel::GearType * rbx::any_cast<RBX::DataModel::GearType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x45addc — __ZN3rbx8any_castIRN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::GearType & rbx::any_cast<RBX::DataModel::GearType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45addc() -> ! {
    todo!("0x45addc RBX::DataModel::GearType & rbx::any_cast<RBX::DataModel::GearType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x45aecc — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::resize(unsigned long,RBX::DataModel::GearType)")]
pub fn stub_0x45aecc() -> ! {
    todo!("0x45aecc std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::resize(unsigned long,RBX::DataModel::GearType)")
}

// 0x45af00 — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::push_back(RBX::DataModel::GearType const&)")]
pub fn stub_0x45af00() -> ! {
    todo!("0x45af00 std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::push_back(RBX::DataModel::GearType const&)")
}

// 0x45af28 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel8GearTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::GearType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::operator[](RBX::Name const* const&)")]
pub use crate::instance::stub_0x45af28 as stub_0x45af28;

// 0x45af80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
pub use crate::instance::stub_0x45af80 as stub_0x45af80;

// 0x45b034 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
pub fn stub_0x45b034() -> ! {
    todo!("0x45b034 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")
}

// 0x45b08c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
pub use crate::instance::stub_0x45b08c as stub_0x45b08c;

// 0x45b0f4 — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,RBX::DataModel::GearType const&)")]
pub fn stub_0x45b0f4() -> ! {
    todo!("0x45b0f4 std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,RBX::DataModel::GearType const&)")
}

// 0x45b1d8 — __ZNSt12_Vector_baseIN3RBX9DataModel8GearTypeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_allocate(unsigned long)")]
pub use crate::instance::stub_0x45b1d8 as stub_0x45b1d8;

// 0x45b1f0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel8GearTypeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DataModel::GearType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearType *,RBX::DataModel::GearType *>(RBX::DataModel::GearType *,RBX::DataModel::GearType *,RBX::DataModel::GearType *)")]
pub fn stub_0x45b1f0() -> ! {
    todo!("0x45b1f0 RBX::DataModel::GearType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearType *,RBX::DataModel::GearType *>(RBX::DataModel::GearType *,RBX::DataModel::GearType *,RBX::DataModel::GearType *)")
}

// 0x45b22c — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,unsigned long,RBX::DataModel::GearType const&)")]
pub fn stub_0x45b22c() -> ! {
    todo!("0x45b22c std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,unsigned long,RBX::DataModel::GearType const&)")
}

// 0x45b3bc — __ZN3rbx8any_castIN3RBX9DataModel16GearGenreSettingENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::GearGenreSetting * rbx::any_cast<RBX::DataModel::GearGenreSetting,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45b3bc() -> ! {
    todo!("0x45b3bc RBX::DataModel::GearGenreSetting * rbx::any_cast<RBX::DataModel::GearGenreSetting,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x45b414 — __ZN3rbx8any_castIRN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::GearGenreSetting & rbx::any_cast<RBX::DataModel::GearGenreSetting &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45b414() -> ! {
    todo!("0x45b414 RBX::DataModel::GearGenreSetting & rbx::any_cast<RBX::DataModel::GearGenreSetting &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x45b504 — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::resize(unsigned long,RBX::DataModel::GearGenreSetting)")]
pub fn stub_0x45b504() -> ! {
    todo!("0x45b504 std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::resize(unsigned long,RBX::DataModel::GearGenreSetting)")
}

// 0x45b538 — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::push_back(RBX::DataModel::GearGenreSetting const&)")]
pub fn stub_0x45b538() -> ! {
    todo!("0x45b538 std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::push_back(RBX::DataModel::GearGenreSetting const&)")
}
