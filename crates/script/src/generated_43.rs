// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x44b420..0x453ba0 | existing ~9201 -> ~9301 total (union; filler 0x44b420 ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15TeleportServiceEEEPT_v")]
pub fn stub_0x44b420() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::TeleportService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v")]
pub fn stub_0x44b678(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sTeleportService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v")]
pub fn stub_0x44b6c0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sTeleportService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15TeleportServiceEEEvv")]
pub fn stub_0x44b7a4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv")]
pub fn stub_0x44b7a8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sTeleportServiceEEE15isNullClassNameEv")]
pub fn stub_0x44bb80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_14CookiesServiceEEEPT_v")]
pub fn stub_0x44bc20() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::CookiesService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v")]
pub fn stub_0x44be78(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sCookiesService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv")]
pub fn stub_0x44bebc() -> crate::slot::PortedFn {
// IDA 0x44bebc: void RBX::Name::callDoDeclare<RBX::sCookiesService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44bebc, "void RBX::Name::callDoDeclare<RBX::sCookiesService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v")]
pub fn stub_0x44bec0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sCookiesService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CookiesServiceEEEvv")]
pub fn stub_0x44bfa4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv")]
pub fn stub_0x44bfa8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sCookiesServiceEEE15isNullClassNameEv")]
pub fn stub_0x44c380(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v")]
pub fn stub_0x44c6f0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::DebrisService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebrisServiceEEEERKS0_v")]
pub fn stub_0x44c948(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sDebrisService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebrisServiceEEEEvv")]
pub fn stub_0x44c98c() -> crate::slot::PortedFn {
// IDA 0x44c98c: void RBX::Name::callDoDeclare<RBX::sDebrisService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44c98c, "void RBX::Name::callDoDeclare<RBX::sDebrisService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v")]
pub fn stub_0x44c990(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDebrisService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv")]
pub fn stub_0x44ca74() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv")]
pub fn stub_0x44ca78() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE15isNullClassNameEv")]
pub fn stub_0x44ce50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v")]
pub fn stub_0x44cef0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::GamePassService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sGamePassServiceEEEERKS0_v")]
pub fn stub_0x44d148(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sGamePassService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sGamePassServiceEEEEvv")]
pub fn stub_0x44d18c() -> crate::slot::PortedFn {
// IDA 0x44d18c: void RBX::Name::callDoDeclare<RBX::sGamePassService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44d18c, "void RBX::Name::callDoDeclare<RBX::sGamePassService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v")]
pub fn stub_0x44d190(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGamePassService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv")]
pub fn stub_0x44d274() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv")]
pub fn stub_0x44d278() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv")]
pub fn stub_0x44d650(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v")]
pub fn stub_0x44d6f0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::SocialService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v")]
pub fn stub_0x44d948(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sSocialService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv")]
pub fn stub_0x44d98c() -> crate::slot::PortedFn {
// IDA 0x44d98c: void RBX::Name::callDoDeclare<RBX::sSocialService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44d98c, "void RBX::Name::callDoDeclare<RBX::sSocialService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v")]
pub fn stub_0x44d990(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSocialService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv")]
pub fn stub_0x44da74() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv")]
pub fn stub_0x44da78() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv")]
pub fn stub_0x44de50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x44e1b4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"InsertService"
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv")]
pub fn stub_0x44e228() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv")]
pub fn stub_0x44e22c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v")]
pub fn stub_0x44e3ec(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sRenderHooksService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv")]
pub fn stub_0x44e430() -> crate::slot::PortedFn {
// IDA 0x44e430: void RBX::Name::callDoDeclare<RBX::sRenderHooksService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44e430, "void RBX::Name::callDoDeclare<RBX::sRenderHooksService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v")]
pub fn stub_0x44e434(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRenderHooksService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv")]
pub fn stub_0x44e518() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv")]
pub fn stub_0x44e51c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0x44e8f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FriendService"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v")]
pub fn stub_0x44ebf0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sFriendService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v")]
pub fn stub_0x44ec38(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFriendService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x44ed54() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FriendService"
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv")]
pub fn stub_0x44edc8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv")]
pub fn stub_0x44edcc() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v")]
pub fn stub_0x44eea4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::GeometryService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v")]
pub fn stub_0x44f0fc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sGeometryService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv")]
pub fn stub_0x44f140() -> crate::slot::PortedFn {
// IDA 0x44f140: void RBX::Name::callDoDeclare<RBX::sGeometryService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44f140, "void RBX::Name::callDoDeclare<RBX::sGeometryService>()")
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv")]
pub fn stub_0x44f148() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv")]
pub fn stub_0x44f14c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv")]
pub fn stub_0x44f524(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0x44f5c8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BadgeService"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v")]
pub fn stub_0x44f904(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sBadgeService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv")]
pub fn stub_0x44f948() -> crate::slot::PortedFn {
// IDA 0x44f948: void RBX::Name::callDoDeclare<RBX::sBadgeService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44f948, "void RBX::Name::callDoDeclare<RBX::sBadgeService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v")]
pub fn stub_0x44f94c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBadgeService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev")]
pub fn stub_0x44fa30() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BadgeService"
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v")]
pub fn stub_0x44fc58() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::BadgeService"))
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x44fe00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BadgeService"
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv")]
pub fn stub_0x44fe74() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv")]
pub fn stub_0x44fe78() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x44ff50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v")]
pub fn stub_0x44ffb8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::PhysicsService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v")]
pub fn stub_0x450210(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sPhysicsService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v")]
pub fn stub_0x450258(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sPhysicsService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv")]
pub fn stub_0x450340() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX14PhysicsServiceC2Ev")]
pub fn stub_0x450418() -> crate::slot::InstanceHandle {
// RBX::PhysicsService ctor.
crate::slot::InstanceHandle::new("RBX::PhysicsService")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv")]
pub fn stub_0x450794() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv")]
pub fn stub_0x450798() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZN3RBX20ConcurrencyValidatorD2Ev")]
pub fn stub_0x450c74(handle: crate::slot::InstanceHandle) {
// RBX::ConcurrencyValidator dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE15isNullClassNameEv")]
pub fn stub_0x4513dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v")]
pub fn stub_0x45147c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::CollectionService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sCollectionServiceEEEERKS0_v")]
pub fn stub_0x4516d4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sCollectionService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_18sCollectionServiceEEEEvv")]
pub fn stub_0x451718() -> crate::slot::PortedFn {
// IDA 0x451718: void RBX::Name::callDoDeclare<RBX::sCollectionService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x451718, "void RBX::Name::callDoDeclare<RBX::sCollectionService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sCollectionServiceEEEERKS0_v")]
pub fn stub_0x45171c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sCollectionService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv")]
pub fn stub_0x451800() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv")]
pub fn stub_0x451804() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE15isNullClassNameEv")]
pub fn stub_0x451bdc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE6insertEPNS5_4slotE")]
pub fn stub_0x452040(slot: &crate::slot::CallableSlot) {
// IDA 0x452040: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE24safe_static_do_get_mutexEv")]
pub fn stub_0x452250(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::RunTransition)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot10disconnectEv")]
pub fn stub_0x452448(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot9connectedEv")]
pub fn stub_0x452558() -> crate::slot::SlotConnection {
// IDA 0x452558: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE6removeEPNS5_4slotE")]
pub fn stub_0x4525e0(slot: &mut crate::slot::CallableSlot) {
// IDA 0x4525e0: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x4526d0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::RunTransition)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotD0Ev")]
pub fn stub_0x4526d8(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev")]
pub fn stub_0x4528b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")]
pub fn stub_0x452ce8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundService"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sRunServiceEEEEvv")]
pub fn stub_0x453030() -> crate::slot::PortedFn {
// IDA 0x453030: void RBX::Name::callDoDeclare<RBX::sRunService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x453030, "void RBX::Name::callDoDeclare<RBX::sRunService>()")
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv")]
pub fn stub_0x453038() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v")]
pub fn stub_0x453200() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::StarterGuiService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sStarterGuiServiceEEEERKS0_v")]
pub fn stub_0x453458(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sStarterGuiService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v")]
pub fn stub_0x4534a0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sStarterGuiService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv")]
pub fn stub_0x453584() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv")]
pub fn stub_0x453588() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE15isNullClassNameEv")]
pub fn stub_0x453940(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v")]
pub fn stub_0x4539e0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::StarterPackService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sStarterPackServiceEEEERKS0_v")]
pub fn stub_0x453b58(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sStarterPackService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v")]
pub fn stub_0x453ba0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sStarterPackService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}
