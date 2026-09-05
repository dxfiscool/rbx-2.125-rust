// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x8e9048..0x937724 | 4111->4211 covered, 1190 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x8e9048 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x8e9048]")]
pub fn stub_0x8e9048(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x8e9048: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x8fff04(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::~BoundYieldFuncDesc()")]
pub fn stub_0x8fff4c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(void),void,0>::~BoundYieldFuncDesc()")]
pub fn stub_0x8fff8c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x8fffb0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16OverlayDataModelEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x8fffb0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::~BoundYieldFuncDesc()")]
pub fn stub_0x8ffff8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x908170 — __ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x908170() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x908328]")]
pub fn stub_0x908328(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(std::string,std::string,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x908a78() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "int", 3)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x908c90() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "int", 3)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::~BoundYieldFuncDesc() [0x908cf8]")]
pub fn stub_0x908cf8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x908ddc() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "int", 3)
}

// 0x909cb8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16OverlayDataModelEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EEC2EMS2_FviiNS3_8functionIFvSI_EEENSL_IFvSsEEEEPKcST_ST_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x909cb8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x909eb4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16OverlayDataModelEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EE16declareSignatureEPKcS7_SM_S7_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x909eb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x909f00 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16OverlayDataModelEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc() [0x909f00]")]
pub fn stub_0x909f00(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x909fe0 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_16OverlayDataModelEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS7_EEENSQ_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x909fe0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(void),void,0>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x90a850() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(void),void,0>::~BoundYieldFuncDesc() [0x90a954]")]
pub fn stub_0x90a954(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(void),void,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x90aa08() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x90bb0c() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x90bc84() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::~BoundYieldFuncDesc() [0x90bcb4]")]
pub fn stub_0x90bcb4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x90bd88() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,std::string,boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,std::string,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x90bfd0() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x90c1f0() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::~BoundYieldFuncDesc() [0x90c23c]")]
pub fn stub_0x90c23c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x90c318() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 2)
}

#[doc(alias = "RBX::ServerScriptService::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x90e708(handle: &crate::slot::InstanceHandle) {
// RBX::ServerScriptService::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ServerScriptService::askForbidChild(RBX::Instance const*)const")]
pub fn stub_0x90e8e0(handle: &crate::slot::InstanceHandle) {
// RBX::ServerScriptService::askForbidChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ServerScriptService::askForbidParent(RBX::Instance const*)const")]
pub fn stub_0x90e8f0(handle: &crate::slot::InstanceHandle) {
// RBX::ServerScriptService::askForbidParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ServerScriptService::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x90e900(handle: &crate::slot::InstanceHandle) {
// RBX::ServerScriptService::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x912ad0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x912b18 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFbiNS2_10AccessTypeEN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEEbLi3EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc()")]
pub fn stub_0x912b18(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x912b1c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()")]
pub fn stub_0x912b1c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x912b5c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x912b5c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::~BoundYieldFuncDesc()")]
pub fn stub_0x912ba4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x9177d0() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "int", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x917948() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "int", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::~BoundYieldFuncDesc() [0x917978]")]
pub fn stub_0x917978(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x917a4c() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "int", 1)
}

// 0x917bec — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EEC2EMS2_FviiNS3_8functionIFvSI_EEENSL_IFvSsEEEEPKcST_ST_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x917bec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x917de8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EE16declareSignatureEPKcS7_SM_S7_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x917de8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x917e34 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::~BoundYieldFuncDesc() [0x917e34]")]
pub fn stub_0x917e34(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x917f14 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiiESI_Li2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS7_EEENSQ_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x917f14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x9180c8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EEC2EMS2_FviNS3_8functionIFvSI_EEENSL_IFvSsEEEEPKcST_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x9180c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x918240 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE16declareSignatureEPKcS7_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x918240() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x918270 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc() [0x918270]")]
pub fn stub_0x918270(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x918344 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS7_EEENSQ_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x918344() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

// 0x9184e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFbiNS2_10AccessTypeEN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEEbLi3EEC2EMS2_FviS3_SB_NS4_8functionIFvbEEENSE_IFvSsEEEEPKcSM_SM_S3_SM_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::AssetService::AccessType,char const*,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::AssetService::AccessType,char const*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x9184e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

// 0x9187cc — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFbiNS2_10AccessTypeEN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEEbLi3EE16declareSignatureEPKcS7_SF_S7_SF_S7_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x9187cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

// 0x918834 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFbiNS2_10AccessTypeEN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEEbLi3EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc() [0x918834]")]
pub fn stub_0x918834(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x9188d4 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFbiNS2_10AccessTypeEN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEEbLi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS4_8functionIFvS7_EEENSJ_IFvSsEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x9188d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x918e68() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "bool", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x919030() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "bool", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::~BoundYieldFuncDesc() [0x91907c]")]
pub fn stub_0x91907c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x91915c() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "bool", 2)
}

// 0x9197f0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12AssetServiceEFbiNS2_10AccessTypeEN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEEbLi3EED2Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc() [0x9197f0]")]
pub fn stub_0x9197f0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x91d460 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
pub fn stub_0x91d460(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x91d554 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x91d554(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x929994 — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEE12isScriptableEv
// was: RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_0x929994() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x92b71c — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEE12isScriptableEv
// was: RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_0x92b71c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::isScriptable(void)const")]
pub fn stub_0x92e638() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::RemoteFunction")
}

// 0x9303d4 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// was: RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_0x9303d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x932014 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEE12isScriptableEv
// was: RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_0x932014() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x937554 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EEC2EMS2_FvS9_S7_NS3_8functionIFvS7_EEENSC_IFvSsEEEEPKcSK_SK_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x937554() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x937724 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EE16declareSignatureEPKcNS0_7VariantESD_SE_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x937724() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}
