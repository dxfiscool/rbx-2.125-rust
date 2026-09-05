// Auto-generated skeletons for rbx-script — Lua/Script/lua filtered
// Filter: Lua|Script|lua (5041 filtered, 1537 remaining not yet in any crate) -> next 120 EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x76fbc8..0x8176dc | script 13971->14091 distinct (filtered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  " and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStopped(void)")]
pub fn stub_0x76fbc8(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::onScriptStopped() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getStack(void)")]
pub fn stub_0x770184(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::DebuggerBreakpoint(void)")]
pub fn stub_0x770384() -> crate::generated::DebuggerBreakpoint {
// DebuggerBreakpoint ctor — disabled breakpoint id 0.
crate::generated::DebuggerBreakpoint { id: 0, enabled: false }
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::DebuggerBreakpoint(int)")]
pub fn stub_0x7704dc() -> crate::generated::DebuggerBreakpoint {
// DebuggerBreakpoint ctor — disabled breakpoint id 0.
crate::generated::DebuggerBreakpoint { id: 0, enabled: false }
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
pub fn stub_0x770764() -> crate::generated::DebuggerBreakpoint {
// DebuggerBreakpoint ctor — disabled breakpoint id 0.
crate::generated::DebuggerBreakpoint { id: 0, enabled: false }
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
pub fn stub_0x770808(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint() [0x770810]")]
pub fn stub_0x770810(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint() [0x7708fc]")]
pub fn stub_0x7708fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint() [0x770904]")]
pub fn stub_0x770904(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::DebuggerWatch(std::string)")]
pub fn stub_0x770910() -> crate::slot::InstanceHandle {
// RBX::Scripting::DebuggerWatch ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "RBX::Scripting::DebuggerManager::getEnabled(void)const")]
pub fn stub_0x7711c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::DebuggerManager getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getScript(void)const")]
pub fn stub_0x771770(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::isDebugging(void)const")]
pub fn stub_0x7717c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::isPaused(void)const")]
pub fn stub_0x771804(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getCurrentLine(void)const")]
pub fn stub_0x77181c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::getLine(void)const")]
pub fn stub_0x7718b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::DebuggerBreakpoint getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onHook(lua_State *,lua_Debug *)")]
pub fn stub_0x772990(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::onHook(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::push_back(RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
pub fn stub_0x772d2c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::remove<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)")]
pub fn stub_0x772fc0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::push_back(RBX::Scripting::DebuggerWatch * const&)")]
pub fn stub_0x772fec(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(lua_State *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>> const&)")]
pub fn stub_0x773188() -> crate::slot::SlotConnection {
// IDA 0x773188: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "bool RBX::Scripting::ScriptDebugger::withPausedThread<bool>(boost::function<bool ()(lua_State *,lua_Debug *)>)")]
pub fn stub_0x773814(handle: &crate::slot::InstanceHandle) {
// bool RBX::Scripting::ScriptDebugger::withPausedThread<bool>(boost::function<bool (lua_Stat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
pub fn stub_0x774288(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerWatch dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::~DebuggerWatch() [0x77436c]")]
pub fn stub_0x77436c(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerWatch dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
pub fn stub_0x774584(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch() [0x774664]")]
pub fn stub_0x774664(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch() [0x77475c]")]
pub fn stub_0x77475c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch() [0x77483c]")]
pub fn stub_0x77483c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv")]
pub fn stub_0x774938() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv")]
pub fn stub_0x774960() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv")]
pub fn stub_0x774aac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv")]
pub fn stub_0x774abc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD1Ev")]
pub fn stub_0x774ad0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD1Ev")]
pub fn stub_0x774ad4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD2Ev")]
pub fn stub_0x774ad8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator12getClassNameEv")]
pub fn stub_0x774b74() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator6createEv")]
pub fn stub_0x774bfc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting19sDebuggerBreakpointEEEEvv")]
pub fn stub_0x7750f4(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::Scripting::sDebuggerBreakpoint>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting19sDebuggerBreakpointEEEERKS0_v")]
pub fn stub_0x7750f8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Scripting::sDebuggerBreakpoint>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorC2Ev")]
pub fn stub_0x7751dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E17static_getCreatorEv")]
pub fn stub_0x775424() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerBreakpoint"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting16sDebuggerManagerEEEEvv")]
pub fn stub_0x775498(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::Scripting::sDebuggerManager>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting16sDebuggerManagerEEEERKS0_v")]
pub fn stub_0x77549c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Scripting::sDebuggerManager>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorD2Ev")]
pub fn stub_0x775580() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting14sDebuggerWatchEEEEvv")]
pub fn stub_0x775620(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::Scripting::sDebuggerWatch>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator6createEv")]
pub fn stub_0x775628() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting15sScriptDebuggerEEEEvv")]
pub fn stub_0x775770(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::Scripting::sScriptDebugger>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0x775778(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Scripting::DebuggerBreakpoint, RBX::Scripting::sDebuggerBr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x775894(bp: crate::generated::DebuggerBreakpoint) {
// DebuggerBreakpoint dtor.
drop(bp);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x775898(bp: crate::generated::DebuggerBreakpoint) {
// DebuggerBreakpoint dtor.
drop(bp);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x775938(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x775940(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x7759e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x7759ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS8_5list3INS8_5valueISsEENS_17reference_wrapperISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x775f14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x776040() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS8_5list4INS8_5valueISsEENS_17reference_wrapperISD_EENSI_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x776a68() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x776b98() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
pub fn stub_0x777400(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_")]
pub fn stub_0x7774cc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0x7775e4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<bool>(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool&,rbx_core::SharedPtr<std::string> &)")]
pub fn stub_0x777708() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&)")]
pub fn stub_0x777a6c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::insert(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
pub fn stub_0x77a7dc(slot: &crate::slot::CallableSlot) {
// IDA 0x77a7dc: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_init_mutex(void)")]
pub fn stub_0x77aa10(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (lua_State*)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::disconnect(void)")]
pub fn stub_0x77ab14(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::connected(void)const")]
pub fn stub_0x77ac24() -> crate::slot::SlotConnection {
// IDA 0x77ac24: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::remove(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
pub fn stub_0x77ac70(slot: &mut crate::slot::CallableSlot) {
// IDA 0x77ac70: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x77ad60(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (lua_State*)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x77ad64(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (lua_State*)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::~slot()")]
pub fn stub_0x77ae54(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::~StepOverBreakpoint()")]
pub fn stub_0x786c5c(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepOverBreakpoint dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::~StepOverBreakpoint() [0x786c60]")]
pub fn stub_0x786c60(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepOverBreakpoint dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::hitTest(lua_State *,lua_Debug *)")]
pub fn stub_0x786c64(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepOverBreakpoint::hitTest(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::~StepOutBreakpoint()")]
pub fn stub_0x786d28(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepOutBreakpoint dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::~StepOutBreakpoint() [0x786d2c]")]
pub fn stub_0x786d2c(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepOutBreakpoint dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::hitTest(lua_State *,lua_Debug *)")]
pub fn stub_0x786d30(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepOutBreakpoint::hitTest(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::~StepInBreakpoint()")]
pub fn stub_0x786e24(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepInBreakpoint dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::~StepInBreakpoint() [0x786e28]")]
pub fn stub_0x786e28(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepInBreakpoint dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::hitTest(lua_State *,lua_Debug *)")]
pub fn stub_0x786e2c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::StepInBreakpoint::hitTest(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(void)")]
pub fn stub_0x786fa0() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::ContentProvider::verifyRequestedScriptSignature(char const*,std::string const&,bool)")]
pub fn stub_0x7eb54c(handle: &crate::slot::InstanceHandle) {
// RBX::ContentProvider::verifyRequestedScriptSignature(char const*, std::string const&, bool~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ContentProvider::verifyScriptSignature(char const*,bool)")]
pub fn stub_0x7eb9b0(handle: &crate::slot::InstanceHandle) {
// RBX::ContentProvider::verifyScriptSignature(char const*, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TestService::startScripts(void)")]
pub fn stub_0x801ed0(handle: &crate::slot::InstanceHandle) {
// RBX::TestService::startScripts() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TestService::stopScripts(void)")]
pub fn stub_0x8025ac(handle: &crate::slot::InstanceHandle) {
// RBX::TestService::stopScripts() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TestService::onScriptEnded(int)")]
pub fn stub_0x802818(handle: &crate::slot::InstanceHandle) {
// RBX::TestService::onScriptEnded(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TestService::onScriptFailed(int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)")]
pub fn stub_0x802830() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "RBX::TestService::filterScript(std::string const&)")]
pub fn stub_0x8029a8(handle: &crate::slot::InstanceHandle) {
// RBX::TestService::filterScript(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS8_E4typeEST_")]
pub fn stub_0x804864() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list_av_6<rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int,rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::TestService::*)(int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int),rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_0x804978() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x807944() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x807a2c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
pub fn stub_0x807b18(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x807c14(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::invoke(boost::detail::function::function_buffer &,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)")]
pub fn stub_0x807c30(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x807c60(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x807d4c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x807e34(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> &,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &> &,int)")]
pub fn stub_0x807f0c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x807f0c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::call<rbx_core::SharedPtr<RBX::TestService>,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>(rbx_core::SharedPtr<RBX::TestService> &,void const*,int &,char const* &,char const* &,rbx_core::SharedPtr<RBX::BaseScript> &,int &)const")]
pub fn stub_0x80800c() -> crate::slot::BindPiece {
// boost::bind fragment (mf5) composing a host BoundCall.
crate::slot::BindPiece::new("mf5")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x80811c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x808f84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::~LuaSyntaxError()")]
pub fn stub_0x81301c(handle: crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError dtor.
drop(handle);
}

#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::LuaSyntaxError(int,std::exception &)")]
pub fn stub_0x813020(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::~LuaSyntaxError() [0x81316c]")]
pub fn stub_0x81316c(handle: crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError dtor.
drop(handle);
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>)")]
pub fn stub_0x8149cc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "RBX::Lua::ArgumentParser::ignore(void)")]
pub fn stub_0x814c44(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::ArgumentParser::ignore() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::ArgumentParser::getClosing(char)")]
pub fn stub_0x814c48(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Lua::ArgumentParser getter.
cell.get()
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,char)")]
pub fn stub_0x814cc0() -> crate::slot::PortedFn {
// IDA 0x814cc0: __gnu_cxx::__normal_iterator<char const*, std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx::__normal_iterator<~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x814cc0, "__gnu_cxx::__normal_iterator<char const*, std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx~")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
pub fn stub_0x814d18() -> crate::slot::PortedFn {
// IDA 0x814d18: __gnu_cxx::__normal_iterator<char const*, std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cxx::__normal_iterato~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x814d18, "__gnu_cxx::__normal_iterator<char const*, std::string> RBX::Lua::ArgumentParser::parseString<__gnu_c~")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x814e78() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 4 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(4)
}

#[doc(alias = "RBX::StringConverter<RBX::Lua::Library>::convertToString(RBX::Lua::Library const&)")]
pub fn stub_0x81706c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Lua::Library>::convertToString(RBX::Lua::Library const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_index(RBX::Lua::Library const&,char const*,lua_State *)")]
pub fn stub_0x817078(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::Bridge<RBX::Lua::Library, true>::on_index(RBX::Lua::Library const&, char const*,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::getApi(lua_State *)")]
pub fn stub_0x817224(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Lua getter.
cell.get()
}

#[doc(alias = "RBX::Lua::registerLibraryTable(lua_State *)")]
pub fn stub_0x8175f4(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::registerLibraryTable(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_newindex(RBX::Lua::Library&,char const*,lua_State *)")]
pub fn stub_0x817624(key: &str) -> ! {
// Bridge<Library>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::LibraryBridge::saveLibraryResult(lua_State *,int,std::string)")]
pub fn stub_0x8176dc(saved: &mut Option<crate::lua::LuaStackValue>, value: &crate::lua::LuaStackValue) {
// LibraryBridge::saveLibraryResult — retains the value.
*saved = Some(value.clone());
}
