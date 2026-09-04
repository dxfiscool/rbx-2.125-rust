//! audio generated_82 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio exhausted (2541 distinct) — filler workspace EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x88c8f0..0x8910ac EA-sorted asc filler after 0x88c8c0, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x88c8f0 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_88c8f0() {
    // IDA 0x88c8f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88c9bc — __ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_88c9bc() -> ! {
    todo!("0x88c9bc RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x88cafc — __ZN3RBX10Reflection11Call1HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEESsESsS6_E4callEPS2_S8_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Plugin*,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_88cafc() -> ! {
    todo!("0x88cafc RBX::Reflection::Call1Helper<RBX::Plugin,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Plugin*,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x88cc7c — __ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::~EventDesc()")]
pub fn stub_88cc7c() {
    // IDA 0x88cc7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88cd30 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_88cd30() -> ! {
    todo!("0x88cd30 RBX::Reflection::EventDescImpl<0,RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x88cf34 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_88cf34() -> ! {
    todo!("0x88cf34 RBX::Reflection::EventDescImpl<0,RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x88cfa8 — __ZNK3RBX10Reflection13EventDescBaseINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_88cfa8() -> ! {
    todo!("0x88cfa8 RBX::Reflection::EventDescBase<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x88cfbc — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::BoundFuncDesc(void (RBX::Plugin::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_88cfbc() -> ! {
    todo!("0x88cfbc RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::BoundFuncDesc(void (RBX::Plugin::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x88d134 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_88d134() -> ! {
    todo!("0x88d134 RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x88d164 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_88d164() {
    // IDA 0x88d164: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88d238 — __ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_88d238() -> ! {
    todo!("0x88d238 RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x88d26c — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_88d26c() -> ! {
    todo!("0x88d26c RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x88d370 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_88d370() {
    // IDA 0x88d370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88d424 — __ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_88d424() -> ! {
    todo!("0x88d424 RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x88d448 — __ZN3RBX10Reflection11Call0HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Plugin*,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_88d448() -> ! {
    todo!("0x88d448 RBX::Reflection::Call0Helper<RBX::Plugin,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),boost::shared_ptr<RBX::Instance>>::call(RBX::Plugin*,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),RBX::Reflection::Variant &)")
}

// 0x88d530 — __ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_88d530() -> ! {
    todo!("0x88d530 RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x88d634 — __ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_88d634() {
    // IDA 0x88d634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88d6e8 — __ZNK3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_88d6e8() -> ! {
    todo!("0x88d6e8 RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x88d70c — __ZN3RBX10Reflection11Call0HelperINS_13PluginManagerEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),rbx_core::SharedPtr<RBX::Instance>>::call(RBX::PluginManager*,rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_88d70c() -> ! {
    todo!("0x88d70c RBX::Reflection::Call0Helper<RBX::PluginManager,boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),boost::shared_ptr<RBX::Instance>>::call(RBX::PluginManager*,boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),RBX::Reflection::Variant &)")
}

// 0x88d7f4 — __ZN3RBX13PluginManagerD2Ev
#[doc(alias = "RBX::PluginManager::~PluginManager()")]
pub fn stub_88d7f4() {
    // IDA 0x88d7f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88d984 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13PluginManagerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PluginManager> RBX::Creatable<RBX::Instance>::create<RBX::PluginManager>(void)")]
pub fn stub_88d984() -> ! {
    todo!("0x88d984 boost::shared_ptr<RBX::PluginManager> RBX::Creatable<RBX::Instance>::create<RBX::PluginManager>(void)")
}

// 0x88da34 — __ZN5boost10shared_ptrIN3RBX13PluginManagerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PluginManager>::shared_ptr<RBX::PluginManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_88da34() -> ! {
    todo!("0x88da34 boost::shared_ptr<RBX::PluginManager>::shared_ptr<RBX::PluginManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x88dafc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13PluginManagerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PluginManager,RBX::PluginManager>(rbx_core::SharedPtr<RBX::PluginManager> const*,RBX::PluginManager *)const")]
pub fn stub_88dafc() {
    // IDA 0x88dafc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x88dbe4 — __ZN5boost6detail12shared_countC2IPN3RBX13PluginManagerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_88dbe4() {
    // IDA 0x88dbe4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x88dcec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_88dcec() {
    // IDA 0x88dcec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88dcf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_88dcf0() {
    // IDA 0x88dcf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88dcf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_88dcf4() {
    // IDA 0x88dcf4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x88dd14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_88dd14() {
    // IDA 0x88dd14: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x88dd2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_88dd2c() {
    // IDA 0x88dd2c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x88dd30 — __GLOBAL__I_a_440
#[doc(alias = "global constructor keyed to_a_440")]
pub fn stub_88dd30() -> ! {
    todo!("0x88dd30 global constructor keyed to_a_440")
}

// 0x88e2c8 — __ZN3RBX11PluginMouseC1Ev
#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
pub fn stub_88e2c8() -> ! {
    todo!("0x88e2c8 RBX::PluginMouse::PluginMouse(void)")
}

// 0x88e2cc — __ZN3RBX11PluginMouseC2Ev
#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
pub fn stub_88e2cc() -> ! {
    todo!("0x88e2cc RBX::PluginMouse::PluginMouse(void)")
}

// 0x88e468 — __ZN3RBX11PluginMouseD0Ev
#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e468() {
    // IDA 0x88e468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e508 — __ZN3RBX11PluginMouseD1Ev
#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e508() {
    // IDA 0x88e508: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e50c — __ZThn32_N3RBX11PluginMouseD0Ev
#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e50c() {
    // IDA 0x88e50c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e514 — __ZThn36_N3RBX11PluginMouseD0Ev
#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e514() {
    // IDA 0x88e514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e51c — __ZN3RBX11PluginMouseD2Ev
#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e51c() {
    // IDA 0x88e51c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e614 — __ZThn32_N3RBX11PluginMouseD1Ev
#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e614() {
    // IDA 0x88e614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e61c — __ZThn36_N3RBX11PluginMouseD1Ev
#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
pub fn stub_88e61c() {
    // IDA 0x88e61c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88e624 — __ZNK3RBX11PluginMouse6getHitEv
#[doc(alias = "RBX::PluginMouse::getHit(void)const")]
pub fn stub_88e624() -> ! {
    todo!("0x88e624 RBX::PluginMouse::getHit(void)const")
}

// 0x88e78c — __ZNK3RBX11PluginMouse23getPartByLocalCharacterERKNS_7UIEventEPKNS_13HitTestFilterERN3G3D7Vector3E
#[doc(alias = "RBX::PluginMouse::getPartByLocalCharacter(RBX::UIEvent const&,RBX::HitTestFilter const*,G3D::Vector3 &)const")]
pub fn stub_88e78c() -> ! {
    todo!("0x88e78c RBX::PluginMouse::getPartByLocalCharacter(RBX::UIEvent const&,RBX::HitTestFilter const*,G3D::Vector3 &)const")
}

// 0x88e8ec — __ZNK3RBX11PluginMouse9getOriginEv
#[doc(alias = "RBX::PluginMouse::getOrigin(void)const")]
pub fn stub_88e8ec() -> ! {
    todo!("0x88e8ec RBX::PluginMouse::getOrigin(void)const")
}

// 0x88e9f4 — __ZNK3RBX11PluginMouse10getUnitRayEv
#[doc(alias = "RBX::PluginMouse::getUnitRay(void)const")]
pub fn stub_88e9f4() -> ! {
    todo!("0x88e9f4 RBX::PluginMouse::getUnitRay(void)const")
}

// 0x88eae0 — __ZNK3RBX11PluginMouse9getTargetEv
#[doc(alias = "RBX::PluginMouse::getTarget(void)const")]
pub fn stub_88eae0() -> ! {
    todo!("0x88eae0 RBX::PluginMouse::getTarget(void)const")
}

// 0x88ec38 — __ZNK3RBX11PluginMouse7getPartEPKNS_13HitTestFilterERN3G3D7Vector3E
#[doc(alias = "RBX::PluginMouse::getPart(RBX::HitTestFilter const*,G3D::Vector3 &)const")]
pub fn stub_88ec38() -> ! {
    todo!("0x88ec38 RBX::PluginMouse::getPart(RBX::HitTestFilter const*,G3D::Vector3 &)const")
}

// 0x88ee2c — __ZNK3RBX11PluginMouse16getTargetSurfaceEv
#[doc(alias = "RBX::PluginMouse::getTargetSurface(void)const")]
pub fn stub_88ee2c() -> ! {
    todo!("0x88ee2c RBX::PluginMouse::getTargetSurface(void)const")
}

// 0x88ef84 — __ZN3RBX11PluginMouse12setDataModelEPNS_9DataModelE
#[doc(alias = "RBX::PluginMouse::setDataModel(RBX::DataModel *)")]
pub fn stub_88ef84() -> ! {
    todo!("0x88ef84 RBX::PluginMouse::setDataModel(RBX::DataModel *)")
}

// 0x88ef8c — __ZN3RBX11PluginMouse15setTargetFilterEPNS_10PVInstanceE
#[doc(alias = "RBX::PluginMouse::setTargetFilter(RBX::PVInstance *)")]
pub fn stub_88ef8c() -> ! {
    todo!("0x88ef8c RBX::PluginMouse::setTargetFilter(RBX::PVInstance *)")
}

// 0x88ef90 — __ZN3RBX11PluginMouse6updateERKNS_7UIEventE
#[doc(alias = "RBX::PluginMouse::update(RBX::UIEvent const&)")]
pub fn stub_88ef90() -> ! {
    todo!("0x88ef90 RBX::PluginMouse::update(RBX::UIEvent const&)")
}

// 0x88f2a8 — __ZNK3RBX11PluginMouse4getXEv
#[doc(alias = "RBX::PluginMouse::getX(void)const")]
pub fn stub_88f2a8() -> ! {
    todo!("0x88f2a8 RBX::PluginMouse::getX(void)const")
}

// 0x88f2b0 — __ZNK3RBX11PluginMouse4getYEv
#[doc(alias = "RBX::PluginMouse::getY(void)const")]
pub fn stub_88f2b0() -> ! {
    todo!("0x88f2b0 RBX::PluginMouse::getY(void)const")
}

// 0x88f2b8 — __ZNK3RBX11PluginMouse12getViewSizeXEv
#[doc(alias = "RBX::PluginMouse::getViewSizeX(void)const")]
pub fn stub_88f2b8() -> ! {
    todo!("0x88f2b8 RBX::PluginMouse::getViewSizeX(void)const")
}

// 0x88f2c0 — __ZNK3RBX11PluginMouse12getViewSizeYEv
#[doc(alias = "RBX::PluginMouse::getViewSizeY(void)const")]
pub fn stub_88f2c0() -> ! {
    todo!("0x88f2c0 RBX::PluginMouse::getViewSizeY(void)const")
}

// 0x88f2c8 — __ZNK3RBX11PluginMouse12getMousePartERKNS_6RbxRayERKNS_14ContactManagerERKSt6vectorIPKNS_9PrimitiveESaISA_EEPKNS_13HitTestFilterERN3G3D7Vector3Ef
#[doc(alias = "RBX::PluginMouse::getMousePart(RBX::RbxRay const&,RBX::ContactManager const&,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> const&,RBX::HitTestFilter const*,G3D::Vector3 &,float)const")]
pub fn stub_88f2c8() -> ! {
    todo!("0x88f2c8 RBX::PluginMouse::getMousePart(RBX::RbxRay const&,RBX::ContactManager const&,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> const&,RBX::HitTestFilter const*,G3D::Vector3 &,float)const")
}

// 0x88f458 — __ZNK3RBX11PluginMouse10getSurfaceERKNS_7UIEventEPKNS_13HitTestFilterERPNS_12PartInstanceERi
#[doc(alias = "RBX::PluginMouse::getSurface(RBX::UIEvent const&,RBX::HitTestFilter const*,RBX::PartInstance *&,int &)const")]
pub fn stub_88f458() -> ! {
    todo!("0x88f458 RBX::PluginMouse::getSurface(RBX::UIEvent const&,RBX::HitTestFilter const*,RBX::PartInstance *&,int &)const")
}

// 0x88f4b8 — __ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")]
pub fn stub_88f4b8() -> ! {
    todo!("0x88f4b8 __ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")
}

// 0x88f4e0 — __ZNK3RBX11PluginMouse11checkActiveEv
#[doc(alias = "RBX::PluginMouse::checkActive(void)const")]
pub fn stub_88f4e0() -> ! {
    todo!("0x88f4e0 RBX::PluginMouse::checkActive(void)const")
}

// 0x88f4e4 — __ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")]
pub fn stub_88f4e4() {
    // IDA 0x88f4e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f50c — __ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv")]
pub fn stub_88f50c() -> ! {
    todo!("0x88f50c __ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv")
}

// 0x88f510 — __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v")]
pub fn stub_88f510() -> ! {
    todo!("0x88f510 __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v")
}

// 0x88f5f0 — __ZN3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_88f5f0() {
    // IDA 0x88f5f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f5f4 — __ZN3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_88f5f4() {
    // IDA 0x88f5f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f694 — __ZThn32_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_88f694() {
    // IDA 0x88f694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f69c — __ZThn32_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_88f69c() {
    // IDA 0x88f69c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f740 — __ZThn36_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_88f740() {
    // IDA 0x88f740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f748 — __ZThn36_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11PluginMouseELZNS_12sPluginMouseEENS_17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_88f748() {
    // IDA 0x88f748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88f7ec — __GLOBAL__I_a_441
#[doc(alias = "global constructor keyed to_a_441")]
pub fn stub_88f7ec() -> ! {
    todo!("0x88f7ec global constructor keyed to_a_441")
}

// 0x88fa74 — __ZN3RBX15BallCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
#[doc(alias = "RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
pub fn stub_88fa74() -> ! {
    todo!("0x88fa74 RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")
}

// 0x88fa78 — __ZN3RBX15BallCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
#[doc(alias = "RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
pub fn stub_88fa78() -> ! {
    todo!("0x88fa78 RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")
}

// 0x88fdc0 — __ZN3RBX15BallCellContactD0Ev
#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
pub fn stub_88fdc0() {
    // IDA 0x88fdc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88fe74 — __ZN3RBX15BallCellContactD1Ev
#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
pub fn stub_88fe74() {
    // IDA 0x88fe74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88fe78 — __ZN3RBX15BallCellContactD2Ev
#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
pub fn stub_88fe78() {
    // IDA 0x88fe78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x88ff94 — __ZN3RBX15BallCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::BallCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
pub fn stub_88ff94() -> ! {
    todo!("0x88ff94 RBX::BallCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")
}

// 0x89016c — __ZN3RBX15BallCellContact16getFarthestPlaneERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getFarthestPlane(float &,G3D::Vector3 const&)")]
pub fn stub_89016c() -> ! {
    todo!("0x89016c RBX::BallCellContact::getFarthestPlane(float &,G3D::Vector3 const&)")
}

// 0x890268 — __ZN3RBX15BallCellContact21newBallPlaneConnectorEPKNS_4POLY4FaceE
#[doc(alias = "RBX::BallCellContact::newBallPlaneConnector(RBX::POLY::Face const*)")]
pub fn stub_890268() -> ! {
    todo!("0x890268 RBX::BallCellContact::newBallPlaneConnector(RBX::POLY::Face const*)")
}

// 0x89047c — __ZN3RBX15BallCellContact23getClosestInVoronoiEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestInVoronoiEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
pub fn stub_89047c() -> ! {
    todo!("0x89047c RBX::BallCellContact::getClosestInVoronoiEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")
}

// 0x890514 — __ZN3RBX15BallCellContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE
#[doc(alias = "RBX::BallCellContact::newBallEdgeConnector(RBX::POLY::Edge const*)")]
pub fn stub_890514() -> ! {
    todo!("0x890514 RBX::BallCellContact::newBallEdgeConnector(RBX::POLY::Edge const*)")
}

// 0x890734 — __ZN3RBX15BallCellContact14getClosestEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
pub fn stub_890734() -> ! {
    todo!("0x890734 RBX::BallCellContact::getClosestEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")
}

// 0x890810 — __ZN3RBX15BallCellContact16getClosestVertexEPKNS_4POLY4EdgeERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestVertex(RBX::POLY::Edge const*,float &,G3D::Vector3 const&)")]
pub fn stub_890810() -> ! {
    todo!("0x890810 RBX::BallCellContact::getClosestVertex(RBX::POLY::Edge const*,float &,G3D::Vector3 const&)")
}

// 0x8908e8 — __ZN3RBX15BallCellContact22newBallVertexConnectorEPKNS_4POLY6VertexE
#[doc(alias = "RBX::BallCellContact::newBallVertexConnector(RBX::POLY::Vertex const*)")]
pub fn stub_8908e8() -> ! {
    todo!("0x8908e8 RBX::BallCellContact::newBallVertexConnector(RBX::POLY::Vertex const*)")
}

// 0x890ad4 — __ZN3RBX15BallCellContact34generateDataForMovingAssemblyStageEv
#[doc(alias = "RBX::BallCellContact::generateDataForMovingAssemblyStage(void)")]
pub fn stub_890ad4() -> ! {
    todo!("0x890ad4 RBX::BallCellContact::generateDataForMovingAssemblyStage(void)")
}

// 0x890ad8 — __ZN3RBX9AllocatorINS_15BallCellContactEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::Allocator(void)")]
pub fn stub_890ad8() -> ! {
    todo!("0x890ad8 RBX::Allocator<RBX::BallCellContact>::Allocator(void)")
}

// 0x890b3c — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE9push_backERKS2_
#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)")]
pub fn stub_890b3c() -> ! {
    todo!("0x890b3c RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)")
}

// 0x890bac — __ZNK3RBX4POLY4Face16pointInExtrusionERKN3G3D7Vector3E
#[doc(alias = "RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const")]
pub fn stub_890bac() -> ! {
    todo!("0x890bac RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const")
}

// 0x890c24 — __ZNK3RBX4POLY4Face5planeEv
#[doc(alias = "RBX::POLY::Face::plane(void)const")]
pub fn stub_890c24() -> ! {
    todo!("0x890c24 RBX::POLY::Face::plane(void)const")
}

// 0x890c88 — __ZNK3G3D4Line8distanceERKNS_7Vector3E
#[doc(alias = "G3D::Line::distance(G3D::Vector3 const&)const")]
pub fn stub_890c88() -> ! {
    todo!("0x890c88 G3D::Line::distance(G3D::Vector3 const&)const")
}

// 0x890ce4 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)")]
pub fn stub_890ce4() -> ! {
    todo!("0x890ce4 RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)")
}

// 0x890d54 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)")]
pub fn stub_890d54() -> ! {
    todo!("0x890d54 RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)")
}

// 0x890dc4 — __ZNK3RBX4POLY4Edge13computeNormalEPKNS0_4FaceE
#[doc(alias = "RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const")]
pub fn stub_890dc4() -> ! {
    todo!("0x890dc4 RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const")
}

// 0x890e50 — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)")]
pub fn stub_890e50() -> ! {
    todo!("0x890e50 RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)")
}

// 0x890ec0 — __ZNK3RBX11CellContact13numConnectorsEv
#[doc(alias = "RBX::CellContact::numConnectors(void)const")]
pub fn stub_890ec0() -> ! {
    todo!("0x890ec0 RBX::CellContact::numConnectors(void)const")
}

// 0x890ec8 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_890ec8() -> ! {
    todo!("0x890ec8 boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x890f18 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_890f18() -> ! {
    todo!("0x890f18 boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x890f50 — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_890f50() -> ! {
    todo!("0x890f50 boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x890fa0 — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_890fa0() -> ! {
    todo!("0x890fa0 boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x890fd8 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_890fd8() -> ! {
    todo!("0x890fd8 boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x891028 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_891028() -> ! {
    todo!("0x891028 boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x891060 — __ZN3RBX9AllocatorINS_15BallCellContactEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::releaseMemory(void)")]
pub fn stub_891060() -> ! {
    todo!("0x891060 RBX::Allocator<RBX::BallCellContact>::releaseMemory(void)")
}

// 0x89107c — __ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_89107c() -> ! {
    todo!("0x89107c boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x8910ac — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_8910ac() -> ! {
    todo!("0x8910ac boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}