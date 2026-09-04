// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: mangled contains DataModel|Instance|Workspace|Game, EA-sorted asc, first 120 NOT in global
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x505420..0x5cab6c | mangled filter matched 20079, available after global dedup 1430
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; //0xADDR pattern + doc(alias) + todo!
// Shard: watchdog dm-wdA (datamodel-a) EA-sorted asc 120 UNIQUE global dedup muse-spark-1.3

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
// 0x505420 — __ZL31addInstanceToIgnorePrimitiveSetN5boost10shared_ptrIN3RBX8InstanceEEERNS_9unordered13unordered_setIPKNS1_9PrimitiveENS_4hashIS8_EESt8equal_toIS8_ESaIS8_EEE
#[doc(alias = "addInstanceToIgnorePrimitiveSet(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &)")]
// was: addInstanceToIgnorePrimitiveSet(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &)
#[doc(alias = "__ZL31addInstanceToIgnorePrimitiveSetN5boost10shared_ptrIN3RBX8InstanceEEERNS_9unordered13unordered_setIPKNS1_9PrimitiveENS_4hashIS8_EESt8equal_toIS8_ESaIS8_EEE")]
pub use crate::instance::stub_0x505420 as stub_0x505420;
// 0x50546c — __ZN3RBX15GeometryService23getPartsTouchingExtentsERKNS_7ExtentsEPKNS_9PrimitiveEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE
#[doc(alias = "RBX::GeometryService::getPartsTouchingExtents(RBX::Extents const&,RBX::Primitive const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)")]
#[doc(alias = "__ZN3RBX15GeometryService23getPartsTouchingExtentsERKNS_7ExtentsEPKNS_9PrimitiveEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE")]
pub use crate::instance::stub_0x50546c as stub_0x50546c;
// 0x505534 — __ZN3RBX15GeometryService31getHitLocationFilterDescendentsEPNS_8InstanceENS_6RbxRayEPPNS_9PrimitiveERNS_6CellIDEb
#[doc(alias = "RBX::GeometryService::getHitLocationFilterDescendents(RBX::Instance *,RBX::RbxRay,RBX::Primitive **,RBX::CellID &,bool)")]
#[doc(alias = "__ZN3RBX15GeometryService31getHitLocationFilterDescendentsEPNS_8InstanceENS_6RbxRayEPPNS_9PrimitiveERNS_6CellIDEb")]
pub use crate::instance::stub_0x505534 as stub_0x505534;
// 0x505704 — __ZN3RBX15GeometryService31getHitLocationFilterDescendentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_6RbxRayEPPNS_9PrimitiveERNS_6CellIDEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::GeometryService::getHitLocationFilterDescendents(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*,RBX::RbxRay,RBX::Primitive **,RBX::CellID &,bool)")]
// was: RBX::GeometryService::getHitLocationFilterDescendents(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*,RBX::RbxRay,RBX::Primitive **,RBX::CellID &,bool)
#[doc(alias = "__ZN3RBX15GeometryService31getHitLocationFilterDescendentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_6RbxRayEPPNS_9PrimitiveERNS_6CellIDEb")]
pub use crate::instance::stub_0x505704 as stub_0x505704;
// 0x505868 — __ZN3RBX15GeometryService26getHitLocationFilterStairsEPNS_8InstanceENS_6RbxRayEPPNS_9PrimitiveE
#[doc(alias = "RBX::GeometryService::getHitLocationFilterStairs(RBX::Instance *,RBX::RbxRay,RBX::Primitive **)")]
#[doc(alias = "__ZN3RBX15GeometryService26getHitLocationFilterStairsEPNS_8InstanceENS_6RbxRayEPPNS_9PrimitiveE")]
pub use crate::instance::stub_0x505868 as stub_0x505868;
// 0x505a68 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS2_9unordered13unordered_setIPKNS_9PrimitiveENS2_4hashISB_EESt8equal_toISB_ESaISB_EEEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISH_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS2_9unordered13unordered_setIPKNS_9PrimitiveENS2_4hashISB_EESt8equal_toISB_ESaISB_EEEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISH_EEEEEEEEvRKT_")]
pub use crate::instance::stub_0x505a68 as stub_0x505a68;
// 0x505b70 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::append(RBX::PartInstance * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6appendERKS3_")]
pub use crate::instance::stub_0x505b70 as stub_0x505b70;
// 0x505bcc — __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
#[doc(alias = "__ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb")]
pub use crate::instance::stub_0x505bcc as stub_0x505bcc;
// 0x505d08 — __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
#[doc(alias = "__ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb")]
pub use crate::instance::stub_0x505d08 as stub_0x505d08;
// 0x506524 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6resizeEib")]
pub use crate::instance::stub_0x506524 as stub_0x506524;
// 0x5065dc — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE7reallocEi")]
pub use crate::instance::stub_0x5065dc as stub_0x5065dc;
// 0x5067c4 — __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperINS_9unordered13unordered_setIPKN3RBX9PrimitiveENS_4hashISA_EESt8equal_toISA_ESaISA_EEEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEERSG_ENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperINS_9unordered13unordered_setIPKN3RBX9PrimitiveENS_4hashISA_EESt8equal_toISA_ESaISA_EEEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEERSG_ENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i")]
pub use crate::instance::stub_0x5067c4 as stub_0x5067c4;
// 0x507970 — __ZN3RBX8Settings25InvalidDescendentDetector7invalidEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Settings::InvalidDescendentDetector *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Settings::InvalidDescendentDetector::invalid(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX8Settings25InvalidDescendentDetector7invalidEPKNS_8InstanceE")]
pub use crate::instance::stub_0x507970 as stub_0x507970;
// 0x507da4 — __ZNK3RBX8Settings19verifyAddDescendantEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::Settings *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Settings::verifyAddDescendant(RBX::Instance const*,RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Settings19verifyAddDescendantEPKNS_8InstanceES3_")]
pub use crate::instance::stub_0x507da4 as stub_0x507da4;
// 0x508cd4 — __ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "resetChild(rbx_core::SharedPtr<RBX::Instance>)")]
// was: resetChild(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "__ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE")]
pub use crate::instance::stub_0x508cd4 as stub_0x508cd4;
// 0x508f18 — __ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_")]
pub use crate::instance::stub_0x508f18 as stub_0x508f18;
// 0x509094 — __ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<void (*)(rbx_core::SharedPtr<RBX::Instance>)>(void (*)(rbx_core::SharedPtr<RBX::Instance>) const&)const")]
// was: void RBX::Instance::visitChildren<void (*)(boost::shared_ptr<RBX::Instance>)>(void (*)(boost::shared_ptr<RBX::Instance>) const&)const
#[doc(alias = "__ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_")]
pub use crate::instance::stub_0x509094 as stub_0x509094;
// 0x50b39c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)")]
// was: boost::shared_ptr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x50b39c as stub_0x50b39c;
// 0x50b44c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Selection>(boost::shared_ptr<RBX::Selection> const&)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE")]
pub use crate::instance::stub_0x50b44c as stub_0x50b44c;
// 0x50b688 — __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x50b688 as stub_0x50b688;
// 0x50b838 — __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x50b838 as stub_0x50b838;
// 0x50b940 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x50b940 as stub_0x50b940;
// 0x50b948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x50b948 as stub_0x50b948;
// 0x50b968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x50b968 as stub_0x50b968;
// 0x50b980 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x50b980 as stub_0x50b980;
// 0x50caa0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
pub use crate::instance::stub_0x50caa0 as stub_0x50caa0;
// 0x50cac8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
pub use crate::instance::stub_0x50cac8 as stub_0x50cac8;
// 0x525368 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x525368 as stub_0x525368;
// 0x52536c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x52536c as stub_0x52536c;
// 0x525370 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x525370 as stub_0x525370;
// 0x525390 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x525390 as stub_0x525390;
// 0x5253a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x5253a8 as stub_0x5253a8;
// 0x562ad0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RocketEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)")]
// was: boost::shared_ptr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6RocketEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x562ad0 as stub_0x562ad0;
// 0x562b84 — __ZN5boost10shared_ptrIN3RBX6RocketEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6RocketEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x562b84 as stub_0x562b84;
// 0x562d34 — __ZN5boost6detail12shared_countC2IPN3RBX6RocketENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6RocketENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x562d34 as stub_0x562d34;
// 0x562e3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x562e3c as stub_0x562e3c;
// 0x562e40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x562e40 as stub_0x562e40;
// 0x562e44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x562e44 as stub_0x562e44;
// 0x562e64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x562e64 as stub_0x562e64;
// 0x562e7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x562e7c as stub_0x562e7c;
// 0x5632a4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyThrustEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)")]
// was: boost::shared_ptr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyThrustEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x5632a4 as stub_0x5632a4;
// 0x563358 — __ZN5boost10shared_ptrIN3RBX10BodyThrustEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10BodyThrustEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x563358 as stub_0x563358;
// 0x563508 — __ZN5boost6detail12shared_countC2IPN3RBX10BodyThrustENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10BodyThrustENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x563508 as stub_0x563508;
// 0x563610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x563610 as stub_0x563610;
// 0x563614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x563614 as stub_0x563614;
// 0x563618 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x563618 as stub_0x563618;
// 0x563638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x563638 as stub_0x563638;
// 0x563650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x563650 as stub_0x563650;
// 0x563a78 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9BodyForceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)")]
// was: boost::shared_ptr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9BodyForceEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x563a78 as stub_0x563a78;
// 0x563b2c — __ZN5boost10shared_ptrIN3RBX9BodyForceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9BodyForceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x563b2c as stub_0x563b2c;
// 0x563cdc — __ZN5boost6detail12shared_countC2IPN3RBX9BodyForceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9BodyForceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x563cdc as stub_0x563cdc;
// 0x563de4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x563de4 as stub_0x563de4;
// 0x563de8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x563de8 as stub_0x563de8;
// 0x563dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x563dec as stub_0x563dec;
// 0x563e0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x563e0c as stub_0x563e0c;
// 0x563e24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x563e24 as stub_0x563e24;
// 0x56424c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19BodyAngularVelocityEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)")]
// was: boost::shared_ptr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19BodyAngularVelocityEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x56424c as stub_0x56424c;
// 0x564300 — __ZN5boost10shared_ptrIN3RBX19BodyAngularVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19BodyAngularVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x564300 as stub_0x564300;
// 0x5644b0 — __ZN5boost6detail12shared_countC2IPN3RBX19BodyAngularVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX19BodyAngularVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x5644b0 as stub_0x5644b0;
// 0x5645b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x5645b8 as stub_0x5645b8;
// 0x5645bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x5645bc as stub_0x5645bc;
// 0x5645c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x5645c0 as stub_0x5645c0;
// 0x5645e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x5645e0 as stub_0x5645e0;
// 0x5645f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x5645f8 as stub_0x5645f8;
// 0x564a20 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyVelocityEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)")]
// was: boost::shared_ptr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyVelocityEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x564a20 as stub_0x564a20;
// 0x564ad4 — __ZN5boost10shared_ptrIN3RBX12BodyVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12BodyVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x564ad4 as stub_0x564ad4;
// 0x564c84 — __ZN5boost6detail12shared_countC2IPN3RBX12BodyVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12BodyVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x564c84 as stub_0x564c84;
// 0x564d8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x564d8c as stub_0x564d8c;
// 0x564d90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x564d90 as stub_0x564d90;
// 0x564d94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x564d94 as stub_0x564d94;
// 0x564db4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x564db4 as stub_0x564db4;
// 0x564dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x564dcc as stub_0x564dcc;
// 0x5651f4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyPositionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)")]
// was: boost::shared_ptr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyPositionEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x5651f4 as stub_0x5651f4;
// 0x5652a8 — __ZN5boost10shared_ptrIN3RBX12BodyPositionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12BodyPositionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x5652a8 as stub_0x5652a8;
// 0x565458 — __ZN5boost6detail12shared_countC2IPN3RBX12BodyPositionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12BodyPositionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x565458 as stub_0x565458;
// 0x565560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x565560 as stub_0x565560;
// 0x565564 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x565564 as stub_0x565564;
// 0x565568 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x565568 as stub_0x565568;
// 0x565588 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x565588 as stub_0x565588;
// 0x5655a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x5655a0 as stub_0x5655a0;
// 0x5659c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)")]
// was: boost::shared_ptr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x5659c8 as stub_0x5659c8;
// 0x565a7c — __ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x565a7c as stub_0x565a7c;
// 0x565c2c — __ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x565c2c as stub_0x565c2c;
// 0x565d34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x565d34 as stub_0x565d34;
// 0x565d38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x565d38 as stub_0x565d38;
// 0x565d3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x565d3c as stub_0x565d3c;
// 0x565d5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x565d5c as stub_0x565d5c;
// 0x565d74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x565d74 as stub_0x565d74;
// 0x5681f4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Handles> RBX::Creatable<RBX::Instance>::create<RBX::Handles>(void)")]
// was: boost::shared_ptr<RBX::Handles> RBX::Creatable<RBX::Instance>::create<RBX::Handles>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x5681f4 as stub_0x5681f4;
// 0x5682a8 — __ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Handles>::shared_ptr<RBX::Handles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Handles>::shared_ptr<RBX::Handles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x5682a8 as stub_0x5682a8;
// 0x568458 — __ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x568458 as stub_0x568458;
// 0x568560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x568560 as stub_0x568560;
// 0x568564 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x568564 as stub_0x568564;
// 0x568568 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x568568 as stub_0x568568;
// 0x568588 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x568588 as stub_0x568588;
// 0x5685a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x5685a0 as stub_0x5685a0;
// 0x5c0994 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_")]
pub use crate::instance::stub_0x5c0994 as stub_0x5c0994;
// 0x5c0b28 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_")]
pub use crate::instance::stub_0x5c0b28 as stub_0x5c0b28;
// 0x5c0b44 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE")]
pub use crate::instance::stub_0x5c0b44 as stub_0x5c0b44;
// 0x5c0ca8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub use crate::instance::stub_0x5c0ca8 as stub_0x5c0ca8;
// 0x5c0e08 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub use crate::instance::stub_0x5c0e08 as stub_0x5c0e08;
// 0x5c0f1c — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::operator()<void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub use crate::instance::stub_0x5c0f1c as stub_0x5c0f1c;
// 0x5c2938 — __ZNK3RBX8Lighting11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Lighting::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Lighting11askAddChildEPKNS_8InstanceE")]
pub use crate::instance::stub_0x5c2938 as stub_0x5c2938;
// 0x5c2b24 — __ZN3RBX8Lighting15onChildRemovingEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Lighting::onChildRemoving(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8Lighting15onChildRemovingEPNS_8InstanceE")]
pub use crate::instance::stub_0x5c2b24 as stub_0x5c2b24;
// 0x5c2b58 — __ZN3RBX8Lighting12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Lighting::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8Lighting12onChildAddedEPNS_8InstanceE")]
pub use crate::instance::stub_0x5c2b58 as stub_0x5c2b58;
// 0x5c2c6c — __ZN3RBX8Lighting14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
#[doc(alias = "RBX::Lighting::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
#[doc(alias = "__ZN3RBX8Lighting14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE")]
pub use crate::instance::stub_0x5c2c6c as stub_0x5c2c6c;
// 0x5c733c — __ZNK3RBX13LocalBackpack12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::LocalBackpack::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX13LocalBackpack12askSetParentEPKNS_8InstanceE")]
pub use crate::instance::stub_0x5c733c as stub_0x5c733c;
// 0x5c8a68 — __ZNK3RBX7Message12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Message *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Message::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX7Message12askSetParentEPKNS_8InstanceE")]
pub use crate::instance::stub_0x5c8a68 as stub_0x5c8a68;
// 0x5c9708 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)")]
// was: boost::shared_ptr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x5c9708 as stub_0x5c9708;
// 0x5c9f04 — __ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x5c9f04 as stub_0x5c9f04;
// 0x5ca0b4 — __ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x5ca0b4 as stub_0x5ca0b4;
// 0x5ca1bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x5ca1bc as stub_0x5ca1bc;
// 0x5ca1c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x5ca1c0 as stub_0x5ca1c0;
// 0x5ca1c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x5ca1c4 as stub_0x5ca1c4;
// 0x5ca1e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x5ca1e4 as stub_0x5ca1e4;
// 0x5ca1fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x5ca1fc as stub_0x5ca1fc;
// 0x5ca804 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)")]
// was: boost::shared_ptr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x5ca804 as stub_0x5ca804;
// 0x5ca8b4 — __ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x5ca8b4 as stub_0x5ca8b4;
// 0x5caa64 — __ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x5caa64 as stub_0x5caa64;
// 0x5cab6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x5cab6c as stub_0x5cab6c;
