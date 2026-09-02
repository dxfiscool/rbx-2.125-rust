//! rendering shard 467 — 100 stubs 0x7110f8..0x7177f4 EA-sorted asc global gap filler not yet in rbx_rendering (global gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (50422->50522 distinct, fallback after 0x7110f8).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7110f8 — __ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_7110f8() -> ! {
    todo!("0x7110f8 __ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x711218 — __ZN5boost9function1IbPN3RBX8InstanceEE5clearEv
#[doc(alias = "boost::function1<bool,RBX::Instance *>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IbPN3RBX8InstanceEE5clearEv")]
pub fn stub_711218() -> ! {
    todo!("0x711218 boost::function1<bool,RBX::Instance *>::clear(void)")
}

// 0x711244 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE14findDescriptorEPKc
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::findDescriptor(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE14findDescriptorEPKc")]
pub fn stub_711244() -> ! {
    todo!("0x711244 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::findDescriptor(char const*)const")
}

// 0x711274 — __GLOBAL__I_a_298
#[doc(alias = "global constructor keyed to_a_298")]
#[doc(alias = "__GLOBAL__I_a_298")]
pub fn stub_711274() -> ! {
    todo!("0x711274 global constructor keyed to'_a_298")
}

// 0x71281c — __ZN3RBX15StringConverterINS_7Region3EE15convertToStringERKS1_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::StringConverter<RBX::Region3>::convertToString(RBX::Region3 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_7Region3EE15convertToStringERKS1_")]
pub fn stub_71281c() -> ! {
    todo!("0x71281c RBX::StringConverter<RBX::Region3>::convertToString(RBX::Region3 const&)")
}

// 0x712980 — __ZN3RBX15StringConverterINS_7Region3EE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Region3>::convertToValue(std::string const&,RBX::Region3&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_7Region3EE14convertToValueERKSsRS1_")]
pub fn stub_712980() -> ! {
    todo!("0x712980 RBX::StringConverter<RBX::Region3>::convertToValue(std::string const&,RBX::Region3&)")
}

// 0x712984 — __ZN3RBX15StringConverterINS_12Region3int16EE15convertToStringERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Region3int16>::convertToString(RBX::Region3int16 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_12Region3int16EE15convertToStringERKS1_")]
pub fn stub_712984() -> ! {
    todo!("0x712984 RBX::StringConverter<RBX::Region3int16>::convertToString(RBX::Region3int16 const&)")
}

// 0x712afc — __ZN3RBX15StringConverterINS_12Region3int16EE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Region3int16>::convertToValue(std::string const&,RBX::Region3int16&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_12Region3int16EE14convertToValueERKSsRS1_")]
pub fn stub_712afc() -> ! {
    todo!("0x712afc RBX::StringConverter<RBX::Region3int16>::convertToValue(std::string const&,RBX::Region3int16&)")
}

// 0x712d58 — __ZN3RBX15StringConverterINS_6RbxRayEE15convertToStringERKS1_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::StringConverter<RBX::RbxRay>::convertToString(RBX::RbxRay const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_6RbxRayEE15convertToStringERKS1_")]
pub fn stub_712d58() -> ! {
    todo!("0x712d58 RBX::StringConverter<RBX::RbxRay>::convertToString(RBX::RbxRay const&)")
}

// 0x713188 — __ZN3RBX15StringConverterINS_6RbxRayEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::RbxRay>::convertToValue(std::string const&,RBX::RbxRay&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_6RbxRayEE14convertToValueERKSsRS1_")]
pub fn stub_713188() -> ! {
    todo!("0x713188 RBX::StringConverter<RBX::RbxRay>::convertToValue(std::string const&,RBX::RbxRay&)")
}

// 0x7133e8 — __ZN3RBX15StringConverterINS_10BrickColorEE15convertToStringERKS1_
// type: int __fastcall(int, RBX::BrickColor *this)
#[doc(alias = "RBX::StringConverter<RBX::BrickColor>::convertToString(RBX::BrickColor const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_10BrickColorEE15convertToStringERKS1_")]
pub fn stub_7133e8() -> ! {
    todo!("0x7133e8 RBX::StringConverter<RBX::BrickColor>::convertToString(RBX::BrickColor const&)")
}

// 0x713400 — __ZN3RBX15StringConverterINS_6CellIDEE15convertToStringERKS1_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::StringConverter<RBX::CellID>::convertToString(RBX::CellID const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_6CellIDEE15convertToStringERKS1_")]
pub fn stub_713400() -> ! {
    todo!("0x713400 RBX::StringConverter<RBX::CellID>::convertToString(RBX::CellID const&)")
}

// 0x713de0 — __ZN3RBX15StringConverterINS_9ContentIdEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *)
#[doc(alias = "RBX::StringConverter<RBX::ContentId>::convertToValue(std::string const&,RBX::ContentId&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_9ContentIdEE14convertToValueERKSsRS1_")]
pub fn stub_713de0() -> ! {
    todo!("0x713de0 RBX::StringConverter<RBX::ContentId>::convertToValue(std::string const&,RBX::ContentId&)")
}

// 0x713f04 — __ZN3RBX15StringConverterINS_10BrickColorEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::BrickColor>::convertToValue(std::string const&,RBX::BrickColor&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_10BrickColorEE14convertToValueERKSsRS1_")]
pub fn stub_713f04() -> ! {
    todo!("0x713f04 RBX::StringConverter<RBX::BrickColor>::convertToValue(std::string const&,RBX::BrickColor&)")
}

// 0x713f20 — __ZN3RBX15StringConverterINS_9ContentIdEE15convertToStringERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::ContentId>::convertToString(RBX::ContentId const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_9ContentIdEE15convertToStringERKS1_")]
pub fn stub_713f20() -> ! {
    todo!("0x713f20 RBX::StringConverter<RBX::ContentId>::convertToString(RBX::ContentId const&)")
}

// 0x713f2c — __GLOBAL__I_a_299
#[doc(alias = "global constructor keyed to_a_299")]
#[doc(alias = "__GLOBAL__I_a_299")]
pub fn stub_713f2c() -> ! {
    todo!("0x713f2c global constructor keyed to'_a_299")
}

// 0x7140c4 — __ZN3RBX15ServiceProvider34findPublicServiceByClassNameStringESs
#[doc(alias = "RBX::ServiceProvider::findPublicServiceByClassNameString(std::string)")]
#[doc(alias = "__ZN3RBX15ServiceProvider34findPublicServiceByClassNameStringESs")]
pub fn stub_7140c4() -> ! {
    todo!("0x7140c4 RBX::ServiceProvider::findPublicServiceByClassNameString(std::string)")
}

// 0x714248 — __ZN3RBX15ServiceProvider33getPublicServiceByClassNameStringESs
#[doc(alias = "RBX::ServiceProvider::getPublicServiceByClassNameString(std::string)")]
#[doc(alias = "__ZN3RBX15ServiceProvider33getPublicServiceByClassNameStringESs")]
pub fn stub_714248() -> ! {
    todo!("0x714248 RBX::ServiceProvider::getPublicServiceByClassNameString(std::string)")
}

// 0x7143cc — __ZN3RBX15ServiceProvider8newIndexEv
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "RBX::ServiceProvider::newIndex(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider8newIndexEv")]
pub fn stub_7143cc() -> ! {
    todo!("0x7143cc RBX::ServiceProvider::newIndex(void)")
}

// 0x7143dc — __ZN3RBX15ServiceProvider6createEPNS_8InstanceERKNS_4NameE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, RBX::Instance *, const RBX::Name *)
#[doc(alias = "RBX::ServiceProvider::create(RBX::Instance *,RBX::Name const&)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createEPNS_8InstanceERKNS_4NameE")]
pub fn stub_7143dc() -> ! {
    todo!("0x7143dc RBX::ServiceProvider::create(RBX::Instance *,RBX::Name const&)")
}

// 0x714540 — __ZNK3RBX15ServiceProvider22findServiceByClassNameERKNS_4NameE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, const RBX::Name *)
#[doc(alias = "RBX::ServiceProvider::findServiceByClassName(RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider22findServiceByClassNameERKNS_4NameE")]
pub fn stub_714540() -> ! {
    todo!("0x714540 RBX::ServiceProvider::findServiceByClassName(RBX::Name const&)const")
}

// 0x714598 — __ZN3RBX15ServiceProvider20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::ServiceProvider::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX15ServiceProvider20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::ServiceProvider::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_714598() -> ! {
    todo!("0x714598 RBX::ServiceProvider::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x7145b8 — __ZN3RBX15ServiceProvider17onDescendantAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::ServiceProvider::onDescendantAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX15ServiceProvider17onDescendantAddedEPNS_8InstanceE")]
pub fn stub_7145b8() -> ! {
    todo!("0x7145b8 RBX::ServiceProvider::onDescendantAdded(RBX::Instance *)")
}

// 0x7145d4 — __ZN3RBX15ServiceProvider15onChildRemovingEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::ServiceProvider::onChildRemoving(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15onChildRemovingEPNS_8InstanceE")]
pub fn stub_7145d4() -> ! {
    todo!("0x7145d4 RBX::ServiceProvider::onChildRemoving(RBX::Instance *)")
}

// 0x7146e8 — __ZN3RBX15ServiceProvider13clearServicesEv
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "RBX::ServiceProvider::clearServices(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider13clearServicesEv")]
pub fn stub_7146e8() -> ! {
    todo!("0x7146e8 RBX::ServiceProvider::clearServices(void)")
}

// 0x714764 — __ZN3RBX15ServiceProvider12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::ServiceProvider::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX15ServiceProvider12onChildAddedEPNS_8InstanceE")]
pub fn stub_714764() -> ! {
    todo!("0x714764 RBX::ServiceProvider::onChildAdded(RBX::Instance *)")
}

// 0x714978 — __ZN3RBX15ServiceProvider11createChildERKNS_4NameENS_11CreatorRoleE
#[doc(alias = "RBX::ServiceProvider::createChild(RBX::Name const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX15ServiceProvider11createChildERKNS_4NameENS_11CreatorRoleE")]
pub fn stub_714978() -> ! {
    todo!("0x714978 RBX::ServiceProvider::createChild(RBX::Name const&,RBX::CreatorRole)")
}

// 0x714a64 — __ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
pub fn stub_714a64() -> ! {
    todo!("0x714a64 RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")
}

// 0x714aa4 — __ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_714aa4() -> ! {
    todo!("0x714aa4 RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::~EventDesc()")
}

// 0x714ac8 — __ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
// was: RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::~EventDesc()
pub fn stub_714ac8() -> ! {
    todo!("0x714ac8 RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::~EventDesc()")
}

// 0x714aec — __ZNSt3mapIPKN3RBX4NameEN5boost10shared_ptrINS0_8InstanceEEESt4lessIS3_ESaISt4pairIKS3_S7_EEEixERSB_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::Name const*,rbx_core::SharedPtr<RBX::Instance>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEN5boost10shared_ptrINS0_8InstanceEEESt4lessIS3_ESaISt4pairIKS3_S7_EEEixERSB_")]
// was: std::map<RBX::Name const*,boost::shared_ptr<RBX::Instance>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::operator[](RBX::Name const* const&)
pub fn stub_714aec() -> ! {
    todo!("0x714aec std::map<RBX::Name const*,boost::shared_ptr<RBX::Instance>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::operator[](RBX::Name const* const&)")
}

// 0x714c34 — __ZNK3RBX15ServiceProvider11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, const RBX::Instance *lpsrc)
#[doc(alias = "RBX::ServiceProvider::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider11askAddChildEPKNS_8InstanceE")]
pub fn stub_714c34() -> ! {
    todo!("0x714c34 RBX::ServiceProvider::askAddChild(RBX::Instance const*)const")
}

// 0x714c6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE14_M_create_nodeERKSA_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_create_node(std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE14_M_create_nodeERKSA_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_create_node(std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_714c6c() -> ! {
    todo!("0x714c6c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_create_node(std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)")
}

// 0x714d5c — __ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_714d5c() -> ! {
    todo!("0x714d5c RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x714ee0 — __ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
// was: RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::~EventDesc()
pub fn stub_714ee0() -> ! {
    todo!("0x714ee0 RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::~EventDesc()")
}

// 0x714f94 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ServiceProvider,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_714f94() -> ! {
    todo!("0x714f94 RBX::Reflection::EventDescImpl<1,RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x7150e8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ServiceProvider,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_7150e8() -> ! {
    todo!("0x7150e8 RBX::Reflection::EventDescImpl<1,RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x715248 — __ZNK3RBX10Reflection13EventDescBaseINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ServiceProvider,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ServiceProvider::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_15ServiceProviderEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
// was: RBX::Reflection::EventDescBase<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_715248() -> ! {
    todo!("0x715248 RBX::Reflection::EventDescBase<RBX::ServiceProvider,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ServiceProvider::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x71525c — __ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_71525c() -> ! {
    todo!("0x71525c RBX::Reflection::EventDesc<RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::~EventDesc()")
}

// 0x715310 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<0,RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_715310() -> ! {
    todo!("0x715310 RBX::Reflection::EventDescImpl<0,RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x715514 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_715514() -> ! {
    todo!("0x715514 RBX::Reflection::EventDescImpl<0,RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x715588 — __ZNK3RBX10Reflection13EventDescBaseINS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_15ServiceProviderEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_715588() -> ! {
    todo!("0x715588 RBX::Reflection::EventDescBase<RBX::ServiceProvider,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::ServiceProvider::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x71559c — __ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS2_FS6_SsEPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS2_FS6_SsEPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_71559c() -> ! {
    todo!("0x71559c RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x715714 — __ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_715714() -> ! {
    todo!("0x715714 RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x715744 — __ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
pub fn stub_715744() -> ! {
    todo!("0x715744 RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")
}

// 0x715810 — __ZNK3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_15ServiceProviderEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_715810() -> ! {
    todo!("0x715810 RBX::Reflection::BoundFuncDesc<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x715950 — __ZN3RBX10Reflection11Call1HelperINS_15ServiceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEESsESsS6_E4callEPS2_S8_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ServiceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::ServiceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_15ServiceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEESsESsS6_E4callEPS2_S8_RNS0_7VariantERKSs")]
// was: RBX::Reflection::Call1Helper<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::ServiceProvider*,boost::shared_ptr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)
pub fn stub_715950() -> ! {
    todo!("0x715950 RBX::Reflection::Call1Helper<RBX::ServiceProvider,boost::shared_ptr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::ServiceProvider*,boost::shared_ptr<RBX::Instance> (RBX::ServiceProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x715ad0 — __GLOBAL__I_a_300
#[doc(alias = "global constructor keyed to_a_300")]
#[doc(alias = "__GLOBAL__I_a_300")]
pub fn stub_715ad0() -> ! {
    todo!("0x715ad0 global constructor keyed to'_a_300")
}

// 0x715ea0 — __ZN3RBX4VerbC2EPNS_13VerbContainerERKSs
// type: _DWORD __fastcall(RBX::Verb *__hidden this, RBX::VerbContainer *, const std::string *)
#[doc(alias = "RBX::Verb::Verb(RBX::VerbContainer *,std::string const&)")]
#[doc(alias = "__ZN3RBX4VerbC2EPNS_13VerbContainerERKSs")]
pub fn stub_715ea0() -> ! {
    todo!("0x715ea0 RBX::Verb::Verb(RBX::VerbContainer *,std::string const&)")
}

// 0x715ed8 — __ZN3RBX13VerbContainer7addVerbEPNS_4VerbE
// type: _DWORD __fastcall(RBX::VerbContainer *__hidden this, RBX::Verb *)
#[doc(alias = "RBX::VerbContainer::addVerb(RBX::Verb *)")]
#[doc(alias = "__ZN3RBX13VerbContainer7addVerbEPNS_4VerbE")]
pub fn stub_715ed8() -> ! {
    todo!("0x715ed8 RBX::VerbContainer::addVerb(RBX::Verb *)")
}

// 0x715f8c — __ZN3RBX4VerbD0Ev
// type: void __fastcall(RBX::Verb *__hidden this)
#[doc(alias = "RBX::Verb::~Verb()")]
#[doc(alias = "__ZN3RBX4VerbD0Ev")]
pub fn stub_715f8c() -> ! {
    todo!("0x715f8c RBX::Verb::~Verb()")
}

// 0x716044 — __ZN3RBX4VerbD1Ev
// type: void __fastcall(RBX::Verb *__hidden this)
#[doc(alias = "RBX::Verb::~Verb()")]
#[doc(alias = "__ZN3RBX4VerbD1Ev")]
pub fn stub_716044() -> ! {
    todo!("0x716044 RBX::Verb::~Verb()")
}

// 0x716068 — __ZN3RBX4VerbD2Ev
// type: void __fastcall(RBX::Verb *__hidden this)
#[doc(alias = "RBX::Verb::~Verb()")]
#[doc(alias = "__ZN3RBX4VerbD2Ev")]
pub fn stub_716068() -> ! {
    todo!("0x716068 RBX::Verb::~Verb()")
}

// 0x71608c — __ZN3RBX13VerbContainer10removeVerbEPNS_4VerbE
// type: _DWORD __fastcall(RBX::VerbContainer *__hidden this, RBX::Verb *)
#[doc(alias = "RBX::VerbContainer::removeVerb(RBX::Verb *)")]
#[doc(alias = "__ZN3RBX13VerbContainer10removeVerbEPNS_4VerbE")]
pub fn stub_71608c() -> ! {
    todo!("0x71608c RBX::VerbContainer::removeVerb(RBX::Verb *)")
}

// 0x7160f4 — __ZN3RBX13VerbContainerC2EPS0_
// type: _DWORD __fastcall(RBX::VerbContainer *__hidden this, VerbContainer *)
#[doc(alias = "RBX::VerbContainer::VerbContainer(RBX::VerbContainer*)")]
#[doc(alias = "__ZN3RBX13VerbContainerC2EPS0_")]
pub fn stub_7160f4() -> ! {
    todo!("0x7160f4 RBX::VerbContainer::VerbContainer(RBX::VerbContainer*)")
}

// 0x71611c — __ZN3RBX13VerbContainer13setVerbParentEPS0_
// type: _DWORD __fastcall(RBX::VerbContainer *__hidden this, RBX::VerbContainer *)
#[doc(alias = "RBX::VerbContainer::setVerbParent(RBX::VerbContainer*)")]
#[doc(alias = "__ZN3RBX13VerbContainer13setVerbParentEPS0_")]
pub fn stub_71611c() -> ! {
    todo!("0x71611c RBX::VerbContainer::setVerbParent(RBX::VerbContainer*)")
}

// 0x716120 — __ZN3RBX13VerbContainerD0Ev
// type: void __fastcall(RBX::VerbContainer *__hidden this)
#[doc(alias = "RBX::VerbContainer::~VerbContainer()")]
#[doc(alias = "__ZN3RBX13VerbContainerD0Ev")]
pub fn stub_716120() -> ! {
    todo!("0x716120 RBX::VerbContainer::~VerbContainer()")
}

// 0x7161c0 — __ZN3RBX13VerbContainerD1Ev
// type: void __fastcall(RBX::VerbContainer *__hidden this)
#[doc(alias = "RBX::VerbContainer::~VerbContainer()")]
#[doc(alias = "__ZN3RBX13VerbContainerD1Ev")]
pub fn stub_7161c0() -> ! {
    todo!("0x7161c0 RBX::VerbContainer::~VerbContainer()")
}

// 0x7161c4 — __ZN3RBX13VerbContainerD2Ev
// type: void __fastcall(RBX::VerbContainer *__hidden this)
#[doc(alias = "RBX::VerbContainer::~VerbContainer()")]
#[doc(alias = "__ZN3RBX13VerbContainerD2Ev")]
pub fn stub_7161c4() -> ! {
    todo!("0x7161c4 RBX::VerbContainer::~VerbContainer()")
}

// 0x7162a4 — __ZN3RBX13VerbContainer7getVerbERKSs
// type: _DWORD __fastcall(RBX::VerbContainer *__hidden this, const std::string *)
#[doc(alias = "RBX::VerbContainer::getVerb(std::string const&)")]
#[doc(alias = "__ZN3RBX13VerbContainer7getVerbERKSs")]
pub fn stub_7162a4() -> ! {
    todo!("0x7162a4 RBX::VerbContainer::getVerb(std::string const&)")
}

// 0x7162bc — __ZN3RBX13VerbContainer7getVerbERKNS_4NameE
// type: _DWORD __fastcall(RBX::VerbContainer *__hidden this, const RBX::Name *)
#[doc(alias = "RBX::VerbContainer::getVerb(RBX::Name const&)")]
#[doc(alias = "__ZN3RBX13VerbContainer7getVerbERKNS_4NameE")]
pub fn stub_7162bc() -> ! {
    todo!("0x7162bc RBX::VerbContainer::getVerb(RBX::Name const&)")
}

// 0x716310 — __ZNSt3mapIPKN3RBX4NameEPNS0_4VerbESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Verb *,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEPNS0_4VerbESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_716310() -> ! {
    todo!("0x716310 std::map<RBX::Name const*,RBX::Verb *,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::operator[](RBX::Name const* const&)")
}

// 0x716368 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE5eraseERS5_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::erase(RBX::Name const* const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE5eraseERS5_")]
pub fn stub_716368() -> ! {
    todo!("0x716368 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::erase(RBX::Name const* const&)")
}

// 0x716390 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE11equal_rangeERS5_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::equal_range(RBX::Name const* const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE11equal_rangeERS5_")]
pub fn stub_716390() -> ! {
    todo!("0x716390 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::equal_range(RBX::Name const* const&)")
}

// 0x7163dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_ESG_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Verb *>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Verb *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_ESG_")]
pub fn stub_7163dc() -> ! {
    todo!("0x7163dc std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Verb *>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Verb *>>)")
}

// 0x71643c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Verb *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_71643c() -> ! {
    todo!("0x71643c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Verb *>> *)")
}

// 0x716464 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Verb *>>,std::pair<RBX::Name const* const,RBX::Verb *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_716464() -> ! {
    todo!("0x716464 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Verb *>>,std::pair<RBX::Name const* const,RBX::Verb *> const&)")
}

// 0x716518 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Verb *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_716518() -> ! {
    todo!("0x716518 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Verb *> const&)")
}

// 0x716570 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Verb *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_716570() -> ! {
    todo!("0x716570 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Verb *> const&)")
}

// 0x7165d8 — __GLOBAL__I_a_301
#[doc(alias = "global constructor keyed to_a_301")]
#[doc(alias = "__GLOBAL__I_a_301")]
pub fn stub_7165d8() -> ! {
    todo!("0x7165d8 global constructor keyed to'_a_301")
}

// 0x7166a0 — __ZN3RBX8AssemblyC1Ev
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::Assembly(void)")]
#[doc(alias = "__ZN3RBX8AssemblyC1Ev")]
pub fn stub_7166a0() -> ! {
    todo!("0x7166a0 RBX::Assembly::Assembly(void)")
}

// 0x7166a4 — __ZN3RBX8AssemblyC2Ev
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::Assembly(void)")]
#[doc(alias = "__ZN3RBX8AssemblyC2Ev")]
pub fn stub_7166a4() -> ! {
    todo!("0x7166a4 RBX::Assembly::Assembly(void)")
}

// 0x716824 — __ZN3RBX8Assembly24computeAssemblyMaxRadiusEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::computeAssemblyMaxRadius(void)")]
#[doc(alias = "__ZN3RBX8Assembly24computeAssemblyMaxRadiusEv")]
pub fn stub_716824() -> ! {
    todo!("0x716824 RBX::Assembly::computeAssemblyMaxRadius(void)")
}

// 0x7168bc — __ZN3RBX8AssemblyD0Ev
// type: void __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::~Assembly()")]
#[doc(alias = "__ZN3RBX8AssemblyD0Ev")]
pub fn stub_7168bc() -> ! {
    todo!("0x7168bc RBX::Assembly::~Assembly()")
}

// 0x71695c — __ZN3RBX8AssemblyD1Ev
// type: void __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::~Assembly()")]
#[doc(alias = "__ZN3RBX8AssemblyD1Ev")]
pub fn stub_71695c() -> ! {
    todo!("0x71695c RBX::Assembly::~Assembly()")
}

// 0x716960 — __ZThn8_N3RBX8AssemblyD0Ev
// type: void __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Assembly::~Assembly()")]
#[doc(alias = "__ZThn8_N3RBX8AssemblyD0Ev")]
pub fn stub_716960() -> ! {
    todo!("0x716960 non-virtual thunk toRBX::Assembly::~Assembly()")
}

// 0x716968 — __ZN3RBX8AssemblyD2Ev
// type: void __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::~Assembly()")]
#[doc(alias = "__ZN3RBX8AssemblyD2Ev")]
pub fn stub_716968() -> ! {
    todo!("0x716968 RBX::Assembly::~Assembly()")
}

// 0x716bf8 — __ZThn8_N3RBX8AssemblyD1Ev
// type: void __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Assembly::~Assembly()")]
#[doc(alias = "__ZThn8_N3RBX8AssemblyD1Ev")]
pub fn stub_716bf8() -> ! {
    todo!("0x716bf8 non-virtual thunk toRBX::Assembly::~Assembly()")
}

// 0x716c00 — __ZN3RBX8Assembly5resetENS_3Sim13AssemblyStateE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Assembly::reset(RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX8Assembly5resetENS_3Sim13AssemblyStateE")]
pub fn stub_716c00() -> ! {
    todo!("0x716c00 RBX::Assembly::reset(RBX::Sim::AssemblyState)")
}

// 0x716df8 — __ZN3RBX8Assembly18sampleAndNotMovingEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::sampleAndNotMoving(void)")]
#[doc(alias = "__ZN3RBX8Assembly18sampleAndNotMovingEv")]
pub fn stub_716df8() -> ! {
    todo!("0x716df8 RBX::Assembly::sampleAndNotMoving(void)")
}

// 0x716e08 — __ZN3RBX8Assembly20preventNeighborSleepEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::preventNeighborSleep(void)")]
#[doc(alias = "__ZN3RBX8Assembly20preventNeighborSleepEv")]
pub fn stub_716e08() -> ! {
    todo!("0x716e08 RBX::Assembly::preventNeighborSleep(void)")
}

// 0x716e14 — __ZN3RBX8Assembly6wakeUpEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::wakeUp(void)")]
#[doc(alias = "__ZN3RBX8Assembly6wakeUpEv")]
pub fn stub_716e14() -> ! {
    todo!("0x716e14 RBX::Assembly::wakeUp(void)")
}

// 0x716e34 — __ZN3RBX8Assembly20getAssemblyPrimitiveEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getAssemblyPrimitive(void)")]
#[doc(alias = "__ZN3RBX8Assembly20getAssemblyPrimitiveEv")]
pub fn stub_716e34() -> ! {
    todo!("0x716e34 RBX::Assembly::getAssemblyPrimitive(void)")
}

// 0x716e98 — __ZNK3RBX8Assembly16getAssemblyStateEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getAssemblyState(void)const")]
#[doc(alias = "__ZNK3RBX8Assembly16getAssemblyStateEv")]
pub fn stub_716e98() -> ! {
    todo!("0x716e98 RBX::Assembly::getAssemblyState(void)const")
}

// 0x716f00 — __ZNK3RBX8Assembly25getConstAssemblyPrimitiveEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getConstAssemblyPrimitive(void)const")]
#[doc(alias = "__ZNK3RBX8Assembly25getConstAssemblyPrimitiveEv")]
pub fn stub_716f00() -> ! {
    todo!("0x716f00 RBX::Assembly::getConstAssemblyPrimitive(void)const")
}

// 0x716f64 — __ZN3RBX8Assembly24getPrimitiveAssemblyFastEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::Assembly::getPrimitiveAssemblyFast(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8Assembly24getPrimitiveAssemblyFastEPNS_9PrimitiveE")]
pub fn stub_716f64() -> ! {
    todo!("0x716f64 RBX::Assembly::getPrimitiveAssemblyFast(RBX::Primitive *)")
}

// 0x716fd0 — __ZN3RBX8Assembly20getPrimitiveAssemblyEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::Assembly::getPrimitiveAssembly(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8Assembly20getPrimitiveAssemblyEPNS_9PrimitiveE")]
pub fn stub_716fd0() -> ! {
    todo!("0x716fd0 RBX::Assembly::getPrimitiveAssembly(RBX::Primitive *)")
}

// 0x716ff8 — __ZN3RBX8Assembly25getConstPrimitiveAssemblyEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Assembly::getConstPrimitiveAssembly(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX8Assembly25getConstPrimitiveAssemblyEPKNS_9PrimitiveE")]
pub fn stub_716ff8() -> ! {
    todo!("0x716ff8 RBX::Assembly::getConstPrimitiveAssembly(RBX::Primitive const*)")
}

// 0x717020 — __ZN3RBX8Assembly15onLowersChangedEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::onLowersChanged(void)")]
#[doc(alias = "__ZN3RBX8Assembly15onLowersChangedEv")]
pub fn stub_717020() -> ! {
    todo!("0x717020 RBX::Assembly::onLowersChanged(void)")
}

// 0x717028 — __ZThn8_N3RBX8Assembly15onLowersChangedEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Assembly::onLowersChanged(void)")]
#[doc(alias = "__ZThn8_N3RBX8Assembly15onLowersChangedEv")]
pub fn stub_717028() -> ! {
    todo!("0x717028 non-virtual thunk toRBX::Assembly::onLowersChanged(void)")
}

// 0x717030 — __ZN3RBX8Assembly16getAssemblyClumpEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getAssemblyClump(void)")]
#[doc(alias = "__ZN3RBX8Assembly16getAssemblyClumpEv")]
pub fn stub_717030() -> ! {
    todo!("0x717030 RBX::Assembly::getAssemblyClump(void)")
}

// 0x717088 — __ZNK3RBX8Assembly21getConstAssemblyClumpEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getConstAssemblyClump(void)const")]
#[doc(alias = "__ZNK3RBX8Assembly21getConstAssemblyClumpEv")]
pub fn stub_717088() -> ! {
    todo!("0x717088 RBX::Assembly::getConstAssemblyClump(void)const")
}

// 0x7170e0 — __ZN3RBX9lessMotorEPKNS_5JointES2_
// type: _DWORD __fastcall(RBX *__hidden this, const RBX::Joint *, const RBX::Joint *)
#[doc(alias = "RBX::lessMotor(RBX::Joint const*,RBX::Joint const*)")]
#[doc(alias = "__ZN3RBX9lessMotorEPKNS_5JointES2_")]
pub fn stub_7170e0() -> ! {
    todo!("0x7170e0 RBX::lessMotor(RBX::Joint const*,RBX::Joint const*)")
}

// 0x71760c — __ZN3RBX8Assembly23notifyMovedFromExternalEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::notifyMovedFromExternal(void)")]
#[doc(alias = "__ZN3RBX8Assembly23notifyMovedFromExternalEv")]
pub fn stub_71760c() -> ! {
    todo!("0x71760c RBX::Assembly::notifyMovedFromExternal(void)")
}

// 0x71767c — __ZN3RBX8Assembly23isAssemblyRootPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::Assembly::isAssemblyRootPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX8Assembly23isAssemblyRootPrimitiveEPKNS_9PrimitiveE")]
pub fn stub_71767c() -> ! {
    todo!("0x71767c RBX::Assembly::isAssemblyRootPrimitive(RBX::Primitive const*)")
}

// 0x717710 — __ZN3RBX8Assembly13otherAssemblyEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::Assembly::otherAssembly(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX8Assembly13otherAssemblyEPNS_4EdgeE")]
pub fn stub_717710() -> ! {
    todo!("0x717710 RBX::Assembly::otherAssembly(RBX::Edge *)")
}

// 0x717790 — __ZNK3RBX8Assembly14getCanThrottleEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getCanThrottle(void)const")]
#[doc(alias = "__ZNK3RBX8Assembly14getCanThrottleEv")]
pub fn stub_717790() -> ! {
    todo!("0x717790 RBX::Assembly::getCanThrottle(void)const")
}

// 0x7177a0 — __ZN3RBX8Assembly18computeCanThrottleEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::Assembly::computeCanThrottle(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX8Assembly18computeCanThrottleEPNS_4EdgeE")]
pub fn stub_7177a0() -> ! {
    todo!("0x7177a0 RBX::Assembly::computeCanThrottle(RBX::Edge *)")
}

// 0x7177d8 — __ZNK3RBX8Assembly13get2dPositionEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::get2dPosition(void)const")]
#[doc(alias = "__ZNK3RBX8Assembly13get2dPositionEv")]
pub fn stub_7177d8() -> ! {
    todo!("0x7177d8 RBX::Assembly::get2dPosition(void)const")
}

// 0x7177f4 — __ZN3RBX8Assembly28gatherPrimitiveExternalEdgesEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::Assembly::gatherPrimitiveExternalEdges(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8Assembly28gatherPrimitiveExternalEdgesEPNS_9PrimitiveE")]
pub fn stub_7177f4() -> ! {
    todo!("0x7177f4 RBX::Assembly::gatherPrimitiveExternalEdges(RBX::Primitive *)")
}
