// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x60698c..0x61bc78 | total filtered 10215, remaining 896->796 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 77 EA-sorted ascending next uncovered gap from 0x60698c

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x60698c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PoseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x60698c as stub_60698c;

// 0x6069a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PoseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x6069a4 as stub_6069a4;

// 0x607274 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Pose::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::instance::stub_0x607274 as stub_607274;

// 0x60740c — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_60740c() -> ! {
    todo!("0x60740c RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x60743c — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_60743c() -> ! {
    todo!("0x60743c RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x607558 — __ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_607558() -> ! {
    todo!("0x607558 RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x60763c — __ZN3RBX10Reflection11Call1HelperINS_4PoseEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Pose,void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Pose*,void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Pose,void (RBX::Pose::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::Pose*,void (RBX::Pose::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_60763c() -> ! {
    todo!("0x60763c RBX::Reflection::Call1Helper<RBX::Pose,void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Pose*,void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x607724 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Pose::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::instance::stub_0x607724 as stub_607724;

// 0x607828 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_607828() -> ! {
    todo!("0x607828 RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x6078dc — __ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_6078dc() -> ! {
    todo!("0x6078dc RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x607900 — __ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Pose*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Pose::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Pose*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Pose::*)(void),RBX::Reflection::Variant &)
pub fn stub_607900() -> ! {
    todo!("0x607900 RBX::Reflection::Call0Helper<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Pose*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),RBX::Reflection::Variant &)")
}

// 0x60948c — __ZN3RBX8Instance15queryTypedChildINS_10PVInstanceEEEPT_i
#[doc(alias = "RBX::PVInstance * RBX::Instance::queryTypedChild<RBX::PVInstance>(int)")]
pub fn stub_60948c() -> ! {
    todo!("0x60948c RBX::PVInstance * RBX::Instance::queryTypedChild<RBX::PVInstance>(int)")
}

// 0x609b58 — __ZNK3RBX8Instance12getTypedRootINS_10PVInstanceEEEPKT_v
#[doc(alias = "RBX::PVInstance const* RBX::Instance::getTypedRoot<RBX::PVInstance>(void)const")]
pub fn stub_609b58() -> ! {
    todo!("0x609b58 RBX::PVInstance const* RBX::Instance::getTypedRoot<RBX::PVInstance>(void)const")
}

// 0x60c3b0 — __ZN3RBX12RootInstance9insertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEb
#[doc(alias = "RBX::RootInstance::insertRaw(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool)")]
// was: RBX::RootInstance::insertRaw(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool)
pub fn stub_60c3b0() -> ! {
    todo!("0x60c3b0 RBX::RootInstance::insertRaw(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool)")
}

// 0x60c438 — __ZN3RBX12RootInstance15publicInsertRawERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_RS1_INS2_8weak_ptrINS_12PartInstanceEEESaISD_EEbb
#[doc(alias = "RBX::RootInstance::publicInsertRaw(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool,bool)")]
// was: RBX::RootInstance::publicInsertRaw(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool,bool)
pub fn stub_60c438() -> ! {
    todo!("0x60c438 RBX::RootInstance::publicInsertRaw(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,bool,bool)")
}

// 0x60c6d8 — __ZN3RBX12RootInstance12insertToTreeERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_b
#[doc(alias = "RBX::RootInstance::insertToTree(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,bool)")]
// was: RBX::RootInstance::insertToTree(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,bool)
pub fn stub_60c6d8() -> ! {
    todo!("0x60c6d8 RBX::RootInstance::insertToTree(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,bool)")
}

// 0x60c7dc — __ZN3RBX12RootInstance25insertRemoteCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EEPKN3G3D7Vector3E
#[doc(alias = "RBX::RootInstance::insertRemoteCharacterView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,G3D::Vector3 const*)")]
// was: RBX::RootInstance::insertRemoteCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,G3D::Vector3 const*)
pub fn stub_60c7dc() -> ! {
    todo!("0x60c7dc RBX::RootInstance::insertRemoteCharacterView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,G3D::Vector3 const*)")
}

// 0x60c86c — __ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE
#[doc(alias = "RBX::RootInstance::insertCharacterView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
// was: RBX::RootInstance::insertCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)
pub fn stub_60c86c() -> ! {
    todo!("0x60c86c RBX::RootInstance::insertCharacterView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")
}

// 0x60c94c — __ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb
#[doc(alias = "RBX::RootInstance::insertIdeView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)")]
// was: RBX::RootInstance::insertIdeView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)
pub fn stub_60c94c() -> ! {
    todo!("0x60c94c RBX::RootInstance::insertIdeView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,std::vector&<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)")
}

// 0x60ca5c — __ZN3RBX12RootInstance12insert3dViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_10PromptModeEbPKN3G3D7Vector3E
#[doc(alias = "RBX::RootInstance::insert3dView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::PromptMode,bool,G3D::Vector3 const*)")]
// was: RBX::RootInstance::insert3dView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::PromptMode,bool,G3D::Vector3 const*)
pub fn stub_60ca5c() -> ! {
    todo!("0x60ca5c RBX::RootInstance::insert3dView(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::PromptMode,bool,G3D::Vector3 const*)")
}

// 0x60cf60 — __ZN3RBX12RootInstance17doInsertInstancesERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_NS_10InsertModeENS_10PromptModeEPKN3G3D7Vector3EPS7_b
#[doc(alias = "RBX::RootInstance::doInsertInstances(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,RBX::InsertMode,RBX::PromptMode,G3D::Vector3 const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>*,bool)")]
// was: RBX::RootInstance::doInsertInstances(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,RBX::InsertMode,RBX::PromptMode,G3D::Vector3 const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>*,bool)
pub fn stub_60cf60() -> ! {
    todo!("0x60cf60 RBX::RootInstance::doInsertInstances(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,RBX::InsertMode,RBX::PromptMode,G3D::Vector3 const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>*,bool)")
}

// 0x60d564 — __ZN3RBX12RootInstance15insertInstancesERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEPS4_NS_10InsertModeENS_10PromptModeEPKN3G3D7Vector3EPS7_
#[doc(alias = "RBX::RootInstance::insertInstances(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,RBX::InsertMode,RBX::PromptMode,G3D::Vector3 const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>*)")]
// was: RBX::RootInstance::insertInstances(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::Instance*,RBX::InsertMode,RBX::PromptMode,G3D::Vector3 const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>*)
pub fn stub_60d564() -> ! {
    todo!("0x60d564 RBX::RootInstance::insertInstances(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&,RBX::Instance*,RBX::InsertMode,RBX::PromptMode,G3D::Vector3 const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>*)")
}

// 0x60d584 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf1IvS5_PS5_EENSD_5list2INS2_3argILi1EEENSD_5valueISH_EEEEEEET0_T_SR_SQ_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>)
pub fn stub_60d584() -> ! {
    todo!("0x60d584 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance*>>>)")
}

// 0x60d5d8 — __ZN3RBX15ServiceProvider6createINS_5TeamsEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Teams * RBX::ServiceProvider::create<RBX::Teams>(RBX::Instance const*)")]
pub use crate::instance::stub_0x60d5d8 as stub_60d5d8;

// 0x60d5f0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4TeamEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Team> RBX::Creatable<RBX::Instance>::create<RBX::Team>(void)")]
// was: boost::shared_ptr<RBX::Team> RBX::Creatable<RBX::Instance>::create<RBX::Team>(void)
pub use crate::instance::stub_0x60d5f0 as stub_60d5f0;

// 0x60d6a0 — __ZN3RBX15ServiceProvider6createINS_8LightingEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Lighting * RBX::ServiceProvider::create<RBX::Lighting>(RBX::Instance const*)")]
pub use crate::instance::stub_0x60d6a0 as stub_60d6a0;

// 0x60d6b8 — __ZN5boost10shared_ptrIN3RBX4TeamEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Team>::shared_ptr<RBX::Team,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Team>::shared_ptr<RBX::Team,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x60d6b8 as stub_60d6b8;

// 0x60d868 — __ZN5boost6detail12shared_countC2IPN3RBX4TeamENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x60d868 as stub_60d868;

// 0x60d970 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_60d970() -> ! {
    todo!("0x60d970 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x60d974 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_60d974() -> ! {
    todo!("0x60d974 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x60d978 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_60d978() -> ! {
    todo!("0x60d978 boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x60d998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x60d998 as stub_60d998;

// 0x60d9b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4TeamENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Team *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x60d9b0 as stub_60d9b0;

// 0x6104bc — __ZNK3RBX9ScreenGui12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::ScreenGui::askSetParent(RBX::Instance const*)const")]
pub fn stub_6104bc() -> ! {
    todo!("0x6104bc RBX::ScreenGui::askSetParent(RBX::Instance const*)const")
}

// 0x6105dc — __ZN3RBX9ScreenGui15render2dContextEPNS_5AdornEPKNS_8InstanceE
#[doc(alias = "RBX::ScreenGui::render2dContext(RBX::Adorn *,RBX::Instance const*)")]
pub fn stub_6105dc() -> ! {
    todo!("0x6105dc RBX::ScreenGui::render2dContext(RBX::Adorn *,RBX::Instance const*)")
}

// 0x610660 — __ZThn96_N3RBX9ScreenGui15render2dContextEPNS_5AdornEPKNS_8InstanceE
#[doc(alias = "non-virtual thunk to RBX::ScreenGui::render2dContext(RBX::Adorn *,RBX::Instance const*)")]
pub fn stub_610660() -> ! {
    todo!("0x610660 non-virtual thunk to RBX::ScreenGui::render2dContext(RBX::Adorn *,RBX::Instance const*)")
}

// 0x610700 — __ZN3RBX9ScreenGui17onDescendantAddedEPNS_8InstanceE
#[doc(alias = "RBX::ScreenGui::onDescendantAdded(RBX::Instance *)")]
pub fn stub_610700() -> ! {
    todo!("0x610700 RBX::ScreenGui::onDescendantAdded(RBX::Instance *)")
}

// 0x610900 — __ZN3RBX9ScreenGui20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ScreenGui::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::ScreenGui::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_610900() -> ! {
    todo!("0x610900 RBX::ScreenGui::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x612124 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_N3rbx7signals10connectionEESt10_Select1stIS8_ESt4lessIS2_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,rbx::signals::connection>> *)")]
pub fn stub_612124() -> ! {
    todo!("0x612124 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,rbx::signals::connection>> *)")
}

// 0x61214c — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_N3rbx7signals10connectionEESt10_Select1stIS8_ESt4lessIS2_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Instance * const,rbx::signals::connection>> *)")]
pub fn stub_61214c() -> ! {
    todo!("0x61214c std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Instance * const,rbx::signals::connection>> *)")
}

// 0x612528 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7GuiMainEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiMain> RBX::Creatable<RBX::Instance>::create<RBX::GuiMain>(void)")]
// was: boost::shared_ptr<RBX::GuiMain> RBX::Creatable<RBX::Instance>::create<RBX::GuiMain>(void)
pub fn stub_612528() -> ! {
    todo!("0x612528 rbx_core::SharedPtr<RBX::GuiMain> RBX::Creatable<RBX::Instance>::create<RBX::GuiMain>(void)")
}

// 0x6125d8 — __ZN5boost10shared_ptrIN3RBX7GuiMainEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiMain>::shared_ptr<RBX::GuiMain,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::GuiMain>::shared_ptr<RBX::GuiMain,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_6125d8() -> ! {
    todo!("0x6125d8 rbx_core::SharedPtr<RBX::GuiMain>::shared_ptr<RBX::GuiMain,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x612788 — __ZN5boost6detail12shared_countC2IPN3RBX7GuiMainENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_612788() -> ! {
    todo!("0x612788 boost::detail::shared_count::shared_count<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x612890 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiMainENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_612890() -> ! {
    todo!("0x612890 boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x612894 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiMainENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_612894() -> ! {
    todo!("0x612894 boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x612898 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiMainENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_612898() -> ! {
    todo!("0x612898 boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x6128b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiMainENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_6128b8() -> ! {
    todo!("0x6128b8 boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x6128d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiMainENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_6128d0() -> ! {
    todo!("0x6128d0 boost::detail::sp_counted_impl_pd<RBX::GuiMain *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x613ad8 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_N3rbx7signals10connectionEESt10_Select1stIS8_ESt4lessIS2_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_insert_unique(std::pair<RBX::Instance * const,rbx::signals::connection> const&)")]
pub fn stub_613ad8() -> ! {
    todo!("0x613ad8 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_insert_unique(std::pair<RBX::Instance * const,rbx::signals::connection> const&)")
}

// 0x613b40 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_N3rbx7signals10connectionEESt10_Select1stIS8_ESt4lessIS2_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,rbx::signals::connection> const&)")]
pub fn stub_613b40() -> ! {
    todo!("0x613b40 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,rbx::signals::connection> const&)")
}

// 0x613b8c — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_N3rbx7signals10connectionEESt10_Select1stIS8_ESt4lessIS2_ESaIS8_EE14_M_create_nodeERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_create_node(std::pair<RBX::Instance * const,rbx::signals::connection> const&)")]
pub fn stub_613b8c() -> ! {
    todo!("0x613b8c std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_create_node(std::pair<RBX::Instance * const,rbx::signals::connection> const&)")
}

// 0x614a00 — __ZN3RBX18ScriptMouseCommandC1EPNS_9WorkspaceE
#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")]
pub use crate::workspace::stub_0x614a00 as stub_614a00;

// 0x614a04 — __ZN3RBX18ScriptMouseCommandC2EPNS_9WorkspaceE
#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")]
pub use crate::workspace::stub_0x614a04 as stub_614a04;

// 0x615d98 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE12onChildAddedEPNS_8InstanceE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onChildAdded(RBX::Instance *)")]
pub fn stub_615d98() -> ! {
    todo!("0x615d98 RBX::SeatImpl<RBX::BasicPartInstance>::onChildAdded(RBX::Instance *)")
}

// 0x615f70 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE14onChildRemovedEPNS_8InstanceE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onChildRemoved(RBX::Instance *)")]
pub fn stub_615f70() -> ! {
    todo!("0x615f70 RBX::SeatImpl<RBX::BasicPartInstance>::onChildRemoved(RBX::Instance *)")
}

// 0x616550 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE15isChildSeatWeldEPNS_8InstanceE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::isChildSeatWeld(RBX::Instance *)")]
pub fn stub_616550() -> ! {
    todo!("0x616550 RBX::SeatImpl<RBX::BasicPartInstance>::isChildSeatWeld(RBX::Instance *)")
}

// 0x6168a0 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE16destroyOtherWeldEN5boost10shared_ptrINS_8InstanceEEEPNS_4WeldE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::destroyOtherWeld(rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)")]
// was: RBX::SeatImpl<RBX::BasicPartInstance>::destroyOtherWeld(boost::shared_ptr<RBX::Instance>,RBX::Weld *)
pub fn stub_6168a0() -> ! {
    todo!("0x6168a0 RBX::SeatImpl<RBX::BasicPartInstance>::destroyOtherWeld(rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)")
}

// 0x6168c8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEENS2_IPNS3_4WeldEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_6168c8() -> ! {
    todo!("0x6168c8 void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x6169a4 — __ZNK5boost4_mfi3mf2IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_4WeldEEclEPS5_S8_SA_
#[doc(alias = "boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)const")]
// was: boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,boost::shared_ptr<RBX::Instance>,RBX::Weld *)const
pub fn stub_6169a4() -> ! {
    todo!("0x6169a4 boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)const")
}

// 0x616a90 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>> const&)
pub fn stub_616a90() -> ! {
    todo!("0x616a90 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>> const&)")
}

// 0x616b04 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE19onEvent_seatTouchedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onEvent_seatTouched(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::SeatImpl<RBX::BasicPartInstance>::onEvent_seatTouched(boost::shared_ptr<RBX::Instance>)
pub fn stub_616b04() -> ! {
    todo!("0x616b04 RBX::SeatImpl<RBX::BasicPartInstance>::onEvent_seatTouched(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x6170d8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_6170d8() -> ! {
    todo!("0x6170d8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x617104 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_617104() -> ! {
    todo!("0x617104 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x6171d8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_6171d8() -> ! {
    todo!("0x6171d8 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x6171f4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_6171f4() -> ! {
    todo!("0x6171f4 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x617210 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS6_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_617210() -> ! {
    todo!("0x617210 void boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0x6172e8 — __ZNK5boost4_mfi3mf1IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
#[doc(alias = "boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_6172e8() -> ! {
    todo!("0x6172e8 boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x6173d0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_6173d0() -> ! {
    todo!("0x6173d0 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x6173fc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_6173fc() -> ! {
    todo!("0x6173fc rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x617738 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SeatEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Seat> RBX::Creatable<RBX::Instance>::create<RBX::Seat>(void)")]
// was: boost::shared_ptr<RBX::Seat> RBX::Creatable<RBX::Instance>::create<RBX::Seat>(void)
pub fn stub_617738() -> ! {
    todo!("0x617738 rbx_core::SharedPtr<RBX::Seat> RBX::Creatable<RBX::Instance>::create<RBX::Seat>(void)")
}

// 0x6177ec — __ZN5boost10shared_ptrIN3RBX4SeatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Seat>::shared_ptr<RBX::Seat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Seat>::shared_ptr<RBX::Seat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_6177ec() -> ! {
    todo!("0x6177ec rbx_core::SharedPtr<RBX::Seat>::shared_ptr<RBX::Seat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x61799c — __ZN5boost6detail12shared_countC2IPN3RBX4SeatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_61799c() -> ! {
    todo!("0x61799c boost::detail::shared_count::shared_count<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x617aa4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_617aa4() -> ! {
    todo!("0x617aa4 boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x617aa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_617aa8() -> ! {
    todo!("0x617aa8 boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x617aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_617aac() -> ! {
    todo!("0x617aac boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x617acc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_617acc() -> ! {
    todo!("0x617acc boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x617ae4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_617ae4() -> ! {
    todo!("0x617ae4 boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x619454 — __ZN3RBX9Selection17onAncestryChangedEPNS_8InstanceE
#[doc(alias = "RBX::Selection::onAncestryChanged(RBX::Instance *)")]
pub fn stub_619454() -> ! {
    todo!("0x619454 RBX::Selection::onAncestryChanged(RBX::Instance *)")
}

// 0x619474 — __ZN3RBX9Selection19removeFromSelectionEPNS_8InstanceE
#[doc(alias = "RBX::Selection::removeFromSelection(RBX::Instance *)")]
pub fn stub_619474() -> ! {
    todo!("0x619474 RBX::Selection::removeFromSelection(RBX::Instance *)")
}

// 0x6196e4 — __ZN3RBX9Selection7connectEPNS_8InstanceE
#[doc(alias = "RBX::Selection::connect(RBX::Instance *)")]
pub fn stub_6196e4() -> ! {
    todo!("0x6196e4 RBX::Selection::connect(RBX::Instance *)")
}

// 0x6197c4 — __ZN3RBX9Selection10disconnectEPNS_8InstanceE
#[doc(alias = "RBX::Selection::disconnect(RBX::Instance *)")]
pub fn stub_6197c4() -> ! {
    todo!("0x6197c4 RBX::Selection::disconnect(RBX::Instance *)")
}

// 0x6198ac — __ZN3RBX9Selection15toggleSelectionEPNS_8InstanceE
#[doc(alias = "RBX::Selection::toggleSelection(RBX::Instance *)")]
pub fn stub_6198ac() -> ! {
    todo!("0x6198ac RBX::Selection::toggleSelection(RBX::Instance *)")
}

// 0x619af8 — __ZN3RBX9Selection10raiseAddedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Selection::raiseAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Selection::raiseAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_619af8() -> ! {
    todo!("0x619af8 RBX::Selection::raiseAdded(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x619c9c — __ZN3RBX9Selection12raiseRemovedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Selection::raiseRemoved(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Selection::raiseRemoved(boost::shared_ptr<RBX::Instance>)
pub fn stub_619c9c() -> ! {
    todo!("0x619c9c RBX::Selection::raiseRemoved(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x619e4c — __ZN3RBX9Selection14addToSelectionEPNS_8InstanceE
#[doc(alias = "RBX::Selection::addToSelection(RBX::Instance *)")]
pub fn stub_619e4c() -> ! {
    todo!("0x619e4c RBX::Selection::addToSelection(RBX::Instance *)")
}

// 0x61a2b8 — __ZN3RBX9Selection12setSelectionEPNS_8InstanceE
#[doc(alias = "RBX::Selection::setSelection(RBX::Instance *)")]
pub fn stub_61a2b8() -> ! {
    todo!("0x61a2b8 RBX::Selection::setSelection(RBX::Instance *)")
}

// 0x61a5fc — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Selection,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_61a5fc() -> ! {
    todo!("0x61a5fc RBX::Reflection::BoundFuncDesc<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x61a620 — __ZN3RBX9Selection12setSelectionEN5boost10shared_ptrIKSt6vectorINS2_INS_8InstanceEEESaIS5_EEEE
#[doc(alias = "RBX::Selection::setSelection(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
// was: RBX::Selection::setSelection(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
pub fn stub_61a620() -> ! {
    todo!("0x61a620 RBX::Selection::setSelection(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")
}

// 0x61a630 — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::~BoundFuncDesc()
pub fn stub_61a630() -> ! {
    todo!("0x61a630 RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")
}

// 0x61a7d4 — __ZNSt3mapIPN3RBX8InstanceEN3rbx7signals10connectionESt4lessIS2_ESaISt4pairIKS2_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Instance *,rbx::signals::connection,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::operator[](RBX::Instance * const&)")]
pub fn stub_61a7d4() -> ! {
    todo!("0x61a7d4 std::map<RBX::Instance *,rbx::signals::connection,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::operator[](RBX::Instance * const&)")
}

// 0x61a8e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9SelectionEPS5_EENSA_5list2INSA_5valueIPSE_EENSI_ISF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>> const&)
pub fn stub_61a8e4() -> ! {
    todo!("0x61a8e4 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>> const&)")
}

// 0x61af48 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9SelectionEPS5_EENSA_5list2INSA_5valueIPSE_EENSI_ISF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>::~callable_slot()
pub fn stub_61af48() -> ! {
    todo!("0x61af48 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>::~callable_slot()")
}

// 0x61af74 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9SelectionEPS5_EENSA_5list2INSA_5valueIPSE_EENSI_ISF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>::~callable_slot()
pub fn stub_61af74() -> ! {
    todo!("0x61af74 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>>::~callable_slot()")
}

// 0x61b048 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9SelectionEPS6_EENSB_5list2INSB_5valueIPSF_EENSJ_ISG_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_61b048() -> ! {
    todo!("0x61b048 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x61b050 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9SelectionEPS6_EENSB_5list2INSB_5valueIPSF_EENSJ_ISG_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_61b050() -> ! {
    todo!("0x61b050 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x61b058 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionEPNS4_8InstanceEEENS0_5list2INS0_5valueIPS5_EENSA_IS7_EEEEEclINS_10shared_ptrIS6_EESI_EEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance *>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance *>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &,rbx_core::SharedPtr<RBX::Instance> &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance *>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance *>>>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> &,boost::shared_ptr<RBX::Instance> &)
pub fn stub_61b058() -> ! {
    todo!("0x61b058 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance *>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance *>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &,rbx_core::SharedPtr<RBX::Instance> &)")
}

// 0x61b070 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9SelectionEPS6_EENSB_5list2INSB_5valueIPSF_EENSJ_ISG_EEEEEELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_61b070() -> ! {
    todo!("0x61b070 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x61b09c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9SelectionEPS6_EENSB_5list2INSB_5valueIPSF_EENSJ_ISG_EEEEEELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_61b09c() -> ! {
    todo!("0x61b09c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::_bi::value<RBX::Instance*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x61b170 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_N3rbx7signals10connectionEESt10_Select1stIS8_ESt4lessIS2_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::pair<RBX::Instance * const,rbx::signals::connection> const&)")]
pub fn stub_61b170() -> ! {
    todo!("0x61b170 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,rbx::signals::connection>,std::_Select1st<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,rbx::signals::connection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,rbx::signals::connection>>,std::pair<RBX::Instance * const,rbx::signals::connection> const&)")
}

// 0x61bc78 — __ZN3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEC2ERKS7_
#[doc(alias = "RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::copy_on_write_ptr(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::copy_on_write_ptr(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_61bc78() -> ! {
    todo!("0x61bc78 RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::copy_on_write_ptr(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}