//! rendering shard 406 — 100 stubs 0x60b634..0x611200 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 43811->43911 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x60b634..0x611200 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x60b634 — __ZN3RBX12RootInstance21computeIdeInsertPointEv
// type: int __fastcall(RBX::RootInstance *this, float *)
#[doc(alias = "__ZN3RBX12RootInstance21computeIdeInsertPointEv")]
#[doc(alias = "RBX::RootInstance::computeIdeInsertPoint(void)")]
// was: __ZN3RBX12RootInstance21computeIdeInsertPointEv
pub fn stub_60b634() -> ! {
    todo!("0x60b634 RBX::RootInstance::computeIdeInsertPoint(void)")
}

// 0x60bc20 — __ZN3RBX12RootInstance17gatherPartExtentsERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBX12RootInstance17gatherPartExtentsERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE")]
#[doc(alias = "RBX::RootInstance::gatherPartExtents(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")]
// was: __ZN3RBX12RootInstance17gatherPartExtentsERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
pub fn stub_60bc20() -> ! {
    todo!("0x60bc20 RBX::RootInstance::gatherPartExtents(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")
}

// 0x60c24c — __ZN3RBX12RootInstance26moveToCharacterInsertPointERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX12RootInstance26moveToCharacterInsertPointERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE")]
#[doc(alias = "RBX::RootInstance::moveToCharacterInsertPoint(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")]
// was: __ZN3RBX12RootInstance26moveToCharacterInsertPointERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
pub fn stub_60c24c() -> ! {
    todo!("0x60c24c RBX::RootInstance::moveToCharacterInsertPoint(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")
}

// 0x60c3b0 — __ZN3RBX12RootInstance9insertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEb
// type: int __fastcall(RBX::Instance *this, int, RBX::Instance *, int, int)
#[doc(alias = "__ZN3RBX12RootInstance9insertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEb")]
#[doc(alias = "RBX::RootInstance::insertRaw(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool)")]
// was: __ZN3RBX12RootInstance9insertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEb
pub fn stub_60c3b0() -> ! {
    todo!("0x60c3b0 RBX::RootInstance::insertRaw(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool)")
}

// 0x60c438 — __ZN3RBX12RootInstance15publicInsertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEbb
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX12RootInstance15publicInsertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEbb")]
#[doc(alias = "RBX::RootInstance::publicInsertRaw(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool,bool)")]
// was: __ZN3RBX12RootInstance15publicInsertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEbb
pub fn stub_60c438() -> ! {
    todo!("0x60c438 RBX::RootInstance::publicInsertRaw(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool,bool)")
}

// 0x60c4c4 — __ZN3RBX12RootInstance18focusCameraOnPartsERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBX12RootInstance18focusCameraOnPartsERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE")]
#[doc(alias = "RBX::RootInstance::focusCameraOnParts(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")]
// was: __ZN3RBX12RootInstance18focusCameraOnPartsERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
pub fn stub_60c4c4() -> ! {
    todo!("0x60c4c4 RBX::RootInstance::focusCameraOnParts(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")
}

// 0x60c698 — __ZN3RBX12RootInstance22movePartsToCameraFocusERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "__ZN3RBX12RootInstance22movePartsToCameraFocusERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE")]
#[doc(alias = "RBX::RootInstance::movePartsToCameraFocus(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")]
// was: __ZN3RBX12RootInstance22movePartsToCameraFocusERSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
pub fn stub_60c698() -> ! {
    todo!("0x60c698 RBX::RootInstance::movePartsToCameraFocus(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> &)")
}

// 0x60c6d8 — __ZN3RBX12RootInstance12insertToTreeERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_b
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX12RootInstance12insertToTreeERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_b")]
#[doc(alias = "RBX::RootInstance::insertToTree(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,bool)")]
// was: __ZN3RBX12RootInstance12insertToTreeERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_b
pub fn stub_60c6d8() -> ! {
    todo!("0x60c6d8 RBX::RootInstance::insertToTree(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,bool)")
}

// 0x60c86c — __ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE
// type: int __fastcall(RBX::Instance *this, int, int)
#[doc(alias = "__ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE")]
#[doc(alias = "RBX::RootInstance::insertCharacterView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
// was: __ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE
pub fn stub_60c86c() -> ! {
    todo!("0x60c86c RBX::RootInstance::insertCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")
}

// 0x60c94c — __ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb
#[doc(alias = "__ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb")]
#[doc(alias = "RBX::RootInstance::insertIdeView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)")]
// was: __ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb
pub fn stub_60c94c() -> ! {
    todo!("0x60c94c RBX::RootInstance::insertIdeView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)")
}

// 0x60ccf0 — __ZN3RBX12RootInstance11insertDecalEPNS_5DecalE
// type: _DWORD __fastcall(RBX::RootInstance *__hidden this, RBX::Decal *)
#[doc(alias = "__ZN3RBX12RootInstance11insertDecalEPNS_5DecalE")]
#[doc(alias = "RBX::RootInstance::insertDecal(RBX::Decal *)")]
// was: __ZN3RBX12RootInstance11insertDecalEPNS_5DecalE
pub fn stub_60ccf0() -> ! {
    todo!("0x60ccf0 RBX::RootInstance::insertDecal(RBX::Decal *)")
}

// 0x60cd0c — __ZN3RBX12RootInstance19insertSpawnLocationEPNS_13SpawnLocationE
// type: _DWORD __fastcall(RBX::RootInstance *__hidden this, RBX::SpawnLocation *)
#[doc(alias = "__ZN3RBX12RootInstance19insertSpawnLocationEPNS_13SpawnLocationE")]
#[doc(alias = "RBX::RootInstance::insertSpawnLocation(RBX::SpawnLocation *)")]
// was: __ZN3RBX12RootInstance19insertSpawnLocationEPNS_13SpawnLocationE
pub fn stub_60cd0c() -> ! {
    todo!("0x60cd0c RBX::RootInstance::insertSpawnLocation(RBX::SpawnLocation *)")
}

// 0x60cf38 — __ZN3RBX12RootInstance15insertHopperBinEPNS_9HopperBinE
// type: _DWORD __fastcall(RBX::RootInstance *__hidden this, RBX::HopperBin *)
#[doc(alias = "__ZN3RBX12RootInstance15insertHopperBinEPNS_9HopperBinE")]
#[doc(alias = "RBX::RootInstance::insertHopperBin(RBX::HopperBin *)")]
// was: __ZN3RBX12RootInstance15insertHopperBinEPNS_9HopperBinE
pub fn stub_60cf38() -> ! {
    todo!("0x60cf38 RBX::RootInstance::insertHopperBin(RBX::HopperBin *)")
}

// 0x60d584 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf1IvS5_PS5_EENSD_5list2INS2_3argILi1EEENSD_5valueISH_EEEEEEET0_T_SR_SQ_
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf1IvS5_PS5_EENSD_5list2INS2_3argILi1EEENSD_5valueISH_EEEEEEET0_T_SR_SQ_")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf1IvS5_PS5_EENSD_5list2INS2_3argILi1EEENSD_5valueISH_EEEEEEET0_T_SR_SQ_
pub fn stub_60d584() -> ! {
    todo!("0x60d584 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>)")
}

// 0x60d5d8 — __ZN3RBX15ServiceProvider6createINS_5TeamsEEEPT_PKNS_8InstanceE
// type: int(void)
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_5TeamsEEEPT_PKNS_8InstanceE")]
#[doc(alias = "RBX::Teams * RBX::ServiceProvider::create<RBX::Teams>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_5TeamsEEEPT_PKNS_8InstanceE
pub fn stub_60d5d8() -> ! {
    todo!("0x60d5d8 RBX::Teams * RBX::ServiceProvider::create<RBX::Teams>(RBX::Instance const*)")
}

// 0x60d5f0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4TeamEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4TeamEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Team> RBX::Creatable<RBX::Instance>::create<RBX::Team>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4TeamEEEN5boost10shared_ptrIT_EEv
pub fn stub_60d5f0() -> ! {
    todo!("0x60d5f0 boost::shared_ptr<RBX::Team> RBX::Creatable<RBX::Instance>::create<RBX::Team>(void)")
}

// 0x60d6a0 — __ZN3RBX15ServiceProvider6createINS_8LightingEEEPT_PKNS_8InstanceE
// type: int(void)
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_8LightingEEEPT_PKNS_8InstanceE")]
#[doc(alias = "RBX::Lighting * RBX::ServiceProvider::create<RBX::Lighting>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_8LightingEEEPT_PKNS_8InstanceE
pub fn stub_60d6a0() -> ! {
    todo!("0x60d6a0 RBX::Lighting * RBX::ServiceProvider::create<RBX::Lighting>(RBX::Instance const*)")
}

// 0x60d6b8 — __ZN5boost10shared_ptrIN3RBX4TeamEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4TeamEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Team>::shared_ptr<RBX::Team,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4TeamEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_60d6b8() -> ! {
    todo!("0x60d6b8 boost::shared_ptr<RBX::Team>::shared_ptr<RBX::Team,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x60d780 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4TeamES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4TeamES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Team,RBX::Team>(rbx_core::SharedPtr<RBX::Team> const*,RBX::Team *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4TeamES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_60d780() -> ! {
    todo!("0x60d780 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Team,RBX::Team>(boost::shared_ptr<RBX::Team> const*,RBX::Team *)const")
}

// 0x60d868 — __ZN5boost6detail12shared_countC2IPN3RBX4TeamENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4TeamENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4TeamENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_60d868() -> ! {
    todo!("0x60d868 boost::detail::shared_count::shared_count<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x60d970 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_60d970() -> ! {
    todo!("0x60d970 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x60d974 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_60d974() -> ! {
    todo!("0x60d974 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x60d978 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_60d978() -> ! {
    todo!("0x60d978 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x60d998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_60d998() -> ! {
    todo!("0x60d998 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x60d9b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_60d9b0() -> ! {
    todo!("0x60d9b0 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x60d9b4 — __ZNSt8auto_ptrIN3RBX5WorldEED2Ev
#[doc(alias = "__ZNSt8auto_ptrIN3RBX5WorldEED2Ev")]
#[doc(alias = "std::auto_ptr<RBX::World>::~auto_ptr()")]
// was: __ZNSt8auto_ptrIN3RBX5WorldEED2Ev
pub fn stub_60d9b4() -> ! {
    todo!("0x60d9b4 std::auto_ptr<RBX::World>::~auto_ptr()")
}

// 0x60da5c — __ZN3RBX12ICameraOwnerD1Ev
// type: void __fastcall(RBX::ICameraOwner *__hidden this)
#[doc(alias = "__ZN3RBX12ICameraOwnerD1Ev")]
#[doc(alias = "RBX::ICameraOwner::~ICameraOwner()")]
// was: __ZN3RBX12ICameraOwnerD1Ev
pub fn stub_60da5c() -> ! {
    todo!("0x60da5c RBX::ICameraOwner::~ICameraOwner()")
}

// 0x60da60 — __ZN3RBX12ICameraOwnerD0Ev
// type: void __fastcall(RBX::ICameraOwner *__hidden this)
#[doc(alias = "__ZN3RBX12ICameraOwnerD0Ev")]
// was: __ZN3RBX12ICameraOwnerD0Ev
pub fn stub_60da60() -> ! {
    todo!("0x60da60 RBX::ICameraOwner::~ICameraOwner()")
}

// 0x60da64 — __GLOBAL__I_a_246
#[doc(alias = "__GLOBAL__I_a_246")]
#[doc(alias = "global constructor keyed to_a_246")]
// was: __GLOBAL__I_a_246
pub fn stub_60da64() -> ! {
    todo!("0x60da64 `global constructor keyed to'_a_246")
}

// 0x60ddb8 — __ZN3RBX8SafeChat9singletonEv
// type: _DWORD __fastcall(RBX::SafeChat *__hidden this)
#[doc(alias = "__ZN3RBX8SafeChat9singletonEv")]
#[doc(alias = "RBX::SafeChat::singleton(void)")]
// was: __ZN3RBX8SafeChat9singletonEv
pub fn stub_60ddb8() -> ! {
    todo!("0x60ddb8 RBX::SafeChat::singleton(void)")
}

// 0x60dddc — __ZL18SafeChat_singletonv
// type: _DWORD __fastcall()
#[doc(alias = "__ZL18SafeChat_singletonv")]
#[doc(alias = "SafeChat_singleton(void)")]
// was: __ZL18SafeChat_singletonv
pub fn stub_60dddc() -> ! {
    todo!("0x60dddc SafeChat_singleton(void)")
}

// 0x60deb4 — __ZN3RBX8SafeChat12loadChildrenEPNS_10ChatOptionEPK10XmlElement
// type: _DWORD __fastcall(RBX::SafeChat *__hidden this, RBX::ChatOption *, const XmlElement *)
#[doc(alias = "__ZN3RBX8SafeChat12loadChildrenEPNS_10ChatOptionEPK10XmlElement")]
#[doc(alias = "RBX::SafeChat::loadChildren(RBX::ChatOption *,XmlElement const*)")]
// was: __ZN3RBX8SafeChat12loadChildrenEPNS_10ChatOptionEPK10XmlElement
pub fn stub_60deb4() -> ! {
    todo!("0x60deb4 RBX::SafeChat::loadChildren(RBX::ChatOption *,XmlElement const*)")
}

// 0x60e178 — __ZN3RBX8SafeChat12loadChatTreeEv
// type: _DWORD __fastcall(RBX::SafeChat *__hidden this)
#[doc(alias = "__ZN3RBX8SafeChat12loadChatTreeEv")]
#[doc(alias = "RBX::SafeChat::loadChatTree(void)")]
// was: __ZN3RBX8SafeChat12loadChatTreeEv
pub fn stub_60e178() -> ! {
    todo!("0x60e178 RBX::SafeChat::loadChatTree(void)")
}

// 0x60e4e8 — __ZN3RBX8SafeChat10getMessageESt6vectorISsSaISsEE
// type: int __fastcall(std::string *this)
#[doc(alias = "__ZN3RBX8SafeChat10getMessageESt6vectorISsSaISsEE")]
#[doc(alias = "RBX::SafeChat::getMessage(std::vector<std::string,std::allocator<std::string>>)")]
// was: __ZN3RBX8SafeChat10getMessageESt6vectorISsSaISsEE
pub fn stub_60e4e8() -> ! {
    todo!("0x60e4e8 RBX::SafeChat::getMessage(std::vector<std::string,std::allocator<std::string>>)")
}

// 0x60e5f0 — __ZN3RBX10ChatOptionD2Ev
// type: void __fastcall(RBX::ChatOption *__hidden this)
#[doc(alias = "__ZN3RBX10ChatOptionD2Ev")]
#[doc(alias = "RBX::ChatOption::~ChatOption()")]
// was: __ZN3RBX10ChatOptionD2Ev
pub fn stub_60e5f0() -> ! {
    todo!("0x60e5f0 RBX::ChatOption::~ChatOption()")
}

// 0x60e6fc — __ZN5boost10scoped_ptrIN3RBX8SafeChatEED1Ev
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX8SafeChatEED1Ev")]
#[doc(alias = "boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX8SafeChatEED1Ev
pub fn stub_60e6fc() -> ! {
    todo!("0x60e6fc boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()")
}

// 0x60e700 — __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::push_back(RBX::ChatOption * const&)")]
// was: __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_
pub fn stub_60e700() -> ! {
    todo!("0x60e700 std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::push_back(RBX::ChatOption * const&)")
}

// 0x60e72c — __ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev")]
#[doc(alias = "boost::scoped_ptr<RBX::ChatOption>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev
pub fn stub_60e72c() -> ! {
    todo!("0x60e72c boost::scoped_ptr<RBX::ChatOption>::~scoped_ptr()")
}

// 0x60e7d4 — __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "__ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatOption **,std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>>,RBX::ChatOption * const&)")]
// was: __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_60e7d4() -> ! {
    todo!("0x60e7d4 std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatOption **,std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>>,RBX::ChatOption * const&)")
}

// 0x60e8b4 — __ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm
pub fn stub_60e8b4() -> ! {
    todo!("0x60e8b4 std::_Vector_base<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_allocate(unsigned long)")
}

// 0x60e8cc — __ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev")]
// was: __ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev
pub fn stub_60e8cc() -> ! {
    todo!("0x60e8cc boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()")
}

// 0x60e974 — __ZN3RBX10ChatOptionC2ESs
// type: int(void)
#[doc(alias = "__ZN3RBX10ChatOptionC2ESs")]
#[doc(alias = "RBX::ChatOption::ChatOption(std::string)")]
// was: __ZN3RBX10ChatOptionC2ESs
pub fn stub_60e974() -> ! {
    todo!("0x60e974 RBX::ChatOption::ChatOption(std::string)")
}

// 0x60ea58 — __GLOBAL__I_a_247
#[doc(alias = "__GLOBAL__I_a_247")]
#[doc(alias = "global constructor keyed to_a_247")]
// was: __GLOBAL__I_a_247
pub fn stub_60ea58() -> ! {
    todo!("0x60ea58 `global constructor keyed to'_a_247")
}

// 0x60ec58 — __ZN3RBX11Scale9Frame14setSlicePrefixESs
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX11Scale9Frame14setSlicePrefixESs")]
#[doc(alias = "RBX::Scale9Frame::setSlicePrefix(std::string)")]
// was: __ZN3RBX11Scale9Frame14setSlicePrefixESs
pub fn stub_60ec58() -> ! {
    todo!("0x60ec58 RBX::Scale9Frame::setSlicePrefix(std::string)")
}

// 0x60ec94 — __ZN3RBX11Scale9FrameC1Ev
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZN3RBX11Scale9FrameC1Ev")]
#[doc(alias = "RBX::Scale9Frame::Scale9Frame(void)")]
// was: __ZN3RBX11Scale9FrameC1Ev
pub fn stub_60ec94() -> ! {
    todo!("0x60ec94 RBX::Scale9Frame::Scale9Frame(void)")
}

// 0x60ec98 — __ZN3RBX11Scale9FrameC2Ev
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZN3RBX11Scale9FrameC2Ev")]
// was: __ZN3RBX11Scale9FrameC2Ev
pub fn stub_60ec98() -> ! {
    todo!("0x60ec98 RBX::Scale9Frame::Scale9Frame(void)")
}

// 0x60efc0 — __ZNK3RBX11Scale9Frame16getScaleEdgeSizeEv
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZNK3RBX11Scale9Frame16getScaleEdgeSizeEv")]
#[doc(alias = "RBX::Scale9Frame::getScaleEdgeSize(void)const")]
// was: __ZNK3RBX11Scale9Frame16getScaleEdgeSizeEv
pub fn stub_60efc0() -> ! {
    todo!("0x60efc0 RBX::Scale9Frame::getScaleEdgeSize(void)const")
}

// 0x60efec — __ZNK3RBX11Scale9Frame14getSlicePrefixEv
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZNK3RBX11Scale9Frame14getSlicePrefixEv")]
#[doc(alias = "RBX::Scale9Frame::getSlicePrefix(void)const")]
// was: __ZNK3RBX11Scale9Frame14getSlicePrefixEv
pub fn stub_60efec() -> ! {
    todo!("0x60efec RBX::Scale9Frame::getSlicePrefix(void)const")
}

// 0x60effc — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED1Ev
pub fn stub_60effc() -> ! {
    todo!("0x60effc RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::~PropDescriptor()")
}

// 0x60f020 — __ZN3RBX11Scale9FrameD1Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZN3RBX11Scale9FrameD1Ev")]
#[doc(alias = "RBX::Scale9Frame::~Scale9Frame()")]
// was: __ZN3RBX11Scale9FrameD1Ev
pub fn stub_60f020() -> ! {
    todo!("0x60f020 RBX::Scale9Frame::~Scale9Frame()")
}

// 0x60f128 — __ZN3RBX11Scale9FrameD0Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZN3RBX11Scale9FrameD0Ev")]
// was: __ZN3RBX11Scale9FrameD0Ev
pub fn stub_60f128() -> ! {
    todo!("0x60f128 RBX::Scale9Frame::~Scale9Frame()")
}

// 0x60f244 — __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv
pub fn stub_60f244() -> ! {
    todo!("0x60f244 __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv")
}

// 0x60f26c — __ZThn32_N3RBX11Scale9FrameD1Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZThn32_N3RBX11Scale9FrameD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Scale9Frame::~Scale9Frame()")]
// was: __ZThn32_N3RBX11Scale9FrameD1Ev
pub fn stub_60f26c() -> ! {
    todo!("0x60f26c non-virtual thunk to RBX::Scale9Frame::~Scale9Frame()")
}

// 0x60f374 — __ZThn32_N3RBX11Scale9FrameD0Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZThn32_N3RBX11Scale9FrameD0Ev")]
// was: __ZThn32_N3RBX11Scale9FrameD0Ev
pub fn stub_60f374() -> ! {
    todo!("0x60f374 non-virtual thunk to RBX::Scale9Frame::~Scale9Frame()")
}

// 0x60f490 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv
pub fn stub_60f490() -> ! {
    todo!("0x60f490 __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv")
}

// 0x60f4b8 — __ZThn36_N3RBX11Scale9FrameD1Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZThn36_N3RBX11Scale9FrameD1Ev")]
// was: __ZThn36_N3RBX11Scale9FrameD1Ev
pub fn stub_60f4b8() -> ! {
    todo!("0x60f4b8 non-virtual thunk to RBX::Scale9Frame::~Scale9Frame()")
}

// 0x60f5c0 — __ZThn36_N3RBX11Scale9FrameD0Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
#[doc(alias = "__ZThn36_N3RBX11Scale9FrameD0Ev")]
// was: __ZThn36_N3RBX11Scale9FrameD0Ev
pub fn stub_60f5c0() -> ! {
    todo!("0x60f5c0 non-virtual thunk to RBX::Scale9Frame::~Scale9Frame()")
}

// 0x60f6dc — __ZN3RBX4Name13callDoDeclareILZNS_12sScale9FrameEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sScale9FrameEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sScale9FrameEEEEvv
pub fn stub_60f6dc() -> ! {
    todo!("0x60f6dc __ZN3RBX4Name13callDoDeclareILZNS_12sScale9FrameEEEEvv")
}

// 0x60f6e0 — __ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v
pub fn stub_60f6e0() -> ! {
    todo!("0x60f6e0 __ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v")
}

// 0x60f7c0 — __ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_60f7c0() -> ! {
    todo!("0x60f7c0 __ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x60f7c4 — __ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_60f7c4() -> ! {
    todo!("0x60f7c4 __ZN3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x60f864 — __ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_60f864() -> ! {
    todo!("0x60f864 __ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x60f86c — __ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_60f86c() -> ! {
    todo!("0x60f86c __ZThn32_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x60f910 — __ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_60f910() -> ! {
    todo!("0x60f910 __ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x60f918 — __ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_60f918() -> ! {
    todo!("0x60f918 __ZThn36_N3RBX10Reflection9DescribedINS_11Scale9FrameELZNS_12sScale9FrameEENS_17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x60f9bc — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::PropDescriptor<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>(char const*,char const*,std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_60f9bc() -> ! {
    todo!("0x60f9bc RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::PropDescriptor<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>(char const*,char const*,std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x60fad0 — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED0Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED0Ev
pub fn stub_60fad0() -> ! {
    todo!("0x60fad0 RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::~PropDescriptor()")
}

// 0x60fafc — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
pub fn stub_60fafc() -> ! {
    todo!("0x60fafc RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::isReadOnly(void)const")
}

// 0x60fb00 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
pub fn stub_60fb00() -> ! {
    todo!("0x60fb00 RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::isWriteOnly(void)const")
}

// 0x60fb04 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_60fb04() -> ! {
    todo!("0x60fb04 RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x60fb2c — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
pub fn stub_60fb2c() -> ! {
    todo!("0x60fb2c RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x60fe04 — __GLOBAL__I_a_248
#[doc(alias = "__GLOBAL__I_a_248")]
#[doc(alias = "global constructor keyed to_a_248")]
// was: __GLOBAL__I_a_248
pub fn stub_60fe04() -> ! {
    todo!("0x60fe04 `global constructor keyed to'_a_248")
}

// 0x6100dc — __ZN3RBX9ScreenGuiC1Ev
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZN3RBX9ScreenGuiC1Ev")]
#[doc(alias = "RBX::ScreenGui::ScreenGui(void)")]
// was: __ZN3RBX9ScreenGuiC1Ev
pub fn stub_6100dc() -> ! {
    todo!("0x6100dc RBX::ScreenGui::ScreenGui(void)")
}

// 0x6100e0 — __ZN3RBX9ScreenGuiC2Ev
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZN3RBX9ScreenGuiC2Ev")]
// was: __ZN3RBX9ScreenGuiC2Ev
pub fn stub_6100e0() -> ! {
    todo!("0x6100e0 RBX::ScreenGui::ScreenGui(void)")
}

// 0x6102ac — __ZN3RBX9ScreenGuiC2EPKc
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, const char *)
#[doc(alias = "__ZN3RBX9ScreenGuiC2EPKc")]
#[doc(alias = "RBX::ScreenGui::ScreenGui(char const*)")]
// was: __ZN3RBX9ScreenGuiC2EPKc
pub fn stub_6102ac() -> ! {
    todo!("0x6102ac RBX::ScreenGui::ScreenGui(char const*)")
}

// 0x61044c — __ZN3RBX9ScreenGui17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "__ZN3RBX9ScreenGui17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::ScreenGui::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX9ScreenGui17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_61044c() -> ! {
    todo!("0x61044c RBX::ScreenGui::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x6104bc — __ZNK3RBX9ScreenGui12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX9ScreenGui12askSetParentEPKNS_8InstanceE")]
#[doc(alias = "RBX::ScreenGui::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX9ScreenGui12askSetParentEPKNS_8InstanceE
pub fn stub_6104bc() -> ! {
    todo!("0x6104bc RBX::ScreenGui::askSetParent(RBX::Instance const*)const")
}

// 0x6104fc — __ZN3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "__ZN3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "RBX::ScreenGui::onHeartbeat(RBX::Heartbeat const&)")]
// was: __ZN3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE
pub fn stub_6104fc() -> ! {
    todo!("0x6104fc RBX::ScreenGui::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x610524 — __ZThn168_N3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "__ZThn168_N3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "non-virtual thunk to RBX::ScreenGui::onHeartbeat(RBX::Heartbeat const&)")]
// was: __ZThn168_N3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE
pub fn stub_610524() -> ! {
    todo!("0x610524 non-virtual thunk to RBX::ScreenGui::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x610588 — __ZNK3RBX9ScreenGui26canProcessMeAndDescendantsEv
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZNK3RBX9ScreenGui26canProcessMeAndDescendantsEv")]
#[doc(alias = "RBX::ScreenGui::canProcessMeAndDescendants(void)const")]
// was: __ZNK3RBX9ScreenGui26canProcessMeAndDescendantsEv
pub fn stub_610588() -> ! {
    todo!("0x610588 RBX::ScreenGui::canProcessMeAndDescendants(void)const")
}

// 0x61058c — __ZN3RBX9ScreenGui17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "__ZN3RBX9ScreenGui17onAncestorChangedERKNS_15AncestorChangedE")]
#[doc(alias = "RBX::ScreenGui::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX9ScreenGui17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_61058c() -> ! {
    todo!("0x61058c RBX::ScreenGui::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x610668 — __ZN3RBX9ScreenGui7processERKNS_8GuiEventE
#[doc(alias = "__ZN3RBX9ScreenGui7processERKNS_8GuiEventE")]
#[doc(alias = "RBX::ScreenGui::process(RBX::GuiEvent const&)")]
// was: __ZN3RBX9ScreenGui7processERKNS_8GuiEventE
pub fn stub_610668() -> ! {
    todo!("0x610668 RBX::ScreenGui::process(RBX::GuiEvent const&)")
}

// 0x610674 — __ZThn92_N3RBX9ScreenGui7processERKNS_8GuiEventE
#[doc(alias = "__ZThn92_N3RBX9ScreenGui7processERKNS_8GuiEventE")]
#[doc(alias = "non-virtual thunk to RBX::ScreenGui::process(RBX::GuiEvent const&)")]
// was: __ZThn92_N3RBX9ScreenGui7processERKNS_8GuiEventE
pub fn stub_610674() -> ! {
    todo!("0x610674 non-virtual thunk to RBX::ScreenGui::process(RBX::GuiEvent const&)")
}

// 0x610680 — __ZN3RBX9ScreenGui17removeModalButtonEPNS_9GuiButtonE
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, RBX::GuiButton *)
#[doc(alias = "__ZN3RBX9ScreenGui17removeModalButtonEPNS_9GuiButtonE")]
#[doc(alias = "RBX::ScreenGui::removeModalButton(RBX::GuiButton *)")]
// was: __ZN3RBX9ScreenGui17removeModalButtonEPNS_9GuiButtonE
pub fn stub_610680() -> ! {
    todo!("0x610680 RBX::ScreenGui::removeModalButton(RBX::GuiButton *)")
}

// 0x6106bc — __ZN3RBX9ScreenGui17insertModalButtonEPNS_9GuiButtonE
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, RBX::GuiButton *)
#[doc(alias = "__ZN3RBX9ScreenGui17insertModalButtonEPNS_9GuiButtonE")]
#[doc(alias = "RBX::ScreenGui::insertModalButton(RBX::GuiButton *)")]
// was: __ZN3RBX9ScreenGui17insertModalButtonEPNS_9GuiButtonE
pub fn stub_6106bc() -> ! {
    todo!("0x6106bc RBX::ScreenGui::insertModalButton(RBX::GuiButton *)")
}

// 0x6106ec — __ZN3RBX9ScreenGui20onModalButtonChangedEPKNS_10Reflection18PropertyDescriptorEPNS_9GuiButtonE
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, const RBX::Reflection::PropertyDescriptor *, RBX::GuiButton *)
#[doc(alias = "__ZN3RBX9ScreenGui20onModalButtonChangedEPKNS_10Reflection18PropertyDescriptorEPNS_9GuiButtonE")]
#[doc(alias = "RBX::ScreenGui::onModalButtonChanged(RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *)")]
// was: __ZN3RBX9ScreenGui20onModalButtonChangedEPKNS_10Reflection18PropertyDescriptorEPNS_9GuiButtonE
pub fn stub_6106ec() -> ! {
    todo!("0x6106ec RBX::ScreenGui::onModalButtonChanged(RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *)")
}

// 0x610700 — __ZN3RBX9ScreenGui17onDescendantAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, RBX::Instance *)
#[doc(alias = "__ZN3RBX9ScreenGui17onDescendantAddedEPNS_8InstanceE")]
#[doc(alias = "RBX::ScreenGui::onDescendantAdded(RBX::Instance *)")]
// was: __ZN3RBX9ScreenGui17onDescendantAddedEPNS_8InstanceE
pub fn stub_610700() -> ! {
    todo!("0x610700 RBX::ScreenGui::onDescendantAdded(RBX::Instance *)")
}

// 0x610900 — __ZN3RBX9ScreenGui20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX9ScreenGui20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::ScreenGui::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX9ScreenGui20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_610900() -> ! {
    todo!("0x610900 RBX::ScreenGui::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x6109a4 — __ZN3RBX9ScreenGui14hasModalDialogEv
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZN3RBX9ScreenGui14hasModalDialogEv")]
#[doc(alias = "RBX::ScreenGui::hasModalDialog(void)")]
// was: __ZN3RBX9ScreenGui14hasModalDialogEv
pub fn stub_6109a4() -> ! {
    todo!("0x6109a4 RBX::ScreenGui::hasModalDialog(void)")
}

// 0x6109cc — __ZN3RBX7GuiMainC2Ev
// type: _DWORD __fastcall(RBX::GuiMain *__hidden this)
#[doc(alias = "__ZN3RBX7GuiMainC2Ev")]
#[doc(alias = "RBX::GuiMain::GuiMain(void)")]
// was: __ZN3RBX7GuiMainC2Ev
pub fn stub_6109cc() -> ! {
    todo!("0x6109cc RBX::GuiMain::GuiMain(void)")
}

// 0x610cac — __ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::push_back(RBX::GuiButton * const&)")]
// was: __ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE9push_backERKS2_
pub fn stub_610cac() -> ! {
    todo!("0x610cac std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::push_back(RBX::GuiButton * const&)")
}

// 0x610cd8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf2IvNS2_9ScreenGuiES6_PNS2_9GuiButtonEEENSB_5list3INSB_5valueIPSF_EENSA_3argILi1EEENSK_ISH_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf2IvNS2_9ScreenGuiES6_PNS2_9GuiButtonEEENSB_5list3INSB_5valueIPSF_EENSA_3argILi1EEENSK_ISH_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>> const&)")]
// was: __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf2IvNS2_9ScreenGuiES6_PNS2_9GuiButtonEEENSB_5list3INSB_5valueIPSF_EENSA_3argILi1EEENSK_ISH_EEEEEEEENS0_10connectionERKT_
pub fn stub_610cd8() -> ! {
    todo!("0x610cd8 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>> const&)")
}

// 0x610d4c — __ZN3RBX9ScreenGuiD1Ev
// type: void __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZN3RBX9ScreenGuiD1Ev")]
#[doc(alias = "RBX::ScreenGui::~ScreenGui()")]
// was: __ZN3RBX9ScreenGuiD1Ev
pub fn stub_610d4c() -> ! {
    todo!("0x610d4c RBX::ScreenGui::~ScreenGui()")
}

// 0x610e94 — __ZN3RBX9ScreenGuiD0Ev
// type: void __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZN3RBX9ScreenGuiD0Ev")]
// was: __ZN3RBX9ScreenGuiD0Ev
pub fn stub_610e94() -> ! {
    todo!("0x610e94 RBX::ScreenGui::~ScreenGui()")
}

// 0x610f34 — __ZN3RBX9ScreenGui17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX9ScreenGui17onServiceProviderEPNS_15ServiceProviderES2_")]
#[doc(alias = "RBX::ScreenGui::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX9ScreenGui17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_610f34() -> ! {
    todo!("0x610f34 RBX::ScreenGui::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x610f3c — __ZNK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv
pub fn stub_610f3c() -> ! {
    todo!("0x610f3c __ZNK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv")
}

// 0x610f54 — __ZThn32_N3RBX9ScreenGuiD1Ev
// type: void __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9ScreenGuiD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::ScreenGui::~ScreenGui()")]
// was: __ZThn32_N3RBX9ScreenGuiD1Ev
pub fn stub_610f54() -> ! {
    todo!("0x610f54 non-virtual thunk to RBX::ScreenGui::~ScreenGui()")
}

// 0x611098 — __ZThn32_N3RBX9ScreenGuiD0Ev
// type: void __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9ScreenGuiD0Ev")]
// was: __ZThn32_N3RBX9ScreenGuiD0Ev
pub fn stub_611098() -> ! {
    todo!("0x611098 non-virtual thunk to RBX::ScreenGui::~ScreenGui()")
}

// 0x6111f0 — __ZThn32_NK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv
pub fn stub_6111f0() -> ! {
    todo!("0x6111f0 __ZThn32_NK3RBX14FactoryProductINS_9ScreenGuiENS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEE12getClassNameEv")
}

// 0x611200 — __ZThn36_N3RBX9ScreenGuiD1Ev
// type: void __fastcall(RBX::ScreenGui *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9ScreenGuiD1Ev")]
// was: __ZThn36_N3RBX9ScreenGuiD1Ev
pub fn stub_611200() -> ! {
    todo!("0x611200 non-virtual thunk to RBX::ScreenGui::~ScreenGui()")
}
