// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted, skip global dedup
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xf3ea94..0xf41894 | total filtered 10215, pending before 974, remaining 874 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; WeakPtr = rbx_core::Weak

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf3ea94 — j___ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::GameSettings>::shared_ptr<RBX::GameSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::GameSettings>::shared_ptr<RBX::GameSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x4ff068`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x4ff068 as stub_0xf3ea94;

// 0xf3eaf4 — j___ZN5boost10shared_ptrIN3RBX9DataModelEEaSERKS3_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::operator=(rbx_core::SharedPtr<RBX::DataModel> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX9DataModelEEaSERKS3_")]
// was: boost::shared_ptr<RBX::DataModel>::operator=(boost::shared_ptr<RBX::DataModel> const&)
pub fn stub_0xf3eaf4() -> ! {
    // BLOCKED: core-owned `SharedPtr` slot infra (no exact-EA native anywhere)
    todo!("0xf3eaf4 rbx_core::SharedPtr<RBX::DataModel>::operator=(rbx_core::SharedPtr<RBX::DataModel> const&)")
}

// 0xf3eb24 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS0_5list2INS0_5valueIPS5_EENSA_ISsEEEEEclIPNS4_9DataModelEEEvRT_
// type: void
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>::operator()<RBX::DataModel *>(RBX::DataModel * &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS0_5list2INS0_5valueIPS5_EENSA_ISsEEEEEclIPNS4_9DataModelEEEvRT_")]
pub fn stub_0xf3eb24() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3eb24 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>::operator()<RBX::DataModel *>(RBX::DataModel * &)")
}

// 0xf3eb64 — j___ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x4ff130`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x4ff130 as stub_0xf3eb64;

// 0xf3eba4 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEEvT_
// type: void
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEEvT_")]
pub fn stub_0xf3eba4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3eba4 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>)")
}

// 0xf3ebe4 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf3ebe4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3ebe4 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf3ebf4 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf3ebf4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3ebf4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0xf3ec04 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf3ec04() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3ec04 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf3eec4 — j___ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
#[doc(alias = "j___ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb")]
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
pub fn stub_0xf3eec4() -> ! {
    // BLOCKED: physics raycast/picking infra (needs RbxRay/workspace-cell traversal)
    todo!("0xf3eec4 G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")
}

// 0xf3eed4 — j___ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
#[doc(alias = "j___ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb")]
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
pub fn stub_0xf3eed4() -> ! {
    // BLOCKED: physics raycast/picking infra (needs RbxRay/workspace-cell traversal)
    todo!("0xf3eed4 G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")
}

// 0xf3eef4 — j___ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperINS_9unordered13unordered_setIPKN3RBX9PrimitiveENS_4hashISA_EESt8equal_toISA_ESaISA_EEEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEERSG_ENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i
// type: void
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperINS_9unordered13unordered_setIPKN3RBX9PrimitiveENS_4hashISA_EESt8equal_toISA_ESaISA_EEEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEERSG_ENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf3eef4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3eef4 void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf3ef74 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS2_9unordered13unordered_setIPKNS_9PrimitiveENS2_4hashISB_EESt8equal_toISB_ESaISB_EEEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISH_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS2_9unordered13unordered_setIPKNS_9PrimitiveENS2_4hashISB_EESt8equal_toISB_ESaISB_EEEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISH_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const
pub fn stub_0xf3ef74() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf3ef74 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const")
}

// 0xf3f164 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50dd7c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50dd7c as stub_0xf3f164;

// 0xf3f174 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50e12c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50e12c as stub_0xf3f174;

// 0xf3f184 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50b39c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50b39c as stub_0xf3f184;

// 0xf3f194 — j___ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50de2c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50de2c as stub_0xf3f194;

// 0xf3f1a4 — j___ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50e1dc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50e1dc as stub_0xf3f1a4;

// 0xf3f1b4 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Selection>(boost::shared_ptr<RBX::Selection> const&)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50b44c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50b44c as stub_0xf3f1b4;

// 0xf3f1c4 — j___ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50b688`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50b688 as stub_0xf3f1c4;

// 0xf3f1e4 — j___ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50dfdc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50dfdc as stub_0xf3f1e4;

// 0xf3f1f4 — j___ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50e38c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50e38c as stub_0xf3f1f4;

// 0xf3f204 — j___ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x50b838`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x50b838 as stub_0xf3f204;

// 0xf3f244 — j___ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<void (*)(rbx_core::SharedPtr<RBX::Instance>)>(void (*)(rbx_core::SharedPtr<RBX::Instance>) const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_")]
// was: void RBX::Instance::visitChildren<void (*)(boost::shared_ptr<RBX::Instance>)>(void (*)(boost::shared_ptr<RBX::Instance>) const&)const
pub fn stub_0xf3f244(
    parent: *const crate::generated_05::Instance,
    visit: fn(SharedPtr<crate::generated_05::Instance>),
) {
    // IDA 0x509094 (decompiled, canonical target of this `j__` thunk):
    // `visitChildren` — null child-list returns at once (0x5090c0-0x5090e2);
    // otherwise each child is retained and passed to the functor in order
    // (0x5090fa-0x509148), releasing the list lock after (0x50914c-0x509154).
    // The retains/releases collapse into `clone`s; the jump collapses into
    // the direct implementation.
    // SAFETY: `parent` must be null or point to a valid `Instance`.
    if parent.is_null() {
        return;
    }
    unsafe {
        for child in (*parent).children.iter() {
            visit(child.clone());
        }
    }
}

// 0xf3f254 — j___ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector const&)const")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_")]
pub fn stub_0xf3f254() -> ! {
    // BLOCKED: functor-based descendant-visit infra (bind functor types unmodeled)
    todo!("0xf3f254 void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector const&)const")
}

// 0xf3f2f4 — j___ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_0xf3f2f4() -> ! {
    // BLOCKED: STL container infra (no native anywhere)
    todo!("0xf3f2f4 std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")
}

// 0xf3f304 — j___ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_0xf3f304() -> ! {
    // BLOCKED: STL container infra (no native anywhere)
    todo!("0xf3f304 std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")
}

// 0xf3f314 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_0xf3f314() -> ! {
    // BLOCKED: STL container infra (no native anywhere)
    todo!("0xf3f314 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")
}

// 0xf3f324 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_0xf3f324() -> ! {
    // BLOCKED: STL container infra (no native anywhere)
    todo!("0xf3f324 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")
}

// 0xf3f484 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatButtonEPNS_5AdornEPKciEEN5boost10shared_ptrIT_EET0_T1_T2_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton> RBX::Creatable<RBX::Instance>::create<RBX::ChatButton,RBX::Adorn *,char const*,int>(RBX::Adorn *,char const*,int)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatButtonEPNS_5AdornEPKciEEN5boost10shared_ptrIT_EET0_T1_T2_")]
// was: boost::shared_ptr<RBX::ChatButton> RBX::Creatable<RBX::Instance>::create<RBX::ChatButton,RBX::Adorn *,char const*,int>(RBX::Adorn *,char const*,int)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5213fc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5213fc as stub_0xf3f484;

// 0xf3f494 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatOutputEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput> RBX::Creatable<RBX::Instance>::create<RBX::ChatOutput>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatOutputEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::ChatOutput> RBX::Creatable<RBX::Instance>::create<RBX::ChatOutput>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5211ec`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5211ec as stub_0xf3f494;

// 0xf3f4a4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatWidgetESsSsEEN5boost10shared_ptrIT_EET0_T1_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget> RBX::Creatable<RBX::Instance>::create<RBX::ChatWidget,std::string,std::string>(std::string,std::string)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatWidgetESsSsEEN5boost10shared_ptrIT_EET0_T1_")]
// was: boost::shared_ptr<RBX::ChatWidget> RBX::Creatable<RBX::Instance>::create<RBX::ChatWidget,std::string,std::string>(std::string,std::string)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5212a0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5212a0 as stub_0xf3f4a4;

// 0xf3f4b4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay> RBX::Creatable<RBX::Instance>::create<RBX::TextDisplay,char const*,char const*>(char const*,char const*)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_")]
// was: boost::shared_ptr<RBX::TextDisplay> RBX::Creatable<RBX::Instance>::create<RBX::TextDisplay,char const*,char const*>(char const*,char const*)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x521594`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x521594 as stub_0xf3f4b4;

// 0xf3f4c4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_13RelativePanelENS_6LayoutEEEN5boost10shared_ptrIT_EET0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel> RBX::Creatable<RBX::Instance>::create<RBX::RelativePanel,RBX::Layout>(RBX::Layout)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_13RelativePanelENS_6LayoutEEEN5boost10shared_ptrIT_EET0_")]
// was: boost::shared_ptr<RBX::RelativePanel> RBX::Creatable<RBX::Instance>::create<RBX::RelativePanel,RBX::Layout>(RBX::Layout)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x521138`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x521138 as stub_0xf3f4c4;

// 0xf3f4d4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEPNS_4VerbEEEN5boost10shared_ptrIT_EET0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton,RBX::Verb *>(RBX::Verb *)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEPNS_4VerbEEEN5boost10shared_ptrIT_EET0_")]
// was: boost::shared_ptr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton,RBX::Verb *>(RBX::Verb *)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x52105c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x52105c as stub_0xf3f4d4;

// 0xf3f4e4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,char const*,char const*>(char const*,char const*)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_")]
// was: boost::shared_ptr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,char const*,char const*>(char const*,char const*)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x52177c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x52177c as stub_0xf3f4e4;

// 0xf3f4f4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayESsSsEEN5boost10shared_ptrIT_EET0_T1_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,std::string,std::string>(std::string,std::string)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayESsSsEEN5boost10shared_ptrIT_EET0_T1_")]
// was: boost::shared_ptr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,std::string,std::string>(std::string,std::string)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x520c28`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x520c28 as stub_0xf3f4f4;

// 0xf3f504 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15NotificationBoxEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox> RBX::Creatable<RBX::Instance>::create<RBX::NotificationBox>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15NotificationBoxEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::NotificationBox> RBX::Creatable<RBX::Instance>::create<RBX::NotificationBox>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x520fa8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x520fa8 as stub_0xf3f504;

// 0xf3f514 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsSettings>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::PhysicsSettings> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsSettings>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x525000`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x525000 as stub_0xf3f514;

// 0xf3f524 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5FrameEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Frame> RBX::Creatable<RBX::Instance>::create<RBX::Frame>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5FrameEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Frame> RBX::Creatable<RBX::Instance>::create<RBX::Frame>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x520ef4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x520ef4 as stub_0xf3f524;

// 0xf3f534 — j___ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x522ff0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x522ff0 as stub_0xf3f534;

// 0xf3f544 — j___ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5235e8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5235e8 as stub_0xf3f544;

// 0xf3f554 — j___ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5232ec`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5232ec as stub_0xf3f554;

// 0xf3f574 — j___ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x521e34`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x521e34 as stub_0xf3f574;

// 0xf3f594 — j___ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x523ccc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x523ccc as stub_0xf3f594;

// 0xf3f5a4 — j___ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5244c0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5244c0 as stub_0xf3f5a4;

// 0xf3f5b4 — j___ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x521b38`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x521b38 as stub_0xf3f5b4;

// 0xf3f5c4 — j___ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5247bc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5247bc as stub_0xf3f5c4;

// 0xf3f5d4 — j___ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings>::shared_ptr<RBX::PhysicsSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::PhysicsSettings>::shared_ptr<RBX::PhysicsSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5250b0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5250b0 as stub_0xf3f5d4;

// 0xf3f5e4 — j___ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::Frame>::shared_ptr<RBX::Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Frame>::shared_ptr<RBX::Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x524ab8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x524ab8 as stub_0xf3f5e4;

// 0xf3f604 — j___ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5231a0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5231a0 as stub_0xf3f604;

// 0xf3f614 — j___ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x523798`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x523798 as stub_0xf3f614;

// 0xf3f624 — j___ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x52349c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x52349c as stub_0xf3f624;

// 0xf3f634 — j___ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x521fe4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x521fe4 as stub_0xf3f634;

// 0xf3f644 — j___ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x523e7c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x523e7c as stub_0xf3f644;

// 0xf3f654 — j___ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x524670`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x524670 as stub_0xf3f654;

// 0xf3f664 — j___ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x521ce8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x521ce8 as stub_0xf3f664;

// 0xf3f674 — j___ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x52496c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x52496c as stub_0xf3f674;

// 0xf3f684 — j___ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x525260`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x525260 as stub_0xf3f684;

// 0xf3f694 — j___ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x524378`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x524378 as stub_0xf3f694;

// 0xf3f6a4 — j___ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x524c68`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x524c68 as stub_0xf3f6a4;

// 0xf3fb64 — j___ZN3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_PKNS_8InstanceE
// type: void
#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(RBX::Instance const*)")]
#[doc(alias = "j___ZN3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_PKNS_8InstanceE")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x52d218`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x52d218 as stub_0xf3fb64;

// 0xf3fca4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::TweenService> RBX::Creatable<RBX::Instance>::create<RBX::TweenService>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::TweenService> RBX::Creatable<RBX::Instance>::create<RBX::TweenService>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x535584`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x535584 as stub_0xf3fca4;

// 0xf3fee4 — j___ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::TweenService>::shared_ptr<RBX::TweenService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::TweenService>::shared_ptr<RBX::TweenService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x535870`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x535870 as stub_0xf3fee4;

// 0xf3fef4 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::TweenService>(rbx_core::SharedPtr<RBX::TweenService> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::TweenService>(boost::shared_ptr<RBX::TweenService> const&)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x535634`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x535634 as stub_0xf3fef4;

// 0xf3ff34 — j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: void
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::operator()<void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf3ff34() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf3ff34 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::operator()<void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0xf40094 — j___ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x535a20`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x535a20 as stub_0xf40094;

// 0xf40204 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_
// type: void
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>)")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_")]
pub fn stub_0xf40204() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf40204 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>)")
}

// 0xf40384 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf40384() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf40384 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf40394 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf40394() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf40394 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &)const")
}

// 0xf403a4 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf403a4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf403a4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf406b4 — j___ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
// type: void
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")]
// was: RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x54d8fc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x54d8fc as stub_0xf406b4;

// 0xf406f4 — j___ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x54d6e0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x54d6e0 as stub_0xf406f4;

// 0xf40704 — j___ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x54d564`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x54d564 as stub_0xf40704;

// 0xf40984 — j___ZN3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::create<RBX::CoreGuiService>(RBX::Instance const*)")]
#[doc(alias = "j___ZN3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x545740`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x545740 as stub_0xf40984;

// 0xf409a4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService> RBX::Creatable<RBX::Instance>::create<RBX::CoreGuiService>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::CoreGuiService> RBX::Creatable<RBX::Instance>::create<RBX::CoreGuiService>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x54ba48`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x54ba48 as stub_0xf409a4;

// 0xf409b4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_18NotificationObjectEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject> RBX::Creatable<RBX::Instance>::create<RBX::NotificationObject>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_18NotificationObjectEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::NotificationObject> RBX::Creatable<RBX::Instance>::create<RBX::NotificationObject>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x546504`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x546504 as stub_0xf409b4;

// 0xf40c14 — j___ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x548f4c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x548f4c as stub_0xf40c14;

// 0xf40c24 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14CoreGuiServiceEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::CoreGuiService>(rbx_core::SharedPtr<RBX::CoreGuiService> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14CoreGuiServiceEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::CoreGuiService>(boost::shared_ptr<RBX::CoreGuiService> const&)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x54baf8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x54baf8 as stub_0xf40c24;

// 0xf40d74 — j___ZN5boost6detail12shared_countC2IPN3RBX14CoreGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX14CoreGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x54bb2c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x54bb2c as stub_0xf40d74;

// 0xf40d84 — j___ZN5boost6detail12shared_countC2IPN3RBX18NotificationObjectENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX18NotificationObjectENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5490fc`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5490fc as stub_0xf40d84;

// 0xf40ea4 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEEvT_
// type: void
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>)")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEEvT_")]
pub fn stub_0xf40ea4() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf40ea4 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>)")
}

// 0xf40fa4 — j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9ScreenGuiEEEPKT_v
// type: void
#[doc(alias = "RBX::ScreenGui const* RBX::Instance::findConstFirstChildOfType<RBX::ScreenGui>(void)const")]
#[doc(alias = "j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9ScreenGuiEEEPKT_v")]
pub fn stub_0xf40fa4(parent: *const crate::generated_05::Instance) -> Option<SharedPtr<crate::generated_05::Instance>> {
    // `findConstFirstChildOfType<ScreenGui>` — `j__` thunk to the template
    // body; the body is the standard child scan (null child-list returns
    // null, else first `isA ScreenGui` hit wins, miss returns null), same
    // shape as 0xaa7a00 (`StarterGear`), 0x3f1cac, and 0xf3e794 (`Flag`).
    // The jump collapses into the direct implementation.
    // SAFETY: `parent` must be null or point to a valid `Instance`.
    use crate::generated_05::instance_is_a;
    if parent.is_null() {
        return None;
    }
    unsafe {
        let children: &[SharedPtr<crate::generated_05::Instance>] = &(*parent).children;
        for child in children.iter() {
            if instance_is_a(SharedPtr::as_ptr(child), "ScreenGui") {
                return Some(child.clone());
            }
        }
        None
    }
}

// 0xf41024 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf41024() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf41024 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf41034 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf41034() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf41034 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &)const")
}

// 0xf41044 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf41044() -> ! {
    // BLOCKED: core-owned `boost::bind` infra (no exact-EA native anywhere)
    todo!("0xf41044 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf417b4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyThrustEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyThrustEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5632a4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5632a4 as stub_0xf417b4;

// 0xf417c4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyPositionEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyPositionEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5651f4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5651f4 as stub_0xf417c4;

// 0xf417d4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyVelocityEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyVelocityEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x564a20`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x564a20 as stub_0xf417d4;

// 0xf417e4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_19BodyAngularVelocityEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_19BodyAngularVelocityEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x56424c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x56424c as stub_0xf417e4;

// 0xf417f4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_6RocketEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_6RocketEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x562ad0`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x562ad0 as stub_0xf417f4;

// 0xf41804 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5659c8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5659c8 as stub_0xf41804;

// 0xf41814 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9BodyForceEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9BodyForceEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x563a78`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x563a78 as stub_0xf41814;

// 0xf41824 — j___ZN5boost10shared_ptrIN3RBX10BodyThrustEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10BodyThrustEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x563358`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x563358 as stub_0xf41824;

// 0xf41834 — j___ZN5boost10shared_ptrIN3RBX12BodyPositionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12BodyPositionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x5652a8`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x5652a8 as stub_0xf41834;

// 0xf41844 — j___ZN5boost10shared_ptrIN3RBX12BodyVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12BodyVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x564ad4`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x564ad4 as stub_0xf41844;

// 0xf41854 — j___ZN5boost10shared_ptrIN3RBX19BodyAngularVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX19BodyAngularVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x564300`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x564300 as stub_0xf41854;

// 0xf41864 — j___ZN5boost10shared_ptrIN3RBX6RocketEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX6RocketEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x562b84`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x562b84 as stub_0xf41864;

// 0xf41874 — j___ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x565a7c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x565a7c as stub_0xf41874;

// 0xf41884 — j___ZN5boost10shared_ptrIN3RBX9BodyForceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX9BodyForceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x563b2c`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x563b2c as stub_0xf41884;

// 0xf41894 — j___ZN5boost6detail12shared_countC2IPN3RBX10BodyThrustENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX10BodyThrustENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x563508`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x563508 as stub_0xf41894;

#[cfg(test)]
mod jump_tests {
    use super::*;

    #[test]
    fn find_screen_gui_null_parent_returns_none() {
        assert!(stub_0xf40fa4(core::ptr::null()).is_none());
    }

    #[test]
    fn visit_children_null_parent_is_noop() {
        fn fail(_: SharedPtr<crate::generated_05::Instance>) {
            panic!("must not visit a null parent");
        }
        stub_0xf3f244(core::ptr::null(), fail);
    }
}
