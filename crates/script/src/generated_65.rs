// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x28d61c..0x292300 | remaining 3970 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x28d61c — __ZNK3RBX10BaseScript11getScriptIdEv
// type: _DWORD __fastcall(RBX::BaseScript *__hidden this)
// was: _DWORD __fastcall(RBX::BaseScript *__hidden this)
#[doc(alias = "RBX::BaseScript::getScriptId(void)const")]
pub fn stub_0x28d61c(handle: &crate::slot::InstanceHandle) {
// RBX::BaseScript::getScriptId(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BaseScript,RBX::ContentId>::~PropDescriptor()")]
pub fn stub_0x28d620(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x28d644 — __ZNK3RBX10BaseScript11getDisabledEv
// type: _DWORD __fastcall(RBX::BaseScript *__hidden this)
// was: _DWORD __fastcall(RBX::BaseScript *__hidden this)
#[doc(alias = "RBX::BaseScript::getDisabled(void)const")]
pub fn stub_0x28d644(handle: &crate::slot::InstanceHandle) {
// RBX::BaseScript::getDisabled(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BaseScript,bool>::~PropDescriptor()")]
pub fn stub_0x28d64c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x28d670 — __ZN3RBX6Script11getHashSlowEv
// type: _DWORD __fastcall(RBX::Script *__hidden this)
// was: _DWORD __fastcall(RBX::Script *__hidden this)
#[doc(alias = "RBX::Script::getHashSlow(void)")]
pub fn stub_0x28d670(handle: &crate::slot::InstanceHandle) {
// RBX::Script::getHashSlow(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Script,std::string ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x28d698(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool>(void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool)")]
pub fn stub_0x28d764() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x28da08 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEENS1_INS2_6ScriptEEESsNS2_25ScriptInformationProvider13RequestResultEbbbS4_S6_SsNS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi5EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_T4_T5_T6_ENSE_9list_av_7IT7_T8_T9_T10_T11_T12_T13_E4typeEEESP_SR_SS_ST_SU_SV_SW_SX_
// type: int __fastcall(int, int, int, int, std::string *)
// was: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list_av_7<rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool,rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>(void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>)")]
pub fn stub_0x28da08() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Script> RBX::weak_from<RBX::Script>(RBX::Script*)")]
pub fn stub_0x28dec0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*)")]
pub fn stub_0x28e0c8() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*)")]
pub fn stub_0x28e0e0() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)")]
pub fn stub_0x28e0f8(handle: &crate::slot::InstanceHandle) {
// RBX::Script::writeXml(boost::function<bool (RBX::Instance*)> const&, RBX::CreatorRole) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x28e114 — __ZNK3RBX6Script12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Script *__hidden this, const Instance *)
// was: _DWORD __fastcall(RBX::Script *__hidden this, const Instance *)
#[doc(alias = "RBX::Script::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x28e114(handle: &crate::slot::InstanceHandle) {
// RBX::Script::askSetParent(RBX::Instance const*)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x28e118() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x28e128() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEE12getClassNameEv")]
pub fn stub_0x28e138() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEE12getClassNameEv")]
pub fn stub_0x28e160() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

// 0x28e188 — __ZN3RBX11LocalScriptD1Ev
// type: void __fastcall(RBX::LocalScript *__hidden this)
// was: void __fastcall(RBX::LocalScript *__hidden this)
#[doc(alias = "RBX::LocalScript::~LocalScript()")]
pub fn stub_0x28e188(handle: crate::slot::InstanceHandle) {
// RBX::LocalScript dtor.
drop(handle);
}

// 0x28e18c — __ZN3RBX11LocalScriptD0Ev
// type: void __fastcall(RBX::LocalScript *__hidden this)
// was: void __fastcall(RBX::LocalScript *__hidden this)
#[doc(alias = "RBX::LocalScript::~LocalScript() [0x28e18c]")]
pub fn stub_0x28e18c(handle: crate::slot::InstanceHandle) {
// RBX::LocalScript dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x28e22c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LocalScript"
}

// 0x28e23c — __ZThn32_N3RBX11LocalScriptD1Ev
// type: void __fastcall(RBX::LocalScript *__hidden this)
// was: void __fastcall(RBX::LocalScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalScript::~LocalScript()")]
pub fn stub_0x28e23c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x28e244 — __ZThn32_N3RBX11LocalScriptD0Ev
// type: void __fastcall(RBX::LocalScript *__hidden this)
// was: void __fastcall(RBX::LocalScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalScript::~LocalScript() [0x28e244]")]
pub fn stub_0x28e244(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x28e24c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LocalScript"
}

// 0x28e25c — __ZThn36_N3RBX11LocalScriptD1Ev
// type: void __fastcall(RBX::LocalScript *__hidden this)
// was: void __fastcall(RBX::LocalScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalScript::~LocalScript() [0x28e25c]")]
pub fn stub_0x28e25c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x28e264 — __ZThn36_N3RBX11LocalScriptD0Ev
// type: void __fastcall(RBX::LocalScript *__hidden this)
// was: void __fastcall(RBX::LocalScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalScript::~LocalScript() [0x28e264]")]
pub fn stub_0x28e264(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x28e26c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sBaseScriptEEEEvv")]
pub fn stub_0x28e270() -> crate::slot::PortedFn {
// IDA 0x28e270: void RBX::Name::callDoDeclare<RBX::sBaseScript>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x28e270, "void RBX::Name::callDoDeclare<RBX::sBaseScript>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v")]
pub fn stub_0x28e274(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBaseScript>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x28e354() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LocalScript"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x28e3c8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x28e464() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x28e4ec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)")]
pub fn stub_0x28e630() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x28e6e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Script,RBX::Script>(rbx_core::SharedPtr<RBX::Script> const*,RBX::Script *)const")]
pub fn stub_0x28e7a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

// 0x28e890 — __ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x28e890() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x28e998(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x28e99c]")]
pub fn stub_0x28e99c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x28e9a0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x28e9c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x28e9d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sScriptEEEEvv")]
pub fn stub_0x28e9dc() -> crate::slot::PortedFn {
// IDA 0x28e9dc: void RBX::Name::callDoDeclare<RBX::sScript>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x28e9dc, "void RBX::Name::callDoDeclare<RBX::sScript>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v")]
pub fn stub_0x28e9e0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sScript>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x28eac0 — __ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
// was: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x28eac0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x28ed04() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Script"
}

// 0x28ed78 — __ZNK3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(void)const")]
pub fn stub_0x28ed78() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::RuntimeScriptService")
}

// 0x28ef40 — __ZNK3RBX15ServiceProvider4findINS_20RuntimeScriptServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::find<RBX::RuntimeScriptService>(void)const")]
pub fn stub_0x28ef40() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::RuntimeScriptService"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)")]
pub fn stub_0x28f0b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const&)")]
pub fn stub_0x28f164(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sRuntimeScriptServiceEEEERKS0_v")]
pub fn stub_0x28f198(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sRuntimeScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RuntimeScriptService>(void)")]
pub fn stub_0x28f1dc() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RuntimeScriptService>(void)")]
pub fn stub_0x28f1e0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

// 0x28f2b8 — __ZN3RBX20RuntimeScriptServiceC2Ev
// type: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this)
// was: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this)
#[doc(alias = "RBX::RuntimeScriptService::RuntimeScriptService(void)")]
pub fn stub_0x28f2b8() -> crate::slot::InstanceHandle {
// RBX::RuntimeScriptService ctor.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x28f430(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x28f434(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x28f4d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x28f4dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

// 0x28f580 — __ZThn36_N3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
// was: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x28f580(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x28f588(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x28f62c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RuntimeScriptService,RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const*,RBX::RuntimeScriptService *)const")]
pub fn stub_0x28f6f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

// 0x28f7dc — __ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x28f7dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x28f8e4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x28f8e8]")]
pub fn stub_0x28f8e8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x28f8ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x28f90c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x28f924() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEE15isNullClassNameEv")]
pub fn stub_0x28f928(handle: &crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct<RBX::Instance, RBX::sRuntimeScriptService>::isNullClassName() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_9DataModelEEENS9_INS1_6ScriptEEESsS3_bbbENS7_5list7INS7_5valueISB_EENSH_ISD_EENSH_ISsEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi5EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x28f9c8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_9DataModelEEENS8_INS1_6ScriptEEESsS3_bbbENS6_5list7INS6_5valueISA_EENSG_ISC_EENSG_ISsEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi5EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x28fb68() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>> const&)")]
pub fn stub_0x28fd0c() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "void boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>)")]
pub fn stub_0x28fe44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x28fff4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x290010 — __ZN5boost6detail8function26void_function_obj_invoker5INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEENS5_INS6_6ScriptEEESsNS6_25ScriptInformationProvider13RequestResultEbbbENS3_5list7INS3_5valueIS8_EENSG_ISA_EENSG_ISsEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi5EEEEEEEvSC_bbfbE6invokeERNS1_15function_bufferESC_bbfb
// type: int __fastcall(int, int, int, int, float, int)
// was: int __fastcall(int, int, int, int, float, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker5<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::invoke(boost::detail::function::function_buffer &,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")]
pub fn stub_0x290010(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x290058(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2901fc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x290398(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool) &,boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &> &,int)")]
pub fn stub_0x290458(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x290458: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x290624 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEENS5_INS6_6ScriptEEESsNS6_25ScriptInformationProvider13RequestResultEbbbENS3_5list7INS3_5valueIS8_EENSG_ISA_EENSG_ISsEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi5EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x290624(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::~storage3()")]
pub fn stub_0x290768() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>)")]
pub fn stub_0x290884() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>)")]
pub fn stub_0x290a34() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x290be4() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x290d94() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

// 0x290f44 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_INS3_INS4_6ScriptEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_SA_SB_SD_
// type: int __fastcall(int, int, int, int)
// was: int __fastcall(int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>)")]
pub fn stub_0x290f44() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

// 0x2910f4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_INS3_INS4_6ScriptEEEEENS2_ISsEEEC2ES7_SA_SB_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: int __fastcall(int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>)")]
pub fn stub_0x2910f4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

// 0x291268 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_INS3_INS4_6ScriptEEEEEEC2ES7_SA_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>)")]
pub fn stub_0x291268() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script>(rbx_core::WeakPtr<RBX::Script> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x291384() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25ScriptInformationProvider13RequestResultEbbbENS7_5list6INS7_5valueISB_EENSH_ISsEENSH_ISD_EENSH_IbEESL_SL_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x291400() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25ScriptInformationProvider13RequestResultEbbbENS6_5list6INS6_5valueISA_EENSG_ISsEENSG_ISC_EENSG_IbEESK_SK_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2915c4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>)")]
pub fn stub_0x29178c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x291960(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_0x29197c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x291998(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x291b5c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_6ScriptEEESsNS3_25ScriptInformationProvider13RequestResultEbbbENS8_5list6INS8_5valueISC_EENSI_ISsEENSI_ISE_EENSI_IbEESM_SM_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int)
// was: int __fastcall(int, void *, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x291b5c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x291d1c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_6ScriptEEESsNS3_25ScriptInformationProvider13RequestResultEbbbENS8_5list6INS8_5valueISC_EENSI_ISsEENSI_ISE_EENSI_IbEESM_SM_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x291d1c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub fn stub_0x291e3c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x291e3c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x291fb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX6ScriptEEESsNS6_25ScriptInformationProvider13RequestResultEbbbENS3_5list6INS3_5valueIS8_EENSE_ISsEENSE_ISA_EENSE_IbEESI_SI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x291fb8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
pub fn stub_0x292180() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
pub fn stub_0x292300() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}
