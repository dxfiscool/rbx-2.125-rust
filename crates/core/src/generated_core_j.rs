//! generated_core_j — 100 core stubs EA-sorted, next after generated_core_i.
//! Source: ida/export.json filtered where demangled/mangled contains "boost" or "rbx::signals", excluding Reflection/Instance/Ogre/RakNet/Network, sorted by EA, next 100 uncovered (lowest EA first).
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.


#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_tostring(rbx::signals::connection const&,lua_State *)")]
// 0x2c5994 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE11on_tostringERKS4_P9lua_State
pub fn stub_2c5994() {
    // IDA 0x2c5994: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")]
// 0x3a82e8 — __ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff
pub fn stub_3a82e8() {
    // IDA 0x3a82e8: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")]
// 0x3a8440 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_
pub fn stub_3a8440() {
    // IDA 0x3a8440: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")]
// 0x3a94e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception
pub fn stub_3a94e0() {
    // IDA 0x3a94e0: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")]
// 0x3a952c — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE22safe_static_init_mutexEv
pub fn stub_3a952c() {
    // IDA 0x3a952c: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_do_get_mutex(void)")]
// 0x3a9530 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE24safe_static_do_get_mutexEv
pub fn stub_3a9530() {
    // IDA 0x3a9530: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::on_error(std::exception &)")]
// 0x3a9788 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE8on_errorERSt9exception
pub fn stub_3a9788() {
    // IDA 0x3a9788: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_init_mutex(void)")]
// 0x3a97d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE22safe_static_init_mutexEv
pub fn stub_3a97d4() {
    // IDA 0x3a97d4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_do_get_mutex(void)")]
// 0x3a97d8 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE24safe_static_do_get_mutexEv
pub fn stub_3a97d8() {
    // IDA 0x3a97d8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::disconnectAll(void)")]
// 0x3a9ffc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13disconnectAllEv
pub fn stub_3a9ffc() {
    // IDA 0x3a9ffc: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::disconnectAll(void)")]
// 0x3aa2d0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13disconnectAllEv
pub fn stub_3aa2d0() {
    // IDA 0x3aa2d0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
// 0x3aa7d8 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6insertEPNS6_4slotE
pub fn stub_3aa7d8() {
    // IDA 0x3aa7d8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::disconnect(void)")]
// 0x3aab08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot10disconnectEv
pub fn stub_3aab08() {
    // IDA 0x3aab08: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::connected(void)const")]
// 0x3aac18 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot9connectedEv
pub fn stub_3aac18() {
    // IDA 0x3aac18: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
// 0x3aacbc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6removeEPNS6_4slotE
pub fn stub_3aacbc() {
    // IDA 0x3aacbc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_init_mutex(void)")]
// 0x3aadac — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot22safe_static_init_mutexEv
pub fn stub_3aadac() {
    // IDA 0x3aadac: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_do_get_mutex(void)")]
// 0x3aadb0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot24safe_static_do_get_mutexEv
pub fn stub_3aadb0() {
    // IDA 0x3aadb0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
// 0x3aaea0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD1Ev
pub fn stub_3aaea0() {
    // IDA 0x3aaea0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
// 0x3aaecc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD0Ev
pub fn stub_3aaecc() {
    // IDA 0x3aaecc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
// 0x3ab3d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6insertEPNS6_4slotE
pub fn stub_3ab3d4() {
    // IDA 0x3ab3d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")]
// 0x3ab704 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot10disconnectEv
pub fn stub_3ab704() {
    // IDA 0x3ab704: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")]
// 0x3ab814 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot9connectedEv
pub fn stub_3ab814() {
    // IDA 0x3ab814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
// 0x3ab860 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6removeEPNS6_4slotE
pub fn stub_3ab860() {
    // IDA 0x3ab860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")]
// 0x3ab950 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot22safe_static_init_mutexEv
pub fn stub_3ab950() {
    // IDA 0x3ab950: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")]
// 0x3ab954 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot24safe_static_do_get_mutexEv
pub fn stub_3ab954() {
    // IDA 0x3ab954: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
// 0x3aba44 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD1Ev
pub fn stub_3aba44() {
    // IDA 0x3aba44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
// 0x3aba70 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD0Ev
pub fn stub_3aba70() {
    // IDA 0x3aba70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::DataModel *)>::operator()(RBX::DataModel *)")]
// 0x4fcf84 — __ZN3rbx7signals16signal_with_argsILi1EFvPN3RBX9DataModelEEEclES4_
pub fn stub_4fcf84() {
    // IDA 0x4fcf84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::on_error(std::exception &)")]
// 0x4fd5ac — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE8on_errorERSt9exception
pub fn stub_4fd5ac() {
    // IDA 0x4fd5ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::disconnectAll(void)")]
// 0x4ff864 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13disconnectAllEv
pub fn stub_4ff864() {
    // IDA 0x4ff864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
// 0x5b09b8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev
pub fn stub_5b09b8() {
    // IDA 0x5b09b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
// 0x5b09e4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev
pub fn stub_5b09e4() {
    // IDA 0x5b09e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(bool)>::operator()(bool)")]
// 0x5e1830 — __ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb
pub fn stub_5e1830() {
    // IDA 0x5e1830: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::IFWHolder>::operator=(rbx_core::SharedPtr<RBX::IFWHolder> const&)")]
// 0x5e6254 — __ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_
// was: boost::shared_ptr<RBX::IFWHolder>::operator=(boost::shared_ptr<RBX::IFWHolder> const&)
pub fn stub_5e6254() {
    // IDA 0x5e6254: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::bad_weak_ptr::~bad_weak_ptr()")]
// 0x5e6290 — __ZN5boost12bad_weak_ptrD1Ev
pub fn stub_5e6290() {
    // IDA 0x5e6290: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::bad_weak_ptr::what(void)const")]
// 0x5e6298 — __ZNK5boost12bad_weak_ptr4whatEv
pub fn stub_5e6298() {
    // IDA 0x5e6298: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x5e62a8 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev
pub fn stub_5e62a8() {
    // IDA 0x5e62a8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x5e6360 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
pub fn stub_5e6360() {
    // IDA 0x5e6360: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")]
// 0x5e6370 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
pub fn stub_5e6370() {
    // IDA 0x5e6370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_weak_ptr> const&)")]
// 0x5e6430 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS4_
pub fn stub_5e6430() {
    // IDA 0x5e6430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot> &)")]
// 0x5e6f20 — __ZN3rbx7signals6signalIFvbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> &)
pub fn stub_5e6f20() {
    // IDA 0x5e6f20: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::on_error(std::exception &)")]
// 0x5e7080 — __ZN3rbx7signals6signalIFvbEE8on_errorERSt9exception
pub fn stub_5e7080() {
    // IDA 0x5e7080: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x5e70a8 — __ZN5boost21intrusive_ptr_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_5e70a8() {
    // IDA 0x5e70a8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::exception &>::operator()(std::exception &)const")]
// 0x5e70e8 — __ZNK5boost9function1IvRSt9exceptionEclES2_
pub fn stub_5e70e8() {
    // IDA 0x5e70e8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::bad_function_call::~bad_function_call()")]
// 0x5e71b0 — __ZN5boost17bad_function_callD0Ev
pub fn stub_5e71b0() {
    // IDA 0x5e71b0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// 0x5e71c8 — __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev
pub fn stub_5e71c8() {
    // IDA 0x5e71c8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// 0x5e7280 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_17bad_function_callEED1Ev
pub fn stub_5e7280() {
    // IDA 0x5e7280: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// 0x5e7288 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
pub fn stub_5e7288() {
    // IDA 0x5e7288: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// 0x5e7290 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
pub fn stub_5e7290() {
    // IDA 0x5e7290: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")]
// 0x5e72a0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
pub fn stub_5e72a0() {
    // IDA 0x5e72a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")]
// 0x5e7360 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
pub fn stub_5e7360() {
    // IDA 0x5e7360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// 0x5e7370 — __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED0Ev
pub fn stub_5e7370() {
    // IDA 0x5e7370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot> const&)")]
// 0x5e7388 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> const&)
pub fn stub_5e7388() {
    // IDA 0x5e7388: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::safe_static_do_get_mutex(void)")]
// 0x5e73b0 — __ZN3rbx7signals6signalIFvbEE24safe_static_do_get_mutexEv
pub fn stub_5e73b0() {
    // IDA 0x5e73b0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unique_lock<boost::mutex>::lock(void)")]
// 0x5e74a8 — __ZN5boost11unique_lockINS_5mutexEE4lockEv
pub fn stub_5e74a8() {
    // IDA 0x5e74a8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::pool<boost::default_user_allocator_malloc_free>::malloc_need_resize(void)")]
// 0x5e8760 — __ZN5boost4poolINS_34default_user_allocator_malloc_freeEE18malloc_need_resizeEv
pub fn stub_5e8760() {
    // IDA 0x5e8760: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
// 0x5e9b98 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_5e9b98() {
    // IDA 0x5e9b98: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::function<void ()(void)>>::~callable_slot()")]
// 0x5e9c90 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_5e9c90() {
    // IDA 0x5e9c90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::function<void ()(void)>>::~callable_slot()")]
// 0x5e9da0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_5e9da0() {
    // IDA 0x5e9da0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::disconnect(void)")]
// 0x5e9ed0 — __ZN3rbx7signals6signalIFvvEE4slot10disconnectEv
pub fn stub_5e9ed0() {
    // IDA 0x5e9ed0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::connected(void)const")]
// 0x5e9fe0 — __ZNK3rbx7signals6signalIFvvEE4slot9connectedEv
pub fn stub_5e9fe0() {
    // IDA 0x5e9fe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_do_get_mutex(void)")]
// 0x5e9ff0 — __ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv
pub fn stub_5e9ff0() {
    // IDA 0x5e9ff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::mutex::~mutex()")]
// 0x5ea0e0 — __ZN5boost5mutexD1Ev
pub fn stub_5ea0e0() {
    // IDA 0x5ea0e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::~slot()")]
// 0x5ea0f8 — __ZN3rbx7signals6signalIFvvEE4slotD0Ev
pub fn stub_5ea0f8() {
    // IDA 0x5ea0f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection::flogPrint(void)")]
// 0x5eaee4 — __ZN3rbx7signals10connection9flogPrintEv
pub fn stub_5eaee4() {
    // IDA 0x5eaee4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot> &)")]
// 0x5f3818 — __ZN3rbx7signals6signalIFvvEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(void)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot> &)
pub fn stub_5f3818() {
    // IDA 0x5f3818: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::on_error(std::exception &)")]
// 0x5f3978 — __ZN3rbx7signals6signalIFvvEE8on_errorERSt9exception
pub fn stub_5f3978() {
    // IDA 0x5f3978: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f39a0 — __ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f39a0() {
    // IDA 0x5f39a0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f39f0 — __ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f39f0() {
    // IDA 0x5f39f0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f3a40 — __ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f3a40() {
    // IDA 0x5f3a40: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f3a90 — __ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f3a90() {
    // IDA 0x5f3a90: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f3ae0 — __ZN5boost14singleton_poolIN3RBX16BallBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f3ae0() {
    // IDA 0x5f3ae0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr()")]
// 0x5f45a0 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED1Ev
pub fn stub_5f45a0() {
    // IDA 0x5f45a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data()")]
// 0x5f45a8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD1Ev
pub fn stub_5f45a8() {
    // IDA 0x5f45a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::operator()(void *)")]
// 0x5f45b0 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv
pub fn stub_5f45b0() {
    // IDA 0x5f45b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::~sp_counted_impl_pd()")]
// 0x5f45c0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev
pub fn stub_5f45c0() {
    // IDA 0x5f45c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")]
// 0x5f46d0 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
pub fn stub_5f46d0() {
    // IDA 0x5f46d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")]
// 0x5f4744 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
pub fn stub_5f4744() {
    // IDA 0x5f4744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::throw_exception<std::length_error>(std::length_error const&)")]
// 0x5f4868 — __ZN5boost15throw_exceptionISt12length_errorEEvRKT_
pub fn stub_5f4868() {
    // IDA 0x5f4868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x5f4948 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev
pub fn stub_5f4948() {
    // IDA 0x5f4948: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x5f4a00 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
pub fn stub_5f4a00() {
    // IDA 0x5f4a00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x5f4a08 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
pub fn stub_5f4a08() {
    // IDA 0x5f4a08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x5f4a10 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
pub fn stub_5f4a10() {
    // IDA 0x5f4a10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// 0x5f4a20 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
pub fn stub_5f4a20() {
    // IDA 0x5f4a20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x5f4ae0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
pub fn stub_5f4ae0() {
    // IDA 0x5f4ae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// 0x5f4af8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
pub fn stub_5f4af8() {
    // IDA 0x5f4af8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x5f4b08 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
pub fn stub_5f4b08() {
    // IDA 0x5f4b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::error_info_injector(std::length_error const&)")]
// 0x5f4b20 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_
pub fn stub_5f4b20() {
    // IDA 0x5f4b20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::error_info_injector<std::length_error> const&)")]
// 0x5f4c08 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_
pub fn stub_5f4c08() {
    // IDA 0x5f4c08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::disconnectAll(void)")]
// 0x5f5198 — __ZN3rbx7signals6signalIFvbEE13disconnectAllEv
pub fn stub_5f5198() {
    // IDA 0x5f5198: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")]
// 0x5f7b48 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_5f7b48() {
    // IDA 0x5f7b48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// 0x5f7e64 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
pub fn stub_5f7e64() {
    // IDA 0x5f7e64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")]
// 0x5f8070 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)
pub fn stub_5f8070() {
    // IDA 0x5f8070: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")]
// 0x5f8094 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)
pub fn stub_5f8094() {
    // IDA 0x5f8094: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_init_mutex(void)")]
// 0x5f80b8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv
pub fn stub_5f80b8() {
    // IDA 0x5f80b8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")]
// 0x5f80bc — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
pub fn stub_5f80bc() {
    // IDA 0x5f80bc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
// 0x5f81b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
pub fn stub_5f81b4() {
    // IDA 0x5f81b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
// 0x5f81e0 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
pub fn stub_5f81e0() {
    // IDA 0x5f81e0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::disconnect(void)")]
// 0x5f82b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv
pub fn stub_5f82b4() {
    // IDA 0x5f82b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::connected(void)const")]
// 0x5f83c4 — __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv
pub fn stub_5f83c4() {
    // IDA 0x5f83c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
