//! core bg21 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct — next 100 uncovered 0xf45c34..0xf4cd64.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr not emitted (boost funcs excluded); single quotes, backticks, double quotes removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::resize(unsigned long,std::string)")]
#[doc(alias = "j___ZNSt6vectorISsSaISsEE6resizeEmSs")]
// 0xf45c34 — j___ZNSt6vectorISsSaISsEE6resizeEmSs
// type: int __fastcall(int, int, int)
pub fn stub_0xf45c34() -> ! {
    todo!("0xf45c34 j___ZNSt6vectorISsSaISsEE6resizeEmSs")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorISsSaISsEED2Ev")]
// 0xf45c44 — j___ZNSt6vectorISsSaISsEED2Ev
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf45c44() -> ! {
    todo!("0xf45c44 j___ZNSt6vectorISsSaISsEED2Ev")
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::resize(unsigned long,unsigned long)")]
#[doc(alias = "j___ZNSt6vectorImSaImEE6resizeEmm")]
// 0xf45c54 — j___ZNSt6vectorImSaImEE6resizeEmm
// type: int __fastcall(int, int, int)
pub fn stub_0xf45c54() -> ! {
    todo!("0xf45c54 j___ZNSt6vectorImSaImEE6resizeEmm")
}

#[doc(alias = "std::vector<double,std::allocator<double>>::resize(unsigned long,double)")]
#[doc(alias = "j___ZNSt6vectorIdSaIdEE6resizeEmd")]
// 0xf45cf4 — j___ZNSt6vectorIdSaIdEE6resizeEmd
// type: int()
pub fn stub_0xf45cf4() -> ! {
    todo!("0xf45cf4 j___ZNSt6vectorIdSaIdEE6resizeEmd")
}

#[doc(alias = "std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_")]
// 0xf48494 — j___ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_
// type: 
pub fn stub_0xf48494() -> ! {
    todo!("0xf48494 j___ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_")
}

#[doc(alias = "std::_Vector_base<std::string,std::allocator<std::string>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseISsSaISsEE11_M_allocateEm")]
// 0xf49db4 — j___ZNSt12_Vector_baseISsSaISsEE11_M_allocateEm
// type: 
pub fn stub_0xf49db4() -> ! {
    todo!("0xf49db4 j___ZNSt12_Vector_baseISsSaISsEE11_M_allocateEm")
}

#[doc(alias = "std::string * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::string *,std::string *>(std::string *,std::string *,std::string *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSsS3_EET0_T_S5_S4_")]
// 0xf49dd4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSsS3_EET0_T_S5_S4_
// type: 
pub fn stub_0xf49dd4() -> ! {
    todo!("0xf49dd4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSsS3_EET0_T_S5_S4_")
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned long *,std::vector<unsigned long,std::allocator<unsigned long>>>,unsigned long,unsigned long const&)")]
#[doc(alias = "j___ZNSt6vectorImSaImEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPmS1_EEmRKm")]
// 0xf49e44 — j___ZNSt6vectorImSaImEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPmS1_EEmRKm
// type: int __fastcall(int, void *__src)
pub fn stub_0xf49e44() -> ! {
    todo!("0xf49e44 j___ZNSt6vectorImSaImEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPmS1_EEmRKm")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<std::string *,unsigned long,std::string>(std::string *,unsigned long,std::string const&,std::__false_type)")]
#[doc(alias = "j___ZSt26__uninitialized_fill_n_auxIPSsmSsEvT_T0_RKT1_St12__false_type")]
// 0xf49e94 — j___ZSt26__uninitialized_fill_n_auxIPSsmSsEvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, void *, int)
pub fn stub_0xf49e94() -> ! {
    todo!("0xf49e94 j___ZSt26__uninitialized_fill_n_auxIPSsmSsEvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> std::operator+<char,std::char_traits<char>,std::allocator<char>>(char const*,std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
#[doc(alias = "j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_EPKS3_RKS6_")]
// 0xf49ea4 — j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_EPKS3_RKS6_
// type: int __fastcall(int, char *__s, int, int, struct _Unwind_Exception *lpuexcpt, std::string *, int, int, int, int)
pub fn stub_0xf49ea4() -> ! {
    todo!("0xf49ea4 j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_EPKS3_RKS6_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(int)>::operator()(int)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFviEEclEi")]
// 0xf4a554 — j___ZN3rbx7signals16signal_with_argsILi1EFviEEclEi
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf4a554() -> ! {
    todo!("0xf4a554 j___ZN3rbx7signals16signal_with_argsILi1EFviEEclEi")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviEE8on_errorERSt9exception")]
// 0xf4a7e4 — j___ZN3rbx7signals6signalIFviEE8on_errorERSt9exception
// type: 
pub fn stub_0xf4a7e4() -> ! {
    todo!("0xf4a7e4 j___ZN3rbx7signals6signalIFviEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::Stats::StatsService::StatsService(void)")]
#[doc(alias = "j___ZN3RBX5Stats12StatsServiceC2Ev")]
// 0xf4bad4 — j___ZN3RBX5Stats12StatsServiceC2Ev
// type: int __fastcall(RBX::Stats::StatsService *this)
pub fn stub_0xf4bad4() -> ! {
    todo!("0xf4bad4 j___ZN3RBX5Stats12StatsServiceC2Ev")
}

#[doc(alias = "RBX::World::reset(void)")]
#[doc(alias = "j___ZN3RBX5World5resetEv")]
// 0xf4bb34 — j___ZN3RBX5World5resetEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_0xf4bb34() -> ! {
    todo!("0xf4bb34 j___ZN3RBX5World5resetEv")
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::push_back(bool (* const&)(void))")]
#[doc(alias = "j___ZNSt6vectorIPFbvESaIS1_EE9push_backERKS1_")]
// 0xf4c104 — j___ZNSt6vectorIPFbvESaIS1_EE9push_backERKS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c104() -> ! {
    todo!("0xf4c104 j___ZNSt6vectorIPFbvESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::push_back(unsigned long * const&)")]
#[doc(alias = "j___ZNSt6vectorIPmSaIS0_EE9push_backERKS0_")]
// 0xf4c124 — j___ZNSt6vectorIPmSaIS0_EE9push_backERKS0_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c124() -> ! {
    todo!("0xf4c124 j___ZNSt6vectorIPmSaIS0_EE9push_backERKS0_")
}

#[doc(alias = "RBX::Body::getBranchIWorld(void)")]
#[doc(alias = "j___ZN3RBX4Body15getBranchIWorldEv")]
// 0xf4c194 — j___ZN3RBX4Body15getBranchIWorldEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_0xf4c194() -> ! {
    todo!("0xf4c194 j___ZN3RBX4Body15getBranchIWorldEv")
}

#[doc(alias = "RBX::Body::getIWorld(void)")]
#[doc(alias = "j___ZN3RBX4Body9getIWorldEv")]
// 0xf4c1a4 — j___ZN3RBX4Body9getIWorldEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_0xf4c1a4() -> ! {
    todo!("0xf4c1a4 j___ZN3RBX4Body9getIWorldEv")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4BodyEEC2Ev")]
// 0xf4c1b4 — j___ZN3RBX9AllocatorINS_4BodyEEC2Ev
// type: 
pub fn stub_0xf4c1b4() -> ! {
    todo!("0xf4c1b4 j___ZN3RBX9AllocatorINS_4BodyEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4BodyEEdlEPv")]
// 0xf4c1c4 — j___ZN3RBX9AllocatorINS_4BodyEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c1c4() -> ! {
    todo!("0xf4c1c4 j___ZN3RBX9AllocatorINS_4BodyEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4CofmEEdlEPv")]
// 0xf4c1d4 — j___ZN3RBX9AllocatorINS_4CofmEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c1d4() -> ! {
    todo!("0xf4c1d4 j___ZN3RBX9AllocatorINS_4CofmEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4CofmEEnwEm")]
// 0xf4c1e4 — j___ZN3RBX9AllocatorINS_4CofmEEnwEm
// type: 
pub fn stub_0xf4c1e4() -> ! {
    todo!("0xf4c1e4 j___ZN3RBX9AllocatorINS_4CofmEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_7SimBodyEEdlEPv")]
// 0xf4c1f4 — j___ZN3RBX9AllocatorINS_7SimBodyEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c1f4() -> ! {
    todo!("0xf4c1f4 j___ZN3RBX9AllocatorINS_7SimBodyEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_7SimBodyEEnwEm")]
// 0xf4c204 — j___ZN3RBX9AllocatorINS_7SimBodyEEnwEm
// type: 
pub fn stub_0xf4c204() -> ! {
    todo!("0xf4c204 j___ZN3RBX9AllocatorINS_7SimBodyEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4CofmEEC2Ev")]
// 0xf4c264 — j___ZN3RBX9AllocatorINS_4CofmEEC2Ev
// type: 
pub fn stub_0xf4c264() -> ! {
    todo!("0xf4c264 j___ZN3RBX9AllocatorINS_4CofmEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv")]
// 0xf4c284 — j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c284() -> ! {
    todo!("0xf4c284 j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv")
}

#[doc(alias = "RBX::PairParams::operator==(RBX::PairParams const&)")]
#[doc(alias = "j___ZN3RBX10PairParamseqERKS0_")]
// 0xf4c294 — j___ZN3RBX10PairParamseqERKS0_
// type: 
pub fn stub_0xf4c294() -> ! {
    todo!("0xf4c294 j___ZN3RBX10PairParamseqERKS0_")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv")]
// 0xf4c2a4 — j___ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c2a4() -> ! {
    todo!("0xf4c2a4 j___ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv")]
// 0xf4c2b4 — j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c2b4() -> ! {
    todo!("0xf4c2b4 j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv")
}

#[doc(alias = "RBX::IndexArray<RBX::Body,&RBX::Body::getLeafBodyIndex>::fastRemove(RBX::Body*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c404 — j___ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c404() -> ! {
    todo!("0xf4c404 j___ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Point,&RBX::Point::getKernelIndex>::fastRemove(RBX::Point*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c414 — j___ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_
// type: int(void)
pub fn stub_0xf4c414() -> ! {
    todo!("0xf4c414 j___ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getContactBodyIndex>::fastRemove(RBX::SimBody*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c434 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c434() -> ! {
    todo!("0xf4c434 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getFreeFallBodyIndex>::fastRemove(RBX::SimBody*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c444 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c444() -> ! {
    todo!("0xf4c444 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getRealTimeBodyIndex>::fastRemove(RBX::SimBody*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c454 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c454() -> ! {
    todo!("0xf4c454 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getContactIndex>::fastRemove(RBX::Connector*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c474 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c474() -> ! {
    todo!("0xf4c474 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getRealTimeIndex>::fastRemove(RBX::Connector*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c494 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c494() -> ! {
    todo!("0xf4c494 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getSecondPassIndex>::fastRemove(RBX::Connector*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_")]
// 0xf4c4a4 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf4c4a4() -> ! {
    todo!("0xf4c4a4 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::KernelData::insertBody(RBX::Body *)")]
#[doc(alias = "j___ZN3RBX10KernelData10insertBodyEPNS_4BodyE")]
// 0xf4c4b4 — j___ZN3RBX10KernelData10insertBodyEPNS_4BodyE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::Body *)
pub fn stub_0xf4c4b4() -> ! {
    todo!("0xf4c4b4 j___ZN3RBX10KernelData10insertBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::removeBody(RBX::Body *)")]
#[doc(alias = "j___ZN3RBX10KernelData10removeBodyEPNS_4BodyE")]
// 0xf4c4c4 — j___ZN3RBX10KernelData10removeBodyEPNS_4BodyE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::Body *)
pub fn stub_0xf4c4c4() -> ! {
    todo!("0xf4c4c4 j___ZN3RBX10KernelData10removeBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::addLeafBody(RBX::Body *)")]
#[doc(alias = "j___ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE")]
// 0xf4c4d4 — j___ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::Body *)
pub fn stub_0xf4c4d4() -> ! {
    todo!("0xf4c4d4 j___ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::addConnector(RBX::Connector *)")]
#[doc(alias = "j___ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE")]
// 0xf4c4e4 — j___ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::Connector *)
pub fn stub_0xf4c4e4() -> ! {
    todo!("0xf4c4e4 j___ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::KernelData::addLeafBodies(RBX::Body *)")]
#[doc(alias = "j___ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE")]
// 0xf4c4f4 — j___ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::Body *)
pub fn stub_0xf4c4f4() -> ! {
    todo!("0xf4c4f4 j___ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::removeConnector(RBX::Connector *)")]
#[doc(alias = "j___ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE")]
// 0xf4c504 — j___ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::Connector *)
pub fn stub_0xf4c504() -> ! {
    todo!("0xf4c504 j___ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::KernelData::addBodyToNewList(RBX::SimBody *)")]
#[doc(alias = "j___ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE")]
// 0xf4c514 — j___ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::SimBody *)
pub fn stub_0xf4c514() -> ! {
    todo!("0xf4c514 j___ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE")
}

#[doc(alias = "RBX::KernelData::removeLeafBodies(RBX::Body *)")]
#[doc(alias = "j___ZN3RBX10KernelData16removeLeafBodiesEPNS_4BodyE")]
// 0xf4c524 — j___ZN3RBX10KernelData16removeLeafBodiesEPNS_4BodyE
// type: int __fastcall(RBX::KernelData *this, RBX::Body *)
pub fn stub_0xf4c524() -> ! {
    todo!("0xf4c524 j___ZN3RBX10KernelData16removeLeafBodiesEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::removeBodyFromCurrentList(RBX::SimBody *)")]
#[doc(alias = "j___ZN3RBX10KernelData25removeBodyFromCurrentListEPNS_7SimBodyE")]
// 0xf4c534 — j___ZN3RBX10KernelData25removeBodyFromCurrentListEPNS_7SimBodyE
// type: _DWORD __fastcall(RBX::KernelData *__hidden this, RBX::SimBody *)
pub fn stub_0xf4c534() -> ! {
    todo!("0xf4c534 j___ZN3RBX10KernelData25removeBodyFromCurrentListEPNS_7SimBodyE")
}

#[doc(alias = "RBX::KernelData::KernelData(void)")]
#[doc(alias = "j___ZN3RBX10KernelDataC2Ev")]
// 0xf4c544 — j___ZN3RBX10KernelDataC2Ev
// type: _DWORD __fastcall(RBX::KernelData *__hidden this)
pub fn stub_0xf4c544() -> ! {
    todo!("0xf4c544 j___ZN3RBX10KernelDataC2Ev")
}

#[doc(alias = "RBX::KernelData::~KernelData()")]
#[doc(alias = "j___ZN3RBX10KernelDataD2Ev")]
// 0xf4c554 — j___ZN3RBX10KernelDataD2Ev
// type: void __fastcall(RBX::KernelData *__hidden this)
pub fn stub_0xf4c554() -> ! {
    todo!("0xf4c554 j___ZN3RBX10KernelDataD2Ev")
}

#[doc(alias = "RBX::KernelIndex::~KernelIndex()")]
#[doc(alias = "j___ZN3RBX11KernelIndexD2Ev")]
// 0xf4c564 — j___ZN3RBX11KernelIndexD2Ev
// type: void __fastcall(RBX::KernelIndex *__hidden this)
pub fn stub_0xf4c564() -> ! {
    todo!("0xf4c564 j___ZN3RBX11KernelIndexD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv")]
// 0xf4c584 — j___ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c584() -> ! {
    todo!("0xf4c584 j___ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv")]
// 0xf4c594 — j___ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c594() -> ! {
    todo!("0xf4c594 j___ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv")]
// 0xf4c5a4 — j___ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c5a4() -> ! {
    todo!("0xf4c5a4 j___ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv")]
// 0xf4c5b4 — j___ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c5b4() -> ! {
    todo!("0xf4c5b4 j___ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv")]
// 0xf4c5c4 — j___ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c5c4() -> ! {
    todo!("0xf4c5c4 j___ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv")]
// 0xf4c5d4 — j___ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4c5d4() -> ! {
    todo!("0xf4c5d4 j___ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_7SimBodyEEC2Ev")]
// 0xf4c5e4 — j___ZN3RBX9AllocatorINS_7SimBodyEEC2Ev
// type: 
pub fn stub_0xf4c5e4() -> ! {
    todo!("0xf4c5e4 j___ZN3RBX9AllocatorINS_7SimBodyEEC2Ev")
}

#[doc(alias = "XmlNameValuePair::setValue(std::string)")]
#[doc(alias = "j___ZN16XmlNameValuePair8setValueESs")]
// 0xf4c614 — j___ZN16XmlNameValuePair8setValueESs
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c614() -> ! {
    todo!("0xf4c614 j___ZN16XmlNameValuePair8setValueESs")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Axes>(RBX::Axes const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4AxesEEERS3_RKT_")]
// 0xf4c914 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4AxesEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c914() -> ! {
    todo!("0xf4c914 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4AxesEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::UDim>(RBX::UDim const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4UDimEEERS3_RKT_")]
// 0xf4c924 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4UDimEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c924() -> ! {
    todo!("0xf4c924 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4UDimEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::UDim2>(RBX::UDim2 const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5UDim2EEERS3_RKT_")]
// 0xf4c934 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5UDim2EEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c934() -> ! {
    todo!("0xf4c934 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5UDim2EEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::RbxRay>(RBX::RbxRay const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6RbxRayEEERS3_RKT_")]
// 0xf4c944 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6RbxRayEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4c944() -> ! {
    todo!("0xf4c944 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6RbxRayEEERS3_RKT_")
}

#[doc(alias = "RBX::BrickColor * rbx::any_cast<RBX::BrickColor,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX10BrickColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca14 — j___ZN3rbx8any_castIN3RBX10BrickColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4ca14() -> ! {
    todo!("0xf4ca14 j___ZN3rbx8any_castIN3RBX10BrickColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3int16 * rbx::any_cast<RBX::Region3int16,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX12Region3int16ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca24 — j___ZN3rbx8any_castIN3RBX12Region3int16ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0xf4ca24() -> ! {
    todo!("0xf4ca24 j___ZN3rbx8any_castIN3RBX12Region3int16ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::SystemAddress * rbx::any_cast<RBX::SystemAddress,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX13SystemAddressENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca34 — j___ZN3rbx8any_castIN3RBX13SystemAddressENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4ca34() -> ! {
    todo!("0xf4ca34 j___ZN3rbx8any_castIN3RBX13SystemAddressENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Axes * rbx::any_cast<RBX::Axes,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX4AxesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca54 — j___ZN3rbx8any_castIN3RBX4AxesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4ca54() -> ! {
    todo!("0xf4ca54 j___ZN3rbx8any_castIN3RBX4AxesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Faces * rbx::any_cast<RBX::Faces,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX5FacesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca64 — j___ZN3rbx8any_castIN3RBX5FacesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4ca64() -> ! {
    todo!("0xf4ca64 j___ZN3rbx8any_castIN3RBX5FacesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UDim2 * rbx::any_cast<RBX::UDim2,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX5UDim2ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca74 — j___ZN3rbx8any_castIN3RBX5UDim2ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4ca74() -> ! {
    todo!("0xf4ca74 j___ZN3rbx8any_castIN3RBX5UDim2ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::RbxRay * rbx::any_cast<RBX::RbxRay,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX6RbxRayENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca84 — j___ZN3rbx8any_castIN3RBX6RbxRayENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0xf4ca84() -> ! {
    todo!("0xf4ca84 j___ZN3rbx8any_castIN3RBX6RbxRayENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 * rbx::any_cast<RBX::Region3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX7Region3ES2_EEPT_PNS_13placement_anyIT0_EE")]
// 0xf4ca94 — j___ZN3rbx8any_castIN3RBX7Region3ES2_EEPT_PNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0xf4ca94() -> ! {
    todo!("0xf4ca94 j___ZN3rbx8any_castIN3RBX7Region3ES2_EEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::NormalId * rbx::any_cast<RBX::NormalId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX8NormalIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4caa4 — j___ZN3rbx8any_castIN3RBX8NormalIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4caa4() -> ! {
    todo!("0xf4caa4 j___ZN3rbx8any_castIN3RBX8NormalIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ContentId * rbx::any_cast<RBX::ContentId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX9ContentIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4cab4 — j___ZN3rbx8any_castIN3RBX9ContentIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0xf4cab4() -> ! {
    todo!("0xf4cab4 j___ZN3rbx8any_castIN3RBX9ContentIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::BrickColor & rbx::any_cast<RBX::BrickColor &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cb54 — j___ZN3rbx8any_castIRN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cb54() -> ! {
    todo!("0xf4cb54 j___ZN3rbx8any_castIRN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3int16 & rbx::any_cast<RBX::Region3int16 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cb64 — j___ZN3rbx8any_castIRN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cb64() -> ! {
    todo!("0xf4cb64 j___ZN3rbx8any_castIRN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Axes & rbx::any_cast<RBX::Axes &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cb74 — j___ZN3rbx8any_castIRN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cb74() -> ! {
    todo!("0xf4cb74 j___ZN3rbx8any_castIRN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Faces & rbx::any_cast<RBX::Faces &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cb84 — j___ZN3rbx8any_castIRN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cb84() -> ! {
    todo!("0xf4cb84 j___ZN3rbx8any_castIRN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UDim2 & rbx::any_cast<RBX::UDim2 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cb94 — j___ZN3rbx8any_castIRN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cb94() -> ! {
    todo!("0xf4cb94 j___ZN3rbx8any_castIRN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::RbxRay & rbx::any_cast<RBX::RbxRay &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cba4 — j___ZN3rbx8any_castIRN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cba4() -> ! {
    todo!("0xf4cba4 j___ZN3rbx8any_castIRN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 & rbx::any_cast<RBX::Region3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")]
// 0xf4cbb4 — j___ZN3rbx8any_castIRN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cbb4() -> ! {
    todo!("0xf4cbb4 j___ZN3rbx8any_castIRN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::NormalId & rbx::any_cast<RBX::NormalId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cbc4 — j___ZN3rbx8any_castIRN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cbc4() -> ! {
    todo!("0xf4cbc4 j___ZN3rbx8any_castIRN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ContentId & rbx::any_cast<RBX::ContentId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cbd4 — j___ZN3rbx8any_castIRN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cbd4() -> ! {
    todo!("0xf4cbd4 j___ZN3rbx8any_castIRN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "bool & rbx::any_cast<bool &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cbe4 — j___ZN3rbx8any_castIRbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4cbe4() -> ! {
    todo!("0xf4cbe4 j___ZN3rbx8any_castIRbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "double & rbx::any_cast<double &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cbf4 — j___ZN3rbx8any_castIRdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cbf4() -> ! {
    todo!("0xf4cbf4 j___ZN3rbx8any_castIRdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "float & rbx::any_cast<float &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc04 — j___ZN3rbx8any_castIRfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cc04() -> ! {
    todo!("0xf4cc04 j___ZN3rbx8any_castIRfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "int & rbx::any_cast<int &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc14 — j___ZN3rbx8any_castIRiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cc14() -> ! {
    todo!("0xf4cc14 j___ZN3rbx8any_castIRiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::string * rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf4cc24 — j___ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf4cc24() -> ! {
    todo!("0xf4cc24 j___ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "bool rbx::any_cast<bool,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc34 — j___ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4cc34() -> ! {
    todo!("0xf4cc34 j___ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "double rbx::any_cast<double,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc44 — j___ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4cc44() -> ! {
    todo!("0xf4cc44 j___ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "float rbx::any_cast<float,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc54 — j___ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4cc54() -> ! {
    todo!("0xf4cc54 j___ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "int rbx::any_cast<int,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc64 — j___ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4cc64() -> ! {
    todo!("0xf4cc64 j___ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "long rbx::any_cast<long,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf4cc74 — j___ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf4cc74() -> ! {
    todo!("0xf4cc74 j___ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Vector_base<RBX::NormalId,std::allocator<RBX::NormalId>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX8NormalIdESaIS1_EE11_M_allocateEm")]
// 0xf4ccd4 — j___ZNSt12_Vector_baseIN3RBX8NormalIdESaIS1_EE11_M_allocateEm
// type: 
pub fn stub_0xf4ccd4() -> ! {
    todo!("0xf4ccd4 j___ZNSt12_Vector_baseIN3RBX8NormalIdESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::NormalId * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::NormalId *,RBX::NormalId *>(RBX::NormalId *,RBX::NormalId *,RBX::NormalId *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NormalIdES5_EET0_T_S7_S6_")]
// 0xf4cce4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NormalIdES5_EET0_T_S7_S6_
// type: 
pub fn stub_0xf4cce4() -> ! {
    todo!("0xf4cce4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NormalIdES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::NormalId,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_8NormalIdESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")]
// 0xf4ccf4 — j___ZNSt3mapIPKN3RBX4NameENS0_8NormalIdESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4ccf4() -> ! {
    todo!("0xf4ccf4 j___ZNSt3mapIPKN3RBX4NameENS0_8NormalIdESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NormalId*,std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>>,RBX::NormalId const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf4cd04 — j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int(void)
pub fn stub_0xf4cd04() -> ! {
    todo!("0xf4cd04 j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NormalId*,std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>>,unsigned long,RBX::NormalId const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// 0xf4cd14 — j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: 
pub fn stub_0xf4cd14() -> ! {
    todo!("0xf4cd14 j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::resize(unsigned long,RBX::NormalId)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE6resizeEmS1_")]
// 0xf4cd24 — j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE6resizeEmS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf4cd24() -> ! {
    todo!("0xf4cd24 j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::push_back(RBX::NormalId const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE9push_backERKS1_")]
// 0xf4cd34 — j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE9push_backERKS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4cd34() -> ! {
    todo!("0xf4cd34 j___ZNSt6vectorIN3RBX8NormalIdESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::NormalId> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// 0xf4cd44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf4cd44() -> ! {
    todo!("0xf4cd44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::NormalId>>,std::pair<RBX::Name const* const,RBX::NormalId> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// 0xf4cd54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf4cd54() -> ! {
    todo!("0xf4cd54 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NormalId>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0xf4cd64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4cd64() -> ! {
    todo!("0xf4cd64 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}
