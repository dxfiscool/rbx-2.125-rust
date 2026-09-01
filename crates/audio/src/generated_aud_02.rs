// Auto-generated skeletons for rbx-audio — gap filler (global EA-sorted, aud_02)
// Filter: FMOD|Audio|Sound (2541 filtered, 0 remaining) -> global gap filler EA-sorted asc next 120 not yet in audio crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x38aec4..0x390464 | audio 22685 -> 22805 distinct
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ` and ' stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x38aec4 — __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
// type: void __fastcall(RBX::ThreadPool::ThreadPoolData *__hidden this)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
#[doc(alias = "__ZN3RBX10ThreadPool14ThreadPoolDataD0Ev")]
pub fn stub_0x38aec4() -> ! {
    todo!("0x38aec4 RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")
}

// 0x38afc8 — __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// type: int __fastcall(int)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> &)")]
#[doc(alias = "__ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE")]
pub fn stub_0x38afc8() -> ! {
    todo!("0x38afc8 RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> &)")
}

// 0x38afd4 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&)")]
#[doc(alias = "__ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_")]
pub fn stub_0x38afd4() -> ! {
    todo!("0x38afd4 rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&)")
}

// 0x38b0b4 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv")]
pub fn stub_0x38b0b4() -> ! {
    todo!("0x38b0b4 std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::pop_front(void)")
}

// 0x38b0ec — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev")]
pub fn stub_0x38b0ec() -> ! {
    todo!("0x38b0ec std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~deque()")
}

// 0x38b1d4 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev")]
pub fn stub_0x38b1d4() -> ! {
    todo!("0x38b1d4 std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~_Deque_base()")
}

// 0x38b200 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// type: void __fastcall(int, int *, int *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_")]
pub fn stub_0x38b200() -> ! {
    todo!("0x38b200 std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>)")
}

// 0x38b338 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm")]
pub fn stub_0x38b338() -> ! {
    todo!("0x38b338 std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")
}

// 0x38b490 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_")]
pub fn stub_0x38b490() -> ! {
    todo!("0x38b490 std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**)")
}

// 0x38b584 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_")]
pub fn stub_0x38b584() -> ! {
    todo!("0x38b584 std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>> const&)")
}

// 0x38b740 — __GLOBAL__I_a_146
// type: 
#[doc(alias = "global constructor keyed to_a_146")]
#[doc(alias = "__GLOBAL__I_a_146")]
pub fn stub_0x38b740() -> ! {
    todo!("0x38b740 global constructor keyed to_a_146")
}

// 0x38b808 — __ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_
// type: void __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_")]
pub fn stub_0x38b808() -> ! {
    todo!("0x38b808 RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")
}

// 0x38b970 — __ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_")]
pub fn stub_0x38b970() -> ! {
    todo!("0x38b970 RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")
}

// 0x38ba5c — __ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_
// type: void __fastcall(std::string *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_")]
pub fn stub_0x38ba5c() -> ! {
    todo!("0x38ba5c RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")
}

// 0x38be8c — __ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_")]
pub fn stub_0x38be8c() -> ! {
    todo!("0x38be8c RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")
}

// 0x38c0e8 — __ZNK3RBX4UDimplERKS0_
// type: int __fastcall(int result, int, int)
#[doc(alias = "RBX::UDim::operator+(RBX::UDim const&)const")]
#[doc(alias = "__ZNK3RBX4UDimplERKS0_")]
pub fn stub_0x38c0e8() -> ! {
    todo!("0x38c0e8 RBX::UDim::operator+(RBX::UDim const&)const")
}

// 0x38c108 — __ZNK3RBX4UDimmiERKS0_
// type: int __fastcall(int result, int, int)
#[doc(alias = "RBX::UDim::operator-(RBX::UDim const&)const")]
#[doc(alias = "__ZNK3RBX4UDimmiERKS0_")]
pub fn stub_0x38c108() -> ! {
    todo!("0x38c108 RBX::UDim::operator-(RBX::UDim const&)const")
}

// 0x38c128 — __ZNK3RBX4UDimngEv
// type: int __fastcall(int result, int)
#[doc(alias = "RBX::UDim::operator-(void)const")]
#[doc(alias = "__ZNK3RBX4UDimngEv")]
pub fn stub_0x38c128() -> ! {
    todo!("0x38c128 RBX::UDim::operator-(void)const")
}

// 0x38c140 — __ZNK3RBX5UDim2mlEN3G3D7Vector2E
// type: _DWORD *__fastcall(_DWORD *result, int, __int32 *)
#[doc(alias = "RBX::UDim2::operator*(G3D::Vector2)const")]
#[doc(alias = "__ZNK3RBX5UDim2mlEN3G3D7Vector2E")]
pub fn stub_0x38c140() -> ! {
    todo!("0x38c140 RBX::UDim2::operator*(G3D::Vector2)const")
}

// 0x38c188 — __ZNK3RBX5UDim2mlEf
// type: _DWORD *__fastcall(_DWORD *result, int, __int32)
#[doc(alias = "RBX::UDim2::operator*(float)const")]
#[doc(alias = "__ZNK3RBX5UDim2mlEf")]
pub fn stub_0x38c188() -> ! {
    todo!("0x38c188 RBX::UDim2::operator*(float)const")
}

// 0x38c1e4 — __ZNK3RBX5UDim2plERKS0_
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::UDim2::operator+(RBX::UDim2 const&)const")]
#[doc(alias = "__ZNK3RBX5UDim2plERKS0_")]
pub fn stub_0x38c1e4() -> ! {
    todo!("0x38c1e4 RBX::UDim2::operator+(RBX::UDim2 const&)const")
}

// 0x38c224 — __ZNK3RBX5UDim2miERKS0_
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::UDim2::operator-(RBX::UDim2 const&)const")]
#[doc(alias = "__ZNK3RBX5UDim2miERKS0_")]
pub fn stub_0x38c224() -> ! {
    todo!("0x38c224 RBX::UDim2::operator-(RBX::UDim2 const&)const")
}

// 0x38c264 — __ZNK3RBX5UDim2ngEv
// type: _DWORD *__fastcall(_DWORD *result, int)
#[doc(alias = "RBX::UDim2::operator-(void)const")]
#[doc(alias = "__ZNK3RBX5UDim2ngEv")]
pub fn stub_0x38c264() -> ! {
    todo!("0x38c264 RBX::UDim2::operator-(void)const")
}

// 0x38c294 — __GLOBAL__I_a_147
// type: 
#[doc(alias = "global constructor keyed to_a_147")]
#[doc(alias = "__GLOBAL__I_a_147")]
pub fn stub_0x38c294() -> ! {
    todo!("0x38c294 global constructor keyed to_a_147")
}

// 0x38c35c — __ZNK3RBX7UIEvent18isTextCharacterKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isTextCharacterKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent18isTextCharacterKeyEv")]
pub fn stub_0x38c35c() -> ! {
    todo!("0x38c35c RBX::UIEvent::isTextCharacterKey(void)const")
}

// 0x38c368 — __ZNK3RBX7UIEvent10isAltEventEv
// type: int __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isAltEvent(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent10isAltEventEv")]
pub fn stub_0x38c368() -> ! {
    todo!("0x38c368 RBX::UIEvent::isAltEvent(void)const")
}

// 0x38c37c — __ZNK3RBX7UIEvent11isCtrlEventEv
// type: int __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isCtrlEvent(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent11isCtrlEventEv")]
pub fn stub_0x38c37c() -> ! {
    todo!("0x38c37c RBX::UIEvent::isCtrlEvent(void)const")
}

// 0x38c390 — __ZNK3RBX7UIEvent19isCarriageReturnKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isCarriageReturnKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent19isCarriageReturnKeyEv")]
pub fn stub_0x38c390() -> ! {
    todo!("0x38c390 RBX::UIEvent::isCarriageReturnKey(void)const")
}

// 0x38c3ac — __ZNK3RBX7UIEvent11isDeleteKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isDeleteKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent11isDeleteKeyEv")]
pub fn stub_0x38c3ac() -> ! {
    todo!("0x38c3ac RBX::UIEvent::isDeleteKey(void)const")
}

// 0x38c3b8 — __ZNK3RBX7UIEvent14isBackspaceKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isBackspaceKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent14isBackspaceKeyEv")]
pub fn stub_0x38c3b8() -> ! {
    todo!("0x38c3b8 RBX::UIEvent::isBackspaceKey(void)const")
}

// 0x38c3c4 — __ZNK3RBX7UIEvent10isClearKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isClearKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent10isClearKeyEv")]
pub fn stub_0x38c3c4() -> ! {
    todo!("0x38c3c4 RBX::UIEvent::isClearKey(void)const")
}

// 0x38c3d0 — __ZNK3RBX7UIEvent11isEscapeKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isEscapeKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent11isEscapeKeyEv")]
pub fn stub_0x38c3d0() -> ! {
    todo!("0x38c3d0 RBX::UIEvent::isEscapeKey(void)const")
}

// 0x38c3dc — __ZNK3RBX7UIEvent14isLeftArrowKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isLeftArrowKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent14isLeftArrowKeyEv")]
pub fn stub_0x38c3dc() -> ! {
    todo!("0x38c3dc RBX::UIEvent::isLeftArrowKey(void)const")
}

// 0x38c3ec — __ZNK3RBX7UIEvent15isRightArrowKeyEv
// type: bool __fastcall(RBX::UIEvent *this)
#[doc(alias = "RBX::UIEvent::isRightArrowKey(void)const")]
#[doc(alias = "__ZNK3RBX7UIEvent15isRightArrowKeyEv")]
pub fn stub_0x38c3ec() -> ! {
    todo!("0x38c3ec RBX::UIEvent::isRightArrowKey(void)const")
}

// 0x38c3fc — __GLOBAL__I_a_148
// type: int()
#[doc(alias = "global constructor keyed to_a_148")]
#[doc(alias = "__GLOBAL__I_a_148")]
pub fn stub_0x38c3fc() -> ! {
    todo!("0x38c3fc global constructor keyed to_a_148")
}

// 0x38c434 — __ZN3RBX5Units20kmsAccelerationToRbxERKN3G3D7Vector3E
// type: _DWORD *__fastcall(_DWORD *this, const Vector3 *)
#[doc(alias = "RBX::Units::kmsAccelerationToRbx(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX5Units20kmsAccelerationToRbxERKN3G3D7Vector3E")]
pub fn stub_0x38c434() -> ! {
    todo!("0x38c434 RBX::Units::kmsAccelerationToRbx(G3D::Vector3 const&)")
}

// 0x38c464 — __ZN3RBX5Units13kmsForceToRbxEf
// type: unsigned __int32 __fastcall(RBX::Units *this, float)
#[doc(alias = "RBX::Units::kmsForceToRbx(float)")]
#[doc(alias = "__ZN3RBX5Units13kmsForceToRbxEf")]
pub fn stub_0x38c464() -> ! {
    todo!("0x38c464 RBX::Units::kmsForceToRbx(float)")
}

// 0x38c478 — __GLOBAL__I_a_149
// type: int()
#[doc(alias = "global constructor keyed to_a_149")]
#[doc(alias = "__GLOBAL__I_a_149")]
pub fn stub_0x38c478() -> ! {
    todo!("0x38c478 global constructor keyed to_a_149")
}

// 0x38c4b0 — __ZN3RBX13UserInputBaseC2Ev
// type: int __fastcall(RBX::UserInputBase *this)
#[doc(alias = "RBX::UserInputBase::UserInputBase(void)")]
#[doc(alias = "__ZN3RBX13UserInputBaseC2Ev")]
pub fn stub_0x38c4b0() -> ! {
    todo!("0x38c4b0 RBX::UserInputBase::UserInputBase(void)")
}

// 0x38c5d4 — __ZNK3RBX13UserInputBase10getNavKeysERNS_7NavKeysEb
// type: int __fastcall(int result, _BYTE *, int)
#[doc(alias = "RBX::UserInputBase::getNavKeys(RBX::NavKeys &,bool)const")]
#[doc(alias = "__ZNK3RBX13UserInputBase10getNavKeysERNS_7NavKeysEb")]
pub fn stub_0x38c5d4() -> ! {
    todo!("0x38c5d4 RBX::UserInputBase::getNavKeys(RBX::NavKeys &,bool)const")
}

// 0x38c6b4 — __ZN3RBX13UserInputBase13getGameCursorEPNS_5AdornE
// type: void __fastcall(RBX::UserInputBase *this, const shared_count *, int)
#[doc(alias = "RBX::UserInputBase::getGameCursor(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UserInputBase13getGameCursorEPNS_5AdornE")]
pub fn stub_0x38c6b4() -> ! {
    todo!("0x38c6b4 RBX::UserInputBase::getGameCursor(RBX::Adorn *)")
}

// 0x38c928 — __ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE
// type: int __fastcall(RBX::UserInputBase *this, RBX::Adorn *, const RBX::TextureId *)
#[doc(alias = "RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)")]
#[doc(alias = "__ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE")]
pub fn stub_0x38c928() -> ! {
    todo!("0x38c928 RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)")
}

// 0x38c974 — __ZN3RBX13UserInputBase16renderGameCursorEPNS_5AdornE
// type: void __fastcall(RBX::UserInputBase *this, RBX::Adorn *)
#[doc(alias = "RBX::UserInputBase::renderGameCursor(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UserInputBase16renderGameCursorEPNS_5AdornE")]
pub fn stub_0x38c974() -> ! {
    todo!("0x38c974 RBX::UserInputBase::renderGameCursor(RBX::Adorn *)")
}

// 0x38cb9c — __ZN3RBX13UserInputBase10removeJobsEv
// type: void __fastcall(RBX::UserInputBase *this)
#[doc(alias = "RBX::UserInputBase::removeJobs(void)")]
#[doc(alias = "__ZN3RBX13UserInputBase10removeJobsEv")]
pub fn stub_0x38cb9c() -> ! {
    todo!("0x38cb9c RBX::UserInputBase::removeJobs(void)")
}

// 0x38cba0 — __GLOBAL__I_a_150
// type: 
#[doc(alias = "global constructor keyed to_a_150")]
#[doc(alias = "__GLOBAL__I_a_150")]
pub fn stub_0x38cba0() -> ! {
    todo!("0x38cba0 global constructor keyed to_a_150")
}

// 0x38cc68 — __ZN3RBX5rot13ESs
// type: void __fastcall(std::string *, std::string *)
#[doc(alias = "RBX::rot13(std::string)")]
#[doc(alias = "__ZN3RBX5rot13ESs")]
pub fn stub_0x38cc68() -> ! {
    todo!("0x38cc68 RBX::rot13(std::string)")
}

// 0x38ce48 — __ZN3RBX15StringConverterIbE15convertToStringERKb
// type: int __fastcall(int, _BYTE *)
#[doc(alias = "RBX::StringConverter<bool>::convertToString(bool const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIbE15convertToStringERKb")]
pub fn stub_0x38ce48() -> ! {
    todo!("0x38ce48 RBX::StringConverter<bool>::convertToString(bool const&)")
}

// 0x38ce78 — __ZN3RBX15StringConverterIbE14convertToValueERKSsRb
// type: int __fastcall(std::string *, _BYTE *)
#[doc(alias = "RBX::StringConverter<bool>::convertToValue(std::string const&,bool &)")]
#[doc(alias = "__ZN3RBX15StringConverterIbE14convertToValueERKSsRb")]
pub fn stub_0x38ce78() -> ! {
    todo!("0x38ce78 RBX::StringConverter<bool>::convertToValue(std::string const&,bool &)")
}

// 0x38cf10 — __ZN3RBX15StringConverterIiE15convertToStringERKi
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::StringConverter<int>::convertToString(int const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIiE15convertToStringERKi")]
pub fn stub_0x38cf10() -> ! {
    todo!("0x38cf10 RBX::StringConverter<int>::convertToString(int const&)")
}

// 0x38cf58 — __ZN3RBX15StringConverterIlE15convertToStringERKl
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::StringConverter<long>::convertToString(long const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIlE15convertToStringERKl")]
pub fn stub_0x38cf58() -> ! {
    todo!("0x38cf58 RBX::StringConverter<long>::convertToString(long const&)")
}

// 0x38cfa0 — __ZN3RBX15StringConverterIiE14convertToValueERKSsRi
// type: int __fastcall(const char **, int *)
#[doc(alias = "RBX::StringConverter<int>::convertToValue(std::string const&,int &)")]
#[doc(alias = "__ZN3RBX15StringConverterIiE14convertToValueERKSsRi")]
pub fn stub_0x38cfa0() -> ! {
    todo!("0x38cfa0 RBX::StringConverter<int>::convertToValue(std::string const&,int &)")
}

// 0x38cff0 — __ZN3RBX15StringConverterIjE15convertToStringERKj
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::StringConverter<unsigned int>::convertToString(unsigned int const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIjE15convertToStringERKj")]
pub fn stub_0x38cff0() -> ! {
    todo!("0x38cff0 RBX::StringConverter<unsigned int>::convertToString(unsigned int const&)")
}

// 0x38d038 — __ZN3RBX15StringConverterIjE14convertToValueERKSsRj
// type: int __fastcall(int *, _DWORD *)
#[doc(alias = "RBX::StringConverter<unsigned int>::convertToValue(std::string const&,unsigned int &)")]
#[doc(alias = "__ZN3RBX15StringConverterIjE14convertToValueERKSsRj")]
pub fn stub_0x38d038() -> ! {
    todo!("0x38d038 RBX::StringConverter<unsigned int>::convertToValue(std::string const&,unsigned int &)")
}

// 0x38d14c — __ZN3RBX15StringConverterIlE14convertToValueERKSsRl
// type: int __fastcall(int *, _DWORD *)
#[doc(alias = "RBX::StringConverter<long>::convertToValue(std::string const&,long &)")]
#[doc(alias = "__ZN3RBX15StringConverterIlE14convertToValueERKSsRl")]
pub fn stub_0x38d14c() -> ! {
    todo!("0x38d14c RBX::StringConverter<long>::convertToValue(std::string const&,long &)")
}

// 0x38d260 — __ZN3RBX15StringConverterIdE14convertToValueERKSsRd
// type: int __fastcall(std::string *, double *)
#[doc(alias = "RBX::StringConverter<double>::convertToValue(std::string const&,double &)")]
#[doc(alias = "__ZN3RBX15StringConverterIdE14convertToValueERKSsRd")]
pub fn stub_0x38d260() -> ! {
    todo!("0x38d260 RBX::StringConverter<double>::convertToValue(std::string const&,double &)")
}

// 0x38d2e0 — __ZN3RBX15StringConverterIdE15convertToStringERKd
// type: int __fastcall(int, double *)
#[doc(alias = "RBX::StringConverter<double>::convertToString(double const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIdE15convertToStringERKd")]
pub fn stub_0x38d2e0() -> ! {
    todo!("0x38d2e0 RBX::StringConverter<double>::convertToString(double const&)")
}

// 0x38d440 — __ZN3RBX15StringConverterIfE14convertToValueERKSsRf
// type: int __fastcall(std::string *this, float *)
#[doc(alias = "RBX::StringConverter<float>::convertToValue(std::string const&,float &)")]
#[doc(alias = "__ZN3RBX15StringConverterIfE14convertToValueERKSsRf")]
pub fn stub_0x38d440() -> ! {
    todo!("0x38d440 RBX::StringConverter<float>::convertToValue(std::string const&,float &)")
}

// 0x38d4c4 — __ZN3RBX15StringConverterIfE15convertToStringERKf
// type: int __fastcall(int, float *)
#[doc(alias = "RBX::StringConverter<float>::convertToString(float const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIfE15convertToStringERKf")]
pub fn stub_0x38d4c4() -> ! {
    todo!("0x38d4c4 RBX::StringConverter<float>::convertToString(float const&)")
}

// 0x38d61c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
// type: int __fastcall(unsigned __int8 **, int *)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")]
#[doc(alias = "__ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_")]
pub fn stub_0x38d61c() -> ! {
    todo!("0x38d61c bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")
}

// 0x38d67c — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
// type: int __fastcall(int *, unsigned int, int)
#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")]
#[doc(alias = "__ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_")]
pub fn stub_0x38d67c() -> ! {
    todo!("0x38d67c bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")
}

// 0x38da14 — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
// type: int __fastcall(unsigned __int8 **, _DWORD *)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")]
#[doc(alias = "__ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_")]
pub fn stub_0x38da14() -> ! {
    todo!("0x38da14 bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")
}

// 0x38da58 — __GLOBAL__I_a_151
// type: 
#[doc(alias = "global constructor keyed to_a_151")]
#[doc(alias = "__GLOBAL__I_a_151")]
pub fn stub_0x38da58() -> ! {
    todo!("0x38da58 global constructor keyed to_a_151")
}

// 0x38db20 — __ZN3RBX12Accoutrement18setAttachmentPointERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Accoutrement *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Accoutrement::setAttachmentPoint(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement18setAttachmentPointERKN3G3D15CoordinateFrameE")]
pub fn stub_0x38db20() -> ! {
    todo!("0x38db20 RBX::Accoutrement::setAttachmentPoint(G3D::CoordinateFrame const&)")
}

// 0x38dc30 — __ZNK3RBX12Accoutrement16getAttachmentPosEv
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Accoutrement::getAttachmentPos(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement16getAttachmentPosEv")]
pub fn stub_0x38dc30() -> ! {
    todo!("0x38dc30 RBX::Accoutrement::getAttachmentPos(void)const")
}

// 0x38dc40 — __ZN3RBX12Accoutrement16setAttachmentPosERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentPos(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement16setAttachmentPosERKN3G3D7Vector3E")]
pub fn stub_0x38dc40() -> ! {
    todo!("0x38dc40 RBX::Accoutrement::setAttachmentPos(G3D::Vector3 const&)")
}

// 0x38dc70 — __ZNK3RBX12Accoutrement20getAttachmentForwardEv
// type: int __fastcall(RBX::Accoutrement *this, int)
#[doc(alias = "RBX::Accoutrement::getAttachmentForward(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement20getAttachmentForwardEv")]
pub fn stub_0x38dc70() -> ! {
    todo!("0x38dc70 RBX::Accoutrement::getAttachmentForward(void)const")
}

// 0x38dcb0 — __ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentForward(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E")]
pub fn stub_0x38dcb0() -> ! {
    todo!("0x38dcb0 RBX::Accoutrement::setAttachmentForward(G3D::Vector3 const&)")
}

// 0x38ddfc — __ZNK3RBX12Accoutrement15getAttachmentUpEv
// type: int __fastcall(RBX::Accoutrement *this, int)
#[doc(alias = "RBX::Accoutrement::getAttachmentUp(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement15getAttachmentUpEv")]
pub fn stub_0x38ddfc() -> ! {
    todo!("0x38ddfc RBX::Accoutrement::getAttachmentUp(void)const")
}

// 0x38de0c — __ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentUp(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E")]
pub fn stub_0x38de0c() -> ! {
    todo!("0x38de0c RBX::Accoutrement::setAttachmentUp(G3D::Vector3 const&)")
}

// 0x38df30 — __ZNK3RBX12Accoutrement18getAttachmentRightEv
// type: int __fastcall(RBX::Accoutrement *this, int)
#[doc(alias = "RBX::Accoutrement::getAttachmentRight(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement18getAttachmentRightEv")]
pub fn stub_0x38df30() -> ! {
    todo!("0x38df30 RBX::Accoutrement::getAttachmentRight(void)const")
}

// 0x38df40 — __ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentRight(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E")]
pub fn stub_0x38df40() -> ! {
    todo!("0x38df40 RBX::Accoutrement::setAttachmentRight(G3D::Vector3 const&)")
}

// 0x38e064 — __ZN3RBX12Accoutrement27setBackendAccoutrementStateEi
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Accoutrement::setBackendAccoutrementState(int)")]
#[doc(alias = "__ZN3RBX12Accoutrement27setBackendAccoutrementStateEi")]
pub fn stub_0x38e064() -> ! {
    todo!("0x38e064 RBX::Accoutrement::setBackendAccoutrementState(int)")
}

// 0x38e084 — __ZN3RBX12AccoutrementC1Ev
// type: RBX::Instance *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::Accoutrement(void)")]
#[doc(alias = "__ZN3RBX12AccoutrementC1Ev")]
pub fn stub_0x38e084() -> ! {
    todo!("0x38e084 RBX::Accoutrement::Accoutrement(void)")
}

// 0x38e4b4 — __ZN3RBX12AccoutrementC2Ev
// type: RBX::Instance *__fastcall(RBX::Accoutrement *this, int)
#[doc(alias = "RBX::Accoutrement::Accoutrement(void)")]
#[doc(alias = "__ZN3RBX12AccoutrementC2Ev")]
pub fn stub_0x38e4b4() -> ! {
    todo!("0x38e4b4 RBX::Accoutrement::Accoutrement(void)")
}

// 0x38e90c — __ZN3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZN3RBX12AccoutrementD0Ev")]
pub fn stub_0x38e90c() -> ! {
    todo!("0x38e90c RBX::Accoutrement::~Accoutrement()")
}

// 0x38e9b8 — __ZN3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZN3RBX12AccoutrementD1Ev")]
pub fn stub_0x38e9b8() -> ! {
    todo!("0x38e9b8 RBX::Accoutrement::~Accoutrement()")
}

// 0x38e9c8 — __ZThn32_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn32_N3RBX12AccoutrementD0Ev")]
pub fn stub_0x38e9c8() -> ! {
    todo!("0x38e9c8 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9d0 — __ZThn36_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn36_N3RBX12AccoutrementD0Ev")]
pub fn stub_0x38e9d0() -> ! {
    todo!("0x38e9d0 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9d8 — __ZThn92_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn92_N3RBX12AccoutrementD0Ev")]
pub fn stub_0x38e9d8() -> ! {
    todo!("0x38e9d8 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9e0 — __ZThn128_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn128_N3RBX12AccoutrementD0Ev")]
pub fn stub_0x38e9e0() -> ! {
    todo!("0x38e9e0 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9e8 — __ZN3RBX12AccoutrementD2Ev
// type: void __fastcall(RBX::Accoutrement *this, int *, int)
#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZN3RBX12AccoutrementD2Ev")]
pub fn stub_0x38e9e8() -> ! {
    todo!("0x38e9e8 RBX::Accoutrement::~Accoutrement()")
}

// 0x38ef1c — __ZThn32_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn32_N3RBX12AccoutrementD1Ev")]
pub fn stub_0x38ef1c() -> ! {
    todo!("0x38ef1c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef2c — __ZThn36_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn36_N3RBX12AccoutrementD1Ev")]
pub fn stub_0x38ef2c() -> ! {
    todo!("0x38ef2c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef3c — __ZThn92_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn92_N3RBX12AccoutrementD1Ev")]
pub fn stub_0x38ef3c() -> ! {
    todo!("0x38ef3c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef4c — __ZThn128_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
#[doc(alias = "__ZThn128_N3RBX12AccoutrementD1Ev")]
pub fn stub_0x38ef4c() -> ! {
    todo!("0x38ef4c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef5c — __ZN3RBX12Accoutrement12onCameraNearEf
// type: unsigned int __fastcall(RBX::Accoutrement *this, float)
#[doc(alias = "RBX::Accoutrement::onCameraNear(float)")]
#[doc(alias = "__ZN3RBX12Accoutrement12onCameraNearEf")]
pub fn stub_0x38ef5c() -> ! {
    todo!("0x38ef5c RBX::Accoutrement::onCameraNear(float)")
}

// 0x38ef98 — __ZThn128_N3RBX12Accoutrement12onCameraNearEf
// type: unsigned int __fastcall(RBX::Accoutrement *this, float)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")]
#[doc(alias = "__ZThn128_N3RBX12Accoutrement12onCameraNearEf")]
pub fn stub_0x38ef98() -> ! {
    todo!("0x38ef98 non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")
}

// 0x38efa0 — __ZN3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE
// type: unsigned int __fastcall(RBX::Instance *, int, int)
#[doc(alias = "RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
#[doc(alias = "__ZN3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE")]
pub fn stub_0x38efa0() -> ! {
    todo!("0x38efa0 RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")
}

// 0x38f014 — __ZThn104_N3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE
// type: unsigned int __fastcall(int, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
#[doc(alias = "__ZThn104_N3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE")]
pub fn stub_0x38f014() -> ! {
    todo!("0x38f014 non-virtual thunk toRBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")
}

// 0x38f01c — __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAll(RBX::ModelInstance *)")]
#[doc(alias = "__ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE")]
pub fn stub_0x38f01c() -> ! {
    todo!("0x38f01c RBX::Accoutrement::dropAll(RBX::ModelInstance *)")
}

// 0x38f024 — __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
// type: RBX::Instance *__fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")]
#[doc(alias = "__ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_")]
pub fn stub_0x38f024() -> ! {
    todo!("0x38f024 RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")
}

// 0x38f054 — __ZNK3RBX12Accoutrement14getHandleConstEv
// type: char *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getHandleConst(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement14getHandleConstEv")]
pub fn stub_0x38f054() -> ! {
    todo!("0x38f054 RBX::Accoutrement::getHandleConst(void)const")
}

// 0x38f1c4 — __ZN3RBX12Accoutrement11getLocationEv
// type: int __fastcall(RBX::Accoutrement *this, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::getLocation(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement11getLocationEv")]
pub fn stub_0x38f1c4() -> ! {
    todo!("0x38f1c4 RBX::Accoutrement::getLocation(void)")
}

// 0x38f1f8 — __ZTv0_n12_N3RBX12Accoutrement11getLocationEv
// type: int __fastcall(RBX::Accoutrement *this, _DWORD *)
#[doc(alias = "virtual thunk toRBX::Accoutrement::getLocation(void)")]
#[doc(alias = "__ZTv0_n12_N3RBX12Accoutrement11getLocationEv")]
pub fn stub_0x38f1f8() -> ! {
    todo!("0x38f1f8 virtual thunk toRBX::Accoutrement::getLocation(void)")
}

// 0x38f20c — __ZN3RBX12Accoutrement17connectTouchEventEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::connectTouchEvent(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement17connectTouchEventEv")]
pub fn stub_0x38f20c() -> ! {
    todo!("0x38f20c RBX::Accoutrement::connectTouchEvent(void)")
}

// 0x38f3ec — __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
// type: RBX::Accoutrement *__fastcall(RBX::Network::Players *, RBX::Accoutrement **, bool)
#[doc(alias = "RBX::Accoutrement::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x38f3ec() -> ! {
    todo!("0x38f3ec RBX::Accoutrement::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x38f47c — __ZN3RBX12Accoutrement19rebuildBackendStateEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::rebuildBackendState(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement19rebuildBackendStateEv")]
pub fn stub_0x38f47c() -> ! {
    todo!("0x38f47c RBX::Accoutrement::rebuildBackendState(void)")
}

// 0x38f4f4 — __ZN3RBX12Accoutrement19computeDesiredStateEv
// type: int __fastcall(RBX::Instance **this, int, bool)
#[doc(alias = "RBX::Accoutrement::computeDesiredState(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement19computeDesiredStateEv")]
pub fn stub_0x38f4f4() -> ! {
    todo!("0x38f4f4 RBX::Accoutrement::computeDesiredState(void)")
}

// 0x38f578 — __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE
// type: int __fastcall(RBX::Accoutrement *this, int, RBX::Network::Players *)
#[doc(alias = "RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")]
#[doc(alias = "__ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE")]
pub fn stub_0x38f578() -> ! {
    todo!("0x38f578 RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")
}

// 0x38f6f0 — __ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::computeDesiredState(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE")]
pub fn stub_0x38f6f0() -> ! {
    todo!("0x38f6f0 RBX::Accoutrement::computeDesiredState(RBX::Instance *)")
}

// 0x38f714 — __ZN3RBX12Accoutrement13upTo_EquippedEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::upTo_Equipped(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement13upTo_EquippedEv")]
pub fn stub_0x38f714() -> ! {
    todo!("0x38f714 RBX::Accoutrement::upTo_Equipped(void)")
}

// 0x38f92c — __ZN3RBX12Accoutrement16upTo_InCharacterEv
// type: void __fastcall(RBX::Humanoid **this)
#[doc(alias = "RBX::Accoutrement::upTo_InCharacter(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement16upTo_InCharacterEv")]
pub fn stub_0x38f92c() -> ! {
    todo!("0x38f92c RBX::Accoutrement::upTo_InCharacter(void)")
}

// 0x38fb1c — __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement16upTo_InWorkspaceEv")]
pub fn stub_0x38fb1c() -> ! {
    todo!("0x38fb1c RBX::Accoutrement::upTo_InWorkspace(void)")
}

// 0x38fbcc — __ZN3RBX12Accoutrement17downFrom_EquippedEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::downFrom_Equipped(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement17downFrom_EquippedEv")]
pub fn stub_0x38fbcc() -> ! {
    todo!("0x38fbcc RBX::Accoutrement::downFrom_Equipped(void)")
}

// 0x38fd24 — __ZN3RBX12Accoutrement18downFrom_HasHandleEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::downFrom_HasHandle(void)")]
#[doc(alias = "__ZN3RBX12Accoutrement18downFrom_HasHandleEv")]
pub fn stub_0x38fd24() -> ! {
    todo!("0x38fd24 RBX::Accoutrement::downFrom_HasHandle(void)")
}

// 0x38fd60 — __ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x38fd60() -> ! {
    todo!("0x38fd60 RBX::Accoutrement::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x38fe18 — __ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int)
#[doc(alias = "RBX::Accoutrement::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x38fe18() -> ! {
    todo!("0x38fe18 RBX::Accoutrement::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x38ff34 — __ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE")]
pub fn stub_0x38ff34() -> ! {
    todo!("0x38ff34 RBX::Accoutrement::onChildAdded(RBX::Instance *)")
}

// 0x38ff5c — __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE")]
pub fn stub_0x38ff5c() -> ! {
    todo!("0x38ff5c RBX::Accoutrement::onChildRemoved(RBX::Instance *)")
}

// 0x38ff84 — __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE
// type: 
#[doc(alias = "RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_0x38ff84() -> ! {
    todo!("0x38ff84 RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x38fff0 — __ZN3RBX3HatC1Ev
// type: RBX::Accoutrement *__fastcall(RBX::Hat *this)
#[doc(alias = "RBX::Hat::Hat(void)")]
#[doc(alias = "__ZN3RBX3HatC1Ev")]
pub fn stub_0x38fff0() -> ! {
    todo!("0x38fff0 RBX::Hat::Hat(void)")
}

// 0x3901bc — __ZNK3RBX12Accoutrement18getAttachmentPointEv
// type: char *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getAttachmentPoint(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement18getAttachmentPointEv")]
pub fn stub_0x3901bc() -> ! {
    todo!("0x3901bc RBX::Accoutrement::getAttachmentPoint(void)const")
}

// 0x3901c0 — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEN3G3D15CoordinateFrameEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEN3G3D15CoordinateFrameEED1Ev")]
pub fn stub_0x3901c0() -> ! {
    todo!("0x3901c0 RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::~PropDescriptor()")
}

// 0x3901e4 — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEN3G3D7Vector3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEN3G3D7Vector3EED1Ev")]
pub fn stub_0x3901e4() -> ! {
    todo!("0x3901e4 RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::~PropDescriptor()")
}

// 0x390208 — __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getBackendAccoutrementState(void)const")]
#[doc(alias = "__ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv")]
pub fn stub_0x390208() -> ! {
    todo!("0x390208 RBX::Accoutrement::getBackendAccoutrementState(void)const")
}

// 0x390210 — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED1Ev")]
pub fn stub_0x390210() -> ! {
    todo!("0x390210 RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")
}

// 0x390234 — __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
// type: void *__fastcall(int, int)
#[doc(alias = "RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")]
#[doc(alias = "__ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i")]
pub fn stub_0x390234() -> ! {
    todo!("0x390234 RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")
}

// 0x390270 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// type: void __fastcall(rbx::signals::connection *, int, int, const void *, int)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")]
pub fn stub_0x390270() -> ! {
    todo!("0x390270 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")
}

// 0x3903f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3903f0() -> ! {
    todo!("0x3903f0 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")
}

// 0x390464 — __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// type: __guard *__fastcall(int *, _DWORD *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
pub fn stub_0x390464() -> ! {
    todo!("0x390464 __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")
}

