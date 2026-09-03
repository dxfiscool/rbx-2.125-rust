//! core bg15 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in /tmp/global_eas.txt — next 100 uncovered after 0xf30bd4 (prior max 0xf30bd4) -> 0xf30c14..0xf32de4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
#[doc(alias = "j___ZN3RBX14BaseThreadPool8PoolDataC2Ev")]
// 0xf30c14 — j___ZN3RBX14BaseThreadPool8PoolDataC2Ev
// type: _DWORD __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
pub fn stub_0xf30c14() -> ! {
    todo!("0xf30c14 j___ZN3RBX14BaseThreadPool8PoolDataC2Ev")
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
#[doc(alias = "j___ZN3RBX14BaseThreadPool8PoolDataD2Ev")]
// 0xf30c24 — j___ZN3RBX14BaseThreadPool8PoolDataD2Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
pub fn stub_0xf30c24() -> ! {
    todo!("0xf30c24 j___ZN3RBX14BaseThreadPool8PoolDataD2Ev")
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
#[doc(alias = "j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_")]
// 0xf30c54 — j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, char, int, int, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xf30c54() -> ! {
    todo!("0xf30c54 j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_")
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
#[doc(alias = "j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_")]
// 0xf30c64 — j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
// type: int __fastcall(int, int, int, int, char, int, int, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xf30c64() -> ! {
    todo!("0xf30c64 j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_")
}

#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm")]
// 0xf30e84 — j___ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
pub fn stub_0xf30e84() -> ! {
    todo!("0xf30e84 j___ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_")]
// 0xf30eb4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf30eb4() -> ! {
    todo!("0xf30eb4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf30f64 — j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: 
pub fn stub_0xf30f64() -> ! {
    todo!("0xf30f64 j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_")]
// 0xf30f74 — j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf30f74() -> ! {
    todo!("0xf30f74 j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev")]
// 0xf30f84 — j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf30f84() -> ! {
    todo!("0xf30f84 j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev")
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
#[doc(alias = "j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
// 0xf31014 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf31014() -> ! {
    todo!("0xf31014 j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
// 0xf31024 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf31024() -> ! {
    todo!("0xf31024 j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")
}

#[doc(alias = "RBX::Accoutrement::getRenderSize(void)")]
#[doc(alias = "j___ZN3RBX12Accoutrement13getRenderSizeEv")]
// 0xf310d4 — j___ZN3RBX12Accoutrement13getRenderSizeEv
// type: _DWORD __fastcall(RBX::Accoutrement *__hidden this)
pub fn stub_0xf310d4() -> ! {
    todo!("0xf310d4 j___ZN3RBX12Accoutrement13getRenderSizeEv")
}

#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "j___ZN3RBX11PVAdornmentD1Ev")]
// 0xf312d4 — j___ZN3RBX11PVAdornmentD1Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
pub fn stub_0xf312d4() -> ! {
    todo!("0xf312d4 j___ZN3RBX11PVAdornmentD1Ev")
}

#[doc(alias = "RBX::AnimationId const& rbx::any_cast<RBX::AnimationId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf31374 — j___ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf31374() -> ! {
    todo!("0xf31374 j___ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "j___ZN3RBX19AnimationTrackStateD0Ev")]
// 0xf314e4 — j___ZN3RBX19AnimationTrackStateD0Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
pub fn stub_0xf314e4() -> ! {
    todo!("0xf314e4 j___ZN3RBX19AnimationTrackStateD0Ev")
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "j___ZN3RBX19AnimationTrackStateD2Ev")]
// 0xf314f4 — j___ZN3RBX19AnimationTrackStateD2Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
pub fn stub_0xf314f4() -> ! {
    todo!("0xf314f4 j___ZN3RBX19AnimationTrackStateD2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvffEEC2Ev")]
// 0xf31514 — j___ZN3rbx13remote_signalIFvffEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf31514() -> ! {
    todo!("0xf31514 j___ZN3rbx13remote_signalIFvffEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvffEED2Ev")]
// 0xf31524 — j___ZN3rbx13remote_signalIFvffEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf31524() -> ! {
    todo!("0xf31524 j___ZN3rbx13remote_signalIFvffEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvfffEEC2Ev")]
// 0xf31534 — j___ZN3rbx13remote_signalIFvfffEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf31534() -> ! {
    todo!("0xf31534 j___ZN3rbx13remote_signalIFvfffEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvfffEED2Ev")]
// 0xf31544 — j___ZN3rbx13remote_signalIFvfffEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf31544() -> ! {
    todo!("0xf31544 j___ZN3rbx13remote_signalIFvfffEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvffffEEC2Ev")]
// 0xf31554 — j___ZN3rbx13remote_signalIFvffffEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf31554() -> ! {
    todo!("0xf31554 j___ZN3rbx13remote_signalIFvffffEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvffffEED2Ev")]
// 0xf31564 — j___ZN3rbx13remote_signalIFvffffEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf31564() -> ! {
    todo!("0xf31564 j___ZN3rbx13remote_signalIFvffffEED2Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff")]
// 0xf31574 — j___ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf31574() -> ! {
    todo!("0xf31574 j___ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff")
}

#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff")]
// 0xf31584 — j___ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
// type: int __fastcall(int, int, int, int, float)
pub fn stub_0xf31584() -> ! {
    todo!("0xf31584 j___ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvfffEE13disconnectAllEv")]
// 0xf315a4 — j___ZN3rbx7signals6signalIFvfffEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf315a4() -> ! {
    todo!("0xf315a4 j___ZN3rbx7signals6signalIFvfffEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv")]
// 0xf315b4 — j___ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf315b4() -> ! {
    todo!("0xf315b4 j___ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv")]
// 0xf315d4 — j___ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf315d4() -> ! {
    todo!("0xf315d4 j___ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE")]
// 0xf315e4 — j___ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf315e4() -> ! {
    todo!("0xf315e4 j___ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE")]
// 0xf315f4 — j___ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf315f4() -> ! {
    todo!("0xf315f4 j___ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception")]
// 0xf31624 — j___ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
// type: 
pub fn stub_0xf31624() -> ! {
    todo!("0xf31624 j___ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvffffEE13disconnectAllEv")]
// 0xf31634 — j___ZN3rbx7signals6signalIFvffffEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf31634() -> ! {
    todo!("0xf31634 j___ZN3rbx7signals6signalIFvffffEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv")]
// 0xf31644 — j___ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf31644() -> ! {
    todo!("0xf31644 j___ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv")]
// 0xf31664 — j___ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf31664() -> ! {
    todo!("0xf31664 j___ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE")]
// 0xf31674 — j___ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf31674() -> ! {
    todo!("0xf31674 j___ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE")]
// 0xf31684 — j___ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf31684() -> ! {
    todo!("0xf31684 j___ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception")]
// 0xf316b4 — j___ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
// type: 
pub fn stub_0xf316b4() -> ! {
    todo!("0xf316b4 j___ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception")
}

#[doc(alias = "std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm")]
// 0xf31a64 — j___ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31a64() -> ! {
    todo!("0xf31a64 j___ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm")]
// 0xf31a74 — j___ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31a74() -> ! {
    todo!("0xf31a74 j___ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")]
// 0xf31a84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31a84() -> ! {
    todo!("0xf31a84 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_")]
// 0xf31ab4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31ab4() -> ! {
    todo!("0xf31ab4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_")
}

#[doc(alias = "RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")]
// 0xf31ac4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31ac4() -> ! {
    todo!("0xf31ac4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf31ad4 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: 
pub fn stub_0xf31ad4() -> ! {
    todo!("0xf31ad4 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm")]
// 0xf31ae4 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31ae4() -> ! {
    todo!("0xf31ae4 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_")]
// 0xf31af4 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31af4() -> ! {
    todo!("0xf31af4 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_")]
// 0xf31b04 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31b04() -> ! {
    todo!("0xf31b04 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_")
}

#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf31b14 — j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf31b14() -> ! {
    todo!("0xf31b14 j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_")]
// 0xf31b24 — j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf31b24() -> ! {
    todo!("0xf31b24 j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "j___ZN3RBX10ArcHandlesD2Ev")]
// 0xf31b44 — j___ZN3RBX10ArcHandlesD2Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
pub fn stub_0xf31b44() -> ! {
    todo!("0xf31b44 j___ZN3RBX10ArcHandlesD2Ev")
}

#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "j___ZN3RBX11HandlesBaseD2Ev")]
// 0xf31be4 — j___ZN3RBX11HandlesBaseD2Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
pub fn stub_0xf31be4() -> ! {
    todo!("0xf31be4 j___ZN3RBX11HandlesBaseD2Ev")
}

#[doc(alias = "RBX::BadgeService::~BadgeService()")]
#[doc(alias = "j___ZN3RBX12BadgeServiceD0Ev")]
// 0xf32284 — j___ZN3RBX12BadgeServiceD0Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
pub fn stub_0xf32284() -> ! {
    todo!("0xf32284 j___ZN3RBX12BadgeServiceD0Ev")
}

#[doc(alias = "RBX::BadgeService::~BadgeService()")]
#[doc(alias = "j___ZN3RBX12BadgeServiceD2Ev")]
// 0xf32294 — j___ZN3RBX12BadgeServiceD2Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
pub fn stub_0xf32294() -> ! {
    todo!("0xf32294 j___ZN3RBX12BadgeServiceD2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsEEC2Ev")]
// 0xf322b4 — j___ZN3rbx13remote_signalIFvSsEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf322b4() -> ! {
    todo!("0xf322b4 j___ZN3rbx13remote_signalIFvSsEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsEED2Ev")]
// 0xf322c4 — j___ZN3rbx13remote_signalIFvSsEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf322c4() -> ! {
    todo!("0xf322c4 j___ZN3rbx13remote_signalIFvSsEED2Ev")
}

#[doc(alias = "std::map<int,std::set<int,std::less<int>,std::allocator<int>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::operator[](int const&)")]
#[doc(alias = "j___ZNSt3mapIiSt3setIiSt4lessIiESaIiEES2_SaISt4pairIKiS4_EEEixERS6_")]
// 0xf32504 — j___ZNSt3mapIiSt3setIiSt4lessIiESaIiEES2_SaISt4pairIKiS4_EEEixERS6_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32504() -> ! {
    todo!("0xf32504 j___ZNSt3mapIiSt3setIiSt4lessIiESaIiEES2_SaISt4pairIKiS4_EEEixERS6_")
}

#[doc(alias = "std::map<int,bool,std::less<int>,std::allocator<std::pair<int const,bool>>>::operator[](int const&)")]
#[doc(alias = "j___ZNSt3mapIibSt4lessIiESaISt4pairIKibEEEixERS3_")]
// 0xf32514 — j___ZNSt3mapIibSt4lessIiESaISt4pairIKibEEEixERS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32514() -> ! {
    todo!("0xf32514 j___ZNSt3mapIibSt4lessIiESaISt4pairIKibEEEixERS3_")
}

#[doc(alias = "std::list<RBX::BadgeService::HotUserHasBadge,std::allocator<RBX::BadgeService::HotUserHasBadge>>::erase(std::_List_iterator<RBX::BadgeService::HotUserHasBadge>,std::_List_iterator<RBX::BadgeService::HotUserHasBadge>)")]
#[doc(alias = "j___ZNSt4listIN3RBX12BadgeService15HotUserHasBadgeESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_")]
// 0xf32524 — j___ZNSt4listIN3RBX12BadgeService15HotUserHasBadgeESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
// type: int __fastcall(int, std::_List_node_base *this)
pub fn stub_0xf32524() -> ! {
    todo!("0xf32524 j___ZNSt4listIN3RBX12BadgeService15HotUserHasBadgeESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_create_node(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_")]
// 0xf32534 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf32534() -> ! {
    todo!("0xf32534 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_")]
// 0xf32544 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf32544() -> ! {
    todo!("0xf32544 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// 0xf32554 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf32554() -> ! {
    todo!("0xf32554 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0xf32564 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32564() -> ! {
    todo!("0xf32564 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_")]
// 0xf32574 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf32574() -> ! {
    todo!("0xf32574 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert_unique(std::pair<int const,bool> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xf32584 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_
// type: 
pub fn stub_0xf32584() -> ! {
    todo!("0xf32584 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,bool>>,std::pair<int const,bool> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xf32594 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf32594() -> ! {
    todo!("0xf32594 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,bool>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf325a4 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf325a4() -> ! {
    todo!("0xf325a4 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,bool> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xf325b4 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf325b4() -> ! {
    todo!("0xf325b4 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert_unique(int const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi")]
// 0xf325c4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf325c4() -> ! {
    todo!("0xf325c4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_copy(std::_Rb_tree_node<int> const*,std::_Rb_tree_node<int>*)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_")]
// 0xf325d4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf325d4() -> ! {
    todo!("0xf325d4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_erase(std::_Rb_tree_node<int> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE")]
// 0xf325e4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf325e4() -> ! {
    todo!("0xf325e4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,int const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi")]
// 0xf325f4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi
// type: 
pub fn stub_0xf325f4() -> ! {
    todo!("0xf325f4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_Rb_tree(std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_")]
// 0xf32604 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_
// type: int __fastcall(int)
pub fn stub_0xf32604() -> ! {
    todo!("0xf32604 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_")
}

#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
#[doc(alias = "j___ZN3RBX12BillboardGuiD2Ev")]
// 0xf32754 — j___ZN3RBX12BillboardGuiD2Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
pub fn stub_0xf32754() -> ! {
    todo!("0xf32754 j___ZN3RBX12BillboardGuiD2Ev")
}

#[doc(alias = "RBX::Camera::~Camera()")]
#[doc(alias = "j___ZN3RBX6CameraD0Ev")]
// 0xf32ab4 — j___ZN3RBX6CameraD0Ev
// type: void __fastcall(RBX::Camera *__hidden this)
pub fn stub_0xf32ab4() -> ! {
    todo!("0xf32ab4 j___ZN3RBX6CameraD0Ev")
}

#[doc(alias = "RBX::Camera::~Camera()")]
#[doc(alias = "j___ZN3RBX6CameraD2Ev")]
// 0xf32ac4 — j___ZN3RBX6CameraD2Ev
// type: void __fastcall(RBX::Camera *__hidden this)
pub fn stub_0xf32ac4() -> ! {
    todo!("0xf32ac4 j___ZN3RBX6CameraD2Ev")
}

#[doc(alias = "RBX::Tolerance::maxExtents(void)")]
#[doc(alias = "j___ZN3RBX9Tolerance10maxExtentsEv")]
// 0xf32ad4 — j___ZN3RBX9Tolerance10maxExtentsEv
// type: _DWORD __fastcall(RBX::Tolerance *__hidden this)
pub fn stub_0xf32ad4() -> ! {
    todo!("0xf32ad4 j___ZN3RBX9Tolerance10maxExtentsEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraMode>(RBX::Camera::CameraMode const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_")]
// 0xf32ae4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32ae4() -> ! {
    todo!("0xf32ae4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_")]
// 0xf32af4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32af4() -> ! {
    todo!("0xf32af4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraPanMode>(RBX::Camera::CameraPanMode const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_")]
// 0xf32b04 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32b04() -> ! {
    todo!("0xf32b04 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv")]
// 0xf32b14 — j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv
// type: int(void)
pub fn stub_0xf32b14() -> ! {
    todo!("0xf32b14 j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv")]
// 0xf32b24 — j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv
// type: int(void)
pub fn stub_0xf32b24() -> ! {
    todo!("0xf32b24 j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv")]
// 0xf32b34 — j___ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv
// type: int(void)
pub fn stub_0xf32b34() -> ! {
    todo!("0xf32b34 j___ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv")]
// 0xf32b44 — j___ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf32b44() -> ! {
    todo!("0xf32b44 j___ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf32b64 — j___ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf32b64() -> ! {
    todo!("0xf32b64 j___ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf32b74 — j___ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf32b74() -> ! {
    todo!("0xf32b74 j___ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraType const& rbx::any_cast<RBX::Camera::CameraType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf32b84 — j___ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf32b84() -> ! {
    todo!("0xf32b84 j___ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraPanMode const& rbx::any_cast<RBX::Camera::CameraPanMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf32b94 — j___ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf32b94() -> ! {
    todo!("0xf32b94 j___ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf32ba4 — j___ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf32ba4() -> ! {
    todo!("0xf32ba4 j___ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::NavKeys::navKeyDown(void)const")]
#[doc(alias = "j___ZNK3RBX7NavKeys10navKeyDownEv")]
// 0xf32cc4 — j___ZNK3RBX7NavKeys10navKeyDownEv
// type: _DWORD __fastcall(RBX::NavKeys *__hidden this)
pub fn stub_0xf32cc4() -> ! {
    todo!("0xf32cc4 j___ZNK3RBX7NavKeys10navKeyDownEv")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm")]
// 0xf32cf4 — j___ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32cf4() -> ! {
    todo!("0xf32cf4 j___ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm")]
// 0xf32d04 — j___ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d04() -> ! {
    todo!("0xf32d04 j___ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm")]
// 0xf32d14 — j___ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d14() -> ! {
    todo!("0xf32d14 j___ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraMode *,RBX::Camera::CameraMode *>(RBX::Camera::CameraMode *,RBX::Camera::CameraMode *,RBX::Camera::CameraMode *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_")]
// 0xf32d34 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d34() -> ! {
    todo!("0xf32d34 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraType *,RBX::Camera::CameraType *>(RBX::Camera::CameraType *,RBX::Camera::CameraType *,RBX::Camera::CameraType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_")]
// 0xf32d44 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d44() -> ! {
    todo!("0xf32d44 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_")]
// 0xf32d54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d54() -> ! {
    todo!("0xf32d54 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf32d74 — j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d74() -> ! {
    todo!("0xf32d74 j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf32d84 — j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d84() -> ! {
    todo!("0xf32d84 j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf32d94 — j___ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32d94() -> ! {
    todo!("0xf32d94 j___ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,RBX::Camera::CameraMode const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf32db4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: 
pub fn stub_0xf32db4() -> ! {
    todo!("0xf32db4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,unsigned long,RBX::Camera::CameraMode const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf32dc4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
pub fn stub_0xf32dc4() -> ! {
    todo!("0xf32dc4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_")]
// 0xf32dd4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf32dd4() -> ! {
    todo!("0xf32dd4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_")]
// 0xf32de4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32de4() -> ! {
    todo!("0xf32de4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_")
}
