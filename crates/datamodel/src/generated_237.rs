// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace strict EA-sorted asc next 150 not yet in datamodel nor any crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 150 stubs | range 0xa242b8..0xa4d904 | strict 10215 total, dm missing before 2405, not-in-any before 2134, after 1984
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias; weak_ptr -> rbx_core::Weak

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xa242b8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(RBX::BevelMesh **, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")]
// was: boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)
pub fn stub_0xa242b8() -> ! {
    todo!("0xa242b8 rbx_core::SharedPtr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")
}

// 0xa246b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa246b0() -> ! {
    todo!("0xa246b0 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa25aa8 — __ZN21AppendOtherCharactersclEN5boost10shared_ptrIN3RBX8InstanceEEE
// type: int __fastcall(int *, int *, int, int)
#[doc(alias = "AppendOtherCharacters::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: AppendOtherCharacters::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa25aa8() -> ! {
    todo!("0xa25aa8 AppendOtherCharacters::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa26548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa26548() -> ! {
    todo!("0xa26548 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa26550 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa26550() -> ! {
    todo!("0xa26550 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa2656c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa2656c() -> ! {
    todo!("0xa2656c boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa26584 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa26584() -> ! {
    todo!("0xa26584 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa2b3d0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: void __fastcall(pthread_mutex_t **, pthread_mutex_t *, __int32 *, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, boost::detail::shared_count *, int, int, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance>*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xa2b3d0() -> ! {
    todo!("0xa2b3d0 std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa2ccc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa2ccc8() -> ! {
    todo!("0xa2ccc8 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa2ccd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa2ccd0() -> ! {
    todo!("0xa2ccd0 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa2e2e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa2e2e4() -> ! {
    todo!("0xa2e2e4 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa2e2e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa2e2e8() -> ! {
    todo!("0xa2e2e8 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa2e2f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa2e2f4() -> ! {
    todo!("0xa2e2f4 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa2e310 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa2e310() -> ! {
    todo!("0xa2e310 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa2e328 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa2e328() -> ! {
    todo!("0xa2e328 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa2f1b0 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_8HumanoidEEEPKT_v
// type: int __fastcall(int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Humanoid const* RBX::Instance::findConstFirstChildOfType<RBX::Humanoid>(void)const")]
pub fn stub_0xa2f1b0() -> ! {
    todo!("0xa2f1b0 RBX::Humanoid const* RBX::Instance::findConstFirstChildOfType<RBX::Humanoid>(void)const")
}

// 0xa2f6ec — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)
pub fn stub_0xa2f6ec() -> ! {
    todo!("0xa2f6ec void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")
}

// 0xa2f8dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0xa2f8dc() -> ! {
    todo!("0xa2f8dc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xa2f900 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa2f900() -> ! {
    todo!("0xa2f900 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa2f918 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xa2f918() -> ! {
    todo!("0xa2f918 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")
}

// 0xa2faf4 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xa2faf4() -> ! {
    todo!("0xa2faf4 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa2fd44 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEclIPFvS7_NS_10shared_ptrINS4_8InstanceEEESC_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int *, int, int, int), int **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_0xa2fd44() -> ! {
    todo!("0xa2fd44 void boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0xa300f8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xa300f8() -> ! {
    todo!("0xa300f8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xa3119c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13InsertServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3119c() -> ! {
    todo!("0xa3119c boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa311a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13InsertServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa311a0() -> ! {
    todo!("0xa311a0 boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa311ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13InsertServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa311ac() -> ! {
    todo!("0xa311ac boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa311c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13InsertServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa311c8() -> ! {
    todo!("0xa311c8 boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa311e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13InsertServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa311e0() -> ! {
    todo!("0xa311e0 boost::detail::sp_counted_impl_pd<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa327cc — __ZN5boost6detail20sp_pointer_constructISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EES8_EEvPNS3_IT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,boost::detail::shared_count &)")]
// was: void boost::detail::sp_pointer_construct<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,boost::detail::shared_count &)
pub fn stub_0xa327cc() -> ! {
    todo!("0xa327cc void boost::detail::sp_pointer_construct<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,boost::detail::shared_count &)")
}

// 0xa3325c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13FriendServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3325c() -> ! {
    todo!("0xa3325c boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa33260 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13FriendServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa33260() -> ! {
    todo!("0xa33260 boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3326c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13FriendServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa3326c() -> ! {
    todo!("0xa3326c boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa33288 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13FriendServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa33288() -> ! {
    todo!("0xa33288 boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa332a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13FriendServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa332a0() -> ! {
    todo!("0xa332a0 boost::detail::sp_counted_impl_pd<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa33e5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
// type: int __fastcall(int, int32_t **)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot> &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot> &)
pub fn stub_0xa33e5c() -> ! {
    todo!("0xa33e5c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot> &)")
}

// 0xa34070 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE8fireItemEPNS0_6signalIS9_E4slotES6_S6_S8_
// type: void __fastcall(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")]
// was: rbx::signals::signal_with_args<3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::fireItem(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)
pub fn stub_0xa34070() -> ! {
    todo!("0xa34070 rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")
}

// 0xa344d8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE5mutexEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::mutex(void)
pub fn stub_0xa344d8() -> ! {
    todo!("0xa344d8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::mutex(void)")
}

// 0xa345ec — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotEEaSERKSD_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot> const&)
pub fn stub_0xa345ec() -> ! {
    todo!("0xa345ec rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot> const&)")
}

// 0xa346a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::safe_static_init_mutex(void)
pub fn stub_0xa346a0() -> ! {
    todo!("0xa346a0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::safe_static_init_mutex(void)")
}

// 0xa380f8 — __ZN3RBX8GuidItemINS_8InstanceEE8Registry12lookupByGuidERKNS_4Guid4DataERPS1_
// type: int __fastcall(int, RBX::Name *, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::GuidItem<RBX::Instance>::Registry::lookupByGuid(RBX::Guid::Data const&,RBX::Instance*&)")]
pub fn stub_0xa380f8() -> ! {
    todo!("0xa380f8 RBX::GuidItem<RBX::Instance>::Registry::lookupByGuid(RBX::Guid::Data const&,RBX::Instance*&)")
}

// 0xa394a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa394a8() -> ! {
    todo!("0xa394a8 boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa394ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa394ac() -> ! {
    todo!("0xa394ac boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa394b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa394b8() -> ! {
    todo!("0xa394b8 boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa394d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa394d4() -> ! {
    todo!("0xa394d4 boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa394ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa394ec() -> ! {
    todo!("0xa394ec boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa39be0 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4nextERNS6_13intrusive_ptrINSB_4slotEEE
// type: int __fastcall(int, int32_t **)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot> &)
pub fn stub_0xa39be0() -> ! {
    todo!("0xa39be0 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")
}

// 0xa39df4 — __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE8fireItemEPNS0_6signalISA_E4slotES5_S9_SsS9_
// type: void __fastcall(int, int, int *, const std::string *, int *)
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa39df4() -> ! {
    todo!("0xa39df4 rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa3a300 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE5mutexEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::mutex(void)
pub fn stub_0xa3a300() -> ! {
    todo!("0xa3a300 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")
}

// 0xa3a414 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSERKSE_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0xa3a414() -> ! {
    todo!("0xa3a414 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")
}

// 0xa3a4c8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)
pub fn stub_0xa3a4c8() -> ! {
    todo!("0xa3a4c8 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")
}

// 0xa3a5b0 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int32_t **)
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot> &)
pub fn stub_0xa3a5b0() -> ! {
    todo!("0xa3a5b0 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")
}

// 0xa3a7c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSERKSB_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0xa3a7c8() -> ! {
    todo!("0xa3a7c8 rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")
}

// 0xa3e220 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3e220() -> ! {
    todo!("0xa3e220 boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3e230 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa3e230() -> ! {
    todo!("0xa3e230 boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa3e248 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa3e248() -> ! {
    todo!("0xa3e248 boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa3ec1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StarterGearENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3ec1c() -> ! {
    todo!("0xa3ec1c boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3ec20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StarterGearENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3ec20() -> ! {
    todo!("0xa3ec20 boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3ec2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StarterGearENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa3ec2c() -> ! {
    todo!("0xa3ec2c boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa3ec48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StarterGearENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa3ec48() -> ! {
    todo!("0xa3ec48 boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa3ec60 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StarterGearENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa3ec60() -> ! {
    todo!("0xa3ec60 boost::detail::sp_counted_impl_pd<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa3ef20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3ef20() -> ! {
    todo!("0xa3ef20 boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3ef24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3ef24() -> ! {
    todo!("0xa3ef24 boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3ef30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa3ef30() -> ! {
    todo!("0xa3ef30 boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa3ef4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa3ef4c() -> ! {
    todo!("0xa3ef4c boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa3ef64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa3ef64() -> ! {
    todo!("0xa3ef64 boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa3f618 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa3f618() -> ! {
    todo!("0xa3f618 boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa3f620 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa3f620() -> ! {
    todo!("0xa3f620 boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa40588 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xa40588() -> ! {
    todo!("0xa40588 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa40838 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_0xa40838() -> ! {
    todo!("0xa40838 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa40914 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0xa40914() -> ! {
    todo!("0xa40914 RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xa40d98 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0xa40d98() -> ! {
    todo!("0xa40d98 RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xa410f0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0xa410f0() -> ! {
    todo!("0xa410f0 RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xa41108 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE5mutexEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::mutex(void)
pub fn stub_0xa41108() -> ! {
    todo!("0xa41108 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")
}

// 0xa41220 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)const
pub fn stub_0xa41220() -> ! {
    todo!("0xa41220 RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")
}

// 0xa413f8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN5boost10shared_ptrINS_8InstanceEEEEEvRKT_
// type: int __fastcall(int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute1<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xa413f8() -> ! {
    todo!("0xa413f8 void RBX::Reflection::GenericSlotWrapper::execute1<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa415b8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::construct_func(char const*,char *)
pub fn stub_0xa415b8() -> ! {
    todo!("0xa415b8 rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::construct_func(char const*,char *)")
}

// 0xa41a90 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa41a90() -> ! {
    todo!("0xa41a90 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa41ab0 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xa41ab0() -> ! {
    todo!("0xa41ab0 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa41d98 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xa41d98() -> ! {
    todo!("0xa41d98 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xa41f30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_0xa41f30() -> ! {
    todo!("0xa41f30 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")
}

// 0xa420b0 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_0xa420b0() -> ! {
    todo!("0xa420b0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")
}

// 0xa420c0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_0xa420c0() -> ! {
    todo!("0xa420c0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")
}

// 0xa421a8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0xa421a8() -> ! {
    todo!("0xa421a8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")
}

// 0xa422b4 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, RBX::Name *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xa422b4() -> ! {
    todo!("0xa422b4 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa4273c — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_0xa4273c() -> ! {
    todo!("0xa4273c RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa42818 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0xa42818() -> ! {
    todo!("0xa42818 RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xa42c9c — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
// type: void __fastcall(int, int, pthread_mutex_t **)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0xa42c9c() -> ! {
    todo!("0xa42c9c RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xa432e0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0xa432e0() -> ! {
    todo!("0xa432e0 RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xa432f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE13disconnectAllEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::disconnectAll(void)
pub fn stub_0xa432f8() -> ! {
    todo!("0xa432f8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::disconnectAll(void)")
}

// 0xa434b0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> const&)const
pub fn stub_0xa434b0() -> ! {
    todo!("0xa434b0 RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> const&)const")
}

// 0xa43684 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEES8_RKNS1_13FriendService15FriendEventTypeENS4_IS3_EENS_3argILi1EEENSE_ILi2EEENSE_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISK_T0_T1_T2_T3_EENSI_9list_av_4IT4_T5_T6_T7_E4typeEEEMSN_FSK_SO_SP_SQ_EST_SU_SV_SW_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_0xa43684() -> ! {
    todo!("0xa43684 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0xa43ce8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEES6_NS_13FriendService15FriendEventTypeEEEvRKT_RKT0_RKT1_
// type: int __fastcall(int, int, int, _DWORD *)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute3<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&)
pub fn stub_0xa43ce8() -> ! {
    todo!("0xa43ce8 void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&)")
}

// 0xa44854 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEES4_NS2_13FriendService15FriendEventTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_SG_RKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: void boost::function3<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)
pub fn stub_0xa44854() -> ! {
    todo!("0xa44854 void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0xa44ccc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0xa44ccc() -> ! {
    todo!("0xa44ccc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xa44cf0 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEvSC_SC_SG_E6invokeERNS1_15function_bufferESC_SC_SG_
// type: int __fastcall(char ***, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")]
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)
pub fn stub_0xa44cf0() -> ! {
    todo!("0xa44cf0 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")
}

// 0xa44d1c — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_SI_RKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xa44d1c() -> ! {
    todo!("0xa44d1c bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa45004 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xa45004() -> ! {
    todo!("0xa45004 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xa45198 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE6insertEPNSA_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *)
pub fn stub_0xa45198() -> ! {
    todo!("0xa45198 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *)")
}

// 0xa45458 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotEEaSEPSC_
// type: int32_t **__fastcall(int32_t **, int32_t *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot*)
pub fn stub_0xa45458() -> ! {
    todo!("0xa45458 rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot*)")
}

// 0xa4550c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE13callable_slotINS2_8functionIS9_EEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>>::~callable_slot()
pub fn stub_0xa4550c() -> ! {
    todo!("0xa4550c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>>::~callable_slot()")
}

// 0xa45518 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE13callable_slotINS2_8functionIS9_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>>::~callable_slot()
pub fn stub_0xa45518() -> ! {
    todo!("0xa45518 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>>::~callable_slot()")
}

// 0xa455cc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::disconnect(void)
pub fn stub_0xa455cc() -> ! {
    todo!("0xa455cc rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::disconnect(void)")
}

// 0xa4574c — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::connected(void)const
pub fn stub_0xa4574c() -> ! {
    todo!("0xa4574c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::connected(void)const")
}

// 0xa45758 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotENS3_8functionISA_EELi3ESA_E4callES7_S7_S9_
// type: void __fastcall(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)
pub fn stub_0xa45758() -> ! {
    todo!("0xa45758 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")
}

// 0xa45bb0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotENS3_8functionISA_EELi3ESA_E4callES7_S7_S9_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)
pub fn stub_0xa45bb0() -> ! {
    todo!("0xa45bb0 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")
}

// 0xa45bbc — __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEES4_NS2_13FriendService15FriendEventTypeEEclES4_S4_S6_
// type: void __fastcall(int *, int *, int *, int)
#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)const")]
// was: boost::function3<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)const
pub fn stub_0xa45bbc() -> ! {
    todo!("0xa45bbc boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)const")
}

// 0xa46118 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE6removeEPNSA_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *)
pub fn stub_0xa46118() -> ! {
    todo!("0xa46118 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot *)")
}

// 0xa46204 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE4slot22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::safe_static_init_mutex(void)
pub fn stub_0xa46204() -> ! {
    todo!("0xa46204 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::safe_static_init_mutex(void)")
}

// 0xa462e8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotENS3_8functionISA_EELi3ESA_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()
pub fn stub_0xa462e8() -> ! {
    todo!("0xa462e8 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()")
}

// 0xa46480 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotENS3_8functionISA_EELi3ESA_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()
pub fn stub_0xa46480() -> ! {
    todo!("0xa46480 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()")
}

// 0xa4648c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_NS5_13FriendService15FriendEventTypeEEE4slotENS3_8functionISA_EELi3ESA_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()
pub fn stub_0xa4648c() -> ! {
    todo!("0xa4648c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::~callable()")
}

// 0xa46540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::~slot()
pub fn stub_0xa46540() -> ! {
    todo!("0xa46540 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::~slot()")
}

// 0xa4659c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::~slot()
pub fn stub_0xa4659c() -> ! {
    todo!("0xa4659c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::slot::~slot()")
}

// 0xa466a4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EEC2EMS3_FS7_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xa466a4() -> ! {
    todo!("0xa466a4 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa46934 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED0Ev
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_0xa46934() -> ! {
    todo!("0xa46934 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xa469d4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0xa469d4() -> ! {
    todo!("0xa469d4 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xa46c0c — __ZN3RBX10Reflection11Call1HelperINS_7Network7PlayersEMS3_FN5boost10shared_ptrINS_8InstanceEEES7_ES7_S7_E4callEPS3_S9_RNS0_7VariantERKS7_
// type: void __fastcall(int, char *, int, _DWORD *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Players*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Network::Players,boost::shared_ptr<RBX::Instance> (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::call(RBX::Network::Players*,boost::shared_ptr<RBX::Instance> (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xa46c0c() -> ! {
    todo!("0xa46c0c RBX::Reflection::Call1Helper<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Players*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa47038 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_0xa47038() -> ! {
    todo!("0xa47038 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0xa47114 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0xa47114() -> ! {
    todo!("0xa47114 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xa47358 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS6_INS1_8InstanceEEESaIS9_EEEEEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)
pub use crate::generated_13::stub_a47358 as stub_0xa47358;

// 0xa4752c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EEC2EMS3_FvS7_SsSsEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, struct _Unwind_Exception *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xa4752c() -> ! {
    todo!("0xa4752c RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa47920 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
pub fn stub_0xa47920() -> ! {
    todo!("0xa47920 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")
}

// 0xa479c0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0xa479c0() -> ! {
    todo!("0xa479c0 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xa47d30 — __ZN3RBX10Reflection11Call3HelperINS_7Network7PlayersEMS3_FvN5boost10shared_ptrINS_8InstanceEEESsSsES7_SsSsvE4callEPS3_S9_RNS0_7VariantERKS7_RKSsSH_
// type: void __fastcall(int, void (__fastcall *)(struct _Unwind_Exception *, int *, int *, int *), int, int, int *, std::string *, std::string *)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),rbx_core::SharedPtr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,std::string const&)")]
// was: RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>,std::string,std::string),boost::shared_ptr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,std::string const&,std::string const&)
pub fn stub_0xa47d30() -> ! {
    todo!("0xa47d30 RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),rbx_core::SharedPtr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,std::string const&)")
}

// 0xa49b0c — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_EC2ESD_PKcSG_SG_SG_SG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xa49b0c() -> ! {
    todo!("0xa49b0c RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa4a0a0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_0xa4a0a0() -> ! {
    todo!("0xa4a0a0 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa4a17c — __ZNK3RBX10Reflection13EventDescImplILi4ENS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E14connectGenericEPNS0_11EventSourceENS6_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0xa4a17c() -> ! {
    todo!("0xa4a17c RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xa4a600 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0xa4a600() -> ! {
    todo!("0xa4a600 RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xa4ad50 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0xa4ad50() -> ! {
    todo!("0xa4ad50 RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xa4ad68 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13disconnectAllEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_0xa4ad68() -> ! {
    todo!("0xa4ad68 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")
}

// 0xa4af20 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E7connectEPNS0_11EventSourceERKNS5_8functionIS9_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> const&)const
pub use crate::generated_13::stub_a4af20 as stub_0xa4af20;

// 0xa4b0f4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS1_8InstanceEEERKSsSD_NS9_IS3_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISO_T0_T1_T2_T3_T4_EENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSR_FSO_SS_ST_SU_SV_ESY_SZ_S10_S11_S12_
// type: void __fastcall(_DWORD *, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub use crate::generated_13::stub_a4b0f4 as stub_0xa4b0f4;

// 0xa4b560 — __ZN3RBX10Reflection18GenericSlotWrapper8execute4INS_7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS9_EEvRKT_RKT0_RKT1_RKT2_
// type: int __fastcall(int, _DWORD *, int, const std::string *, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xa4b560() -> ! {
    todo!("0xa4b560 void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa4bd28 — __ZN5boost9function4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSJ_EENSA_5list5INSA_5valueINS5_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
// was: void boost::function4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)
pub fn stub_0xa4bd28() -> ! {
    todo!("0xa4bd28 void boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0xa4c1a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub use crate::generated_13::stub_a4c1a0 as stub_0xa4c1a0;

// 0xa4c1c4 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEvSC_SH_SsSH_E6invokeERNS1_15function_bufferESC_SH_SsSH_
// type: int __fastcall(char ***, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa4c1c4() -> ! {
    todo!("0xa4c1c4 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa4c1f8 — __ZNK5boost6detail8function13basic_vtable4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS3_8InstanceEEESsS9_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS3_10Reflection18GenericSlotWrapperERKS6_RKS9_RKSsSL_EENSC_5list5INSC_5valueINS7_ISH_EEEENS_3argILi1EEENST_ILi2EEENST_ILi3EEENST_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub use crate::generated_13::stub_a4c1f8 as stub_0xa4c1f8;

// 0xa4c4e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub use crate::generated_13::stub_a4c4e0 as stub_0xa4c4e0;

// 0xa4c674 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE6insertEPNSB_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0xa4c674() -> ! {
    todo!("0xa4c674 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0xa4c934 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSEPSD_
// type: int32_t **__fastcall(int32_t **, int32_t *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_0xa4c934() -> ! {
    todo!("0xa4c934 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")
}

// 0xa4c9e8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13callable_slotINS6_8functionISA_EEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_0xa4c9e8() -> ! {
    todo!("0xa4c9e8 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")
}

// 0xa4c9f4 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13callable_slotINS6_8functionISA_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_0xa4c9f4() -> ! {
    todo!("0xa4c9f4 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")
}

// 0xa4caa8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_0xa4caa8() -> ! {
    todo!("0xa4caa8 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")
}

// 0xa4cc28 — __ZNK3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub use crate::generated_13::stub_a4cc28 as stub_0xa4cc28;

// 0xa4cc34 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_E4callES6_SA_SsSA_
// type: void __fastcall(int, int, int *, const std::string *, int *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa4cc34() -> ! {
    todo!("0xa4cc34 rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa4d130 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_E4callES6_SA_SsSA_
// type: void __fastcall(int, int, int *, const std::string *, int *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xa4d130() -> ! {
    todo!("0xa4d130 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa4d148 — __ZNK5boost9function4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_EclES4_S7_SsS7_
// type: void __fastcall(int *, int, int *, const std::string *, int *)
#[doc(alias = "boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0xa4d148() -> ! {
    todo!("0xa4d148 boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xa4d734 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE6removeEPNSB_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0xa4d734() -> ! {
    todo!("0xa4d734 rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0xa4d820 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slot22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub use crate::generated_13::stub_a4d820 as stub_0xa4d820;

// 0xa4d904 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0xa4d904() -> ! {
    todo!("0xa4d904 rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}
