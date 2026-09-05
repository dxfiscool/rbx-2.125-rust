// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted, skip global dedup
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xf43ac4..0xf5a6a4 | total filtered 10215, pending before 641, remaining 541 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; WeakPtr = rbx_core::WeakPtr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf43ac4 — j___ZN3RBX10Reflection11Call1HelperINS_16KeyframeSequenceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequence,void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::KeyframeSequence*,void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_16KeyframeSequenceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")]
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequence,void (RBX::KeyframeSequence::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::KeyframeSequence*,void (RBX::KeyframeSequence::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5b9454`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5b9454 as stub_0xf43ac4;

// 0xf43ad4 — j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5b953c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5b953c as stub_0xf43ad4;

// 0xf43ae4 — j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5b9224`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5b9224 as stub_0xf43ae4;

// 0xf43af4 — j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::KeyframeSequence::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5b908c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5b908c as stub_0xf43af4;

// 0xf43ba4 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX16KeyframeSequenceEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEEPSC_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX16KeyframeSequenceEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEEPSC_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf43ba4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf43ba4 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf43bb4 — j___ZN5boost3_bi5list3INS0_5valueIPKN3RBX16KeyframeSequenceEEENS_3argILi1EEENS2_IPSt6vectorIPNS3_10CachedPoseESaISC_EEEEEclINS_4_mfi4cmf2IvS4_RKNS_10shared_ptrINS3_8InstanceEEESF_EENS0_5list1ISP_EEEEvNS0_4typeIvEERT_RT0_i
// type: int()
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIPKN3RBX16KeyframeSequenceEEENS_3argILi1EEENS2_IPSt6vectorIPNS3_10CachedPoseESaISC_EEEEEclINS_4_mfi4cmf2IvS4_RKNS_10shared_ptrINS3_8InstanceEEESF_EENS0_5list1ISP_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf43bb4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf43bb4 void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf43bc4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi4cmf1IvN3RBX16KeyframeSequenceERKNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPKS5_EENS_3argILi1EEEEEEclIS8_EEvRKT_
// type: int()
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi4cmf1IvN3RBX16KeyframeSequenceERKNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPKS5_EENS_3argILi1EEEEEEclIS8_EEvRKT_")]
// was: void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xf43bc4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf43bc4 void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf43c04 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const
pub fn stub_0xf43c04() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf43c04 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")
}

// 0xf43c14 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const
pub fn stub_0xf43c14() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf43c14 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")
}

// 0xf43ec4 — j___ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEES7_S6_E4callEPS2_S9_RNS0_7VariantERKS7_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEES7_S6_E4callEPS2_S9_RNS0_7VariantERKS7_")]
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,boost::shared_ptr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,boost::shared_ptr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,boost::shared_ptr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)
pub fn stub_0xf43ec4() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf43ec4 RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")
}

// 0xf43ed4 — j___ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_")]
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xf43ed4() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf43ed4 RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf43ee4 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,boost::shared_ptr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5bf888`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5bf888 as stub_0xf43ee4;

// 0xf43ef4 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EEC2EMS2_FS6_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EEC2EMS2_FS6_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,boost::shared_ptr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5bf710`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5bf710 as stub_0xf43ef4;

// 0xf43f04 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5bfde8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5bfde8 as stub_0xf43f04;

// 0xf43f14 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FS3_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FS3_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5bfc50`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5bfc50 as stub_0xf43f14;

// 0xf43fa4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_16KeyframeSequenceEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_16KeyframeSequenceEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5bbe64`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5bbe64 as stub_0xf43fa4;

// 0xf43ff4 — j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5be9f8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5be9f8 as stub_0xf43ff4;

// 0xf44034 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::operator()<void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0xf44034() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf44034 void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0xf440a4 — j___ZN5boost6detail12shared_countC2IPN3RBX16KeyframeSequenceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX16KeyframeSequenceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5beba8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5beba8 as stub_0xf440a4;

// 0xf440f4 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>)
pub fn stub_0xf440f4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf440f4 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)")
}

// 0xf44254 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf44254() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf44254 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf44264 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf44264() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf44264 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf44274 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf44274() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf44274 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf44614 — j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_3SkyEEEPKT_v
// type: int()
#[doc(alias = "RBX::Sky const* RBX::Instance::findConstFirstChildOfType<RBX::Sky>(void)const")]
#[doc(alias = "j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_3SkyEEEPKT_v")]
pub fn stub_0xf44614(parent: *const crate::generated_05::Instance) -> Option<SharedPtr<crate::generated_05::Instance>> {
    // `findConstFirstChildOfType<Sky>` — `j__` thunk to the template
    // body; the body is the standard child scan (null parent returns
    // null, else first `isA Sky` hit wins, miss returns null), same
    // shape as 0xf40fa4 (`ScreenGui`) and 0xaa7a00 (`StarterGear`).
    // The jump collapses into the direct implementation.
    // SAFETY: `parent` must be null or point to a valid `Instance`.
    use crate::generated_05::instance_is_a;
    if parent.is_null() {
        return None;
    }
    unsafe {
        let children: &[SharedPtr<crate::generated_05::Instance>] = &(*parent).children;
        for child in children.iter() {
            if instance_is_a(SharedPtr::as_ptr(child), "Sky") {
                return Some(child.clone());
            }
        }
        None
    }
}

// 0xf44734 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5c9708`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5c9708 as stub_0xf44734;

// 0xf44744 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ca804`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ca804 as stub_0xf44744;

// 0xf44754 — j___ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5c9f04`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5c9f04 as stub_0xf44754;

// 0xf44764 — j___ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ca8b4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ca8b4 as stub_0xf44764;

// 0xf44774 — j___ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ca0b4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ca0b4 as stub_0xf44774;

// 0xf44784 — j___ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5caa64`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5caa64 as stub_0xf44784;

// 0xf44884 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3G3D7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_ENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3G3D7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_ENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf44884() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf44884 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf44894 — j___ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf44894() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf44894 void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448a4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3G3D15CoordinateFrameEEENS4_IPNS5_7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_PKS9_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3G3D15CoordinateFrameEEENS4_IPNS5_7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_PKS9_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf448a4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf448a4 void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448b4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf448b4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf448b4 void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448c4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEENS4_IKN3G3D15CoordinateFrameEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_RSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEENS4_IKN3G3D15CoordinateFrameEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_RSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf448c4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf448c4 void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448d4 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0xf448d4() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf448d4 RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf448e4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D15CoordinateFrameEPKNS7_7Vector3EENS3_5list3INS2_3argILi1EEENS3_5valueIPS8_EENSJ_IPSB_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D15CoordinateFrameEPKNS7_7Vector3EENS3_5list3INS2_3argILi1EEENS3_5valueIPS8_EENSJ_IPSB_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const
pub fn stub_0xf448e4() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf448e4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const")
}

// 0xf448f4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D7Vector3EENS3_5list2INS2_3argILi1EEENS3_5valueIPS8_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D7Vector3EENS3_5list2INS2_3argILi1EEENS3_5valueIPS8_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const
pub fn stub_0xf448f4() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf448f4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const")
}

// 0xf44904 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const
pub fn stub_0xf44904() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf44904 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const")
}

// 0xf44914 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const
pub fn stub_0xf44914() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf44914 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const")
}

// 0xf44924 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsERKN3G3D15CoordinateFrameEENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS7_EENSI_ISB_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsERKN3G3D15CoordinateFrameEENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS7_EENSI_ISB_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const
pub fn stub_0xf44924() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf44924 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const")
}

// 0xf44a14 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0xf44a14() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf44a14 RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf44a24 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0xf44a24() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf44a24 RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf44aa4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4PART5WedgeEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4PART5WedgeEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5d7738`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5d7738 as stub_0xf44aa4;

// 0xf44ab4 — j___ZN5boost10shared_ptrIN3RBX4PART5WedgeEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX4PART5WedgeEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5d77ec`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5d77ec as stub_0xf44ab4;

// 0xf44ac4 — j___ZN5boost6detail12shared_countC2IPN3RBX4PART5WedgeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX4PART5WedgeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5d799c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5d799c as stub_0xf44ac4;

// 0xf44b64 — j___ZN3RBX10Reflection11Call1HelperINS_12PartInstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbEbSB_E4callEPS2_SD_RNS0_7VariantERKb
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_12PartInstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbEbSB_E4callEPS2_SD_RNS0_7VariantERKb")]
// was: RBX::Reflection::Call1Helper<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::PartInstance*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)
pub fn stub_0xf44b64() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf44b64 RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")
}

// 0xf44b84 — j___ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5f15e0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5f15e0 as stub_0xf44b84;

// 0xf44b94 — j___ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EEC2EMS2_FSB_bEPKcSH_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EEC2EMS2_FSB_bEPKcSH_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5f1434`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5f1434 as stub_0xf44b94;

// 0xf44e34 — j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ebfc4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ebfc4 as stub_0xf44e34;

// 0xf44e44 — j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ea1cc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ea1cc as stub_0xf44e44;

// 0xf44e54 — j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ebe34`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ebe34 as stub_0xf44e54;

// 0xf44ec4 — j___ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")]
#[doc(alias = "j___ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_")]
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,RBX::PartInstance*)
pub fn stub_0xf44ec4() -> ! {
    // BLOCKED: core-owned `TouchedSignal`/`boost::bind` slot infra (no exact-EA native anywhere)
    todo!("0xf44ec4 RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")
}

// 0xf44ef4 — j___ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "j___ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xf44ef4() -> ! {
    // BLOCKED: core-owned `TouchedSignal`/`boost::bind` slot infra (no exact-EA native anywhere)
    todo!("0xf44ef4 RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf44f04 — j___ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
#[doc(alias = "j___ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_0xf44f04() -> ! {
    // BLOCKED: core-owned `TouchedSignal`/`boost::bind` slot infra (no exact-EA native anywhere)
    todo!("0xf44f04 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")
}

// 0xf44f14 — j___ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "j___ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::PartInstance::TouchedSignal::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xf44f14() -> ! {
    // BLOCKED: core-owned `TouchedSignal`/`boost::bind` slot infra (no exact-EA native anywhere)
    todo!("0xf44f14 RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf450e4 — j___ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")]
#[doc(alias = "j___ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_")]
// was: void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>)
pub fn stub_0xf450e4() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf450e4 void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")
}

// 0xf450f4 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_S6_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,RBX::Primitive *)")]
#[doc(alias = "j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_S6_")]
// was: void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,RBX::Primitive *)
pub fn stub_0xf450f4() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf450f4 void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,RBX::Primitive *)")
}

// 0xf45174 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5e0ff8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5e0ff8 as stub_0xf45174;

// 0xf45274 — j___ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv
// type: int()
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::singleton(void)
pub fn stub_0xf45274() -> ! {
    // BLOCKED: core-owned `rbx::implementation::typed_holder` infra (no exact-EA native in core)
    todo!("0xf45274 rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")
}

// 0xf452b4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0xf452b4() -> ! {
    // BLOCKED: core-owned `rbx::signals` slot infra (no exact-EA native anywhere)
    todo!("0xf452b4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0xf452c4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_0xf452c4() -> ! {
    // BLOCKED: core-owned `rbx::signals` slot infra (no exact-EA native anywhere)
    todo!("0xf452c4 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")
}

// 0xf452d4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)
pub fn stub_0xf452d4() -> ! {
    // BLOCKED: core-owned `rbx::signals` slot infra (no exact-EA native anywhere)
    todo!("0xf452d4 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")
}

// 0xf452e4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5ec580`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5ec580 as stub_0xf452e4;

// 0xf452f4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::flogPrint(void)
pub fn stub_0xf452f4() -> ! {
    // BLOCKED: core-owned `rbx::signals` dispatch infra (no exact-EA native anywhere)
    todo!("0xf452f4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")
}

// 0xf45404 — j___ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: boost::shared_ptr<RBX::Instance> const& rbx::any_cast<boost::shared_ptr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_0xf45404() -> ! {
    // BLOCKED: core-owned `rbx::placement_any<RBX::Region3>` holder infra (no exact-EA native in core)
    todo!("0xf45404 rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf45414 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xf45414() -> ! {
    // BLOCKED: core-owned `rbx::signals` slot infra (no exact-EA native anywhere)
    todo!("0xf45414 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf47424 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>)")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEEvT_")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>)
pub fn stub_0xf47424() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47424 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>)")
}

// 0xf47484 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0xf47484() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf47484 RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf47494 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0xf47494() -> ! {
    // BLOCKED: reflection descriptor/dispatch glue (no exact-EA native anywhere)
    todo!("0xf47494 RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf47504 — j___ZNK5boost4_mfi3mf1IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "j___ZNK5boost4_mfi3mf1IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_")]
// was: boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0xf47504() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47504 boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf47514 — j___ZNK5boost4_mfi3mf2IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_7Motor6DEEclEPS5_S8_SA_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *)const")]
#[doc(alias = "j___ZNK5boost4_mfi3mf2IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_7Motor6DEEclEPS5_S8_SA_")]
// was: boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Motor6D *>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,boost::shared_ptr<RBX::Instance>,RBX::Motor6D *)const
pub fn stub_0xf47514() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47514 boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *)const")
}

// 0xf47524 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf47524() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47524 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf47534 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf47534() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47534 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf47544 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf47544() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47544 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ModelInstance>,rbx_core::WeakPtr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf476a4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::Creatable<RBX::Instance>::create<RBX::Sky>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Sky> RBX::Creatable<RBX::Instance>::create<RBX::Sky>(void)
pub fn stub_0xf476a4() -> SharedPtr<crate::instance::Sky> {
    // IDA 0xf476a4: `Creatable::create<Sky>` — `operator new` + default ctor
    // + adoption; same collapse as 0x4f0004 (`Fire`).
    SharedPtr::new(crate::instance::Sky::default())
}

// 0xf476b4 — j___ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf476b4() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf476b4 rbx_core::SharedPtr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf476c4 — j___ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0xf476c4() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf476c4 boost::detail::shared_count::shared_count<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf47764 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Smoke> RBX::Creatable<RBX::Instance>::create<RBX::Smoke>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Smoke> RBX::Creatable<RBX::Instance>::create<RBX::Smoke>(void)
pub fn stub_0xf47764() -> SharedPtr<crate::instance::Smoke> {
    // IDA 0xf47764: `Creatable::create<Smoke>` — `operator new` + default ctor
    // + adoption; same collapse as 0x4f0004 (`Fire`).
    SharedPtr::new(crate::instance::Smoke::default())
}

// 0xf47774 — j___ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Smoke>::shared_ptr<RBX::Smoke,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Smoke>::shared_ptr<RBX::Smoke,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf47774() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf47774 rbx_core::SharedPtr<RBX::Smoke>::shared_ptr<RBX::Smoke,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf47784 — j___ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0xf47784() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf47784 boost::detail::shared_count::shared_count<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf47974 — j___ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13SpawnLocationENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// type: int __fastcall(int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>)")]
#[doc(alias = "j___ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13SpawnLocationENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>)
pub fn stub_0xf47974() -> ! {
    // BLOCKED: core-owned `TouchedSignal`/`boost::bind` slot infra (no exact-EA native anywhere)
    todo!("0xf47974 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>)")
}

// 0xf479c4 — j___ZN3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_PKNS_8InstanceE
// type: int()
#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(RBX::Instance const*)")]
#[doc(alias = "j___ZN3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_0xf479c4(instance: *const crate::generated_05::Instance) -> Option<SharedPtr<crate::instance::DebrisService>> {
    // `ServiceProvider::create<DebrisService>` — provider lookup, null
    // yields empty, else default-construct + adopt. Same shape as 0x545740
    // (`CoreGuiService`).
    // SAFETY: `instance` must be null or point to a valid `Instance`.
    if instance.is_null() {
        return None;
    }
    Some(SharedPtr::new(crate::instance::DebrisService::default()))
}

// 0xf479f4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_13SpawnLocationEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpawnLocation> RBX::Creatable<RBX::Instance>::create<RBX::SpawnLocation>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_13SpawnLocationEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::SpawnLocation> RBX::Creatable<RBX::Instance>::create<RBX::SpawnLocation>(void)
pub fn stub_0xf479f4() -> SharedPtr<crate::instance::SpawnLocation> {
    // IDA 0xf479f4: `Creatable::create<SpawnLocation>` — `operator new` + default ctor
    // + adoption; same collapse as 0x4f0004 (`Fire`).
    SharedPtr::new(crate::instance::SpawnLocation::default())
}

// 0xf47a04 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14SpawnerServiceEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpawnerService> RBX::Creatable<RBX::Instance>::create<RBX::SpawnerService>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14SpawnerServiceEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::SpawnerService> RBX::Creatable<RBX::Instance>::create<RBX::SpawnerService>(void)
pub fn stub_0xf47a04() -> SharedPtr<crate::instance::SpawnerService> {
    // IDA 0xf47a04: `Creatable::create<SpawnerService>` — `operator new` + default ctor
    // + adoption; same collapse as 0x4f0004 (`Fire`).
    SharedPtr::new(crate::instance::SpawnerService::default())
}

// 0xf47a14 — j___ZN5boost10shared_ptrIN3RBX13SpawnLocationEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpawnLocation>::shared_ptr<RBX::SpawnLocation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13SpawnLocationEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::SpawnLocation>::shared_ptr<RBX::SpawnLocation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf47a14() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf47a14 rbx_core::SharedPtr<RBX::SpawnLocation>::shared_ptr<RBX::SpawnLocation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf47a24 — j___ZN5boost10shared_ptrIN3RBX14SpawnerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpawnerService>::shared_ptr<RBX::SpawnerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX14SpawnerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::SpawnerService>::shared_ptr<RBX::SpawnerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnerService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf47a24() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf47a24 rbx_core::SharedPtr<RBX::SpawnerService>::shared_ptr<RBX::SpawnerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnerService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf47a34 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14SpawnerServiceEEERS3_RKNS0_IT_EE
// type: int()
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::SpawnerService>(rbx_core::SharedPtr<RBX::SpawnerService> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14SpawnerServiceEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::SpawnerService>(boost::shared_ptr<RBX::SpawnerService> const&)
pub fn stub_0xf47a34() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf47a34 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::SpawnerService>(rbx_core::SharedPtr<RBX::SpawnerService> const&)")
}

// 0xf47a44 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX13SpawnLocationEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIPN3RBX13SpawnLocationEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_0xf47a44() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf47a44 void boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SpawnLocation,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0xf5a404 — j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8df684`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8df684 as stub_0xf5a404;

// 0xf5a414 — j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EEC2EMS2_FvS6_ibEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EEC2EMS2_FvS6_ibEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8df468`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8df468 as stub_0xf5a414;

// 0xf5a424 — j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8d88a4`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8d88a4 as stub_0xf5a424;

// 0xf5a434 — j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EEC2EMS2_FvS6_ibS7_EPKcSD_SD_SD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),char const*,char const*,char const*,char const*,bool,char const*,RBX::MarketplaceService::CurrencyType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EEC2EMS2_FvS6_ibS7_EPKcSD_SD_SD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::BoundFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),char const*,char const*,char const*,char const*,bool,char const*,RBX::MarketplaceService::CurrencyType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8d85c8`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8d85c8 as stub_0xf5a434;

// 0xf5a4e4 — j___ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_ib
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,int,bool)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_ib")]
// was: RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>,int,bool)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8d23c4`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8d23c4 as stub_0xf5a4e4;

// 0xf5a4f4 — j___ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_ib
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::fireAndReplicateEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_ib")]
// was: RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::fireAndReplicateEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8cda20`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8cda20 as stub_0xf5a4f4;

// 0xf5a534 — j___ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE14replicateEventEPNS0_11EventSourceES6_ibS7_
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE14replicateEventEPNS0_11EventSourceES6_ibS7_")]
// was: RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8d2ad0`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8d2ad0 as stub_0xf5a534;

// 0xf5a544 — j___ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE21fireAndReplicateEventEPS2_S6_ibS7_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::fireAndReplicateEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE21fireAndReplicateEventEPS2_S6_ibS7_")]
// was: RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::fireAndReplicateEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8cd8d4`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8cd8d4 as stub_0xf5a544;

// 0xf5a5b4 — j___ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8d8294`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8d8294 as stub_0xf5a5b4;

// 0xf5a5c4 — j___ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::generated_datamodel_shard_276::stub_0x8df0c4`;
// re-exported so the two addresses cannot drift.
pub use crate::generated_datamodel_shard_276::stub_0x8df0c4 as stub_0xf5a5c4;

// 0xf5a6a4 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibEEC2Ev")]
// was: rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>::remote_signal(void)
pub fn stub_0xf5a6a4() -> ! {
    // BLOCKED: core-owned `LatchedSignal`/`remote_signal` core (no native anywhere)
    todo!("0xf5a6a4 rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>::remote_signal(void)")
}

#[cfg(test)]
mod jump_tests {
    use super::*;

    #[test]
    fn find_sky_null_parent_returns_none() {
        assert!(stub_0xf44614(core::ptr::null()).is_none());
    }

    #[test]
    fn find_sky_hit_returns_first_sky_child() {
        let mut parent = crate::generated_05::Instance::default();
        let other = SharedPtr::new(crate::generated_05::Instance::default());
        let mut sky = crate::generated_05::Instance::default();
        sky.class_name = "Sky";
        let sky_child = SharedPtr::new(sky);
        parent.children.push(other);
        parent.children.push(sky_child.clone());
        let hit = stub_0xf44614(&parent as *const _);
        assert!(hit.is_some());
        assert!(SharedPtr::ptr_eq(&hit.unwrap(), &sky_child));
    }

    #[test]
    fn find_sky_miss_returns_none() {
        let parent = crate::generated_05::Instance::default();
        assert!(stub_0xf44614(&parent as *const _).is_none());
    }

    #[test]
    fn create_leaves_construct() {
        assert_eq!(SharedPtr::strong_count(&stub_0xf476a4()), 1);
        assert_eq!(SharedPtr::strong_count(&stub_0xf47764()), 1);
        assert_eq!(SharedPtr::strong_count(&stub_0xf479f4()), 1);
        assert_eq!(SharedPtr::strong_count(&stub_0xf47a04()), 1);
    }

    #[test]
    fn debris_service_create_null_yields_empty() {
        assert!(stub_0xf479c4(core::ptr::null()).is_none());
        let parent = crate::generated_05::Instance::default();
        let svc = stub_0xf479c4(&parent as *const _).expect("provider create");
        assert!(svc.items.is_empty());
    }
}
