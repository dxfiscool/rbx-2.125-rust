#![allow(unused_attributes, dead_code, unused_variables, non_snake_case, non_camel_case_types, clippy::all)]
//! core shard oh — 100 core boost stubs EA-sorted, next uncovered filtered.
//! Source: ida/export.json (85545 funcs) filtered where (mangled or demangled contains "boost") routed to core (not Reflection/Instance/DataModel/Ogre/RakNet), sorted by EA, next 100 uncovered after existing core stubs.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>)")]
// 0x8149cc — __ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvvENS9_5list0EEEEET_SF_SF_T0_ — __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>)
pub fn stub_0x8149cc() -> ! {
    todo!("0x8149cc __ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvvENS9_5list0EEEEET_SF_SF_T0_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>)")]
// 0x814e78 — __ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvPSt6vectorISsSaISsEES7_S7_ENS9_5list3INS9_5valueISE_EENS8_3argILi1EEENSK_ILi2EEEEEEEEET_SP_SP_T0_ — __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0x814e78() -> ! {
    todo!("0x814e78 __ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvPSt6vectorISsSaISsEES7_S7_ENS9_5list3INS9_5valueISE_EENS8_3argILi1EEENSK_ILi2EEEEEEEEET_SP_SP_T0_")
}

#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)")]
// 0x81c940 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE — void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)
// was: void boost::intrusive_ptr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)
// type: int __fastcall(int32_t *__theValue)
pub fn stub_0x81c940() -> ! {
    todo!("0x81c940 __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token> const&)")]
// 0x87edc4 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY15MegaClusterMeshENS1_15Vector3ComparerEE5TokenEEaSERKSA_ — rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token> const&)
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token> const&)
pub fn stub_0x87edc4() -> ! {
    todo!("0x87edc4 __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY15MegaClusterMeshENS1_15Vector3ComparerEE5TokenEEaSERKSA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)")]
// 0x880160 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY15MegaClusterMeshENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_ — rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)
pub fn stub_0x880160() -> ! {
    todo!("0x880160 __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY15MegaClusterMeshENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)")]
// 0x880234 — __ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY15MegaClusterMeshENS3_15Vector3ComparerEE5TokenEEEPT_ — boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x880234() -> ! {
    todo!("0x880234 __ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY15MegaClusterMeshENS3_15Vector3ComparerEE5TokenEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// 0x880820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEED1Ev — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
pub fn stub_0x880820() -> ! {
    todo!("0x880820 __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// 0x880824 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEED0Ev — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
pub fn stub_0x880824() -> ! {
    todo!("0x880824 __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::dispose(void)")]
// 0x880828 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::dispose(void)
pub fn stub_0x880828() -> ! {
    todo!("0x880828 __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)")]
// 0x8808d0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)
pub fn stub_0x8808d0() -> ! {
    todo!("0x8808d0 __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)")]
// 0x8808d4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)
pub fn stub_0x8808d4() -> ! {
    todo!("0x8808d4 __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::PhysicsSender::start(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
// 0x9c0dd4 — __ZN3RBX7Network13PhysicsSender5startEN5boost10shared_ptrIS1_EE — RBX::Network::PhysicsSender::start(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)
// was: RBX::Network::PhysicsSender::start(boost::shared_ptr<RBX::Network::PhysicsSender>)
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, void *, int, char, char, int, int, int, int)
pub fn stub_0x9c0dd4() -> ! {
    todo!("0x9c0dd4 __ZN3RBX7Network13PhysicsSender5startEN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job>::reset(void)")]
// 0x9c2f6c — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender3JobEE5resetEv — rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job>::reset(void)
// was: boost::shared_ptr<RBX::Network::PhysicsSender::Job>::reset(void)
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9c2f6c() -> ! {
    todo!("0x9c2f6c __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender3JobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob>::reset(void)")]
// 0x9c300c — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender8TouchJobEE5resetEv — rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob>::reset(void)
// was: boost::shared_ptr<RBX::Network::PhysicsSender::TouchJob>::reset(void)
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9c300c() -> ! {
    todo!("0x9c300c __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender8TouchJobEE5resetEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob> *,RBX::Network::PhysicsSender::TouchJob *,boost::detail::shared_count &)")]
// 0x9c3854 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender8TouchJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE — void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob> *,RBX::Network::PhysicsSender::TouchJob *,boost::detail::shared_count &)
// was: void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(boost::shared_ptr<RBX::Network::PhysicsSender::TouchJob> *,RBX::Network::PhysicsSender::TouchJob *,boost::detail::shared_count &)
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0x9c3854() -> ! {
    todo!("0x9c3854 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender8TouchJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob> const*,RBX::Network::PhysicsSender::TouchJob *)const")]
// 0x9c3a04 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender8TouchJobES8_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob> const*,RBX::Network::PhysicsSender::TouchJob *)const
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(boost::shared_ptr<RBX::Network::PhysicsSender::TouchJob> const*,RBX::Network::PhysicsSender::TouchJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0x9c3a04() -> ! {
    todo!("0x9c3a04 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender8TouchJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::~sp_counted_impl_p()")]
// 0x9c3cb0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED1Ev — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::~sp_counted_impl_p()
// type: void()
pub fn stub_0x9c3cb0() -> ! {
    todo!("0x9c3cb0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::~sp_counted_impl_p()")]
// 0x9c3cb4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED0Ev — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::~sp_counted_impl_p()
// type: void __fastcall(void *)
pub fn stub_0x9c3cb4() -> ! {
    todo!("0x9c3cb4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::dispose(void)")]
// 0x9c3cc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::dispose(void)
// type: int __fastcall(int)
pub fn stub_0x9c3cc0() -> ! {
    todo!("0x9c3cc0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::get_deleter(std::type_info const&)")]
// 0x9c3cd4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::get_deleter(std::type_info const&)
// type: int()
pub fn stub_0x9c3cd4() -> ! {
    todo!("0x9c3cd4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::get_untyped_deleter(void)")]
// 0x9c3cd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::get_untyped_deleter(void)
// type: int()
pub fn stub_0x9c3cd8() -> ! {
    todo!("0x9c3cd8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job> *,RBX::Network::PhysicsSender::Job *,boost::detail::shared_count &)")]
// 0x9c3cdc — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE — void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job> *,RBX::Network::PhysicsSender::Job *,boost::detail::shared_count &)
// was: void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(boost::shared_ptr<RBX::Network::PhysicsSender::Job> *,RBX::Network::PhysicsSender::Job *,boost::detail::shared_count &)
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0x9c3cdc() -> ! {
    todo!("0x9c3cdc __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job> const*,RBX::Network::PhysicsSender::Job *)const")]
// 0x9c3e8c — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job> const*,RBX::Network::PhysicsSender::Job *)const
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(boost::shared_ptr<RBX::Network::PhysicsSender::Job> const*,RBX::Network::PhysicsSender::Job *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0x9c3e8c() -> ! {
    todo!("0x9c3e8c __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::~sp_counted_impl_p()")]
// 0x9c4138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED1Ev — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::~sp_counted_impl_p()
// type: void()
pub fn stub_0x9c4138() -> ! {
    todo!("0x9c4138 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::~sp_counted_impl_p()")]
// 0x9c413c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED0Ev — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::~sp_counted_impl_p()
// type: void __fastcall(void *)
pub fn stub_0x9c413c() -> ! {
    todo!("0x9c413c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::dispose(void)")]
// 0x9c4148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::dispose(void)
// type: int __fastcall(int)
pub fn stub_0x9c4148() -> ! {
    todo!("0x9c4148 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::get_deleter(std::type_info const&)")]
// 0x9c415c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::get_deleter(std::type_info const&)
// type: int()
pub fn stub_0x9c415c() -> ! {
    todo!("0x9c415c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::get_untyped_deleter(void)")]
// 0x9c4160 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::get_untyped_deleter(void)
// type: int()
pub fn stub_0x9c4160() -> ! {
    todo!("0x9c4160 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
// 0x9c469c — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev — rbx::signals::signal<void ()(RBX::TouchPair const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>>::~callable_slot()
// type: int __fastcall(int)
pub fn stub_0x9c469c() -> ! {
    todo!("0x9c469c __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
// 0x9c46f8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev — rbx::signals::signal<void ()(RBX::TouchPair const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
pub fn stub_0x9c46f8() -> ! {
    todo!("0x9c46f8 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)")]
// 0x9c4980 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ — rbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)
// type: int __fastcall(_DWORD *)
pub fn stub_0x9c4980() -> ! {
    todo!("0x9c4980 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)")]
// 0x9c499c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ — `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)
// type: int __fastcall(_DWORD *)
pub fn stub_0x9c499c() -> ! {
    todo!("0x9c499c __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::TouchJob(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
// 0x9c56a4 — __ZN3RBX7Network13PhysicsSender8TouchJobC2EN5boost10shared_ptrIS1_EE — RBX::Network::PhysicsSender::TouchJob::TouchJob(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)
// was: RBX::Network::PhysicsSender::TouchJob::TouchJob(boost::shared_ptr<RBX::Network::PhysicsSender>)
// type: RBX::TaskScheduler::Job *__fastcall(RBX::TaskScheduler::Job *, _DWORD *)
pub fn stub_0x9c56a4() -> ! {
    todo!("0x9c56a4 __ZN3RBX7Network13PhysicsSender8TouchJobC2EN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Network::PhysicsSender::Job::Job(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
// 0x9c5fdc — __ZN3RBX7Network13PhysicsSender3JobC2EN5boost10shared_ptrIS1_EE — RBX::Network::PhysicsSender::Job::Job(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)
// was: RBX::Network::PhysicsSender::Job::Job(boost::shared_ptr<RBX::Network::PhysicsSender>)
// type: RBX::TaskScheduler::Job *__fastcall(RBX::TaskScheduler::Job *, _DWORD *)
pub fn stub_0x9c5fdc() -> ! {
    todo!("0x9c5fdc __ZN3RBX7Network13PhysicsSender3JobC2EN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players>::reset(void)")]
// 0x9cb40c — __ZN5boost10shared_ptrIN3RBX7Network7PlayersEE5resetEv — rbx_core::SharedPtr<RBX::Network::Players>::reset(void)
// was: boost::shared_ptr<RBX::Network::Players>::reset(void)
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9cb40c() -> ! {
    todo!("0x9cb40c __ZN5boost10shared_ptrIN3RBX7Network7PlayersEE5resetEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(rbx_core::SharedPtr<RBX::Network::NetworkOwnerJob> *,RBX::Network::NetworkOwnerJob *,boost::detail::shared_count &)")]
// 0x9cb9f0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15NetworkOwnerJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE — void boost::detail::sp_pointer_construct<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(rbx_core::SharedPtr<RBX::Network::NetworkOwnerJob> *,RBX::Network::NetworkOwnerJob *,boost::detail::shared_count &)
// was: void boost::detail::sp_pointer_construct<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(boost::shared_ptr<RBX::Network::NetworkOwnerJob> *,RBX::Network::NetworkOwnerJob *,boost::detail::shared_count &)
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0x9cb9f0() -> ! {
    todo!("0x9cb9f0 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15NetworkOwnerJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(rbx_core::SharedPtr<RBX::Network::NetworkOwnerJob> const*,RBX::Network::NetworkOwnerJob *)const")]
// 0x9cbba0 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network15NetworkOwnerJobES7_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(rbx_core::SharedPtr<RBX::Network::NetworkOwnerJob> const*,RBX::Network::NetworkOwnerJob *)const
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(boost::shared_ptr<RBX::Network::NetworkOwnerJob> const*,RBX::Network::NetworkOwnerJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0x9cbba0() -> ! {
    todo!("0x9cbba0 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network15NetworkOwnerJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::~sp_counted_impl_p()")]
// 0x9cbe4c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEED1Ev — boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::~sp_counted_impl_p()
// type: void()
pub fn stub_0x9cbe4c() -> ! {
    todo!("0x9cbe4c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::~sp_counted_impl_p()")]
// 0x9cbe50 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEED0Ev — boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::~sp_counted_impl_p()
// type: void __fastcall(void *)
pub fn stub_0x9cbe50() -> ! {
    todo!("0x9cbe50 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::dispose(void)")]
// 0x9cbe5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::dispose(void)
// type: int __fastcall(int)
pub fn stub_0x9cbe5c() -> ! {
    todo!("0x9cbe5c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::get_deleter(std::type_info const&)")]
// 0x9cbe70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::get_deleter(std::type_info const&)
// type: int()
pub fn stub_0x9cbe70() -> ! {
    todo!("0x9cbe70 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::get_untyped_deleter(void)")]
// 0x9cbe74 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::get_untyped_deleter(void)
// type: int()
pub fn stub_0x9cbe74() -> ! {
    todo!("0x9cbe74 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ServerReplicator> RBX::shared_from<RBX::Network::ServerReplicator>(RBX::Network::ServerReplicator*)")]
// 0x9e34e0 — __ZN3RBX11shared_fromINS_7Network16ServerReplicatorEEEN5boost10shared_ptrIT_EEPS5_ — rbx_core::SharedPtr<RBX::Network::ServerReplicator> RBX::shared_from<RBX::Network::ServerReplicator>(RBX::Network::ServerReplicator*)
// was: boost::shared_ptr<RBX::Network::ServerReplicator> RBX::shared_from<RBX::Network::ServerReplicator>(RBX::Network::ServerReplicator*)
// type: void __fastcall(int, int)
pub fn stub_0x9e34e0() -> ! {
    todo!("0x9e34e0 __ZN3RBX11shared_fromINS_7Network16ServerReplicatorEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob>::reset(void)")]
// 0x9e5a18 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StreamJobEE5resetEv — rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob>::reset(void)
// was: boost::shared_ptr<RBX::Network::Replicator::StreamJob>::reset(void)
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9e5a18() -> ! {
    todo!("0x9e5a18 __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StreamJobEE5resetEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> *,RBX::Network::Replicator::StreamJob *,boost::detail::shared_count &)")]
// 0x9e63f8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator9StreamJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE — void boost::detail::sp_pointer_construct<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> *,RBX::Network::Replicator::StreamJob *,boost::detail::shared_count &)
// was: void boost::detail::sp_pointer_construct<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(boost::shared_ptr<RBX::Network::Replicator::StreamJob> *,RBX::Network::Replicator::StreamJob *,boost::detail::shared_count &)
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0x9e63f8() -> ! {
    todo!("0x9e63f8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator9StreamJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> const*,RBX::Network::Replicator::StreamJob *)const")]
// 0x9e65a8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator9StreamJobES8_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> const*,RBX::Network::Replicator::StreamJob *)const
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(boost::shared_ptr<RBX::Network::Replicator::StreamJob> const*,RBX::Network::Replicator::StreamJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0x9e65a8() -> ! {
    todo!("0x9e65a8 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator9StreamJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()")]
// 0x9e6854 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED1Ev — boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()
// type: void()
pub fn stub_0x9e6854() -> ! {
    todo!("0x9e6854 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()")]
// 0x9e6858 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED0Ev — boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()
// type: void __fastcall(void *)
pub fn stub_0x9e6858() -> ! {
    todo!("0x9e6858 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::dispose(void)")]
// 0x9e6864 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::dispose(void)
// type: int __fastcall(int)
pub fn stub_0x9e6864() -> ! {
    todo!("0x9e6864 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_deleter(std::type_info const&)")]
// 0x9e6878 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_deleter(std::type_info const&)
// type: int()
pub fn stub_0x9e6878() -> ! {
    todo!("0x9e6878 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_untyped_deleter(void)")]
// 0x9e687c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_untyped_deleter(void)
// type: int()
pub fn stub_0x9e687c() -> ! {
    todo!("0x9e687c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(rbx_core::SharedPtr<RBX::Network::ServerReplicator const> const&)")]
// 0x9e8a8c — __ZN3RBX7Network16ServerReplicator15ServerStatsItemC2ERKN5boost10shared_ptrIKS1_EE — RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(rbx_core::SharedPtr<RBX::Network::ServerReplicator const> const&)
// was: RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(boost::shared_ptr<RBX::Network::ServerReplicator const> const&)
// type: RBX::Stats::Item *__fastcall(RBX::Stats::Item *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Network::Replicator::StatsItem *, int, int, int, int)
pub fn stub_0x9e8a8c() -> ! {
    todo!("0x9e8a8c __ZN3RBX7Network16ServerReplicator15ServerStatsItemC2ERKN5boost10shared_ptrIKS1_EE")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>>(RBX::Network::PropSync::detail::PropertyKey const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> const&)")]
// 0x9ff380 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISC_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEbERSA_RKT_ — std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>>(RBX::Network::PropSync::detail::PropertyKey const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> const&)
// type: int __fastcall(_DWORD *, _DWORD *, unsigned int *, _QWORD **)
pub fn stub_0x9ff380() -> ! {
    todo!("0x9ff380 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISC_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEbERSA_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::reserve_for_insert(unsigned long)")]
// 0x9ff5c0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::reserve_for_insert(unsigned long)
// type: _DWORD *__fastcall(int, unsigned int)
pub fn stub_0x9ff5c0() -> ! {
    todo!("0x9ff5c0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::create_buckets(unsigned long)")]
// 0x9ff768 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::create_buckets(unsigned long)
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_0x9ff768() -> ! {
    todo!("0x9ff768 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> *)")]
// 0x9ffadc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISC_EESM_ — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> *)
// type: int __fastcall(_DWORD *, int, int)
pub fn stub_0x9ffadc() -> ! {
    todo!("0x9ffadc __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISC_EESM_")
}

#[doc(alias = "RBX::Network::ChatMessage::ChatMessage(char const*,RBX::Network::ChatMessage::ChatType,rbx_core::SharedPtr<RBX::Network::Player>)")]
// 0xa097a4 — __ZN3RBX7Network11ChatMessageC2EPKcNS1_8ChatTypeEN5boost10shared_ptrINS0_6PlayerEEE — RBX::Network::ChatMessage::ChatMessage(char const*,RBX::Network::ChatMessage::ChatType,rbx_core::SharedPtr<RBX::Network::Player>)
// was: RBX::Network::ChatMessage::ChatMessage(char const*,RBX::Network::ChatMessage::ChatType,boost::shared_ptr<RBX::Network::Player>)
// type: RBX::Guid *__fastcall(RBX::Guid *, int, int, _DWORD *, struct _Unwind_Exception *lpuexcpt, int, char, char, int, int, int, int, int, int)
pub fn stub_0xa097a4() -> ! {
    todo!("0xa097a4 __ZN3RBX7Network11ChatMessageC2EPKcNS1_8ChatTypeEN5boost10shared_ptrINS0_6PlayerEEE")
}

#[doc(alias = "RBX::Network::ChatMessage::ChatMessage(char const*,RBX::Network::ChatMessage::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,rbx_core::SharedPtr<RBX::Network::Player>)")]
// 0xa09b94 — __ZN3RBX7Network11ChatMessageC2EPKcNS1_8ChatTypeEN5boost10shared_ptrINS0_6PlayerEEES8_ — RBX::Network::ChatMessage::ChatMessage(char const*,RBX::Network::ChatMessage::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,rbx_core::SharedPtr<RBX::Network::Player>)
// was: RBX::Network::ChatMessage::ChatMessage(char const*,RBX::Network::ChatMessage::ChatType,boost::shared_ptr<RBX::Network::Player>,boost::shared_ptr<RBX::Network::Player>)
// type: RBX::Guid *__fastcall(RBX::Guid *, int, int, _DWORD *, struct _Unwind_Exception *lpuexcpt, int, char, char, int, int, int, int, int, int)
pub fn stub_0xa09b94() -> ! {
    todo!("0xa09b94 __ZN3RBX7Network11ChatMessageC2EPKcNS1_8ChatTypeEN5boost10shared_ptrINS0_6PlayerEEES8_")
}

#[doc(alias = "RBX::Network::AbuseReport::addMessage(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&)")]
// 0xa0a15c — __ZN3RBX7Network11AbuseReport10addMessageEN5boost10shared_ptrINS0_6PlayerEEERKNS0_11ChatMessageE — RBX::Network::AbuseReport::addMessage(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&)
// was: RBX::Network::AbuseReport::addMessage(boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&)
// type: void __fastcall(int, int *, const std::string *, int)
pub fn stub_0xa0a15c() -> ! {
    todo!("0xa0a15c __ZN3RBX7Network11AbuseReport10addMessageEN5boost10shared_ptrINS0_6PlayerEEERKNS0_11ChatMessageE")
}

#[doc(alias = "RBX::Network::AbuseReporter::processRequests(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)")]
// 0xa0ac84 — __ZN3RBX7Network13AbuseReporter15processRequestsEN5boost10shared_ptrINS1_4dataEEESs — RBX::Network::AbuseReporter::processRequests(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)
// was: RBX::Network::AbuseReporter::processRequests(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)
// type: int __fastcall(struct _Unwind_Exception **, std::string *)
pub fn stub_0xa0ac84() -> ! {
    todo!("0xa0ac84 __ZN3RBX7Network13AbuseReporter15processRequestsEN5boost10shared_ptrINS1_4dataEEESs")
}

#[doc(alias = "RBX::Network::AbuseReporter::add(RBX::Network::AbuseReport &,rbx_core::SharedPtr<RBX::Network::Player>,std::list<RBX::Network::ChatMessage,std::allocator<RBX::Network::ChatMessage>> const&)")]
// 0xa0ba5c — __ZN3RBX7Network13AbuseReporter3addERNS0_11AbuseReportEN5boost10shared_ptrINS0_6PlayerEEERKSt4listINS0_11ChatMessageESaIS9_EE — RBX::Network::AbuseReporter::add(RBX::Network::AbuseReport &,rbx_core::SharedPtr<RBX::Network::Player>,std::list<RBX::Network::ChatMessage,std::allocator<RBX::Network::ChatMessage>> const&)
// was: RBX::Network::AbuseReporter::add(RBX::Network::AbuseReport &,boost::shared_ptr<RBX::Network::Player>,std::list<RBX::Network::ChatMessage,std::allocator<RBX::Network::ChatMessage>> const&)
// type: void __fastcall(int, _QWORD *, __int32 *, pthread_mutex_t **, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, boost::mutex *, char, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xa0ba5c() -> ! {
    todo!("0xa0ba5c __ZN3RBX7Network13AbuseReporter3addERNS0_11AbuseReportEN5boost10shared_ptrINS0_6PlayerEEERKSt4listINS0_11ChatMessageESaIS9_EE")
}

#[doc(alias = "RBX::Network::Players::friendServiceRequest(bool,rbx_core::WeakPtr<RBX::Network::Player>,int)")]
// 0xa14640 — __ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi — RBX::Network::Players::friendServiceRequest(bool,rbx_core::WeakPtr<RBX::Network::Player>,int)
// was: RBX::Network::Players::friendServiceRequest(bool,boost::weak_ptr<RBX::Network::Player>,int)
// type: void __fastcall(RBX::ServiceProvider *, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xa14640() -> ! {
    todo!("0xa14640 __ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi")
}

#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)")]
// 0xa1b6b0 — __ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_ — boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)
// was: boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)
// type: void __fastcall(int, int, int *, const std::string *)
pub fn stub_0xa1b6b0() -> ! {
    todo!("0xa1b6b0 __ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>)")]
// 0xa1bc50 — __ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_ — boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>)
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>)
// type: void __fastcall(int *, int, int, int *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
pub fn stub_0xa1bc50() -> ! {
    todo!("0xa1bc50 __ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>)")]
// 0xa1bf30 — __ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_ — boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>)
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>)
// type: void __fastcall(int, pthread_mutex_t *, int, int, pthread_mutex_t **)
pub fn stub_0xa1bf30() -> ! {
    todo!("0xa1bf30 __ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)")]
// 0xa1daf8 — __ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_ — rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)
// was: boost::shared_ptr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)
// type: void __fastcall(int, int)
pub fn stub_0xa1daf8() -> ! {
    todo!("0xa1daf8 __ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)")]
// 0xa1e654 — __ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_ — rbx_core::SharedPtr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)
// was: boost::shared_ptr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)
// type: void __fastcall(int, int)
pub fn stub_0xa1e654() -> ! {
    todo!("0xa1e654 __ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)")]
// 0xa20280 — __ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_ — rbx_core::WeakPtr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)
// was: boost::weak_ptr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)
// type: void __fastcall(int, int)
pub fn stub_0xa20280() -> ! {
    todo!("0xa20280 __ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,rbx_core::WeakPtr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>)")]
// 0xa207bc — __ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_ — boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,rbx_core::WeakPtr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>)
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,boost::weak_ptr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>)
// type: void __fastcall(_DWORD *, int, int, int, int *)
pub fn stub_0xa207bc() -> ! {
    todo!("0xa207bc __ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")]
// 0xa20ac8 — __ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_ — rbx_core::WeakPtr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)
// was: boost::weak_ptr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)
// type: void __fastcall(int, int)
pub fn stub_0xa20ac8() -> ! {
    todo!("0xa20ac8 __ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()")]
// 0xa27768 — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED1Ev — rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()
// was: rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()
// type: int()
pub fn stub_0xa27768() -> ! {
    todo!("0xa27768 __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()")]
// 0xa27774 — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED0Ev — rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()
// was: rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()
// type: void __fastcall(void *)
pub fn stub_0xa27774() -> ! {
    todo!("0xa27774 __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)")]
// 0xa279b4 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi — rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)
// was: rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)
// type: int __fastcall(int, pthread_mutex_t *, int)
pub fn stub_0xa279b4() -> ! {
    todo!("0xa279b4 __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)")]
// 0xa279dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi — `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)
// type: int __fastcall(int, pthread_mutex_t *, int)
pub fn stub_0xa279dc() -> ! {
    todo!("0xa279dc __ZThn4_N3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list2<bool &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int> &,boost::_bi::list2<bool &,int &> &,int)")]
// 0xa27a04 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEclINS_4_mfi3mf3IvS5_bSC_iEENS0_5list2IRbRiEEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list2<bool &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int> &,boost::_bi::list2<bool &,int &> &,int)
// was: void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list2<bool &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int> &,boost::_bi::list2<bool &,int &> &,int)
// type: void __fastcall(int *, int, unsigned __int8 **, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
pub fn stub_0xa27a04() -> ! {
    todo!("0xa27a04 __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEclINS_4_mfi3mf3IvS5_bSC_iEENS0_5list2IRbRiEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>::operator()(RBX::Network::Players*,bool,rbx_core::WeakPtr<RBX::Network::Player>,int)const")]
// 0xa27be8 — __ZNK5boost4_mfi3mf3IvN3RBX7Network7PlayersEbNS_8weak_ptrINS3_6PlayerEEEiEclEPS4_bS7_i — boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>::operator()(RBX::Network::Players*,bool,rbx_core::WeakPtr<RBX::Network::Player>,int)const
// was: boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>::operator()(RBX::Network::Players*,bool,boost::weak_ptr<RBX::Network::Player>,int)const
// type: void __fastcall(char **, int, int, int *, int)
pub fn stub_0xa27be8() -> ! {
    todo!("0xa27be8 __ZNK5boost4_mfi3mf3IvN3RBX7Network7PlayersEbNS_8weak_ptrINS3_6PlayerEEEiEclEPS4_bS7_i")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
// 0xa27f9c — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED2Ev — rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()
// was: rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xa27f9c() -> ! {
    todo!("0xa27f9c __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED2Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
// 0xa28174 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()
// was: rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()
// type: int __fastcall(int)
pub fn stub_0xa28174() -> ! {
    todo!("0xa28174 __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
// 0xa28180 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()
// was: rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()
// type: void __fastcall(void *)
pub fn stub_0xa28180() -> ! {
    todo!("0xa28180 __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED0Ev")
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::list4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
// 0xa284b0 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_ — boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::list4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)
// was: boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>::list4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>)
// type: int __fastcall(int, int, int *, int)
pub fn stub_0xa284b0() -> ! {
    todo!("0xa284b0 __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::storage4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
// 0xa28674 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_ — boost::_bi::storage4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::storage4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)
// was: boost::_bi::storage4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>::storage4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>)
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xa28674() -> ! {
    todo!("0xa28674 __ZN5boost3_bi8storage4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
// 0xa28cd4 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED1Ev — rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()
// type: int __fastcall(int)
pub fn stub_0xa28cd4() -> ! {
    todo!("0xa28cd4 __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
// 0xa28d30 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED0Ev — rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
pub fn stub_0xa28d30() -> ! {
    todo!("0xa28d30 __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)")]
// 0xa28fc4 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs — rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)
// type: int __fastcall(int, int)
pub fn stub_0xa28fc4() -> ! {
    todo!("0xa28fc4 __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)")]
// 0xa28fe0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs — `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)
// type: int __fastcall(int, int)
pub fn stub_0xa28fe0() -> ! {
    todo!("0xa28fe0 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool> &,boost::_bi::list1<std::string &> &,int)")]
// 0xa28ffc — __ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS2_IPKcEENS_3argILi1EEENS2_IbEEEclINS_4_mfi3mf4IvS5_iRKSsSK_bEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool> &,boost::_bi::list1<std::string &> &,int)
// type: void __fastcall(int *, char **, _DWORD *)
pub fn stub_0xa28ffc() -> ! {
    todo!("0xa28ffc __ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS2_IPKcEENS_3argILi1EEENS2_IbEEEclINS_4_mfi3mf4IvS5_iRKSsSK_bEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()")]
// 0xa292a8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED1Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()
// type: int __fastcall(int)
pub fn stub_0xa292a8() -> ! {
    todo!("0xa292a8 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()")]
// 0xa29304 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED0Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
pub fn stub_0xa29304() -> ! {
    todo!("0xa29304 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)")]
// 0xa29410 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)
// type: int __fastcall(_DWORD *)
pub fn stub_0xa29410() -> ! {
    todo!("0xa29410 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)")]
// 0xa29430 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv — `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)
// type: int __fastcall(_DWORD *)
pub fn stub_0xa29430() -> ! {
    todo!("0xa29430 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0xa29a78 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED1Ev — rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
// type: int __fastcall(int)
pub fn stub_0xa29a78() -> ! {
    todo!("0xa29a78 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0xa29ad4 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED0Ev — rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
pub fn stub_0xa29ad4() -> ! {
    todo!("0xa29ad4 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
// 0xa29d68 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_ — rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xa29d68() -> ! {
    todo!("0xa29d68 __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
// 0xa29d90 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_ — `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xa29d90() -> ! {
    todo!("0xa29d90 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)")]
// 0xa29db8 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)
// type: void __fastcall(int *, char **, int)
pub fn stub_0xa29db8() -> ! {
    todo!("0xa29db8 __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0xa2a8e8 — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED1Ev — rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()
// type: int __fastcall(int)
pub fn stub_0xa2a8e8() -> ! {
    todo!("0xa2a8e8 __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0xa2a944 — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED0Ev — rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
pub fn stub_0xa2a944() -> ! {
    todo!("0xa2a944 __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
// 0xa2abd8 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs — rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa2abd8() -> ! {
    todo!("0xa2abd8 __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
// 0xa2abf4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs — `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa2abf4() -> ! {
    todo!("0xa2abf4 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list3<std::string &,std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string> &,boost::_bi::list3<std::string &,std::string &,std::string &> &,int)")]
// 0xa2ac10 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf4IvS5_iSsSsSsEENS0_5list3IRSsSJ_SJ_EEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list3<std::string &,std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string> &,boost::_bi::list3<std::string &,std::string &,std::string &> &,int)
// type: void __fastcall(int *, int, const std::string **)
pub fn stub_0xa2ac10() -> ! {
    todo!("0xa2ac10 __ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf4IvS5_iSsSsSsEENS0_5list3IRSsSJ_SJ_EEEEvNS0_4typeIvEERT_RT0_i")
}
