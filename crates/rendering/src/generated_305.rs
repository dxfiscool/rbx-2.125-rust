//! rendering shard 305 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 33040->33140 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33040 before -> 33140 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x441417 (lowest remaining 0x441418..0x445890, next lowest 0x445974)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x441418 — __ZNK3RBX15ServiceProvider4findINS_14SpawnerServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::SpawnerService * RBX::ServiceProvider::find<RBX::SpawnerService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_14SpawnerServiceEEEPT_v
pub fn stub_441418() -> ! {
    todo!("0x441418 RBX::SpawnerService * RBX::ServiceProvider::find<RBX::SpawnerService>(void)const")
}

// 0x44158c — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sSpawnerServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sSpawnerServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sSpawnerServiceEEE15isNullClassNameEv
pub fn stub_44158c() -> ! {
    todo!("0x44158c __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sSpawnerServiceEEE15isNullClassNameEv")
}

// 0x44162c — __ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v
pub fn stub_44162c() -> ! {
    todo!("0x44162c __ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v")
}

// 0x441670 — __ZN3RBX4Name13callDoDeclareILZNS_15sSpawnerServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sSpawnerServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sSpawnerServiceEEEEvv
pub fn stub_441670() -> ! {
    todo!("0x441670 __ZN3RBX4Name13callDoDeclareILZNS_15sSpawnerServiceEEEEvv")
}

// 0x441674 — __ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v
pub fn stub_441674() -> ! {
    todo!("0x441674 __ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v")
}

// 0x441758 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14SpawnerServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SpawnerService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14SpawnerServiceEEEvv
pub fn stub_441758() -> ! {
    todo!("0x441758 void RBX::ServiceProvider::callDoGetClassIndex<RBX::SpawnerService>(void)")
}

// 0x44175c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14SpawnerServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SpawnerService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_14SpawnerServiceEEEmv
pub fn stub_44175c() -> ! {
    todo!("0x44175c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SpawnerService>(void)")
}

// 0x441838 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5TeamsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Teams,RBX::Teams>(rbx_core::SharedPtr<RBX::Teams> const*,RBX::Teams *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5TeamsES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_441838() -> ! {
    todo!("0x441838 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Teams,RBX::Teams>(boost::shared_ptr<RBX::Teams> const*,RBX::Teams *)const")
}

// 0x441928 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_441928() -> ! {
    todo!("0x441928 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x441930 — __ZN3RBX14FactoryProductINS_5TeamsENS_8InstanceELZNS_6sTeamsEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_5TeamsENS_8InstanceELZNS_6sTeamsEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_5TeamsENS_8InstanceELZNS_6sTeamsEES2_E17static_getCreatorEv
pub fn stub_441930() -> ! {
    todo!("0x441930 __ZN3RBX14FactoryProductINS_5TeamsENS_8InstanceELZNS_6sTeamsEES2_E17static_getCreatorEv")
}

// 0x4419a8 — __ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_4419a8() -> ! {
    todo!("0x4419a8 __ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x441ac8 — __ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev
pub fn stub_441ac8() -> ! {
    todo!("0x441ac8 __ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev")
}

// 0x441cf0 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
pub fn stub_441cf0() -> ! {
    todo!("0x441cf0 rbx::signals::signal<void ()(RBX::UIEvent const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> &)")
}

// 0x441e50 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE8on_errorERSt9exception
pub fn stub_441e50() -> ! {
    todo!("0x441e50 rbx::signals::signal<void ()(RBX::UIEvent const&)>::on_error(std::exception &)")
}

// 0x441e78 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSERKSB_
// type: int *__fastcall(int *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSERKSB_
pub fn stub_441e78() -> ! {
    todo!("0x441e78 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> const&)")
}

// 0x441e9c — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE22safe_static_init_mutexEv
pub fn stub_441e9c() -> ! {
    todo!("0x441e9c rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_init_mutex(void)")
}

// 0x441ea0 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv
pub fn stub_441ea0() -> ! {
    todo!("0x441ea0 rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_do_get_mutex(void)")
}

// 0x441f98 — __ZNK3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::UserInputService * RBX::ServiceProvider::find<RBX::UserInputService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_v
pub fn stub_441f98() -> ! {
    todo!("0x441f98 RBX::UserInputService * RBX::ServiceProvider::find<RBX::UserInputService>(void)const")
}

// 0x442110 — __ZNK3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E7Creator12getClassNameEv
pub fn stub_442110() -> ! {
    todo!("0x442110 __ZNK3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E7Creator12getClassNameEv")
}

// 0x442180 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_16UserInputServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::UserInputService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_16UserInputServiceEEEvv
pub fn stub_442180() -> ! {
    todo!("0x442180 void RBX::ServiceProvider::callDoGetClassIndex<RBX::UserInputService>(void)")
}

// 0x442184 — __ZN5boost20dynamic_pointer_castIN3RBX9GuiTargetENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// type: void *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiTarget> boost::dynamic_pointer_cast<RBX::GuiTarget,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN5boost20dynamic_pointer_castIN3RBX9GuiTargetENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
pub fn stub_442184() -> ! {
    todo!("0x442184 boost::shared_ptr<RBX::GuiTarget> boost::dynamic_pointer_cast<RBX::GuiTarget,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x4421cc — __ZN5boost10shared_ptrIN3RBX8InstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::shared_ptr<RBX::Instance>(rbx_core::WeakPtr<RBX::Instance> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_4421cc() -> ! {
    todo!("0x4421cc boost::shared_ptr<RBX::Instance>::shared_ptr<RBX::Instance>(boost::weak_ptr<RBX::Instance> const&,boost::detail::sp_nothrow_tag)")
}

// 0x442248 — __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_442248() -> ! {
    todo!("0x442248 __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x442368 — __ZN3RBX10Reflection9DescribedINS_9GuiButtonELZNS_10sGuiButtonEENS_17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiButtonELZNS_10sGuiButtonEENS_17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9GuiButtonELZNS_10sGuiButtonEENS_17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_442368() -> ! {
    todo!("0x442368 __ZN3RBX10Reflection9DescribedINS_9GuiButtonELZNS_10sGuiButtonEENS_17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x442488 — __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_442488() -> ! {
    todo!("0x442488 __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x4425a8 — __ZN3RBX10Reflection9DescribedINS_9GuiBase2dELZNS_10sGuiBase2dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase2dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiBase2dELZNS_10sGuiBase2dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase2dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9GuiBase2dELZNS_10sGuiBase2dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase2dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_4425a8() -> ! {
    todo!("0x4425a8 __ZN3RBX10Reflection9DescribedINS_9GuiBase2dELZNS_10sGuiBase2dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase2dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x4426c8 — __ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_4426c8() -> ! {
    todo!("0x4426c8 __ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x4427e8 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE15isNullClassNameEv
pub fn stub_4427e8() -> ! {
    todo!("0x4427e8 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE15isNullClassNameEv")
}

// 0x442888 — __ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v
pub fn stub_442888() -> ! {
    todo!("0x442888 __ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v")
}

// 0x4428d0 — __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v
pub fn stub_4428d0() -> ! {
    todo!("0x4428d0 __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v")
}

// 0x4429b8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GuiService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv
pub fn stub_4429b8() -> ! {
    todo!("0x4429b8 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GuiService>(void)")
}

// 0x442de0 — __ZNK3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_v
pub fn stub_442de0() -> ! {
    todo!("0x442de0 RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(void)const")
}

// 0x442fa8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ControllerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ControllerService,RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const*,RBX::ControllerService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ControllerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_442fa8() -> ! {
    todo!("0x442fa8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ControllerService,RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const*,RBX::ControllerService *)const")
}

// 0x443098 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_443098() -> ! {
    todo!("0x443098 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4430a0 — __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_4430a0() -> ! {
    todo!("0x4430a0 __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x4431c0 — __ZNSt8auto_ptrI10XmlElementED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(XmlElement **)
#[doc(alias = "std::auto_ptr<XmlElement>::~auto_ptr()")]
// was: __ZNSt8auto_ptrI10XmlElementED2Ev
pub fn stub_4431c0() -> ! {
    todo!("0x4431c0 std::auto_ptr<XmlElement>::~auto_ptr()")
}

// 0x443280 — __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev
pub fn stub_443280() -> ! {
    todo!("0x443280 __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev")
}

// 0x44331c — __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNameEv
pub fn stub_44331c() -> ! {
    todo!("0x44331c __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNameEv")
}

// 0x443388 — __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator6createEv
pub fn stub_443388() -> ! {
    todo!("0x443388 __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator6createEv")
}

// 0x4434cc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19ServerScriptServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ServerScriptService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_19ServerScriptServiceEEEN5boost10shared_ptrIT_EEv
pub fn stub_4434cc() -> ! {
    todo!("0x4434cc boost::shared_ptr<RBX::ServerScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ServerScriptService>(void)")
}

// 0x44357c — __ZN5boost10shared_ptrIN3RBX19ServerScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService>::shared_ptr<RBX::ServerScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX19ServerScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_44357c() -> ! {
    todo!("0x44357c boost::shared_ptr<RBX::ServerScriptService>::shared_ptr<RBX::ServerScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x443644 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19ServerScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerScriptService,RBX::ServerScriptService>(rbx_core::SharedPtr<RBX::ServerScriptService> const*,RBX::ServerScriptService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19ServerScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_443644() -> ! {
    todo!("0x443644 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerScriptService,RBX::ServerScriptService>(boost::shared_ptr<RBX::ServerScriptService> const*,RBX::ServerScriptService *)const")
}

// 0x443730 — __ZN5boost6detail12shared_countC2IPN3RBX19ServerScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX19ServerScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_443730() -> ! {
    todo!("0x443730 boost::detail::shared_count::shared_count<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x443838 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_443838() -> ! {
    todo!("0x443838 boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44383c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_44383c() -> ! {
    todo!("0x44383c boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x443840 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_443840() -> ! {
    todo!("0x443840 boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x443860 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_443860() -> ! {
    todo!("0x443860 boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x443878 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_443878() -> ! {
    todo!("0x443878 boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x44387c — __ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v
pub fn stub_44387c() -> ! {
    todo!("0x44387c __ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")
}

// 0x4438c0 — __ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv
pub fn stub_4438c0() -> ! {
    todo!("0x4438c0 __ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv")
}

// 0x4438c4 — __ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v
pub fn stub_4438c4() -> ! {
    todo!("0x4438c4 __ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")
}

// 0x4439a8 — __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev
pub fn stub_4439a8() -> ! {
    todo!("0x4439a8 __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev")
}

// 0x443bd0 — __ZNK3RBX15ServiceProvider4findINS_19ServerScriptServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ServerScriptService * RBX::ServiceProvider::find<RBX::ServerScriptService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_19ServerScriptServiceEEEPT_v
pub fn stub_443bd0() -> ! {
    todo!("0x443bd0 RBX::ServerScriptService * RBX::ServiceProvider::find<RBX::ServerScriptService>(void)const")
}

// 0x443d44 — __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv
pub fn stub_443d44() -> ! {
    todo!("0x443d44 __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv")
}

// 0x443dac — __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv
pub fn stub_443dac() -> ! {
    todo!("0x443dac __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv")
}

// 0x443e20 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19ServerScriptServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerScriptService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19ServerScriptServiceEEEvv
pub fn stub_443e20() -> ! {
    todo!("0x443e20 void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerScriptService>(void)")
}

// 0x443e24 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_19ServerScriptServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerScriptService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_19ServerScriptServiceEEEmv
pub fn stub_443e24() -> ! {
    todo!("0x443e24 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerScriptService>(void)")
}

// 0x443efc — __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorD2Ev
pub fn stub_443efc() -> ! {
    todo!("0x443efc __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorD2Ev")
}

// 0x443f98 — __ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator12getClassNameEv
pub fn stub_443f98() -> ! {
    todo!("0x443f98 __ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator12getClassNameEv")
}

// 0x444004 — __ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator6createEv
pub fn stub_444004() -> ! {
    todo!("0x444004 __ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator6createEv")
}

// 0x444148 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ReplicatedStorageEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ReplicatedStorage> RBX::Creatable<RBX::Instance>::create<RBX::ReplicatedStorage>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ReplicatedStorageEEEN5boost10shared_ptrIT_EEv
pub fn stub_444148() -> ! {
    todo!("0x444148 boost::shared_ptr<RBX::ReplicatedStorage> RBX::Creatable<RBX::Instance>::create<RBX::ReplicatedStorage>(void)")
}

// 0x4441f8 — __ZN5boost10shared_ptrIN3RBX17ReplicatedStorageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ReplicatedStorage>::shared_ptr<RBX::ReplicatedStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX17ReplicatedStorageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_4441f8() -> ! {
    todo!("0x4441f8 boost::shared_ptr<RBX::ReplicatedStorage>::shared_ptr<RBX::ReplicatedStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4442c0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ReplicatedStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ReplicatedStorage,RBX::ReplicatedStorage>(rbx_core::SharedPtr<RBX::ReplicatedStorage> const*,RBX::ReplicatedStorage *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ReplicatedStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4442c0() -> ! {
    todo!("0x4442c0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ReplicatedStorage,RBX::ReplicatedStorage>(boost::shared_ptr<RBX::ReplicatedStorage> const*,RBX::ReplicatedStorage *)const")
}

// 0x4443ac — __ZN5boost6detail12shared_countC2IPN3RBX17ReplicatedStorageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX17ReplicatedStorageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4443ac() -> ! {
    todo!("0x4443ac boost::detail::shared_count::shared_count<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4444b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4444b8() -> ! {
    todo!("0x4444b8 boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4444bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4444bc() -> ! {
    todo!("0x4444bc boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4444d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4444d4() -> ! {
    todo!("0x4444d4 boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4444d8 — __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v
pub fn stub_4444d8() -> ! {
    todo!("0x4444d8 __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v")
}

// 0x444520 — __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v
pub fn stub_444520() -> ! {
    todo!("0x444520 __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v")
}

// 0x444604 — __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorC2Ev
pub fn stub_444604() -> ! {
    todo!("0x444604 __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorC2Ev")
}

// 0x444830 — __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E15isNullClassNameEv
pub fn stub_444830() -> ! {
    todo!("0x444830 __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E15isNullClassNameEv")
}

// 0x444898 — __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E17static_getCreatorEv
pub fn stub_444898() -> ! {
    todo!("0x444898 __ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E17static_getCreatorEv")
}

// 0x444910 — __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD2Ev
pub fn stub_444910() -> ! {
    todo!("0x444910 __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD2Ev")
}

// 0x4449ac — __ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator12getClassNameEv
pub fn stub_4449ac() -> ! {
    todo!("0x4449ac __ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator12getClassNameEv")
}

// 0x444a18 — __ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator6createEv
pub fn stub_444a18() -> ! {
    todo!("0x444a18 __ZNK3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7Creator6createEv")
}

// 0x444b5c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ServerStorageEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerStorage> RBX::Creatable<RBX::Instance>::create<RBX::ServerStorage>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ServerStorageEEEN5boost10shared_ptrIT_EEv
pub fn stub_444b5c() -> ! {
    todo!("0x444b5c boost::shared_ptr<RBX::ServerStorage> RBX::Creatable<RBX::Instance>::create<RBX::ServerStorage>(void)")
}

// 0x444c0c — __ZN5boost10shared_ptrIN3RBX13ServerStorageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerStorage>::shared_ptr<RBX::ServerStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13ServerStorageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_444c0c() -> ! {
    todo!("0x444c0c boost::shared_ptr<RBX::ServerStorage>::shared_ptr<RBX::ServerStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x444cd4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ServerStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerStorage,RBX::ServerStorage>(rbx_core::SharedPtr<RBX::ServerStorage> const*,RBX::ServerStorage *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ServerStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_444cd4() -> ! {
    todo!("0x444cd4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerStorage,RBX::ServerStorage>(boost::shared_ptr<RBX::ServerStorage> const*,RBX::ServerStorage *)const")
}

// 0x444dc0 — __ZN5boost6detail12shared_countC2IPN3RBX13ServerStorageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13ServerStorageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_444dc0() -> ! {
    todo!("0x444dc0 boost::detail::shared_count::shared_count<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x444ec8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_444ec8() -> ! {
    todo!("0x444ec8 boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x444ecc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_444ecc() -> ! {
    todo!("0x444ecc boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x444ed0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_444ed0() -> ! {
    todo!("0x444ed0 boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x444ef0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_444ef0() -> ! {
    todo!("0x444ef0 boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x444f08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_444f08() -> ! {
    todo!("0x444f08 boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x444f0c — __ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v
pub fn stub_444f0c() -> ! {
    todo!("0x444f0c __ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v")
}

// 0x444f50 — __ZN3RBX4Name13callDoDeclareILZNS_14sServerStorageEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sServerStorageEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sServerStorageEEEEvv
pub fn stub_444f50() -> ! {
    todo!("0x444f50 __ZN3RBX4Name13callDoDeclareILZNS_14sServerStorageEEEEvv")
}

// 0x444f54 — __ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v
pub fn stub_444f54() -> ! {
    todo!("0x444f54 __ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v")
}

// 0x445038 — __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorC2Ev
pub fn stub_445038() -> ! {
    todo!("0x445038 __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorC2Ev")
}

// 0x445260 — __ZNK3RBX15ServiceProvider4findINS_13ServerStorageEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ServerStorage * RBX::ServiceProvider::find<RBX::ServerStorage>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_13ServerStorageEEEPT_v
pub fn stub_445260() -> ! {
    todo!("0x445260 RBX::ServerStorage * RBX::ServiceProvider::find<RBX::ServerStorage>(void)const")
}

// 0x4453d4 — __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E15isNullClassNameEv
pub fn stub_4453d4() -> ! {
    todo!("0x4453d4 __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E15isNullClassNameEv")
}

// 0x44543c — __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E17static_getCreatorEv
pub fn stub_44543c() -> ! {
    todo!("0x44543c __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E17static_getCreatorEv")
}

// 0x4454b0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ServerStorageEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerStorage>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ServerStorageEEEvv
pub fn stub_4454b0() -> ! {
    todo!("0x4454b0 void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerStorage>(void)")
}

// 0x4454b4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ServerStorageEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerStorage>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ServerStorageEEEmv
pub fn stub_4454b4() -> ! {
    todo!("0x4454b4 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerStorage>(void)")
}

// 0x44558c — __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v
pub fn stub_44558c() -> ! {
    todo!("0x44558c RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")
}

// 0x445700 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv
pub fn stub_445700() -> ! {
    todo!("0x445700 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")
}

// 0x445768 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv
pub fn stub_445768() -> ! {
    todo!("0x445768 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")
}

// 0x4457dc — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv
pub fn stub_4457dc() -> ! {
    todo!("0x4457dc __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")
}

// 0x445848 — __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
pub fn stub_445848() -> ! {
    todo!("0x445848 __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

// 0x44588c — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv
pub fn stub_44588c() -> ! {
    todo!("0x44588c __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")
}

// 0x445890 — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
pub fn stub_445890() -> ! {
    todo!("0x445890 __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}
