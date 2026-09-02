// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|RBX::Workspace (10215 filtered, 0 remaining) — EA-sorted asc next 100 DM gaps not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x44abc..0x4a21c | dm distinct before 19966, after 20066, dm missing 65579->65479
// Shard: 194 EA-sorted asc next 100 DM gaps after 193 (filtered exhausted, global filler 0x44abc..0x4a21c)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x44abc — -[CameraControl init:delegate:]
// type: id __cdecl(CameraControl *self, SEL, CGRect, id)
#[doc(alias = "-[CameraControl init:delegate:]")]
pub fn stub_44abc() -> ! {
    todo!("0x44abc -[CameraControl init:delegate:]")
}

// 0x44b90 — -[CameraControl dealloc]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl dealloc]")]
pub fn stub_44b90() -> ! {
    todo!("0x44b90 -[CameraControl dealloc]")
}

// 0x44bbc — -[CameraControl setupPostMouseEventConnection]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl setupPostMouseEventConnection]")]
pub fn stub_44bbc() -> ! {
    todo!("0x44bbc -[CameraControl setupPostMouseEventConnection]")
}

// 0x44cd4 — -[CameraControl postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(CameraControl *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[CameraControl postMouseEventProcessed:inputObject:event:]")]
pub fn stub_44cd4() -> ! {
    todo!("0x44cd4 -[CameraControl postMouseEventProcessed:inputObject:event:]")
}

// 0x44d04 — -[CameraControl doCameraPanTouchBegan]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchBegan]")]
pub fn stub_44d04() -> ! {
    todo!("0x44d04 -[CameraControl doCameraPanTouchBegan]")
}

// 0x44dec — -[CameraControl doCameraPanTouchEnded]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchEnded]")]
pub fn stub_44dec() -> ! {
    todo!("0x44dec -[CameraControl doCameraPanTouchEnded]")
}

// 0x44e58 — -[CameraControl doCameraPanTouchMove]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchMove]")]
pub fn stub_44e58() -> ! {
    todo!("0x44e58 -[CameraControl doCameraPanTouchMove]")
}

// 0x450a0 — -[CameraControl touchesBegan:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesBegan:withEvent:]")]
pub fn stub_450a0() -> ! {
    todo!("0x450a0 -[CameraControl touchesBegan:withEvent:]")
}

// 0x45124 — -[CameraControl touchesEnded:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesEnded:withEvent:]")]
pub fn stub_45124() -> ! {
    todo!("0x45124 -[CameraControl touchesEnded:withEvent:]")
}

// 0x45234 — -[CameraControl touchesCancelled:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesCancelled:withEvent:]")]
pub fn stub_45234() -> ! {
    todo!("0x45234 -[CameraControl touchesCancelled:withEvent:]")
}

// 0x45344 — -[CameraControl touchesMoved:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesMoved:withEvent:]")]
pub fn stub_45344() -> ! {
    todo!("0x45344 -[CameraControl touchesMoved:withEvent:]")
}

// 0x45454 — -[CameraControl .cxx_construct]
// type: id __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl .cxx_construct]")]
pub fn stub_45454() -> ! {
    todo!("0x45454 -[CameraControl .cxx_construct]")
}

// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
pub fn stub_4546c() -> ! {
    todo!("0x4546c rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")
}

// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn stub_45554() -> ! {
    todo!("0x45554 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")
}

// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
pub fn stub_45764() -> ! {
    todo!("0x45764 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")
}

// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
pub fn stub_45808() -> ! {
    todo!("0x45808 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")
}

// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
pub fn stub_458ac() -> ! {
    todo!("0x458ac rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")
}

// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
pub fn stub_459a4() -> ! {
    todo!("0x459a4 rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")
}

// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
pub fn stub_45aa0() -> ! {
    todo!("0x45aa0 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")
}

// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
pub fn stub_45b74() -> ! {
    todo!("0x45b74 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")
}

// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
pub fn stub_45c4c() -> ! {
    todo!("0x45c4c rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")
}

// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
pub fn stub_45d5c() -> ! {
    todo!("0x45d5c rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")
}

// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn stub_45d68() -> ! {
    todo!("0x45d68 rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")
}

// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
pub fn stub_45d98() -> ! {
    todo!("0x45d98 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")
}

// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
pub fn stub_45dc8() -> ! {
    todo!("0x45dc8 boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")
}

// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn stub_45eb0() -> ! {
    todo!("0x45eb0 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")
}

// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
pub fn stub_45fa0() -> ! {
    todo!("0x45fa0 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")
}

// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_45fa4() -> ! {
    todo!("0x45fa4 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")
}

// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn stub_46094() -> ! {
    todo!("0x46094 rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")
}

// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn stub_46168() -> ! {
    todo!("0x46168 rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")
}

// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn stub_46240() -> ! {
    todo!("0x46240 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")
}

// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn stub_462ec() -> ! {
    todo!("0x462ec rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")
}

// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
pub fn stub_4639c() -> ! {
    todo!("0x4639c boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")
}

// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_463cc() -> ! {
    todo!("0x463cc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")
}

// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
pub fn stub_4642c() -> ! {
    todo!("0x4642c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")
}

// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
pub fn stub_46464() -> ! {
    todo!("0x46464 boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")
}

// 0x46490 — __GLOBAL__I_a_15
#[doc(alias = "global constructor keyed to_a_15")]
// was: global constructor keyed to_a_15
pub fn stub_46490() -> ! {
    todo!("0x46490 global constructor keyed to_a_15")
}

// 0x466cc — -[CharacterMove init:]
// type: id __cdecl(CharacterMove *self, SEL, CGRect)
#[doc(alias = "-[CharacterMove init:]")]
pub fn stub_466cc() -> ! {
    todo!("0x466cc -[CharacterMove init:]")
}

// 0x46704 — -[CharacterMove setupCharacterMoveConnection]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove setupCharacterMoveConnection]")]
pub fn stub_46704() -> ! {
    todo!("0x46704 -[CharacterMove setupCharacterMoveConnection]")
}

// 0x467e8 — -[CharacterMove localCharacterMovementEnabledChange:]
// type: void __cdecl(CharacterMove *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[CharacterMove localCharacterMovementEnabledChange:]")]
pub fn stub_467e8() -> ! {
    todo!("0x467e8 -[CharacterMove localCharacterMovementEnabledChange:]")
}

// 0x467ec — -[CharacterMove touchesEnded:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesEnded:withEvent:]")]
pub fn stub_467ec() -> ! {
    todo!("0x467ec -[CharacterMove touchesEnded:withEvent:]")
}

// 0x468bc — -[CharacterMove touchesCancelled:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesCancelled:withEvent:]")]
pub fn stub_468bc() -> ! {
    todo!("0x468bc -[CharacterMove touchesCancelled:withEvent:]")
}

// 0x4698c — -[CharacterMove cancelMovement]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove cancelMovement]")]
pub fn stub_4698c() -> ! {
    todo!("0x4698c -[CharacterMove cancelMovement]")
}

// 0x469e8 — -[CharacterMove touchesMoved:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesMoved:withEvent:]")]
pub fn stub_469e8() -> ! {
    todo!("0x469e8 -[CharacterMove touchesMoved:withEvent:]")
}

// 0x46c18 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
pub fn stub_46c18() -> ! {
    todo!("0x46c18 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")
}

// 0x46c8c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_46c8c() -> ! {
    todo!("0x46c8c rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

// 0x46d38 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_46d38() -> ! {
    todo!("0x46d38 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

// 0x46de8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_46de8() -> ! {
    todo!("0x46de8 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0x46df8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_46df8() -> ! {
    todo!("0x46df8 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0x46e08 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_46e08() -> ! {
    todo!("0x46e08 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x46eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_46eb4() -> ! {
    todo!("0x46eb4 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x46f64 — __GLOBAL__I_a_16
#[doc(alias = "global constructor keyed to_a_16")]
// was: global constructor keyed to_a_16
pub fn stub_46f64() -> ! {
    todo!("0x46f64 global constructor keyed to_a_16")
}

// 0x47178 — -[ControlComponent init]
// type: ControlComponent *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent init]")]
pub fn stub_47178() -> ! {
    todo!("0x47178 -[ControlComponent init]")
}

// 0x471c0 — -[ControlComponent findControlView]
// type: id __cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent findControlView]")]
pub fn stub_471c0() -> ! {
    todo!("0x471c0 -[ControlComponent findControlView]")
}

// 0x47274 — -[ControlComponent getGameFromControlView]
// type: Game *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getGameFromControlView]")]
pub fn stub_47274() -> ! {
    todo!("0x47274 -[ControlComponent getGameFromControlView]")
}

// 0x47424 — __GLOBAL__I_a_17
#[doc(alias = "global constructor keyed to_a_17")]
// was: global constructor keyed to_a_17
pub fn stub_47424() -> ! {
    todo!("0x47424 global constructor keyed to_a_17")
}

// 0x47638 — -[ControlView init:withGame:]
// type: id __cdecl(ControlView *self, SEL, CGRect, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView init:withGame:]")]
pub fn stub_47638() -> ! {
    todo!("0x47638 -[ControlView init:withGame:]")
}

// 0x47904 — -[ControlView dealloc]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView dealloc]")]
pub fn stub_47904() -> ! {
    todo!("0x47904 -[ControlView dealloc]")
}

// 0x479f8 — -[ControlView setGame:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView setGame:]")]
pub fn stub_479f8() -> ! {
    todo!("0x479f8 -[ControlView setGame:]")
}

// 0x47aec — -[ControlView gotStartLeaveGameNotification:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView gotStartLeaveGameNotification:]")]
pub fn stub_47aec() -> ! {
    todo!("0x47aec -[ControlView gotStartLeaveGameNotification:]")
}

// 0x47afc — -[ControlView dataModelChanged:]
// type: void __cdecl(ControlView *self, SEL, DataModel *)
#[doc(alias = "-[ControlView dataModelChanged:]")]
pub fn stub_47afc() -> ! {
    todo!("0x47afc -[ControlView dataModelChanged:]")
}

// 0x47b38 — -[ControlView setControlVisibility:]
// type: void __cdecl(ControlView *self, SEL, char)
#[doc(alias = "-[ControlView setControlVisibility:]")]
pub fn stub_47b38() -> ! {
    todo!("0x47b38 -[ControlView setControlVisibility:]")
}

// 0x47b90 — ___36-[ControlView setControlVisibility:]_block_invoke
#[doc(alias = "___36-[ControlView setControlVisibility:]_block_invoke")]
pub fn stub_47b90() -> ! {
    todo!("0x47b90 ___36-[ControlView setControlVisibility:]_block_invoke")
}

// 0x47c04 — ___copy_helper_block__8
#[doc(alias = "___copy_helper_block__8")]
pub fn stub_47c04() -> ! {
    todo!("0x47c04 ___copy_helper_block__8")
}

// 0x47c10 — ___destroy_helper_block__8
#[doc(alias = "___destroy_helper_block__8")]
pub fn stub_47c10() -> ! {
    todo!("0x47c10 ___destroy_helper_block__8")
}

// 0x47c18 — -[ControlView showControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView showControls]")]
pub fn stub_47c18() -> ! {
    todo!("0x47c18 -[ControlView showControls]")
}

// 0x47c2c — -[ControlView hideControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView hideControls]")]
pub fn stub_47c2c() -> ! {
    todo!("0x47c2c -[ControlView hideControls]")
}

// 0x47c40 — -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")]
pub fn stub_47c40() -> ! {
    todo!("0x47c40 -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")
}

// 0x47d48 — -[ControlView postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessed:inputObject:event:]")]
pub fn stub_47d48() -> ! {
    todo!("0x47d48 -[ControlView postMouseEventProcessed:inputObject:event:]")
}

// 0x47d78 — -[ControlView setupLocalPlayerConnections]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupLocalPlayerConnections]")]
pub fn stub_47d78() -> ! {
    todo!("0x47d78 -[ControlView setupLocalPlayerConnections]")
}

// 0x47d7c — -[ControlView textBoxFocusGained:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[ControlView textBoxFocusGained:]")]
pub fn stub_47d7c() -> ! {
    todo!("0x47d7c -[ControlView textBoxFocusGained:]")
}

// 0x47ea4 — -[ControlView getGame]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, ControlView *self, SEL)
#[doc(alias = "-[ControlView getGame]")]
pub fn stub_47ea4() -> ! {
    todo!("0x47ea4 -[ControlView getGame]")
}

// 0x47f48 — -[ControlView setupEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupEvents]")]
pub fn stub_47f48() -> ! {
    todo!("0x47f48 -[ControlView setupEvents]")
}

// 0x4818c — -[ControlView disconnectEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView disconnectEvents]")]
pub fn stub_4818c() -> ! {
    todo!("0x4818c -[ControlView disconnectEvents]")
}

// 0x481cc — -[ControlView bindToUserInputService:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView bindToUserInputService:]")]
pub fn stub_481cc() -> ! {
    todo!("0x481cc -[ControlView bindToUserInputService:]")
}

// 0x48604 — -[ControlView bindUserInputService]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView bindUserInputService]")]
pub fn stub_48604() -> ! {
    todo!("0x48604 -[ControlView bindUserInputService]")
}

// 0x487d4 — -[ControlView isValidUserInputProperty:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView isValidUserInputProperty:]")]
pub fn stub_487d4() -> ! {
    todo!("0x487d4 -[ControlView isValidUserInputProperty:]")
}

// 0x48918 — -[ControlView userInputPropertyChangedOnOverlay:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnOverlay:]")]
pub fn stub_48918() -> ! {
    todo!("0x48918 -[ControlView userInputPropertyChangedOnOverlay:]")
}

// 0x48a50 — -[ControlView setupInputControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupInputControls]")]
pub fn stub_48a50() -> ! {
    todo!("0x48a50 -[ControlView setupInputControls]")
}

// 0x48fe8 — -[ControlView gameLoaded]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView gameLoaded]")]
pub fn stub_48fe8() -> ! {
    todo!("0x48fe8 -[ControlView gameLoaded]")
}

// 0x48ff8 — -[ControlView invalidateTapGesture:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView invalidateTapGesture:]")]
pub fn stub_48ff8() -> ! {
    todo!("0x48ff8 -[ControlView invalidateTapGesture:]")
}

// 0x49018 — -[ControlView createNativeMenu]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView createNativeMenu]")]
pub fn stub_49018() -> ! {
    todo!("0x49018 -[ControlView createNativeMenu]")
}

// 0x4908c — -[ControlView checkTouchesForTap:withEvent:]
// type: id __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView checkTouchesForTap:withEvent:]")]
pub fn stub_4908c() -> ! {
    todo!("0x4908c -[ControlView checkTouchesForTap:withEvent:]")
}

// 0x4918c — -[ControlView sendMouseEventToGame:withTouch:]
// type: void __cdecl(ControlView *self, SEL, UIEvent, id)
#[doc(alias = "-[ControlView sendMouseEventToGame:withTouch:]")]
pub fn stub_4918c() -> ! {
    todo!("0x4918c -[ControlView sendMouseEventToGame:withTouch:]")
}

// 0x49314 — -[ControlView touchesBegan:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesBegan:withEvent:]")]
pub fn stub_49314() -> ! {
    todo!("0x49314 -[ControlView touchesBegan:withEvent:]")
}

// 0x4951c — -[ControlView touchesEnded:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesEnded:withEvent:]")]
pub fn stub_4951c() -> ! {
    todo!("0x4951c -[ControlView touchesEnded:withEvent:]")
}

// 0x49684 — -[ControlView touchesMoved:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesMoved:withEvent:]")]
pub fn stub_49684() -> ! {
    todo!("0x49684 -[ControlView touchesMoved:withEvent:]")
}

// 0x497d0 — -[ControlView checkTapTouchMove:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView checkTapTouchMove:]")]
pub fn stub_497d0() -> ! {
    todo!("0x497d0 -[ControlView checkTapTouchMove:]")
}

// 0x49920 — -[ControlView touchesCancelled:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesCancelled:withEvent:]")]
pub fn stub_49920() -> ! {
    todo!("0x49920 -[ControlView touchesCancelled:withEvent:]")
}

// 0x499e0 — -[ControlView twoFingerPinch:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView twoFingerPinch:]")]
pub fn stub_499e0() -> ! {
    todo!("0x499e0 -[ControlView twoFingerPinch:]")
}

// 0x49acc — -[ControlView oneFingerSingleTap]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView oneFingerSingleTap]")]
pub fn stub_49acc() -> ! {
    todo!("0x49acc -[ControlView oneFingerSingleTap]")
}

// 0x49bb4 — -[ControlView gestureRecognizer:shouldReceiveTouch:]
// type: char __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView gestureRecognizer:shouldReceiveTouch:]")]
pub fn stub_49bb4() -> ! {
    todo!("0x49bb4 -[ControlView gestureRecognizer:shouldReceiveTouch:]")
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

// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)
pub fn stub_49f64() -> ! {
    todo!("0x49f64 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")
}

// 0x4a04c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")]
pub fn stub_4a04c() -> ! {
    todo!("0x4a04c rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")
}

// 0x4a148 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_4a148() -> ! {
    todo!("0x4a148 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0x4a150 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_4a150() -> ! {
    todo!("0x4a150 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0x4a158 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_4a158() -> ! {
    todo!("0x4a158 boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")
}

// 0x4a21c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4a21c() -> ! {
    todo!("0x4a21c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")
}
