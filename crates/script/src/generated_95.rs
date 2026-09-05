// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x937770..0xceae54 | 4211->4311 covered, 1090 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x937770 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::~BoundYieldFuncDesc() [0x937770]")]
pub fn stub_0x937770(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x937898 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSH_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x937898() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x937a8c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EEC2EMS2_FvS7_NS3_8functionIFvS7_EEENSA_IFvSsEEEEPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x937a8c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x937c08 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x937c08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x937c38 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc() [0x937c38]")]
pub fn stub_0x937c38(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x937d40 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSF_IFvSsEEE
// type: int __fastcall(int, int, int, int, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x937d40() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0xa04c10 — __ZN3RBX7Network7Players14reportAbuseLuaEN5boost10shared_ptrINS_8InstanceEEESsSs
// type: void __fastcall(RBX::Network::Players *, _DWORD *, int, int)
// was: RBX::Network::Players::reportAbuseLua(boost::shared_ptr<RBX::Instance>,std::string,std::string)
#[doc(alias = "RBX::Network::Players::reportAbuseLua(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string)")]
pub fn stub_0xa04c10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)")]
pub fn stub_0xa16238(handle: &crate::slot::InstanceHandle) {
// RBX::Network::Players::reportScriptSecurityError(int, std::string, std::string, std::strin~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0xa23aa8]")]
pub fn stub_0xa23aa8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa23ab0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa23ad0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa23ae8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::find<RBX::ScriptInformationProvider>(void)const")]
pub fn stub_0xa31da0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ScriptInformationProvider"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ScriptInformationProvider>(void)")]
pub fn stub_0xa32418() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

// 0xa324e0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_25ScriptInformationProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>(boost::shared_ptr<RBX::ScriptInformationProvider> const*,RBX::ScriptInformationProvider *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const*,RBX::ScriptInformationProvider *)const")]
pub fn stub_0xa324e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0xa327a0]")]
pub fn stub_0xa327a0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa327b0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa327c8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0xa851a8 — __ZN3RBX7Network6Player29loadCharacterAppearanceScriptEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
// was: RBX::Network::Player::loadCharacterAppearanceScript(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Network::Player::loadCharacterAppearanceScript(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0xa851a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0xa8e5f4 — __ZL29setAppearanceParentNullScriptN5boost10shared_ptrIN3RBX8InstanceEEE
// type: void __fastcall(RBX::Instance **)
// was: setAppearanceParentNullScript(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "setAppearanceParentNullScript(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0xa8e5f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::~BoundYieldFuncDesc()")]
pub fn stub_0xa962e4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::~BoundYieldFuncDesc()")]
pub fn stub_0xa9632c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xa963a0() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "bool", 0)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
pub fn stub_0xa96560(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0xa96cd0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEiESJ_Li1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()")]
pub fn stub_0xa96cd0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::~BoundYieldFuncDesc()")]
pub fn stub_0xa96d38(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::~BoundYieldFuncDesc()")]
pub fn stub_0xa96da0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_0xaae1f4() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Network::Player")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isScriptable(void)const")]
pub fn stub_0xaaef48() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Network::Player")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isScriptable(void)const")]
pub fn stub_0xab1d08() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Network::Player")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_0xab4fd4() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Network::Player")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xabd670() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::~BoundYieldFuncDesc() [0xabd8dc]")]
pub fn stub_0xabd8dc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0xabd9d8() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xabdc94() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "int", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::~BoundYieldFuncDesc() [0xabdf00]")]
pub fn stub_0xabdf00(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0xabdffc() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "int", 1)
}

// 0xabe2b8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEiESJ_Li1EEC2EMS3_FviNS4_8functionIFvSJ_EEENSM_IFvSsEEEEPKcSU_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xabe2b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0xabe5ac — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEiESJ_Li1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc() [0xabe5ac]")]
pub fn stub_0xabe5ac(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0xabe6a8 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEiESJ_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS4_8functionIFvS8_EEENSR_IFvSsEEE
// type: void __fastcall(int, int, int, int *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0xabe6a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::isScriptable(void)const")]
pub fn stub_0xac5e98() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Network::Player")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::~BoundYieldFuncDesc() [0xac8970]")]
pub fn stub_0xac8970(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0xac8a4c() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "bool", 0)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xac9400() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::~BoundYieldFuncDesc() [0xac966c]")]
pub fn stub_0xac966c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0xac9768() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::~BoundYieldFuncDesc() [0xac9a24]")]
pub fn stub_0xac9a24(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0xac9b00() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "std::string", 0)
}
