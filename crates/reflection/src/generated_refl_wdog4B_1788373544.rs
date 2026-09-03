//! reflection — generated_refl_wdog4B_1788373544 — 120 stubs EA-sorted asc 0x3a9364..0x3af66c (reflection gap filler distinct not yet in crates/reflection/src — next 120 uncovered; RBX::Reflection filter yielded 0 remaining so global reflection-gap asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc not yet in crates/reflection/src — next 120 uncovered sorted asc (RBX::Reflection strict filter exhausted 0 remaining -> fallback to global reflection-gap)
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3a9364 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev")]
pub fn stub_0x3a9364() {
    // IDA 0x3a9364: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a9368 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev")]
pub fn stub_0x3a9368() {
    // IDA 0x3a9368: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a936c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv")]
pub fn stub_0x3a936c() -> ! {
    todo!("0x3a936c boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")
}

// 0x3a9378 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a9378() -> ! {
    todo!("0x3a9378 boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")
}

// 0x3a937c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv")]
pub fn stub_0x3a937c() -> ! {
    todo!("0x3a937c boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")
}

// 0x3a9380 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_0x3a9380() -> ! {
    todo!("0x3a9380 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> &)")
}

// 0x3a94e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception")]
pub fn stub_0x3a94e0() -> ! {
    todo!("0x3a94e0 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")
}

// 0x3a9508 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSERKSA_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSERKSA_")]
pub fn stub_0x3a9508() -> ! {
    todo!("0x3a9508 rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> const&)")
}

// 0x3a952c — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE22safe_static_init_mutexEv")]
pub fn stub_0x3a952c() -> ! {
    todo!("0x3a952c rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")
}

// 0x3a9530 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE24safe_static_do_get_mutexEv")]
pub fn stub_0x3a9530() -> ! {
    todo!("0x3a9530 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_do_get_mutex(void)")
}

// 0x3a9628 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_0x3a9628() -> ! {
    todo!("0x3a9628 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> &)")
}

// 0x3a9788 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE8on_errorERSt9exception")]
pub fn stub_0x3a9788() -> ! {
    todo!("0x3a9788 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::on_error(std::exception &)")
}

// 0x3a97b0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSERKSA_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSERKSA_")]
pub fn stub_0x3a97b0() -> ! {
    todo!("0x3a97b0 rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> const&)")
}

// 0x3a97d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE22safe_static_init_mutexEv")]
pub fn stub_0x3a97d4() -> ! {
    todo!("0x3a97d4 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_init_mutex(void)")
}

// 0x3a97d8 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE24safe_static_do_get_mutexEv")]
pub fn stub_0x3a97d8() -> ! {
    todo!("0x3a97d8 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_do_get_mutex(void)")
}

// 0x3a98d0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3a98d0() -> ! {
    todo!("0x3a98d0 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")
}

// 0x3a9944 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv")]
pub fn stub_0x3a9944() -> ! {
    todo!("0x3a9944 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")
}

// 0x3a9990 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev")]
pub fn stub_0x3a9990() {
    // IDA 0x3a9990: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a99bc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev")]
pub fn stub_0x3a99bc() {
    // IDA 0x3a99bc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a9a90 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3a9a90() -> ! {
    todo!("0x3a9a90 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9a98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3a9a98() {
    // IDA 0x3a9a98: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vect` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a9aa0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_0x3a9aa0() -> ! {
    todo!("0x3a9aa0 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")
}

// 0x3a9ab8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x3a9ab8() {
    // IDA 0x3a9ab8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a9ae4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x3a9ae4() {
    // IDA 0x3a9ae4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a9bb8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3a9bb8() -> ! {
    todo!("0x3a9bb8 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")
}

// 0x3a9c2c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv")]
pub fn stub_0x3a9c2c() -> ! {
    todo!("0x3a9c2c RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")
}

// 0x3a9c78 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev")]
pub fn stub_0x3a9c78() {
    // IDA 0x3a9c78: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a9ca4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev")]
pub fn stub_0x3a9ca4() {
    // IDA 0x3a9ca4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a9d78 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3a9d78() -> ! {
    todo!("0x3a9d78 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9d80 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3a9d80() {
    // IDA 0x3a9d80: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vect` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a9d88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_0x3a9d88() -> ! {
    todo!("0x3a9d88 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")
}

// 0x3a9da0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x3a9da0() {
    // IDA 0x3a9da0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a9dcc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x3a9dcc() {
    // IDA 0x3a9dcc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a9ea0 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev")]
pub fn stub_0x3a9ea0() -> ! {
    todo!("0x3a9ea0 rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")
}

// 0x3a9ffc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13disconnectAllEv")]
pub fn stub_0x3a9ffc() -> ! {
    todo!("0x3a9ffc rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::disconnectAll(void)")
}

// 0x3aa174 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev")]
pub fn stub_0x3aa174() -> ! {
    todo!("0x3aa174 rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")
}

// 0x3aa2d0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13disconnectAllEv")]
pub fn stub_0x3aa2d0() -> ! {
    todo!("0x3aa2d0 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::disconnectAll(void)")
}

// 0x3aa448 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")]
pub fn stub_0x3aa448() -> ! {
    todo!("0x3aa448 RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")
}

// 0x3aa5a4 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff")]
pub fn stub_0x3aa5a4() -> ! {
    todo!("0x3aa5a4 RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")
}

// 0x3aa764 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3aa764() -> ! {
    todo!("0x3aa764 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0x3aa7d8 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6insertEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6insertEPNS6_4slotE")]
pub fn stub_0x3aa7d8() -> ! {
    todo!("0x3aa7d8 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")
}

// 0x3aa9e4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSEPS9_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSEPS9_")]
pub fn stub_0x3aa9e4() -> ! {
    todo!("0x3aa9e4 rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot*)")
}

// 0x3aaa08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev")]
pub fn stub_0x3aaa08() {
    // IDA 0x3aaa08: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3aaa34 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev")]
pub fn stub_0x3aaa34() {
    // IDA 0x3aaa34: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3aab08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot10disconnectEv")]
pub fn stub_0x3aab08() -> ! {
    todo!("0x3aab08 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::disconnect(void)")
}

// 0x3aac18 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot9connectedEv")]
pub fn stub_0x3aac18() -> ! {
    todo!("0x3aac18 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::connected(void)const")
}

// 0x3aac24 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff")]
pub fn stub_0x3aac24() -> ! {
    todo!("0x3aac24 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3aac50 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff")]
pub fn stub_0x3aac50() {
    // IDA 0x3aac50: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3aac7c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x3aac7c() -> ! {
    todo!("0x3aac7c void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")
}

// 0x3aacbc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6removeEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6removeEPNS6_4slotE")]
pub fn stub_0x3aacbc() -> ! {
    todo!("0x3aacbc rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")
}

// 0x3aadac — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x3aadac() -> ! {
    todo!("0x3aadac rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_init_mutex(void)")
}

// 0x3aadb0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x3aadb0() -> ! {
    todo!("0x3aadb0 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_do_get_mutex(void)")
}

// 0x3aaea0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD1Ev")]
pub fn stub_0x3aaea0() {
    // IDA 0x3aaea0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3aaecc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD0Ev")]
pub fn stub_0x3aaecc() {
    // IDA 0x3aaecc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3aafa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev")]
pub fn stub_0x3aafa0() {
    // IDA 0x3aafa0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3aafcc — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev")]
pub fn stub_0x3aafcc() {
    // IDA 0x3aafcc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ab0a0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")]
pub fn stub_0x3ab0a0() -> ! {
    todo!("0x3ab0a0 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")
}

// 0x3ab0a4 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")]
pub fn stub_0x3ab0a4() -> ! {
    todo!("0x3ab0a4 RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")
}

// 0x3ab200 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_")]
pub fn stub_0x3ab200() -> ! {
    todo!("0x3ab200 RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")
}

// 0x3ab360 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3ab360() -> ! {
    todo!("0x3ab360 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")
}

// 0x3ab3d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6insertEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6insertEPNS6_4slotE")]
pub fn stub_0x3ab3d4() -> ! {
    todo!("0x3ab3d4 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")
}

// 0x3ab5e0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSEPS9_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSEPS9_")]
pub fn stub_0x3ab5e0() -> ! {
    todo!("0x3ab5e0 rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot*)")
}

// 0x3ab604 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x3ab604() {
    // IDA 0x3ab604: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ab630 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x3ab630() {
    // IDA 0x3ab630: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ab704 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot10disconnectEv")]
pub fn stub_0x3ab704() -> ! {
    todo!("0x3ab704 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")
}

// 0x3ab814 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot9connectedEv")]
pub fn stub_0x3ab814() -> ! {
    todo!("0x3ab814 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")
}

// 0x3ab820 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_0x3ab820() -> ! {
    todo!("0x3ab820 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")
}

// 0x3ab834 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_0x3ab834() {
    // IDA 0x3ab834: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,v` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3ab848 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_")]
pub fn stub_0x3ab848() -> ! {
    todo!("0x3ab848 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")
}

// 0x3ab860 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6removeEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6removeEPNS6_4slotE")]
pub fn stub_0x3ab860() -> ! {
    todo!("0x3ab860 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")
}

// 0x3ab950 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x3ab950() -> ! {
    todo!("0x3ab950 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")
}

// 0x3ab954 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x3ab954() -> ! {
    todo!("0x3ab954 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")
}

// 0x3aba44 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD1Ev")]
pub fn stub_0x3aba44() {
    // IDA 0x3aba44: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3aba70 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD0Ev")]
pub fn stub_0x3aba70() {
    // IDA 0x3aba70: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3abb44 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
pub fn stub_0x3abb44() {
    // IDA 0x3abb44: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3abb70 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
pub fn stub_0x3abb70() {
    // IDA 0x3abb70: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3abc44 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")]
pub fn stub_0x3abc44() -> ! {
    todo!("0x3abc44 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")
}

// 0x3abc48 — __ZNK3RBX11HandlesBase13getHandleTypeEv
#[doc(alias = "RBX::HandlesBase::getHandleType(void)const")]
#[doc(alias = "__ZNK3RBX11HandlesBase13getHandleTypeEv")]
pub fn stub_0x3abc48() -> ! {
    todo!("0x3abc48 RBX::HandlesBase::getHandleType(void)const")
}

// 0x3abc4c — __ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv
#[doc(alias = "RBX::HandlesBase::getHandlesNormalIdMask(void)const")]
#[doc(alias = "__ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv")]
pub fn stub_0x3abc4c() -> ! {
    todo!("0x3abc4c RBX::HandlesBase::getHandlesNormalIdMask(void)const")
}

// 0x3abe4c — __ZN3RBX11HandlesBaseD2Ev
#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZN3RBX11HandlesBaseD2Ev")]
pub fn stub_0x3abe4c() {
    // IDA 0x3abe4c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac098 — __ZN3RBX11HandlesBaseD1Ev
#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZN3RBX11HandlesBaseD1Ev")]
pub fn stub_0x3ac098() {
    // IDA 0x3ac098: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac09c — __ZN3RBX11HandlesBaseD0Ev
#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZN3RBX11HandlesBaseD0Ev")]
pub fn stub_0x3ac09c() {
    // IDA 0x3ac09c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ac13c — __ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
pub fn stub_0x3ac13c() -> ! {
    todo!("0x3ac13c __ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")
}

// 0x3ac140 — __ZThn32_N3RBX11HandlesBaseD1Ev
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZThn32_N3RBX11HandlesBaseD1Ev")]
pub fn stub_0x3ac140() {
    // IDA 0x3ac140: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac148 — __ZThn32_N3RBX11HandlesBaseD0Ev
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZThn32_N3RBX11HandlesBaseD0Ev")]
pub fn stub_0x3ac148() {
    // IDA 0x3ac148: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ac1ec — __ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
pub fn stub_0x3ac1ec() -> &'static str {
    // IDA 0x3ac1ec: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x3ac1ec family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "HandlesBase"
}

// 0x3ac1f0 — __ZThn36_N3RBX11HandlesBaseD1Ev
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZThn36_N3RBX11HandlesBaseD1Ev")]
pub fn stub_0x3ac1f0() {
    // IDA 0x3ac1f0: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac1f8 — __ZThn36_N3RBX11HandlesBaseD0Ev
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
#[doc(alias = "__ZThn36_N3RBX11HandlesBaseD0Ev")]
pub fn stub_0x3ac1f8() {
    // IDA 0x3ac1f8: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ac29c — __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")]
pub fn stub_0x3ac29c() -> ! {
    todo!("0x3ac29c __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")
}

// 0x3ac2e0 — __ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")]
pub fn stub_0x3ac2e0() -> ! {
    todo!("0x3ac2e0 __ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")
}

// 0x3ac2e4 — __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")]
pub fn stub_0x3ac2e4() -> ! {
    todo!("0x3ac2e4 __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")
}

// 0x3ac3c8 — __ZN3RBX13PartAdornmentD1Ev
#[doc(alias = "RBX::PartAdornment::~PartAdornment()")]
#[doc(alias = "__ZN3RBX13PartAdornmentD1Ev")]
pub fn stub_0x3ac3c8() {
    // IDA 0x3ac3c8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac510 — __ZN3RBX13PartAdornmentD0Ev
#[doc(alias = "RBX::PartAdornment::~PartAdornment()")]
#[doc(alias = "__ZN3RBX13PartAdornmentD0Ev")]
pub fn stub_0x3ac510() {
    // IDA 0x3ac510: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ac5b0 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
pub fn stub_0x3ac5b0() -> ! {
    todo!("0x3ac5b0 __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")
}

// 0x3ac5b4 — __ZThn32_N3RBX13PartAdornmentD1Ev
#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment()")]
#[doc(alias = "__ZThn32_N3RBX13PartAdornmentD1Ev")]
pub fn stub_0x3ac5b4() {
    // IDA 0x3ac5b4: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac6fc — __ZThn32_N3RBX13PartAdornmentD0Ev
#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment()")]
#[doc(alias = "__ZThn32_N3RBX13PartAdornmentD0Ev")]
pub fn stub_0x3ac6fc() {
    // IDA 0x3ac6fc: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ac858 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
pub fn stub_0x3ac858() -> &'static str {
    // IDA 0x3ac858: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x3ac858 family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "PartAdornment"
}

// 0x3ac85c — __ZThn36_N3RBX13PartAdornmentD1Ev
#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment()")]
#[doc(alias = "__ZThn36_N3RBX13PartAdornmentD1Ev")]
pub fn stub_0x3ac85c() {
    // IDA 0x3ac85c: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3ac9a4 — __ZThn36_N3RBX13PartAdornmentD0Ev
#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment()")]
#[doc(alias = "__ZThn36_N3RBX13PartAdornmentD0Ev")]
pub fn stub_0x3ac9a4() {
    // IDA 0x3ac9a4: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3acb00 — __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")]
pub fn stub_0x3acb00() -> ! {
    todo!("0x3acb00 __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")
}

// 0x3acb44 — __ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")]
pub fn stub_0x3acb44() -> ! {
    todo!("0x3acb44 __ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")
}

// 0x3acb48 — __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")]
pub fn stub_0x3acb48() -> ! {
    todo!("0x3acb48 __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")
}

// 0x3ad3bc — __ZN5boost9function3IvN3G3D7Vector34AxisEffE5clearEv
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvN3G3D7Vector34AxisEffE5clearEv")]
pub fn stub_0x3ad3bc() -> ! {
    todo!("0x3ad3bc boost::function3<void,G3D::Vector3::Axis,float,float>::clear(void)")
}

// 0x3adb10 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::function<void ()(G3D::Vector3::Axis,float,float)>>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_0x3adb10() -> ! {
    todo!("0x3adb10 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::function<void ()(G3D::Vector3::Axis,float,float)>>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&)")
}

// 0x3adc04 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_0x3adc04() -> ! {
    todo!("0x3adc04 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*)")
}

// 0x3add00 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_0x3add00() {
    // IDA 0x3add00: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ade10 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
pub fn stub_0x3ade10() {
    // IDA 0x3ade10: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3adf40 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff")]
pub fn stub_0x3adf40() -> ! {
    todo!("0x3adf40 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3adf48 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff")]
pub fn stub_0x3adf48() {
    // IDA 0x3adf48: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3adf50 — __ZNK5boost9function3IvN3G3D7Vector34AxisEffEclES3_ff
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")]
#[doc(alias = "__ZNK5boost9function3IvN3G3D7Vector34AxisEffEclES3_ff")]
pub fn stub_0x3adf50() -> ! {
    todo!("0x3adf50 boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")
}

// 0x3ae028 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev")]
pub fn stub_0x3ae028() {
    // IDA 0x3ae028: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ae138 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev")]
pub fn stub_0x3ae138() {
    // IDA 0x3ae138: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ae268 — __ZN5boost9function3IvN3G3D7Vector34AxisEffE13assign_to_ownERKS4_
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")]
#[doc(alias = "__ZN5boost9function3IvN3G3D7Vector34AxisEffE13assign_to_ownERKS4_")]
pub fn stub_0x3ae268() -> ! {
    todo!("0x3ae268 boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")
}

// 0x3aeb04 — __ZN5boost9function1IvN3G3D7Vector34AxisEE5clearEv
#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvN3G3D7Vector34AxisEE5clearEv")]
pub fn stub_0x3aeb04() -> ! {
    todo!("0x3aeb04 boost::function1<void,G3D::Vector3::Axis>::clear(void)")
}

// 0x3af234 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::function<void ()(G3D::Vector3::Axis)>>(boost::function<void ()(G3D::Vector3::Axis)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_0x3af234() -> ! {
    todo!("0x3af234 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::function<void ()(G3D::Vector3::Axis)>>(boost::function<void ()(G3D::Vector3::Axis)> const&)")
}

// 0x3af328 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>*>(boost::function<void ()(G3D::Vector3::Axis)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_0x3af328() -> ! {
    todo!("0x3af328 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>*>(boost::function<void ()(G3D::Vector3::Axis)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis)>*)")
}

// 0x3af424 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_0x3af424() {
    // IDA 0x3af424: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3af534 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
pub fn stub_0x3af534() {
    // IDA 0x3af534: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3af664 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")]
pub fn stub_0x3af664() -> ! {
    todo!("0x3af664 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")
}

// 0x3af66c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")]
pub fn stub_0x3af66c() {
    // IDA 0x3af66c: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call( ` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}
