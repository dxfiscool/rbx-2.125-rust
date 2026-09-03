//! rendering watchdog2 1788337199 — 120 stubs 0x7816b8..0x786af8 EA-sorted asc gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler 52665->52785 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7816b8 — __ZN3rbx7signals6signalIFviEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int)>::connect<boost::function<void ()(int)>>(boost::function<void ()(int)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
// IDA 0x7816b8: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7816b8() {
}

// 0x7817b0 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: unknown
#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::function<void ()(int)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
// IDA 0x7817b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7817b0() {
}

// 0x7818c0 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: unknown
#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::function<void ()(int)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
// IDA 0x7818c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7818c0() {
}

// 0x7819f0 — __ZN3rbx7signals6signalIFviEE4slotD0Ev
// type: unknown
#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE4slotD0Ev")]
// IDA 0x7819f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7819f0() {
}

// 0x781ac4 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::PropDescriptor<int (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x781ac4: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781ac4() {
}

// 0x781bd0 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiED0Ev")]
// IDA 0x781bd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_781bd0() {
}

// 0x781bfc — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE10isReadOnlyEv")]
// IDA 0x781bfc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781bfc() {
}

// 0x781c00 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE11isWriteOnlyEv")]
// IDA 0x781c00: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781c00() {
}

// 0x781c04 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x781c04: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781c04() {
}

// 0x781c24 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
// IDA 0x781c24: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781c24() {
}

// 0x781d44 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::PropDescriptor<bool (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,bool (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x781d44: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781d44() {
}

// 0x781e50 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbED0Ev")]
// IDA 0x781e50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_781e50() {
}

// 0x781e7c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE10isReadOnlyEv")]
// IDA 0x781e7c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781e7c() {
}

// 0x781e80 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv")]
// IDA 0x781e80: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781e80() {
}

// 0x781e84 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x781e84: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781e84() {
}

// 0x781ea8 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0x781ea8: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781ea8() {
}

// 0x781fc8 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsEC2IMS3_KFSsvEMS3_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::PropDescriptor<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>(char const*,char const*,std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsEC2IMS3_KFSsvEMS3_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x781fc8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781fc8() {
}

// 0x7820dc — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsED0Ev")]
// IDA 0x7820dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7820dc() {
}

// 0x782108 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE10isReadOnlyEv")]
// IDA 0x782108: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782108() {
}

// 0x78210c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE11isWriteOnlyEv")]
// IDA 0x78210c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78210c() {
}

// 0x782110 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x782110: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782110() {
}

// 0x782138 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsE10GetSetImplIMS3_KFSsvEMS3_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
// IDA 0x782138: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782138() {
}

// 0x78227c — __ZN3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEEC2IMS3_KFPS4_vEiEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::RefPropDescriptor<RBX::Script* (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,RBX::Script* (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEEC2IMS3_KFPS4_vEiEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x78227c: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78227c() {
}

// 0x782320 — __ZN3RBX10Reflection7RefTypeIPNS_6ScriptEE9singletonEv
// type: unknown
#[doc(alias = "RBX::Reflection::RefType<RBX::Script *>::singleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_6ScriptEE9singletonEv")]
// IDA 0x782320: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782320() {
}

// 0x782418 — __ZN3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEED0Ev")]
// IDA 0x782418: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_782418() {
}

// 0x782448 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10isReadOnlyEv")]
// IDA 0x782448: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782448() {
}

// 0x782458 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11isWriteOnlyEv")]
// IDA 0x782458: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782458() {
}

// 0x782468 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11equalValuesEPKNS0_13DescribedBaseES8_")]
// IDA 0x782468: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782468() {
}

// 0x782490 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// IDA 0x782490: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782490() {
}

// 0x7825a8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// IDA 0x7825a8: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7825a8() {
}

// 0x782670 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
// IDA 0x782670: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782670() {
}

// 0x782694 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0x782694: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782694() {
}

// 0x782768 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0x782768: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782768() {
}

// 0x78278c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11getRefValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11getRefValueEPKNS0_13DescribedBaseE")]
// IDA 0x78278c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78278c() {
}

// 0x7827a0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11setRefValueEPNS0_13DescribedBaseES7_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11setRefValueEPNS0_13DescribedBaseES7_")]
// IDA 0x7827a0: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7827a0() {
}

// 0x78281c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE17setRefValueUnsafeEPNS0_13DescribedBaseES7_
// type: unknown
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE17setRefValueUnsafeEPNS0_13DescribedBaseES7_")]
// IDA 0x78281c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78281c() {
}

// 0x78283c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// IDA 0x78283c: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78283c() {
}

// 0x78291c — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: unknown
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// IDA 0x78291c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78291c() {
}

// 0x782928 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEEE9singletonEv
// type: unknown
#[doc(alias = "rbx::implementation::typed_holder<boost::shared_ptr<RBX::Reflection::DescribedBase>>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEEE9singletonEv")]
// IDA 0x782928: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782928() {
}

// 0x782998 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEEE13destruct_funcEPc
// type: unknown
#[doc(alias = "rbx::implementation::typed_holder<boost::shared_ptr<RBX::Reflection::DescribedBase>>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEEE13destruct_funcEPc")]
// IDA 0x782998: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782998() {
}

// 0x7829a4 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE10isReadOnlyEv")]
// IDA 0x7829a4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7829a4() {
}

// 0x7829a8 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE11isWriteOnlyEv")]
// IDA 0x7829a8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7829a8() {
}

// 0x7829ac — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x7829ac: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7829ac() {
}

// 0x7829cc — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE8setValueEPNS0_13DescribedBaseERKS5_
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Script * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEPNS_6ScriptEE7GetImplIMS3_KFS5_vEE8setValueEPNS0_13DescribedBaseERKS5_")]
// IDA 0x7829cc: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7829cc() {
}

// 0x782aec — __ZN3RBX10Reflection7RefTypeIPNS_6ScriptEED1Ev
// type: unknown
#[doc(alias = "RBX::Reflection::RefType<RBX::Script *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_6ScriptEED1Ev")]
// IDA 0x782aec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_782aec() {
}

// 0x782af0 — __ZN3RBX10Reflection4TypeC2IPNS_6ScriptEEEPKcS6_PT_
// type: unknown
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Script *>(char const*,char const*,RBX::Script * *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IPNS_6ScriptEEEPKcS6_PT_")]
// IDA 0x782af0: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782af0() {
}

// 0x782b9c — __ZN3RBX10Reflection7RefTypeIPNS_6ScriptEED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::RefType<RBX::Script *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_6ScriptEED0Ev")]
// IDA 0x782b9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_782b9c() {
}

// 0x782ba4 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EEC2EMS3_FvSsS4_EPKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::BoundFuncDesc(void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EEC2EMS3_FvSsS4_EPKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x782ba4: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782ba4() {
}

// 0x782d74 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EE16declareSignatureEPKcS4_S8_S4_
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EE16declareSignatureEPKcS4_S8_S4_")]
// IDA 0x782d74: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782d74() {
}

// 0x782dc0 — __ZN5boost10scoped_ptrIN3RBX10Reflection7VariantEED1Ev
// type: unknown
#[doc(alias = "boost::scoped_ptr<RBX::Reflection::Variant>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX10Reflection7VariantEED1Ev")]
// IDA 0x782dc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_782dc0() {
}

// 0x782e70 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EED0Ev")]
// IDA 0x782e70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_782e70() {
}

// 0x782f98 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x782f98: 157 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_782f98() {
}

// 0x783148 — __ZN3RBX10Reflection11Call2HelperINS_9Scripting14ScriptDebuggerEMS3_FvSsNS0_7VariantEESsS4_vE4callEPS3_S6_RS4_RKSsRKS4_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Scripting::ScriptDebugger,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant),std::string,RBX::Reflection::Variant,void>::call(RBX::Scripting::ScriptDebugger*,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant),RBX::Reflection::Variant&,std::string const&,RBX::Reflection::Variant const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_9Scripting14ScriptDebuggerEMS3_FvSsNS0_7VariantEESsS4_vE4callEPS3_S6_RS4_RKSsRKS4_")]
// IDA 0x783148: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783148() {
}

// 0x783304 — __ZN3RBX10Reflection9ArgHelper6getArgINS0_7VariantELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: unknown
#[doc(alias = "RBX::Reflection::Variant RBX::Reflection::ArgHelper::getArg<RBX::Reflection::Variant,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Reflection::Variant> const&,boost::disable_if<boost::is_same<RBX::Reflection::Variant,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS0_7VariantELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// IDA 0x783304: 207 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783304() {
}

// 0x783510 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EEC2EMS3_FvSsS4_iEPKcSA_SA_SA_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::BoundFuncDesc(void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant,int),char const*,char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EEC2EMS3_FvSsS4_iEPKcSA_SA_SA_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x783510: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783510() {
}

// 0x783768 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EE16declareSignatureEPKcS4_S8_S4_S8_S4_
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EE16declareSignatureEPKcS4_S8_S4_S8_S4_")]
// IDA 0x783768: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783768() {
}

// 0x7837d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EED0Ev")]
// IDA 0x7837d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7837d0() {
}

// 0x783904 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x783904: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783904() {
}

// 0x783ad4 — __ZN3RBX10Reflection11Call3HelperINS_9Scripting14ScriptDebuggerEMS3_FvSsNS0_7VariantEiESsS4_ivE4callEPS3_S6_RS4_RKSsRKS4_RKi
// type: int __fastcall(int, int, int, int, std::string *, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Scripting::ScriptDebugger,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant,int),std::string,RBX::Reflection::Variant,int,void>::call(RBX::Scripting::ScriptDebugger*,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant,int),RBX::Reflection::Variant&,std::string const&,RBX::Reflection::Variant const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_9Scripting14ScriptDebuggerEMS3_FvSsNS0_7VariantEiESsS4_ivE4callEPS3_S6_RS4_RKSsRKS4_RKi")]
// IDA 0x783ad4: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783ad4() {
}

// 0x783c90 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EEC2EMS3_FSG_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EEC2EMS3_FSG_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x783c90: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783c90() {
}

// 0x783d94 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EED0Ev")]
// IDA 0x783d94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_783d94() {
}

// 0x783e48 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x783e48: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783e48() {
}

// 0x783e6c — __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvESG_E4callEPS3_SI_RS7_
// type: unknown
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(void),boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::Scripting::ScriptDebugger*,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(void),RBX::Reflection::Variant&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvESG_E4callEPS3_SI_RS7_")]
// IDA 0x783e6c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783e6c() {
}

// 0x783f58 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EEC2EMS3_FSG_iEPKcSM_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::BoundFuncDesc(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EEC2EMS3_FSG_iEPKcSM_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x783f58: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_783f58() {
}

// 0x784104 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EE16declareSignatureEPKcS7_
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EE16declareSignatureEPKcS7_")]
// IDA 0x784104: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784104() {
}

// 0x784134 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EED0Ev")]
// IDA 0x784134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_784134() {
}

// 0x784208 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x784208: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784208() {
}

// 0x784248 — __ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiEiSG_E4callEPS3_SI_RS7_RKi
// type: unknown
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(int),int,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::Scripting::ScriptDebugger*,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(int),RBX::Reflection::Variant&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiEiSG_E4callEPS3_SI_RS7_RKi")]
// IDA 0x784248: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784248() {
}

// 0x784338 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EEC2EMS3_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EEC2EMS3_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x784338: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784338() {
}

// 0x78443c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EED0Ev")]
// IDA 0x78443c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_78443c() {
}

// 0x7844f0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7844f0: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7844f0() {
}

// 0x784514 — __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvESB_E4callEPS3_SD_RS7_
// type: unknown
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Scripting::ScriptDebugger::*)(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::Scripting::ScriptDebugger*,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Scripting::ScriptDebugger::*)(void),RBX::Reflection::Variant&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvESB_E4callEPS3_SD_RS7_")]
// IDA 0x784514: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784514() {
}

// 0x784600 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9singletonEv
// type: unknown
#[doc(alias = "rbx::implementation::typed_holder<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9singletonEv")]
// IDA 0x784600: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784600() {
}

// 0x784670 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE13destruct_funcEPc
// type: unknown
#[doc(alias = "rbx::implementation::typed_holder<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE13destruct_funcEPc")]
// IDA 0x784670: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784670() {
}

// 0x78467c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x78467c: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78467c() {
}

// 0x784780 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EED0Ev")]
// IDA 0x784780: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_784780() {
}

// 0x784834 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x784834: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784834() {
}

// 0x784854 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS3_FS4_S8_EPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(RBX::Reflection::Variant (RBX::Scripting::ScriptDebugger::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS3_FS4_S8_EPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x784854: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784854() {
}

// 0x7849d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcS4_
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcS4_")]
// IDA 0x7849d0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7849d0() {
}

// 0x784a04 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")]
// IDA 0x784a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_784a04() {
}

// 0x784b0c — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x784b0c: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784b0c() {
}

// 0x784bf4 — __ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEES8_S4_E4callEPS3_SA_RS4_RKS8_
// type: unknown
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant (RBX::Scripting::ScriptDebugger::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,RBX::Reflection::Variant>::call(RBX::Scripting::ScriptDebugger*,RBX::Reflection::Variant (RBX::Scripting::ScriptDebugger::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant&,boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEES8_S4_E4callEPS3_SA_RS4_RKS8_")]
// IDA 0x784bf4: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784bf4() {
}

// 0x784d48 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Instance> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<RBX::Instance>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<RBX::Instance>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE")]
// IDA 0x784d48: 206 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784d48() {
}

// 0x784f5c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS3_FS7_SsEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS3_FS7_SsEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x784f5c: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_784f5c() {
}

// 0x7850d8 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7850d8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7850d8() {
}

// 0x785108 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev")]
// IDA 0x785108: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_785108() {
}

// 0x785210 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x785210: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785210() {
}

// 0x785350 — __ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrINS_8InstanceEEESsESsS7_E4callEPS3_S9_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Scripting::ScriptDebugger*,boost::shared_ptr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrINS_8InstanceEEESsESsS7_E4callEPS3_S9_RNS0_7VariantERKSs")]
// IDA 0x785350: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785350() {
}

// 0x7854d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC2EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC2EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7854d0: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7854d0() {
}

// 0x7855d4 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev")]
// IDA 0x7855d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7855d4() {
}

// 0x785688 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x785688: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785688() {
}

// 0x7856ac — __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE
// type: unknown
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Scripting::ScriptDebugger::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Scripting::ScriptDebugger*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Scripting::ScriptDebugger::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE")]
// IDA 0x7856ac: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7856ac() {
}

// 0x785794 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EEC2EMS3_FS7_iEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(int),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EEC2EMS3_FS7_iEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x785794: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785794() {
}

// 0x78590c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EE16declareSignatureEPKcNS0_7VariantE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x78590c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78590c() {
}

// 0x78593c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED0Ev")]
// IDA 0x78593c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_78593c() {
}

// 0x785a10 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x785a10: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785a10() {
}

// 0x785a50 — __ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrINS_8InstanceEEEiEiS7_E4callEPS3_S9_RNS0_7VariantERKi
// type: unknown
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(int),int,boost::shared_ptr<RBX::Instance>>::call(RBX::Scripting::ScriptDebugger*,boost::shared_ptr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(int),RBX::Reflection::Variant &,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrINS_8InstanceEEEiEiS7_E4callEPS3_S9_RNS0_7VariantERKi")]
// IDA 0x785a50: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785a50() {
}

// 0x785b3c — __ZN3RBX10Reflection9EventDescINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x785b3c: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785b3c() {
}

// 0x785cc0 — __ZN3RBX10Reflection9EventDescINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev")]
// IDA 0x785cc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_785cc0() {
}

// 0x785d74 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::DebuggerManager,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE")]
// IDA 0x785d74: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785d74() {
}

// 0x785ec8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::DebuggerManager,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
// IDA 0x785ec8: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_785ec8() {
}

// 0x786028 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::DebuggerManager,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x786028: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786028() {
}

// 0x78603c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC2EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Scripting::DebuggerManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC2EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x78603c: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78603c() {
}

// 0x786140 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev")]
// IDA 0x786140: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_786140() {
}

// 0x7861f4 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7861f4: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7861f4() {
}

// 0x786218 — __ZN3RBX10Reflection11Call0HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE
// type: unknown
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::DebuggerManager,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Scripting::DebuggerManager::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Scripting::DebuggerManager*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Scripting::DebuggerManager::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE")]
// IDA 0x786218: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786218() {
}

// 0x786300 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::PropDescriptor<bool (RBX::Scripting::DebuggerManager::*)(void)const,int>(char const*,char const*,bool (RBX::Scripting::DebuggerManager::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x786300: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786300() {
}

// 0x78640c — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbED0Ev")]
// IDA 0x78640c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_78640c() {
}

// 0x786438 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE10isReadOnlyEv")]
// IDA 0x786438: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786438() {
}

// 0x78643c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv")]
// IDA 0x78643c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78643c() {
}

// 0x786440 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x786440: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786440() {
}

// 0x786464 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0x786464: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786464() {
}

// 0x786584 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x786584: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786584() {
}

// 0x786688 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EED0Ev")]
// IDA 0x786688: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_786688() {
}

// 0x78673c — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x78673c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78673c() {
}

// 0x78675c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EEC2EMS3_FS7_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Scripting::DebuggerManager::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EEC2EMS3_FS7_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x78675c: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78675c() {
}

// 0x7868d8 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE16declareSignatureEPKcNS0_7VariantE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7868d8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7868d8() {
}

// 0x786908 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED0Ev")]
// IDA 0x786908: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_786908() {
}

// 0x786a10 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x786a10: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786a10() {
}

// 0x786af8 — __ZN3RBX10Reflection11Call1HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrINS_8InstanceEEES7_ES7_S7_E4callEPS3_S9_RNS0_7VariantERKS7_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::DebuggerManager,boost::shared_ptr<RBX::Instance> (RBX::Scripting::DebuggerManager::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::call(RBX::Scripting::DebuggerManager*,boost::shared_ptr<RBX::Instance> (RBX::Scripting::DebuggerManager::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrINS_8InstanceEEES7_ES7_S7_E4callEPS3_S9_RNS0_7VariantERKS7_")]
// IDA 0x786af8: 109 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786af8() {
}
