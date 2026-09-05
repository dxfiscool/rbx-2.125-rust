// Auto-generated shard FO — 150 stubs EA-sorted asc 0x43c74..0x4bb40 (global gap filler not yet in reflection, 22117->22267 distinct)
// Source: ida/export.json (85545 funcs) EA asc not in crates/reflection/src/*.rs, next 150
// Format: // 0xADDR - mangled + doc alias + stub using rbx_core::SharedPtr not boost

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `ControlView` tap/gesture/input state (IDA 0x48604-0x49bb4, no
/// canonical elsewhere): tap-touch capture, mouse/tool event counts,
/// pinch time + zoom counts, menu/input build counts and input-service
/// binds. Positions and service calls live out of slice.
pub(crate) static TAP_TOUCH_SET: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MOUSE_EVENTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static TOOL_EVENTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PINCH_TIME: std::sync::LazyLock<parking_lot::Mutex<f64>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(-1.0));
pub(crate) static PINCH_ZOOMS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static MENU_BUILDS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static INPUT_SETUP_BUILDS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static USERINPUT_BINDS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

// 0x43c74 — __ZN3RBX18FunctionMarshaller10StaticDataD1Ev
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller10StaticDataD1Ev")]
pub fn stub_43c74() {
    // IDA 0x43c74: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x43c78 — __ZN3RBX18FunctionMarshaller10StaticDataD2Ev
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller10StaticDataD2Ev")]
pub fn stub_43c78() {
    // IDA 0x43c78: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x43d14 — __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)")]
#[doc(alias = "__ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_")]
pub fn stub_43d14() {
    // IDA 0x43d14: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43d14`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43d14()
}

// 0x43d6c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_")]
pub fn stub_43d6c() {
    // IDA 0x43d6c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43d6c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43d6c()
}

// 0x43d94 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_")]
pub fn stub_43d94() {
    // IDA 0x43d94: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43d94`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43d94()
}

// 0x43de0 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")]
pub fn stub_43de0() {
    // IDA 0x43de0: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43de0`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43de0()
}

// 0x43e40 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_43e40() {
    // IDA 0x43e40: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43e40`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43e40()
}

// 0x43e68 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
pub fn stub_43e68() {
    // IDA 0x43e68: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43e68`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43e68()
}

// 0x43f1c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_43f1c() {
    // IDA 0x43f1c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43f1c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43f1c()
}

// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_43f74() {
    // IDA 0x43f74: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43f74`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43f74()
}

// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
#[doc(alias = "__ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv")]
pub fn stub_43fdc() {
    // IDA 0x43fdc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x43fdc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x43fdc()
}

// 0x441a8 — __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv
// type: _DWORD __fastcall(RBX::FunctionMarshaller *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv")]
pub fn stub_441a8() {
    // IDA 0x441a8: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x441a8`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x441a8()
}

// 0x441ac — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv
// type: void *__fastcall(RBX::FunctionMarshaller *this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv")]
pub fn stub_441ac() -> bool {
    // IDA 0x441ac: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x441ac`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x441ac()
}

// 0x442bc — __ZN5boost15recursive_mutexC2Ev
// type: _DWORD __fastcall(boost::recursive_mutex *__hidden this)
#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
#[doc(alias = "__ZN5boost15recursive_mutexC2Ev")]
pub fn stub_442bc() {
    // IDA 0x442bc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x442bc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x442bc()
}

// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev")]
pub fn stub_44564() {
    // IDA 0x44564: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm")]
pub fn stub_44590() {
    // IDA 0x44590: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44590`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44590()
}

// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm")]
pub fn stub_446e8() {
    // IDA 0x446e8: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x446e8`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x446e8()
}

// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_")]
pub fn stub_44700() {
    // IDA 0x44700: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44700`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44700()
}

// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
#[doc(alias = "__ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_")]
pub fn stub_447f4() {
    // IDA 0x447f4: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x447f4`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x447f4()
}

// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_")]
pub fn stub_44888() {
    // IDA 0x44888: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44888`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44888()
}

// 0x44924 — __GLOBAL__I_a_14
#[doc(alias = "global constructor keyed to_a_14")]
#[doc(alias = "__GLOBAL__I_a_14")]
pub fn stub_44924() {
    // IDA 0x44924: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44924`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44924()
}

// 0x44abc — -[CameraControl init:delegate:]
// type: id __cdecl(CameraControl *self, SEL, CGRect, id)
#[doc(alias = "-[CameraControl init:delegate:]")]
pub fn stub_44abc(x: f32, y: f32, width: f32, height: f32) -> crate::generated_bg_11::CameraControlInit {
    // IDA 0x44abc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44abc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44abc(x, y, width, height)
}

// 0x44b90 — -[CameraControl dealloc]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl dealloc]")]
pub fn stub_44b90() {
    // IDA 0x44b90: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44b90`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44b90()
}

// 0x44bbc — -[CameraControl setupPostMouseEventConnection]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl setupPostMouseEventConnection]")]
pub fn stub_44bbc() {
    // IDA 0x44bbc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44bbc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44bbc()
}

// 0x44cd4 — -[CameraControl postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(CameraControl *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[CameraControl postMouseEventProcessed:inputObject:event:]")]
pub fn stub_44cd4(consumed: bool, is_camera_touch: bool) {
    // IDA 0x44cd4: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44cd4`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44cd4(consumed, is_camera_touch)
}

// 0x44d04 — -[CameraControl doCameraPanTouchBegan]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchBegan]")]
pub fn stub_44d04() {
    // IDA 0x44d04: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44d04`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44d04()
}

// 0x44dec — -[CameraControl doCameraPanTouchEnded]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchEnded]")]
pub fn stub_44dec() {
    // IDA 0x44dec: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44dec`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44dec()
}

// 0x44e58 — -[CameraControl doCameraPanTouchMove]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchMove]")]
pub fn stub_44e58() {
    // IDA 0x44e58: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x44e58`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x44e58()
}

// 0x450a0 — -[CameraControl touchesBegan:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesBegan:withEvent:]")]
pub fn stub_450a0(touch_count: u32) {
    // IDA 0x450a0: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x450a0`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x450a0(touch_count)
}

// 0x45124 — -[CameraControl touchesEnded:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesEnded:withEvent:]")]
pub fn stub_45124(matching: bool, ended: u32) {
    // IDA 0x45124: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45124`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45124(matching, ended)
}

// 0x45234 — -[CameraControl touchesCancelled:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesCancelled:withEvent:]")]
pub fn stub_45234(matching: bool, cancelled: u32) {
    // IDA 0x45234: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45234`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45234(matching, cancelled)
}

// 0x45344 — -[CameraControl touchesMoved:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesMoved:withEvent:]")]
pub fn stub_45344(camera_in_set: bool) {
    // IDA 0x45344: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45344`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45344(camera_in_set)
}

// 0x45454 — -[CameraControl .cxx_construct]
// type: id __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl .cxx_construct]")]
pub fn stub_45454() {
    // IDA 0x45454: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45454`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45454()
}

// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_4546c() {
    // IDA 0x4546c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x4546c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x4546c()
}

// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE")]
pub fn stub_45554() {
    // IDA 0x45554: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45554`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45554()
}

// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_")]
pub fn stub_45764() {
    // IDA 0x45764: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45764`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45764()
}

// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_")]
pub fn stub_45808() {
    // IDA 0x45808: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45808`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45808()
}

// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv")]
pub fn stub_458ac() -> u32 {
    // IDA 0x458ac: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x458ac`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x458ac()
}

// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_459a4() {
    // IDA 0x459a4: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x459a4`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x459a4()
}

// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_45aa0() {
    // IDA 0x45aa0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
pub fn stub_45b74() {
    // IDA 0x45b74: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv")]
pub fn stub_45c4c() {
    // IDA 0x45c4c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45c4c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45c4c()
}

// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv")]
pub fn stub_45d5c() -> bool {
    // IDA 0x45d5c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45d5c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45d5c()
}

// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")]
pub fn stub_45d68() {
    // IDA 0x45d68: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45d68`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45d68()
}

// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")]
pub fn stub_45d98() {
    // IDA 0x45d98: non-virtual thunk to `"'rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *` (IDA demangle) -- this/arg-adjust + tail-call. Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
#[doc(alias = "__ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_")]
pub fn stub_45dc8() {
    // IDA 0x45dc8: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45dc8`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45dc8()
}

// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE")]
pub fn stub_45eb0() {
    // IDA 0x45eb0: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45eb0`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45eb0()
}

// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv")]
pub fn stub_45fa0() {
    // IDA 0x45fa0: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45fa0`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45fa0()
}

// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_45fa4() -> u32 {
    // IDA 0x45fa4: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x45fa4`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x45fa4()
}

// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev")]
pub fn stub_46094() {
    // IDA 0x46094: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev")]
pub fn stub_46168() {
    // IDA 0x46168: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev")]
pub fn stub_46240() {
    // IDA 0x46240: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev")]
pub fn stub_462ec() {
    // IDA 0x462ec: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
#[doc(alias = "__ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_")]
pub fn stub_4639c() {
    // IDA 0x4639c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x4639c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x4639c()
}

// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_463cc(get_typeinfo: bool) -> &'static str {
    // IDA 0x463cc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x463cc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x463cc(get_typeinfo)
}

// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_")]
pub fn stub_4642c() {
    // IDA 0x4642c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x4642c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x4642c()
}

// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv")]
pub fn stub_46464() {
    // IDA 0x46464: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x46464`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x46464()
}

// 0x46490 — __GLOBAL__I_a_15
#[doc(alias = "global constructor keyed to_a_15")]
#[doc(alias = "__GLOBAL__I_a_15")]
pub fn stub_46490() {
    // IDA 0x46490: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x46490`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x46490()
}

// 0x466cc — -[CharacterMove init:]
// type: id __cdecl(CharacterMove *self, SEL, CGRect)
#[doc(alias = "-[CharacterMove init:]")]
pub fn stub_466cc(x: f32, y: f32, width: f32, height: f32) -> crate::generated_bg_11::CharacterMoveInit {
    // IDA 0x466cc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x466cc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x466cc(x, y, width, height)
}

// 0x46704 — -[CharacterMove setupCharacterMoveConnection]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove setupCharacterMoveConnection]")]
pub fn stub_46704(service_present: bool) {
    // IDA 0x46704: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x46704`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x46704(service_present)
}

// 0x467e8 — -[CharacterMove localCharacterMovementEnabledChange:]
// type: void __cdecl(CharacterMove *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[CharacterMove localCharacterMovementEnabledChange:]")]
pub fn stub_467e8() {
    // IDA 0x467e8: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x467e8`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x467e8()
}

// 0x467ec — -[CharacterMove touchesEnded:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesEnded:withEvent:]")]
pub fn stub_467ec(thumbstick_match: bool) {
    // IDA 0x467ec: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x467ec`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x467ec(thumbstick_match)
}

// 0x468bc — -[CharacterMove touchesCancelled:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesCancelled:withEvent:]")]
pub fn stub_468bc(thumbstick_match: bool) {
    // IDA 0x468bc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x468bc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x468bc(thumbstick_match)
}

// 0x4698c — -[CharacterMove cancelMovement]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove cancelMovement]")]
pub fn stub_4698c(service_present: bool) {
    // IDA 0x4698c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x4698c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x4698c(service_present)
}

// 0x469e8 — -[CharacterMove touchesMoved:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesMoved:withEvent:]")]
pub fn stub_469e8(thumbstick_match: bool, service_present: bool, dx: f32, dy: f32) {
    // IDA 0x469e8: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x469e8`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x469e8(thumbstick_match, service_present, dx, dy)
}

// 0x46f64 — __GLOBAL__I_a_16
#[doc(alias = "global constructor keyed to_a_16")]
#[doc(alias = "__GLOBAL__I_a_16")]
pub fn stub_46f64() {
    // IDA 0x46f64: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x46f64`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x46f64()
}

// 0x47178 — -[ControlComponent init]
// type: ControlComponent *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent init]")]
pub fn stub_47178() {
    // IDA 0x47178: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47178`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47178()
}

// 0x471c0 — -[ControlComponent findControlView]
// type: id __cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent findControlView]")]
pub fn stub_471c0(is_self_view: bool, ancestor_view_present: bool) -> bool {
    // IDA 0x471c0: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x471c0`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x471c0(is_self_view, ancestor_view_present)
}

// 0x47274 — -[ControlComponent getGameFromControlView]
// type: Game *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getGameFromControlView]")]
pub fn stub_47274(view_found: bool, game_present: bool) -> bool {
    // IDA 0x47274: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47274`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47274(view_found, game_present)
}

// 0x47338 — -[ControlComponent getUserInputServiceForGameDataModel]
// type: UserInputService *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getUserInputServiceForGameDataModel]")]
pub fn stub_47338(game_present: bool, service_present: bool) -> bool {
    // IDA 0x47338: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47338`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47338(game_present, service_present)
}

// 0x47424 — __GLOBAL__I_a_17
#[doc(alias = "global constructor keyed to_a_17")]
#[doc(alias = "__GLOBAL__I_a_17")]
pub fn stub_47424() {
    // IDA 0x47424: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47424`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47424()
}

// 0x47638 — -[ControlView init:withGame:]
// type: id __cdecl(ControlView *self, SEL, CGRect, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView init:withGame:]")]
pub fn stub_47638(x: f32, y: f32, width: f32, height: f32, game_present: bool) -> crate::generated_bg_11::ControlViewInit {
    // IDA 0x47638: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47638`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47638(x, y, width, height, game_present)
}

// 0x47904 — -[ControlView dealloc]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView dealloc]")]
pub fn stub_47904() {
    // IDA 0x47904: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47904`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47904()
}

// 0x479f8 — -[ControlView setGame:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView setGame:]")]
pub fn stub_479f8(game_present: bool) {
    // IDA 0x479f8: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x479f8`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x479f8(game_present)
}

// 0x47aec — -[ControlView gotStartLeaveGameNotification:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView gotStartLeaveGameNotification:]")]
pub fn stub_47aec() {
    // IDA 0x47aec: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47aec`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47aec()
}

// 0x47afc — -[ControlView dataModelChanged:]
// type: void __cdecl(ControlView *self, SEL, DataModel *)
#[doc(alias = "-[ControlView dataModelChanged:]")]
pub fn stub_47afc(datamodel_present: bool) {
    // IDA 0x47afc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47afc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47afc(datamodel_present)
}

// 0x47b38 — -[ControlView setControlVisibility:]
// type: void __cdecl(ControlView *self, SEL, char)
#[doc(alias = "-[ControlView setControlVisibility:]")]
pub fn stub_47b38(visible: bool) {
    // IDA 0x47b38: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47b38`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47b38(visible)
}

// 0x47b90 — ___36-[ControlView setControlVisibility:]_block_invoke
#[doc(alias = "___36-[ControlView setControlVisibility:]_block_invoke")]
pub fn stub_47b90(visible: bool) {
    // IDA 0x47b90: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47b90`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47b90(visible)
}

// 0x47c04 — ___copy_helper_block__8
#[doc(alias = "___copy_helper_block__8")]
pub fn stub_47c04() {
    // IDA 0x47c04: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47c04`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47c04()
}

// 0x47c10 — ___destroy_helper_block__8
#[doc(alias = "___destroy_helper_block__8")]
pub fn stub_47c10() {
    // IDA 0x47c10: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47c10`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47c10()
}

// 0x47c18 — -[ControlView showControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView showControls]")]
pub fn stub_47c18() {
    // IDA 0x47c18: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47c18`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47c18()
}

// 0x47c2c — -[ControlView hideControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView hideControls]")]
pub fn stub_47c2c() {
    // IDA 0x47c2c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47c2c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47c2c()
}

// 0x47c40 — -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")]
pub fn stub_47c40(consumed: bool, is_tap: bool) {
    // IDA 0x47c40: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47c40`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47c40(consumed, is_tap)
}

// 0x47d48 — -[ControlView postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessed:inputObject:event:]")]
pub fn stub_47d48(consumed: bool, is_tap: bool) {
    // IDA 0x47d48: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47d48`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47d48(consumed, is_tap)
}

// 0x47d78 — -[ControlView setupLocalPlayerConnections]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupLocalPlayerConnections]")]
pub fn stub_47d78() {
    // IDA 0x47d78: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47d78`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47d78()
}

// 0x47d7c — -[ControlView textBoxFocusGained:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[ControlView textBoxFocusGained:]")]
pub fn stub_47d7c(textbox_present: bool) {
    // IDA 0x47d7c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47d7c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47d7c(textbox_present)
}

// 0x47ea4 — -[ControlView getGame]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, ControlView *self, SEL)
#[doc(alias = "-[ControlView getGame]")]
pub fn stub_47ea4() -> bool {
    // IDA 0x47ea4: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47ea4`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47ea4()
}

// 0x47f48 — -[ControlView setupEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupEvents]")]
pub fn stub_47f48() {
    // IDA 0x47f48: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x47f48`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x47f48()
}

// 0x4818c — -[ControlView disconnectEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView disconnectEvents]")]
pub fn stub_4818c() {
    // IDA 0x4818c: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x4818c`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x4818c()
}

// 0x481cc — -[ControlView bindToUserInputService:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView bindToUserInputService:]")]
pub fn stub_481cc(datamodel_present: bool, modal: bool) {
    // IDA 0x481cc: duplicate of the canonical cutover at
    // `crate::generated_bg_11::stub_0x481cc`. Delegate to keep one
    // source of truth.
    crate::generated_bg_11::stub_0x481cc(datamodel_present, modal)
}

// 0x48604 — -[ControlView bindUserInputService]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView bindUserInputService]")]
pub fn stub_48604(datamodel_present: bool, overlay_present: bool) {
    // IDA 0x48604: `bindUserInputService` binds the datamodel (0x48686)
    // and the overlay datamodel (0x486c8) via `bindToUserInputService:`.
    // The binds record here.
    if datamodel_present {
        USERINPUT_BINDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    if overlay_present {
        USERINPUT_BINDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x48774 — -[ControlView checkUserInputPropertyChanged:onDataModel:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView checkUserInputPropertyChanged:onDataModel:]")]
pub fn stub_48774(datamodel_present: bool, is_modal_prop: bool, service_present: bool, modal_on: bool) -> bool {
    // IDA 0x48774: `checkUserInputPropertyChanged:onDataModel:`
    // returns 0 without a datamodel, a `ModalEnabled` prop and a
    // found service (0x4877e-0x48792); else it shows or hides by
    // `getModalEnabled` (0x487a0-0x487ca) and returns 1. The branch
    // reports here.
    if !datamodel_present || !is_modal_prop || !service_present {
        return false;
    }
    if modal_on {
        crate::generated_bg_11::stub_0x47c18();
    } else {
        crate::generated_bg_11::stub_0x47c2c();
    }
    true
}

// 0x487d4 — -[ControlView isValidUserInputProperty:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView isValidUserInputProperty:]")]
pub fn stub_487d4(game_present: bool, name: Option<&str>) -> bool {
    // IDA 0x487d4: `isValidUserInputProperty:` needs a game and a
    // descriptor (0x487e8-0x487ee) whose name is not "Parent"
    // (0x48804).
    game_present && matches!(name, Some(n) if n != "Parent")
}

// 0x4880c — -[ControlView userInputPropertyChangedOnDataModel:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnDataModel:]")]
pub fn stub_4880c(valid: bool, datamodel_present: bool, is_modal_prop: bool, service_present: bool, modal_on: bool) -> bool {
    // IDA 0x4880c: `userInputPropertyChangedOnDataModel:` runs the
    // check on a valid property (0x4883a-0x488aa). It sequences here.
    if !valid {
        return false;
    }
    stub_48774(datamodel_present, is_modal_prop, service_present, modal_on)
}

// 0x48918 — -[ControlView userInputPropertyChangedOnOverlay:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnOverlay:]")]
pub fn stub_48918(valid: bool, overlay_present: bool, is_modal_prop: bool, service_present: bool, modal_on: bool) -> bool {
    // IDA 0x48918: `userInputPropertyChangedOnOverlay:` runs the same
    // check against the overlay datamodel (same shape as 0x4880c). It
    // sequences here.
    if !valid {
        return false;
    }
    stub_48774(overlay_present, is_modal_prop, service_present, modal_on)
}

// 0x48a50 — -[ControlView setupInputControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupInputControls]")]
pub fn stub_48a50() {
    // IDA 0x48a50: `setupInputControls` fixes tap/pinch constants
    // (0.19/20/0.08/-1.0, 0x48a8c-0x48ada) and rebuilds the camera,
    // character, jump, menu and keyboard controls (0x48b1a-0x48fae).
    // The rebuild records here; geometry is drop glue.
    INPUT_SETUP_BUILDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x48fe8 — -[ControlView gameLoaded]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView gameLoaded]")]
pub fn stub_48fe8() {
    // IDA 0x48fe8: `gameLoaded` shows the controls (0x48ff4). It
    // sequences the show here.
    crate::generated_bg_11::stub_0x47c18()
}

// 0x48ff8 — -[ControlView invalidateTapGesture:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView invalidateTapGesture:]")]
pub fn stub_48ff8(clear: bool) {
    // IDA 0x48ff8: `invalidateTapGesture:` clears a matching or nil
    // tap (0x48ffc-0x49012). The clear records here.
    if clear {
        TAP_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x49018 — -[ControlView createNativeMenu]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView createNativeMenu]")]
pub fn stub_49018() {
    // IDA 0x49018: `createNativeMenu` allocs the `MenuButton` at its
    // fixed frame and adds it (0x49038-0x49088). The build records
    // here; geometry is drop glue.
    MENU_BUILDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4908c — -[ControlView checkTouchesForTap:withEvent:]
// type: id __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView checkTouchesForTap:withEvent:]")]
pub fn stub_4908c(tap_set: bool, tap_in_set: bool, service_present: bool) -> bool {
    // IDA 0x4908c: `checkTouchesForTap:` single-taps when the captured
    // tap is in the set (0x490ba-0x49170), else reports miss
    // (0x49184). The hit reports here.
    if tap_set && tap_in_set {
        stub_49acc(service_present, true);
        return true;
    }
    false
}

// 0x4918c — -[ControlView sendMouseEventToGame:withTouch:]
// type: void __cdecl(ControlView *self, SEL, UIEvent, id)
#[doc(alias = "-[ControlView sendMouseEventToGame:withTouch:]")]
pub fn stub_4918c(service_present: bool) {
    // IDA 0x4918c: `sendMouseEventToGame:` creates the input service
    // and sends the mouse event (0x491fe-0x49286). The send records
    // here.
    if service_present {
        MOUSE_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x49314 — -[ControlView touchesBegan:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesBegan:withEvent:]")]
pub fn stub_49314(touch_count: u32, tap_free: bool, service_present: bool) {
    // IDA 0x49314: `touchesBegan:` captures a lone touch as `tapTouch`
    // with its begin position plus a delayed invalidate (0x4935e-0x49402),
    // then sends one mouse event per touch (0x49472-0x49500). The
    // capture + sends record here.
    if tap_free && touch_count == 1 {
        TAP_TOUCH_SET.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    if service_present {
        MOUSE_EVENTS.fetch_add(touch_count, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4951c — -[ControlView touchesEnded:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesEnded:withEvent:]")]
pub fn stub_4951c(tap_set: bool, tap_in_set: bool, other: u32, service_present: bool) {
    // IDA 0x4951c: `touchesEnded:` resets `pinchTime` (0x4955c), taps
    // via `checkTouchesForTap:` (0x4956c), invalidates a matching tap
    // and sends the rest as mouse-up events (0x495f8-0x4964a). It
    // sequences here.
    let tapped = stub_4908c(tap_set, tap_in_set, service_present);
    if tapped {
        TAP_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    if service_present {
        MOUSE_EVENTS.fetch_add(other, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x49684 — -[ControlView touchesMoved:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesMoved:withEvent:]")]
pub fn stub_49684(tap_in_set: bool, beyond_tolerance: bool, count: u32, service_present: bool) {
    // IDA 0x49684: `touchesMoved:` checks the tap move (0x496b8) then
    // sends one mouse event per touch (0x4972a-0x497b4). It sequences
    // here.
    stub_497d0(tap_in_set, beyond_tolerance);
    if service_present {
        MOUSE_EVENTS.fetch_add(count, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x497d0 — -[ControlView checkTapTouchMove:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView checkTapTouchMove:]")]
pub fn stub_497d0(tap_in_set: bool, beyond_tolerance: bool) {
    // IDA 0x497d0: `checkTapTouchMove:` invalidates the tap when the
    // captured touch moved past `tapTouchMoveTolerance` (0x498b4-0x49904).
    // The clear records here.
    if tap_in_set && beyond_tolerance {
        TAP_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x49920 — -[ControlView touchesCancelled:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesCancelled:withEvent:]")]
pub fn stub_49920(tap_match: bool) {
    // IDA 0x49920: `touchesCancelled:` clears a matching `tapTouch`
    // (0x499a2-0x499c6) with no mouse traffic. The clear records
    // here.
    if tap_match {
        TAP_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x499e0 — -[ControlView twoFingerPinch:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView twoFingerPinch:]")]
pub fn stub_499e0(began: bool, zoom_nonzero: bool, service_present: bool) {
    // IDA 0x499e0: `twoFingerPinch:` resets the scale on begin
    // (0x49a0e-0x49a20), ends the camera pan, clears the tap
    // (0x49a3c-0x49a50) and zooms by scale delta when nonzero
    // (0x49a6c-0x49ab8). The zoom records here.
    let _ = began;
    TAP_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
    if service_present && zoom_nonzero {
        PINCH_ZOOMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x49acc — -[ControlView oneFingerSingleTap]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView oneFingerSingleTap]")]
pub fn stub_49acc(service_present: bool, tap_set: bool) {
    // IDA 0x49acc: `oneFingerSingleTap` clears the tap (0x49b36) and
    // fires tool + mouse events through the input service (0x49b48-0x49ba8).
    // The events record here.
    if service_present && tap_set {
        TAP_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
        TOOL_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        MOUSE_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x49bb4 — -[ControlView gestureRecognizer:shouldReceiveTouch:]
// type: char __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView gestureRecognizer:shouldReceiveTouch:]")]
pub fn stub_49bb4(is_pinch: bool, hit_view: bool, first: bool, now: f64, delay: f64) -> bool {
    // IDA 0x49bb4: `gestureRecognizer:shouldReceiveTouch:` accepts
    // non-pinch recognizers (0x49bd2); a pinch needs a self/camera hit
    // (0x49c2e), stamps `pinchTime` on first contact (0x49c56-0x49c5c)
    // and otherwise accepts within `pinchZoomDelay` (0x49c70-0x49c90).
    if !is_pinch {
        return true;
    }
    if !hit_view {
        return false;
    }
    if first {
        *PINCH_TIME.lock() = now;
        return true;
    }
    now - *PINCH_TIME.lock() <= delay
}

// 0x49ca0 — -[ControlView .cxx_destruct]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView .cxx_destruct]")]
pub fn stub_49ca0() -> ! {
    todo!("0x49ca0 -[ControlView .cxx_destruct]")
}

// 0x49e18 — -[ControlView .cxx_construct]
// type: id __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView .cxx_construct]")]
pub fn stub_49e18() -> ! {
    todo!("0x49e18 -[ControlView .cxx_construct]")
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_49e7c() -> ! {
    todo!("0x49e7c rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")
}

// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
pub fn stub_49f64() -> ! {
    todo!("0x49f64 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)")
}

// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE")]
pub fn stub_4a28c() -> ! {
    todo!("0x4a28c rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)")
}

// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_")]
pub fn stub_4a49c() -> ! {
    todo!("0x4a49c boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)")
}

// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv")]
pub fn stub_4a540() -> ! {
    todo!("0x4a540 rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)")
}

// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_")]
pub fn stub_4a544() -> ! {
    todo!("0x4a544 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Tex")
}

// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev")]
pub fn stub_4a640() {
    // IDA 0x4a640: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev")]
pub fn stub_4a714() {
    // IDA 0x4a714: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv")]
pub fn stub_4a7ec() -> ! {
    todo!("0x4a7ec rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)")
}

// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv")]
pub fn stub_4a8fc() -> ! {
    todo!("0x4a8fc rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const")
}

// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")]
pub fn stub_4a908() -> ! {
    todo!("0x4a908 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)")
}

// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")]
pub fn stub_4a9dc() {
    // IDA 0x4a9dc: non-virtual thunk to `"'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ` (IDA demangle) -- this/arg-adjust + tail-call. Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// type: int(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
#[doc(alias = "__ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_")]
pub fn stub_4a9e4() -> ! {
    todo!("0x4a9e4 boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const")
}

// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE")]
pub fn stub_4aaf4() -> ! {
    todo!("0x4aaf4 rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)")
}

// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv")]
pub fn stub_4abe4() -> ! {
    todo!("0x4abe4 rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")
}

// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_4abe8() -> ! {
    todo!("0x4abe8 rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")
}

// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev")]
pub fn stub_4acd8() {
    // IDA 0x4acd8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev")]
pub fn stub_4adac() {
    // IDA 0x4adac: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev")]
pub fn stub_4ae84() {
    // IDA 0x4ae84: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev")]
pub fn stub_4af30() {
    // IDA 0x4af30: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// type: int(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_")]
pub fn stub_4afe0() -> ! {
    todo!("0x4afe0 boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)")
}

// 0x4b010 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_4b010() -> ! {
    todo!("0x4b010 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boos")
}

// 0x4b070 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_4b070() -> ! {
    todo!("0x4b070 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,vo")
}

// 0x4b088 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_4b088() -> ! {
    todo!("0x4b088 void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void ")
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE")]
pub fn stub_4b164() -> ! {
    todo!("0x4b164 rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_")]
pub fn stub_4b374() -> ! {
    todo!("0x4b374 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_")]
pub fn stub_4b418() -> ! {
    todo!("0x4b418 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv")]
pub fn stub_4b4bc() -> ! {
    todo!("0x4b4bc rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv")]
pub fn stub_4b4c0() -> ! {
    todo!("0x4b4c0 rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")
}

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_4b5b8() -> ! {
    todo!("0x4b5b8 rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,")
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_4b6b4() {
    // IDA 0x4b6b4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
pub fn stub_4b788() {
    // IDA 0x4b788: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv")]
pub fn stub_4b860() -> ! {
    todo!("0x4b860 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv")]
pub fn stub_4b970() -> ! {
    todo!("0x4b970 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")]
pub fn stub_4b97c() -> ! {
    todo!("0x4b97c rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")]
pub fn stub_4b984() {
    // IDA 0x4b984: non-virtual thunk to `"'rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RB` (IDA demangle) -- this/arg-adjust + tail-call. Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
#[doc(alias = "__ZNK5boost9function1IvPN3RBX9DataModelEEclES3_")]
pub fn stub_4b98c() -> ! {
    todo!("0x4b98c boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE")]
pub fn stub_4ba50() -> ! {
    todo!("0x4ba50 rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv")]
pub fn stub_4bb40() -> ! {
    todo!("0x4bb40 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")
}
