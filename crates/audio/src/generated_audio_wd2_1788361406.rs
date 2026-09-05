//! audio generated_audio_wd2_1788361406 — 100 stubs EA-sorted asc gap filler (FMOD exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x66b477 | rbx_core::SharedPtr not boost
//! Range 0x66b478..0x66e340 | existing 37023 -> 37123 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
use crate::generated_134::{XmlIntSlot, XmlReadValue};
use crate::generated_audio_wd_watchdog18::{
    TextBoxBoolProp, TextBoxBoolSlot, TextBoxState, TextBoxXAlignProp, TextBoxYAlignProp,
    XAlignmentVariant, YAlignmentVariant, XALIGNMENT_ITEMS, YALIGNMENT_ITEMS, xalignment_index,
    xalignment_name, yalignment_index, yalignment_name,
};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x66b478 — __ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev
// demangled: rbx::remote_signal<void ()(RBX::UDim2)>::~remote_signal()
#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev")]
pub fn stub_66b478() {
    // IDA 0x66b478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66b5c4 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv")]
pub fn stub_66b5c4() {
    // IDA 0x66b5c4 (`rbx::signals::signal<void (RBX::UDim2)>::
    // disconnectAll`): locked slot-list teardown (0x66b5c4+).
    // Connections fold into the host fire-closure seams. Carrier
    // no-op.
}

// 0x66b73c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_")]
pub fn stub_66b73c() {
    // IDA 0x66b73c (`boost::intrusive_ptr<signal<void
    // (RBX::UDim2)>::slot>::operator=`): addref-new/release-old
    // (0x66b73e-0x66b756). `Arc` move — carrier no-op.
}

// 0x66b760 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE22safe_static_init_mutexEv")]
pub fn stub_66b760() {
    // IDA 0x66b760 (`rbx::signals::signal<void (RBX::UDim2)>::
    // safe_static_init_mutex`): forwards to the once-mutex getter
    // (host: the 0x66b764 twin folds). Carrier no-op.
}

// 0x66b764 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv")]
pub fn stub_66b764() {
    // IDA 0x66b764 (`rbx::signals::signal<void (RBX::UDim2)>::
    // safe_static_do_get_mutex`): once-guarded static mutex init
    // (0x66b7c0-0x66b7f0). Host mutexes fold. Carrier no-op.
}

// 0x66b85c — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEED2Ev
// demangled: RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::~EventReplicatorBase()
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEED2Ev")]
pub fn stub_66b85c() {
    // IDA 0x66b85c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66b98c — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEED2Ev
// demangled: RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::~EventReplicatorBase()
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEED2Ev")]
pub fn stub_66b98c() {
    // IDA 0x66b98c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66babc — __ZNK3RBX15ServiceProvider4findINS_11TextServiceEEEPT_v
// demangled: RBX::TextService * RBX::ServiceProvider::find<RBX::TextService>(void)const
#[doc(alias = "RBX::TextService * RBX::ServiceProvider::find<RBX::TextService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_11TextServiceEEEPT_v")]
pub fn stub_66babc() -> bool {
    // IDA 0x66babc (`RBX::ServiceProvider::find<TextService>`):
    // provider lookup returning the service or null (0x66babc+).
    // No provider exists in the host: the exact miss floor.
    false
}

// 0x66bc30 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::TextService> RBX::Creatable<RBX::Instance>::create<RBX::TextService>(void)
#[doc(alias = "boost::shared_ptr<RBX::TextService> RBX::Creatable<RBX::Instance>::create<RBX::TextService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_66bc30() {
    // IDA 0x66bc30 (`RBX::Creatable<Instance>::create<TextService>`):
    // heap-allocates the 0x64-byte service (0x66bc64), runs its C2
    // (0x66bc88) and wraps it in the `shared_ptr`+`Deleter`
    // (0x66bc96). No `TextService` state is modeled in the host.
    // Carrier no-op.
}

// 0x66be0c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11TextServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TextService>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TextService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_11TextServiceEEEmv")]
pub fn stub_66be0c() {
    // IDA 0x66be0c (`RBX::ServiceProvider::doGetClassIndex<TextService>`):
    // once-guarded provider-slot allocation (0x66be68-0x66beb6).
    // The provider registry folds away. Carrier no-op.
}

// 0x66bee4 — __ZN5boost10shared_ptrIN3RBX11TextServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::TextService>::shared_ptr<RBX::TextService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::shared_ptr<RBX::TextService>::shared_ptr<RBX::TextService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11TextServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_66bee4() {
    // IDA 0x66bee4 (`boost::shared_ptr<TextService>::shared_ptr`
    // with the `Creatable::Deleter`): installs the count and
    // accepts the `enable_shared_from_this` owner
    // (0x66bf04-0x66bf6c). `SharedPtr` is `Arc` in the host.
    // Carrier no-op.
}

// 0x66bfac — __ZN5boost6detail12shared_countC2IPN3RBX11TextServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11TextServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_66bfac() {
    // IDA 0x66bfac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x66c0b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_66c0b4() {
    // IDA 0x66c0b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66c0b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_66c0b8() {
    // IDA 0x66c0b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x66c0d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_66c0d8() {
    // IDA 0x66c0d8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x66c0f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_66c0f0() {
    // IDA 0x66c0f0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x66c194 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,int>(char const*,char const*,bool (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,int>(char const*,char const*,bool (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_66c194(name: &str, category: &str, attributes: u32, permissions: u32) -> TextBoxBoolProp {
    // IDA 0x66c194 (`RBX::Reflection::PropDescriptor<TextBox,
    // bool>::PropDescriptor`): builds the `GetImpl` member-pair
    // cell (getter/setter at 0x66c1e4-0x66c1e8, host: folds into
    // the caller's `TextBoxBoolSlot`) plus the typed descriptor
    // identity (0x66c232). Host: the identity half.
    TextBoxBoolProp::new(name, category, attributes, permissions)
}

// 0x66c2a0 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbED0Ev")]
pub fn stub_66c2a0() {
    // IDA 0x66c2a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66c2cc — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")]
pub fn stub_66c2cc() -> bool {
    // IDA 0x66c2cc (`RBX::Reflection::PropDescriptor<TextBox,
    // bool>::GetImpl::isReadOnly`): returns constant 1
    // (0x66c2ce — the getter-only impl throws in `setValue`).
    true
}

// 0x66c2d0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")]
pub fn stub_66c2d0() -> bool {
    // IDA 0x66c2d0 (`RBX::Reflection::PropDescriptor<TextBox,
    // bool>::GetImpl::isWriteOnly`): returns constant 0
    // (0x66c2d2).
    false
}

// 0x66c2d4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_66c2d4(state: &TextBoxState, slot: TextBoxBoolSlot) -> bool {
    // IDA 0x66c2d4 (`RBX::Reflection::PropDescriptor<TextBox,
    // bool>::GetImpl::getValue`): dispatches the stored getter
    // member-pointer over the object (0x66c2d6-0x66c2f6, host:
    // the `slot` selects the `TextBoxState` bool).
    state.bool_slot(slot)
}

// 0x66c2f8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_66c2f8() -> ! {
    // IDA 0x66c2f8 (`RBX::Reflection::PropDescriptor<TextBox,
    // bool>::GetImpl::setValue`): unconditionally throws
    // `std::runtime_error("can't set value")` (0x66c324-0x66c408)
    // — the impl is getter-only. Host: panic.
    panic!("can't set value")
}

// 0x66c418 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextBox::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextBox::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_66c418() {
    // IDA 0x66c418 (`RBX::Reflection::PropDescriptor<TextBox,
    // Vector2>::PropDescriptor`): same generic shape as the bool
    // C2 at 0x66c194 (member pair + typed identity,
    // 0x66c43e-0x66c4d4). No `Vector2` member is identified in
    // this range, so only the registry half exists: carrier
    // no-op.
}

// 0x66c524 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EED0Ev")]
pub fn stub_66c524() {
    // IDA 0x66c524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66c550 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10isReadOnlyEv
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10isReadOnlyEv")]
pub fn stub_66c550() -> bool {
    // IDA 0x66c550 (`TypedPropertyDescriptor<Vector2>::isReadOnly`):
    // forwards to the +40 impl slot 0 (0x66c55c) — the `TextBox`
    // `GetImpl` at 0x66c638 answers 1. Host: the forwarded value.
    true
}

// 0x66c560 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE11isWriteOnlyEv
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE11isWriteOnlyEv")]
pub fn stub_66c560() -> bool {
    // IDA 0x66c560 (`TypedPropertyDescriptor<Vector2>::isWriteOnly`):
    // forwards to the +40 impl slot 1 (0x66c56c) — the `TextBox`
    // `GetImpl` at 0x66c63c answers 0. Host: the forwarded value.
    false
}

// 0x66c570 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_66c570(first: [f32; 2], second: [f32; 2]) -> bool {
    // IDA 0x66c570 (`TypedPropertyDescriptor<Vector2>::equalValues`):
    // reads both values through the +40 impl slot 2
    // (0x66c584-0x66c590) and compares lane-wise (0x66c5a4-0x66c5bc).
    // The reads fold into the params.
    first == second
}

// 0x66c5c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_66c5c0(value: [f32; 2]) -> [f32; 2] {
    // IDA 0x66c5c0 (`TypedPropertyDescriptor<Vector2>::getVariant`):
    // reads the value through the +40 impl slot 2, tags it with
    // the `Vector2` singleton and placement-moves it into the
    // `Variant` (0x66c5d4-0x66c5e8). The `Variant` machinery folds;
    // host returns the read edge.
    value
}

// 0x66c5ec — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_66c5ec() -> ! {
    // IDA 0x66c5ec (`TypedPropertyDescriptor<Vector2>::copyValue`):
    // reads the source through the +40 impl slot 2 then writes
    // through slot 3 (0x66c602-0x66c612) — the `TextBox` `GetImpl`
    // `setValue` at 0x66c668 unconditionally throws. Host: panic.
    panic!("can't set value")
}

// 0x66c614 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EED1Ev
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EED1Ev")]
pub fn stub_66c614() {
    // IDA 0x66c614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66c638 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE10isReadOnlyEv")]
pub fn stub_66c638() -> bool {
    // IDA 0x66c638 (`PropDescriptor<TextBox, Vector2>::GetImpl::
    // isReadOnly`): returns constant 1 (0x66c63a — the impl throws
    // in `setValue`).
    true
}

// 0x66c63c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv")]
pub fn stub_66c63c() -> bool {
    // IDA 0x66c63c (`PropDescriptor<TextBox, Vector2>::GetImpl::
    // isWriteOnly`): returns constant 0 (0x66c63e).
    false
}

// 0x66c640 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_66c640(value: [f32; 2]) -> [f32; 2] {
    // IDA 0x66c640 (`PropDescriptor<TextBox, Vector2>::GetImpl::
    // getValue`): dispatches the stored getter member-pointer over
    // the object (0x66c642-0x66c664). No `Vector2`-returning
    // `TextBox` member is identified in this range (same gap as
    // the 0x66c418 C2): host passes the read edge through.
    value
}

// 0x66c668 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_66c668() -> ! {
    // IDA 0x66c668 (`PropDescriptor<TextBox, Vector2>::GetImpl::
    // setValue`): `__noreturn`, unconditionally throws
    // `std::runtime_error("can't set value")` — the impl is
    // getter-only. Host: panic.
    panic!("can't set value")
}

// 0x66c788 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_66c788(name: &str, category: &str, attributes: u32, permissions: u32) -> TextBoxYAlignProp {
    // IDA 0x66c788 (`EnumPropDescriptor<TextBox, YAlignment>` ctor):
    // the `TextBox` `classDescriptor` call, the
    // `EnumDesc<YAlignment>` singleton once-init and the
    // `PropertyDescriptor` base init with name/category/
    // attributes/permissions plus the impl holding the
    // getter/setter member-pointer pair. The pair folds into the
    // `y_alignment` field (see below).
    TextBoxYAlignProp::new(name, category, attributes, permissions)
}

// 0x66c93c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED0Ev")]
pub fn stub_66c93c() {
    // IDA 0x66c93c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66c968 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10isReadOnlyEv")]
pub fn stub_66c968() -> bool {
    // IDA 0x66c968 (`EnumPropDescriptor<YAlignment>::isReadOnly`):
    // delegates to the inner `GetSet` at +44 — always readable.
    false
}

// 0x66c978 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11isWriteOnlyEv")]
pub fn stub_66c978() -> bool {
    // IDA 0x66c978 (`EnumPropDescriptor<YAlignment>::isWriteOnly`):
    // delegates to the inner `GetSet` at +44 — always writable.
    false
}

// 0x66c988 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_66c988(first: &TextBoxState, second: &TextBoxState) -> bool {
    // IDA 0x66c988 (`EnumPropDescriptor<YAlignment>::equalValues`):
    // reads the inner value for both instances via the +44
    // `GetSet` and compares (0x66c998-0x66c9ae). Host: compare
    // the alignments.
    first.y_alignment == second.y_alignment
}

// 0x66c9b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_66c9b0(state: &TextBoxState) -> YAlignmentVariant {
    // IDA 0x66c9b0 (`EnumPropDescriptor<YAlignment>::getVariant`):
    // reads the inner index, tags it with the plain-`int`
    // singleton and placement-moves it in (0x66c9be-0x66c9d2, same
    // int-tagged shape as the `FontSize` twin at 0x6707a8). Host:
    // the `YAlignment` tag.
    YAlignmentVariant::YAlignment(state.y_alignment)
}

// 0x66c9d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_66c9d4(state: &mut TextBoxState, variant: &YAlignmentVariant) {
    // IDA 0x66c9d4 (`EnumPropDescriptor<YAlignment>::setVariant`):
    // an int-typed variant runs `any_cast<int>` (0x66ca52-0x66caa0);
    // anything else runs `Variant::convert<int>` (throws on
    // failure, 0x66ca54-0x66ca90); then the +72 setter (0x66cab0).
    // Host: convert-or-throw, then store.
    let value = match *variant {
        YAlignmentVariant::YAlignment(value) => value,
        _ => panic!("Unable to convert variant to int (IDA 0x66c9d4)"),
    };
    state.y_alignment = value;
}

// 0x66cb20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_66cb20(first: &TextBoxState, second: &mut TextBoxState) {
    // IDA 0x66cb20 (`EnumPropDescriptor<YAlignment>::copyValue`):
    // inner `getValue` on the source then inner `setValue` on the
    // target (0x66cb32-0x66cb42). Host: copy the alignment.
    second.y_alignment = first.y_alignment;
}

// 0x66cb44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::hasStringValue(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14hasStringValueEv")]
pub fn stub_66cb44() -> bool {
    // IDA 0x66cb44 (`EnumPropDescriptor<YAlignment>::hasStringValue`):
    // returns 1 — always stringable.
    true
}

// 0x66cb48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_66cb48(state: &TextBoxState) -> String {
    // IDA 0x66cb48 (`EnumPropDescriptor<YAlignment>::getStringValue`):
    // reads the enum-desc singleton slot, the inner value via the
    // +44 `GetSet` and `EnumDesc::convertToString`
    // (0x66cb52-0x66cb6a). Host: the grounded item name (`Top` 0,
    // `Center` 1, `Bottom` 2 — IDA 0x7d8808-0x7d8834).
    yalignment_name(state.y_alignment).to_owned()
}

// 0x66cb6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_66cb6c(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x66cb6c (`EnumPropDescriptor<YAlignment>::setStringValue`):
    // `Name::lookup` + `EnumDesc::convertToValue`; on a hit the
    // inner `setValue` runs and 1 returns, else 0
    // (0x66cb76-0x66cba8). Host: table position decides.
    match YALIGNMENT_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.y_alignment = YALIGNMENT_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x66cbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_66cbac(state: &TextBoxState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x66cbac (`EnumPropDescriptor<YAlignment>::writeValue`):
    // inner `getValue`, `clearValue`, int tag `5` at +16, value at
    // +20, returns 5 (0x66cbba-0x66cbca). Same shape as the
    // `FontSize` twin at 0x6709a4.
    out.value_type = 5;
    out.int_value = state.y_alignment as i32;
    5
}

// 0x66cbcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_66cbcc(state: &mut TextBoxState, xml: &XmlReadValue) {
    // IDA 0x66cbcc (`EnumPropDescriptor<YAlignment>::readValue`):
    // xsi:nil early-out (0x66cbf0); an int pair runs `setIntValue`
    // (index→value with -1 rejection) and returns on success; a
    // string pair runs lookup + convert + inner set, a miss running
    // the +64 reset hook before asserting (folds away); anything
    // else hits `ReleaseAssert(false)` (Reflection.h:359, host
    // seam). The `enumToItem` map is dense identity, so the int
    // path reads the table. Same shape as the `FontSize` twin at
    // 0x6709c4.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if *value >= 0 {
                if let Some((_, align)) = YALIGNMENT_ITEMS.get(*value as usize) {
                    state.y_alignment = *align;
                    return;
                }
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x66cbcc)");
            }
        }
        XmlReadValue::Text(text) => {
            if stub_66cb6c(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x66cbcc)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x66cbcc)");
            }
        }
    }
}

// 0x66ce0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_66ce0c(state: &TextBoxState) -> i32 {
    // IDA 0x66ce0c (`EnumPropDescriptor<YAlignment>::getIndexValue`):
    // inner `getValue` + `EnumDesc::convertToIndex` (0x66ce1a).
    // Host: the item index of the live value.
    yalignment_index(state.y_alignment)
}

// 0x66ce28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_66ce28(state: &mut TextBoxState, index: u32) -> bool {
    // IDA 0x66ce28 (`EnumPropDescriptor<YAlignment>::setIndexValue`):
    // bounds-checks the index against the item count, stores
    // `items[index]` through the inner `setValue` and returns 1,
    // else 0 (0x66ce2e-0x66ce58).
    match YALIGNMENT_ITEMS.get(index as usize) {
        Some((_, align)) => {
            state.y_alignment = *align;
            true
        }
        None => false,
    }
}

// 0x66ce5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_66ce5c(state: &TextBoxState) -> u32 {
    // IDA 0x66ce5c (`EnumPropDescriptor<YAlignment>::getEnumValue`):
    // the inner `getValue` straight through (0x66ce5c+).
    state.y_alignment
}

// 0x66ce64 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_66ce64(state: &mut TextBoxState, value: u32) -> bool {
    // IDA 0x66ce64 (`EnumPropDescriptor<YAlignment>::setEnumValue`):
    // `find_if` with `equalValue` over the items; on a hit the
    // inner `setValue` runs and 1 returns, else 0
    // (0x66ce8e-0x66ceac). Host: membership decides.
    if YALIGNMENT_ITEMS.iter().any(|(_, v)| *v == value) {
        state.y_alignment = value;
        true
    } else {
        false
    }
}

// 0x66ceb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_66ceb0(state: &TextBoxState) -> i32 {
    // IDA 0x66ceb0 (`EnumPropDescriptor<YAlignment>::getEnumItem`):
    // inner `getValue` + `EnumDesc::convertToItem` (0x66ceb6-0x66cece).
    // Host: the item position of the live value (-1 when missing).
    yalignment_index(state.y_alignment)
}

// 0x66ced0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_66ced0(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x66ced0 (`EnumPropDescriptor<YAlignment>::setStringValue`
    // over `Name`): `convertToValue` on the name (0x66cee6); on a
    // hit the inner `setValue` runs and 1 returns, else 0
    // (0x66ceec-0x66cf02). Same string edge as 0x66cb6c — host
    // forwards into that twin (`Name` folds into `&str`).
    stub_66cb6c(state, name)
}

// 0x66cf04 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(RBX::Name const&,RBX::TextService::YAlignment&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(RBX::Name const&,RBX::TextService::YAlignment&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_66cf04(name: &str) -> Option<u32> {
    // IDA 0x66cf04 (`EnumDesc<YAlignment>::convertToValue(Name)`):
    // red-black-tree search for the name id (0x66cf06-0x66cf38),
    // returning 1 with the value on a hit (else 0). Host: the
    // table value (`None` on a miss).
    YALIGNMENT_ITEMS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, v)| *v)
}

// 0x66cf80 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToItem(RBX::TextService::YAlignment const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToItem(RBX::TextService::YAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_")]
pub fn stub_66cf80(value: u32) -> i32 {
    // IDA 0x66cf80 (`EnumDesc<YAlignment>::convertToItem`):
    // FLog::Asserts-gated `value>=0` (enumconverter.h:273,
    // 0x66cf94-0x66cfda) and `value<enumToItem.size` (line 274,
    // 0x66cfde-0x66d01c) ReleaseAsserts — host seams; then the
    // value-indexed item (LABEL_13). Host: asserts + table index.
    if flog_asserts() {
        assert!(
            value < YALIGNMENT_ITEMS.len() as u32,
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274 (IDA 0x66cf80)"
        );
    }
    yalignment_index(value)
}

// 0x66d04c — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToIndex(RBX::TextService::YAlignment)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToIndex(RBX::TextService::YAlignment)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_")]
pub fn stub_66d04c(value: u32) -> i32 {
    // IDA 0x66d04c (`EnumDesc<YAlignment>::convertToIndex`):
    // FLog::Asserts-gated `value>=0` (enumconverter.h:350,
    // 0x66d060-0x66d096) — host seam; then the value-indexed
    // `enumToItem` entry or -1 past the end (0x66d0a6-0x66d0ba).
    // Host: the table index (-1 on a miss).
    yalignment_index(value)
}

// 0x66d0bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_66d0bc(state: &mut TextBoxState, index: i32) -> bool {
    // IDA 0x66d0bc (`EnumPropDescriptor<YAlignment>::setIntValue`):
    // rejects negative indices (0x66d0c6), bounds-checks against
    // the item count and rejects `-1`-valued items (0x66d0d8-0x66d0e4),
    // then stores through the inner `setValue` and returns 1, else
    // 0 (0x66d0e6-0x66d0f2). Table values are non-negative by
    // type, so the `-1` check folds away.
    if index >= 0 {
        if let Some((_, align)) = YALIGNMENT_ITEMS.get(index as usize) {
            state.y_alignment = *align;
            return true;
        }
    }
    false
}

// 0x66d0fc — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(RBX::TextService::YAlignment const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(RBX::TextService::YAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_")]
pub fn stub_66d0fc(value: i32) -> String {
    // IDA 0x66d0fc (`EnumDesc<YAlignment>::convertToString`):
    // FLog::Asserts-gated `value >= 0` (enumconverter.h:262) and
    // `value < enumToItem.size` (line 263) ReleaseAsserts — host
    // seams; then a negative value yields "", else the
    // value-indexed item name. Same shape as the `XAlignment` twin
    // at 0x66e380. Host: asserts + table name with "" fallback.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262 (IDA 0x66d0fc)"
        );
        assert!(
            (value as usize) < YALIGNMENT_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263 (IDA 0x66d0fc)"
        );
    }
    if value >= 0 {
        yalignment_name(value as u32).to_owned()
    } else {
        String::new()
    }
}

// 0x66d29c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_66d29c() -> bool {
    // IDA 0x66d29c (`GetSetImpl<YAlignment>::isReadOnly`): returns
    // constant 0 (0x66d29e).
    false
}

// 0x66d2a0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_66d2a0() -> bool {
    // IDA 0x66d2a0 (`GetSetImpl<YAlignment>::isWriteOnly`): returns
    // constant 0 (0x66d2a2).
    false
}

// 0x66d2a4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_66d2a4(state: &TextBoxState) -> u32 {
    // IDA 0x66d2a4 (`GetSetImpl<YAlignment>::getValue`): the
    // member-pointer resolve (null described reads at offset 0
    // with the +536 `Instance`-to-mixin adjust, 0x66d2a6-0x66d2b4;
    // virtual when the low bit is set) tail-calling the getter.
    // The member is `getYAlignment` (IDA 0x668744, the only
    // `YAlignment` getter); the pointer folds into the field.
    state.y_alignment
}

// 0x66d2d0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_66d2d0(state: &mut TextBoxState, value: u32) {
    // IDA 0x66d2d0 (`GetSetImpl<YAlignment>::setValue`): the
    // member-pointer resolve over +12/+16 tail-calling the setter
    // with the input word. The member is `setYAlignment` (the only
    // `YAlignment` setter on `TextBox`); the pointer folds into
    // the field (its raises fold into the store).
    state.y_alignment = value;
}

// 0x66d2f4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE13initSingletonEv")]
pub fn stub_66d2f4() {
    // IDA 0x66d2f4 (`Singleton<EnumDesc<YAlignment>>::initSingleton`):
    // thunk tail-calling `doGetSingleton` (host: stub_066d2f8).
    // The singleton folds into the host table — carrier no-op.
}

// 0x66d2f8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv")]
pub fn stub_66d2f8() {
    // IDA 0x66d2f8 (`Singleton<EnumDesc<YAlignment>>::doGetSingleton`):
    // `__cxa_guard` once-init constructing the `EnumDesc` and
    // registering `__cxa_atexit` teardown (0x66d354+). Host
    // statics initialize on use — carrier no-op.
}

// 0x66d3e8 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED1Ev")]
pub fn stub_66d3e8() {
    // IDA 0x66d3e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66d3ec — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev")]
pub fn stub_66d3ec() {
    // IDA 0x66d3ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66d5c0 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED0Ev")]
pub fn stub_66d5c0() {
    // IDA 0x66d5c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66d660 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupEPKc")]
pub fn stub_66d660(name: &str) -> Option<u32> {
    // IDA 0x66d660 (`EnumDesc<YAlignment>::lookup(name)`):
    // `Name::lookup` (0x66d66c) + `convertToValue` (0x66d67a); on a
    // hit `convertToItem` (0x66d686), else 0. Host: the item index
    // (`None` on a miss); the lookup folds into the compare.
    YALIGNMENT_ITEMS
        .iter()
        .position(|(n, _)| *n == name)
        .map(|i| i as u32)
}

// 0x66d690 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupERKNS0_7VariantE")]
pub fn stub_66d690(variant: &YAlignmentVariant) -> Option<u32> {
    // IDA 0x66d690 (`EnumDesc<YAlignment>::lookup(variant)`):
    // `any_cast<YAlignment>` (throws on a miss, host: stub_066d8f4)
    // then `convertToItem` (0x66d6ac). Host: the item index of the
    // cast value.
    YALIGNMENT_ITEMS
        .iter()
        .position(|(_, v)| *v == stub_66d8f4(variant))
        .map(|i| i as u32)
}

// 0x66d6b0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_66d6b0(index: u32) -> Option<u32> {
    // IDA 0x66d6b0 (`EnumDesc<YAlignment>::convertToValue(index)`):
    // `count > index` gates reading the indexed item's value plus
    // the `Type` tag and placement (0x66d6be-0x66d6d8), returning 1
    // (else 0). Host: the value (`None` past the end); the
    // tag/placement fold away.
    YALIGNMENT_ITEMS.get(index as usize).map(|(_, v)| *v)
}

// 0x66d6e4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringEmRSs")]
pub fn stub_66d6e4(index: u32, out: &mut String) -> bool {
    // IDA 0x66d6e4 (`EnumDesc<YAlignment>::convertToString(index)`):
    // `count > index` gates reading the value and converting it to
    // a name assigned into the out string, returning 1 (else 0 with
    // `out` untouched). Host: assign on hit, report.
    match YALIGNMENT_ITEMS.get(index as usize) {
        Some((_, value)) => {
            *out = yalignment_name(*value).to_owned();
            true
        }
        None => false,
    }
}

// 0x66d828 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::YAlignment>(RBX::TextService::YAlignment const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::YAlignment>(RBX::TextService::YAlignment const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_")]
pub fn stub_66d828(value: u32) -> u32 {
    // IDA 0x66d828 (`placement_any::operator=<YAlignment>`):
    // ensures the holder singleton, then stores the value and
    // (re)tags the holder (destroying the old payload first). Host
    // values are `Copy` with the tag in the type — the move is
    // identity.
    value
}

// 0x66d878 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv
// demangled: rbx::implementation::typed_holder<RBX::TextService::YAlignment>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv")]
pub fn stub_66d878() {
    // IDA 0x66d878 (`typed_holder<YAlignment>::singleton`):
    // `__cxa_guard` once-init publishing the typeinfo and the
    // construct/destruct funcs. Host type tags need no init —
    // carrier no-op.
}

// 0x66d8e4 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::YAlignment>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc")]
pub fn stub_66d8e4() {
    // IDA 0x66d8e4 (`typed_holder<YAlignment>::construct_func`):
    // copies the held value pointer when non-null. Host values are
    // `Copy` — carrier no-op.
}

// 0x66d8f0 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::YAlignment>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc")]
pub fn stub_66d8f0() {
    // IDA 0x66d8f0 (`typed_holder<YAlignment>::destruct_func`):
    // empty body — carrier no-op.
}

// 0x66d8f4 — __ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::YAlignment const& rbx::any_cast<RBX::TextService::YAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::TextService::YAlignment const& rbx::any_cast<RBX::TextService::YAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_66d8f4(variant: &YAlignmentVariant) -> u32 {
    // IDA 0x66d8f4 (`any_cast<YAlignment>`): null input misses; the
    // typeinfo-pointer or mangled-name
    // (`N3RBX11TextService10YAlignmentE`) match returns the payload,
    // else `bad_placement_any_cast` is thrown (host: panic). Host:
    // tagged match.
    match *variant {
        YAlignmentVariant::YAlignment(value) => value,
        _ => panic!("rbx::bad_placement_any_cast (IDA 0x66d8f4)"),
    }
}

// 0x66d9e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_66d9e4() {
    // IDA 0x66d9e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x66da0c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_66da0c(name: &str, category: &str, attributes: u32, permissions: u32) -> TextBoxXAlignProp {
    // IDA 0x66da0c (`EnumPropDescriptor<TextBox, XAlignment>` ctor):
    // the `TextBox` `classDescriptor` call, the
    // `EnumDesc<XAlignment>` singleton once-init and the
    // `PropertyDescriptor` base init with name/category/
    // attributes/permissions plus the impl holding the
    // getter/setter member-pointer pair. The pair folds into the
    // `x_alignment` field (see below).
    TextBoxXAlignProp::new(name, category, attributes, permissions)
}

// 0x66dbc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED0Ev")]
pub fn stub_66dbc0() {
    // IDA 0x66dbc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66dbec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10isReadOnlyEv")]
pub fn stub_66dbec() -> bool {
    // IDA 0x66dbec (`EnumPropDescriptor<XAlignment>::isReadOnly`):
    // delegates to the inner `GetSet` at +44 — always readable.
    false
}

// 0x66dbfc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11isWriteOnlyEv")]
pub fn stub_66dbfc() -> bool {
    // IDA 0x66dbfc (`EnumPropDescriptor<XAlignment>::isWriteOnly`):
    // delegates to the inner `GetSet` at +44 — always writable.
    false
}

// 0x66dc0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_66dc0c(first: &TextBoxState, second: &TextBoxState) -> bool {
    // IDA 0x66dc0c (`EnumPropDescriptor<XAlignment>::equalValues`):
    // reads the inner value for both instances via the +44 `GetSet`
    // and compares. Host: compare the alignments.
    first.x_alignment == second.x_alignment
}

// 0x66dc34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_66dc34(state: &TextBoxState) -> XAlignmentVariant {
    // IDA 0x66dc34 (`EnumPropDescriptor<XAlignment>::getVariant`):
    // reads the inner value, tags it with the plain-`int`
    // singleton and placement-moves it in. Host: the `XAlignment`
    // tag.
    XAlignmentVariant::XAlignment(state.x_alignment)
}

// 0x66dc58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_66dc58(state: &mut TextBoxState, variant: &XAlignmentVariant) {
    // IDA 0x66dc58 (`EnumPropDescriptor<XAlignment>::setVariant`):
    // an int-typed variant runs `any_cast<int>`; anything else runs
    // `Variant::convert<int>` (throws on failure, 0x66dcda-0x66dd14);
    // then the +72 setter. Host: convert-or-throw, then store.
    let value = match *variant {
        XAlignmentVariant::XAlignment(value) => value,
        _ => panic!("Unable to convert variant to int (IDA 0x66dc58)"),
    };
    state.x_alignment = value;
}

// 0x66dda4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_66dda4(first: &TextBoxState, second: &mut TextBoxState) {
    // IDA 0x66dda4 (`EnumPropDescriptor<XAlignment>::copyValue`):
    // inner `getValue` on the source then inner `setValue` on the
    // target. Host: copy the alignment.
    second.x_alignment = first.x_alignment;
}

// 0x66ddc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::hasStringValue(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14hasStringValueEv")]
pub fn stub_66ddc8() -> bool {
    // IDA 0x66ddc8 (`EnumPropDescriptor<XAlignment>::hasStringValue`):
    // returns 1 — always stringable.
    true
}

// 0x66ddcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_66ddcc(state: &TextBoxState) -> String {
    // IDA 0x66ddcc (`EnumPropDescriptor<XAlignment>::getStringValue`):
    // reads the enum-desc singleton slot, the inner value via the
    // +44 `GetSet` and `EnumDesc::convertToString`. Host: the
    // grounded item name (`Left` 0, `Center` 2, `Right` 1 — IDA
    // 0x7d8548).
    xalignment_name(state.x_alignment).to_owned()
}

// 0x66ddf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_66ddf0(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x66ddf0 (`EnumPropDescriptor<XAlignment>::setStringValue`):
    // `Name::lookup` + `EnumDesc::convertToValue`; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Host: table
    // position decides.
    match XALIGNMENT_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.x_alignment = XALIGNMENT_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x66de30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_66de30(state: &TextBoxState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x66de30 (`EnumPropDescriptor<XAlignment>::writeValue`):
    // inner `getValue`, `clearValue`, int tag `5` at +16, value at
    // +20, returns 5. Same shape as the `FontSize` twin at 0x6709a4.
    out.value_type = 5;
    out.int_value = state.x_alignment as i32;
    5
}

// 0x66de50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_66de50(state: &mut TextBoxState, xml: &XmlReadValue) {
    // IDA 0x66de50 (`EnumPropDescriptor<XAlignment>::readValue`):
    // xsi:nil early-out (0x66de74); an int pair runs `setIntValue`
    // (index→value with -1 rejection, 0x66dec6) and returns on
    // success; a string pair runs lookup + convert + inner set; a
    // miss runs the +64 reset hook before asserting (folds away);
    // anything else hits `ReleaseAssert(false)`
    // (Reflection.h:359, host seam). Same shape as the `FontSize`
    // twin at 0x6709c4.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if *value >= 0 {
                if let Some((_, align)) = XALIGNMENT_ITEMS.get(*value as usize) {
                    state.x_alignment = *align;
                    return;
                }
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x66de50)");
            }
        }
        XmlReadValue::Text(text) => {
            if stub_66ddf0(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x66de50)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x66de50)");
            }
        }
    }
}

// 0x66e090 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_66e090(state: &TextBoxState) -> i32 {
    // IDA 0x66e090 (`EnumPropDescriptor<XAlignment>::getIndexValue`):
    // inner `getValue` + `EnumDesc::convertToIndex`. Host: the item
    // index of the live value.
    xalignment_index(state.x_alignment)
}

// 0x66e0ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_66e0ac(state: &mut TextBoxState, index: u32) -> bool {
    // IDA 0x66e0ac (`EnumPropDescriptor<XAlignment>::setIndexValue`):
    // bounds-checks the index against the item count, stores
    // `items[index]` through the inner `setValue` and returns 1,
    // else 0.
    match XALIGNMENT_ITEMS.get(index as usize) {
        Some((_, align)) => {
            state.x_alignment = *align;
            true
        }
        None => false,
    }
}

// 0x66e0e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_66e0e0(state: &TextBoxState) -> u32 {
    // IDA 0x66e0e0 (`EnumPropDescriptor<XAlignment>::getEnumValue`):
    // the inner `getValue` straight through.
    state.x_alignment
}

// 0x66e0e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_66e0e8(state: &mut TextBoxState, value: u32) -> bool {
    // IDA 0x66e0e8 (`EnumPropDescriptor<XAlignment>::setEnumValue`):
    // `find_if` with `equalValue` over the items; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Host:
    // membership decides.
    if XALIGNMENT_ITEMS.iter().any(|(_, v)| *v == value) {
        state.x_alignment = value;
        true
    } else {
        false
    }
}

// 0x66e134 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_66e134(state: &TextBoxState) -> i32 {
    // IDA 0x66e134 (`EnumPropDescriptor<XAlignment>::getEnumItem`):
    // inner `getValue` + `EnumDesc::convertToItem`. Host: the item
    // position of the live value (-1 when missing).
    xalignment_index(state.x_alignment)
}

// 0x66e154 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_66e154(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x66e154 (`EnumPropDescriptor<XAlignment>::setStringValue`
    // over `Name`): `convertToValue` on the name; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Same string edge
    // as 0x66ddf0 — host forwards into that twin (`Name` folds into
    // `&str`).
    stub_66ddf0(state, name)
}

// 0x66e188 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(RBX::Name const&,RBX::TextService::XAlignment&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(RBX::Name const&,RBX::TextService::XAlignment&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_66e188(name: &str) -> Option<u32> {
    // IDA 0x66e188 (`EnumDesc<XAlignment>::convertToValue(Name)`):
    // red-black-tree search for the name id, returning 1 with the
    // value on a hit (else 0). Host: the table value (`None` on a
    // miss).
    XALIGNMENT_ITEMS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, v)| *v)
}

// 0x66e204 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToItem(RBX::TextService::XAlignment const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToItem(RBX::TextService::XAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_")]
pub fn stub_66e204(value: u32) -> i32 {
    // IDA 0x66e204 (`EnumDesc<XAlignment>::convertToItem`):
    // FLog::Asserts-gated `value>=0` (enumconverter.h:273) and
    // `value<enumToItem.size` (line 274) ReleaseAsserts — host
    // seams; then the value-indexed item. Host: asserts + table
    // index.
    if flog_asserts() {
        assert!(
            value < XALIGNMENT_ITEMS.len() as u32,
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274 (IDA 0x66e204)"
        );
    }
    xalignment_index(value)
}

// 0x66e2d0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToIndex(RBX::TextService::XAlignment)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToIndex(RBX::TextService::XAlignment)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_")]
pub fn stub_66e2d0(value: u32) -> i32 {
    // IDA 0x66e2d0 (`EnumDesc<XAlignment>::convertToIndex`):
    // FLog::Asserts-gated `value>=0` (enumconverter.h:350) — host
    // seam; then the value-indexed `enumToItem` entry or -1 past
    // the end. Host: the table index (-1 on a miss).
    xalignment_index(value)
}

// 0x66e340 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_66e340(state: &mut TextBoxState, index: i32) -> bool {
    // IDA 0x66e340 (`EnumPropDescriptor<XAlignment>::setIntValue`):
    // rejects negative indices, bounds-checks against the item
    // count and rejects `-1`-valued items, then stores through the
    // inner `setValue` and returns 1, else 0. Table values are
    // non-negative by type, so the `-1` check folds away.
    if index >= 0 {
        if let Some((_, align)) = XALIGNMENT_ITEMS.get(index as usize) {
            state.x_alignment = *align;
            return true;
        }
    }
    false
}
