//! rendering shard 380 — 100 stubs 0x5540e0..0x55a800 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 41260->41360 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5540e0..0x55a800 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5540e0 — __ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev
// IDA 0x5540e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5540e0() {
}

// 0x55410c — __ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev
// IDA 0x55410c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55410c() {
}

// 0x5541e0 — __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_
#[doc(alias = "__ZN5boost9function2IvSsSsE13assign_to_ownERKS1_")]
#[doc(alias = "boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)")]
// was: __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_
// IDA 0x5541e0: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5541e0() {
}

// 0x55483c — __ZN3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_PKNS_8InstanceE
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_PKNS_8InstanceE")]
#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::create<RBX::ScriptContext>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_PKNS_8InstanceE
// IDA 0x55483c: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55483c() {
}

// 0x554854 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(rbx_core::WeakPtr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x554854: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_554854() {
}

// 0x5548d0 — __ZN3RBX3Lua9ThreadRefC2EPNS0_6detail13LiveThreadRefE
#[doc(alias = "__ZN3RBX3Lua9ThreadRefC2EPNS0_6detail13LiveThreadRefE")]
#[doc(alias = "RBX::Lua::ThreadRef::ThreadRef(RBX::Lua::detail::LiveThreadRef *)")]
// was: __ZN3RBX3Lua9ThreadRefC2EPNS0_6detail13LiveThreadRefE
// IDA 0x5548d0: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5548d0() {
}

// 0x554a3c — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// IDA 0x554a3c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_554a3c() {
}

// 0x554a64 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// IDA 0x554a64: 15 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_554a64() {
}

// 0x554a8c — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE
#[doc(alias = "__ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE")]
#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)")]
// was: __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE
// IDA 0x554a8c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_554a8c() {
}

// 0x554ab4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x554ab4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_554ab4() {
}

// 0x554adc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x554adc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_554adc() {
}

// 0x554b04 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED2Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED2Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED2Ev
// IDA 0x554b04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_554b04() {
}

// 0x554c64 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsSsSsiNS_3Lua15WeakFunctionRefEELi5EED2Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsSsSsiNS_3Lua15WeakFunctionRefEELi5EED2Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsSsSsiNS_3Lua15WeakFunctionRefEELi5EED2Ev
// IDA 0x554c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_554c64() {
}

// 0x554da4 — __GLOBAL__I_a_208
#[doc(alias = "__GLOBAL__I_a_208")]
#[doc(alias = "global constructor keyed to_a_208")]
// was: __GLOBAL__I_a_208
// IDA 0x554da4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_554da4() {
}

// 0x555598 — __ZN3RBX18registerBodyMoversEv
// type: _DWORD __fastcall(RBX *__hidden this)
#[doc(alias = "__ZN3RBX18registerBodyMoversEv")]
#[doc(alias = "RBX::registerBodyMovers(void)")]
// was: __ZN3RBX18registerBodyMoversEv
// IDA 0x555598: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_555598() {
}

// 0x5555d8 — __ZN3RBX9BodyMoverC2EPKc
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, const char *)
#[doc(alias = "__ZN3RBX9BodyMoverC2EPKc")]
#[doc(alias = "RBX::BodyMover::BodyMover(char const*)")]
// was: __ZN3RBX9BodyMoverC2EPKc
// IDA 0x5555d8: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5555d8() {
}

// 0x555878 — __ZN3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZN3RBX9BodyMoverD0Ev")]
#[doc(alias = "RBX::BodyMover::~BodyMover()")]
// was: __ZN3RBX9BodyMoverD0Ev
// IDA 0x555878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555878() {
}

// 0x555918 — __ZN3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZN3RBX9BodyMoverD1Ev")]
#[doc(alias = "RBX::BodyMover::~BodyMover()")]
// was: __ZN3RBX9BodyMoverD1Ev
// IDA 0x555918: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_555918() {
}

// 0x55591c — __ZThn32_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9BodyMoverD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn32_N3RBX9BodyMoverD0Ev
// IDA 0x55591c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55591c() {
}

// 0x555924 — __ZThn36_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9BodyMoverD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn36_N3RBX9BodyMoverD0Ev
// IDA 0x555924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555924() {
}

// 0x55592c — __ZThn92_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn92_N3RBX9BodyMoverD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn92_N3RBX9BodyMoverD0Ev
// IDA 0x55592c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55592c() {
}

// 0x555934 — __ZThn124_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn124_N3RBX9BodyMoverD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn124_N3RBX9BodyMoverD0Ev
// IDA 0x555934: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555934() {
}

// 0x55593c — __ZThn244_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn244_N3RBX9BodyMoverD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn244_N3RBX9BodyMoverD0Ev
// IDA 0x55593c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55593c() {
}

// 0x555944 — __ZN3RBX9BodyMoverD2Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZN3RBX9BodyMoverD2Ev")]
#[doc(alias = "RBX::BodyMover::~BodyMover()")]
// was: __ZN3RBX9BodyMoverD2Ev
// IDA 0x555944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555944() {
}

// 0x555b68 — __ZThn32_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9BodyMoverD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn32_N3RBX9BodyMoverD1Ev
// IDA 0x555b68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b68() {
}

// 0x555b70 — __ZThn36_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9BodyMoverD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn36_N3RBX9BodyMoverD1Ev
// IDA 0x555b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b70() {
}

// 0x555b78 — __ZThn92_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn92_N3RBX9BodyMoverD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn92_N3RBX9BodyMoverD1Ev
// IDA 0x555b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b78() {
}

// 0x555b80 — __ZThn124_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn124_N3RBX9BodyMoverD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn124_N3RBX9BodyMoverD1Ev
// IDA 0x555b80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b80() {
}

// 0x555b88 — __ZThn244_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn244_N3RBX9BodyMoverD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
// was: __ZThn244_N3RBX9BodyMoverD1Ev
// IDA 0x555b88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b88() {
}

// 0x555b90 — __ZN3RBX9BodyMover12computeForceEb
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, bool)
#[doc(alias = "__ZN3RBX9BodyMover12computeForceEb")]
#[doc(alias = "RBX::BodyMover::computeForce(bool)")]
// was: __ZN3RBX9BodyMover12computeForceEb
// IDA 0x555b90: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_555b90() {
}

// 0x556034 — __ZThn244_N3RBX9BodyMover12computeForceEb
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, bool)
#[doc(alias = "__ZThn244_N3RBX9BodyMover12computeForceEb")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::computeForce(bool)")]
// was: __ZThn244_N3RBX9BodyMover12computeForceEb
// IDA 0x556034: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556034() {
}

// 0x556140 — __ZN3RBX9BodyMover9stepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZN3RBX9BodyMover9stepWorldEv")]
#[doc(alias = "RBX::BodyMover::stepWorld(void)")]
// was: __ZN3RBX9BodyMover9stepWorldEv
// IDA 0x556140: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556140() {
}

// 0x55627c — __ZThn92_N3RBX9BodyMover9stepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn92_N3RBX9BodyMover9stepWorldEv")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::stepWorld(void)")]
// was: __ZThn92_N3RBX9BodyMover9stepWorldEv
// IDA 0x55627c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55627c() {
}

// 0x556284 — __ZN3RBX9BodyMover13getEngineBodyEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZN3RBX9BodyMover13getEngineBodyEv")]
#[doc(alias = "RBX::BodyMover::getEngineBody(void)")]
// was: __ZN3RBX9BodyMover13getEngineBodyEv
// IDA 0x556284: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556284() {
}

// 0x556318 — __ZThn92_N3RBX9BodyMover13getEngineBodyEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn92_N3RBX9BodyMover13getEngineBodyEv")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::getEngineBody(void)")]
// was: __ZThn92_N3RBX9BodyMover13getEngineBodyEv
// IDA 0x556318: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556318() {
}

// 0x556320 — __ZN3RBX9BodyMover24duplicateBodyMoverExistsEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "__ZN3RBX9BodyMover24duplicateBodyMoverExistsEPNS_9PrimitiveES2_")]
#[doc(alias = "RBX::BodyMover::duplicateBodyMoverExists(RBX::Primitive *,RBX::Primitive *)")]
// was: __ZN3RBX9BodyMover24duplicateBodyMoverExistsEPNS_9PrimitiveES2_
// IDA 0x556320: 30 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556320() {
}

// 0x556368 — __ZN3RBX9BodyMover17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBX9BodyMover17onAncestorChangedERKNS_15AncestorChangedE")]
#[doc(alias = "RBX::BodyMover::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX9BodyMover17onAncestorChangedERKNS_15AncestorChangedE
// IDA 0x556368: 372 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556368() {
}

// 0x556780 — __ZNK3RBX9BodyMover12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX9BodyMover12askSetParentEPKNS_8InstanceE")]
#[doc(alias = "RBX::BodyMover::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX9BodyMover12askSetParentEPKNS_8InstanceE
// IDA 0x556780: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556780() {
}

// 0x5567bc — __ZN3RBX6Rocket9setTargetEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Rocket *__hidden this, RBX::PartInstance *)
#[doc(alias = "__ZN3RBX6Rocket9setTargetEPNS_12PartInstanceE")]
#[doc(alias = "RBX::Rocket::setTarget(RBX::PartInstance *)")]
// was: __ZN3RBX6Rocket9setTargetEPNS_12PartInstanceE
// IDA 0x5567bc: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5567bc() {
}

// 0x5568b0 — __ZN3RBX6Rocket4fireEv
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZN3RBX6Rocket4fireEv")]
#[doc(alias = "RBX::Rocket::fire(void)")]
// was: __ZN3RBX6Rocket4fireEv
// IDA 0x5568b0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5568b0() {
}

// 0x5568dc — __ZN3RBX6Rocket5abortEv
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZN3RBX6Rocket5abortEv")]
#[doc(alias = "RBX::Rocket::abort(void)")]
// was: __ZN3RBX6Rocket5abortEv
// IDA 0x5568dc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5568dc() {
}

// 0x55690c — __ZN3RBX6RocketC2Ev
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZN3RBX6RocketC2Ev")]
#[doc(alias = "RBX::Rocket::Rocket(void)")]
// was: __ZN3RBX6RocketC2Ev
// IDA 0x55690c: 225 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55690c() {
}

// 0x556bb0 — __ZN3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZN3RBX6RocketD0Ev")]
#[doc(alias = "RBX::Rocket::~Rocket()")]
// was: __ZN3RBX6RocketD0Ev
// IDA 0x556bb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556bb0() {
}

// 0x556c50 — __ZN3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZN3RBX6RocketD1Ev")]
#[doc(alias = "RBX::Rocket::~Rocket()")]
// was: __ZN3RBX6RocketD1Ev
// IDA 0x556c50: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_556c50() {
}

// 0x556c54 — __ZThn32_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn32_N3RBX6RocketD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn32_N3RBX6RocketD0Ev
// IDA 0x556c54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c54() {
}

// 0x556c5c — __ZThn36_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn36_N3RBX6RocketD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn36_N3RBX6RocketD0Ev
// IDA 0x556c5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c5c() {
}

// 0x556c64 — __ZThn92_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn92_N3RBX6RocketD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn92_N3RBX6RocketD0Ev
// IDA 0x556c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c64() {
}

// 0x556c6c — __ZThn124_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn124_N3RBX6RocketD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn124_N3RBX6RocketD0Ev
// IDA 0x556c6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c6c() {
}

// 0x556c74 — __ZThn244_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn244_N3RBX6RocketD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn244_N3RBX6RocketD0Ev
// IDA 0x556c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c74() {
}

// 0x556c7c — __ZThn304_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn304_N3RBX6RocketD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn304_N3RBX6RocketD0Ev
// IDA 0x556c7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c7c() {
}

// 0x556c84 — __ZN3RBX6RocketD2Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZN3RBX6RocketD2Ev")]
#[doc(alias = "RBX::Rocket::~Rocket()")]
// was: __ZN3RBX6RocketD2Ev
// IDA 0x556c84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c84() {
}

// 0x556e1c — __ZThn32_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn32_N3RBX6RocketD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn32_N3RBX6RocketD1Ev
// IDA 0x556e1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e1c() {
}

// 0x556e24 — __ZThn36_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn36_N3RBX6RocketD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn36_N3RBX6RocketD1Ev
// IDA 0x556e24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e24() {
}

// 0x556e2c — __ZThn92_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn92_N3RBX6RocketD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn92_N3RBX6RocketD1Ev
// IDA 0x556e2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e2c() {
}

// 0x556e34 — __ZThn124_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn124_N3RBX6RocketD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn124_N3RBX6RocketD1Ev
// IDA 0x556e34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e34() {
}

// 0x556e3c — __ZThn244_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn244_N3RBX6RocketD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn244_N3RBX6RocketD1Ev
// IDA 0x556e3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e3c() {
}

// 0x556e44 — __ZThn304_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZThn304_N3RBX6RocketD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
// was: __ZThn304_N3RBX6RocketD1Ev
// IDA 0x556e44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e44() {
}

// 0x556e4c — __ZN3RBX6Rocket9onSteppedERKNS_7SteppedE
#[doc(alias = "__ZN3RBX6Rocket9onSteppedERKNS_7SteppedE")]
#[doc(alias = "RBX::Rocket::onStepped(RBX::Stepped const&)")]
// was: __ZN3RBX6Rocket9onSteppedERKNS_7SteppedE
// IDA 0x556e4c: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556e4c() {
}

// 0x55705c — __ZThn304_N3RBX6Rocket9onSteppedERKNS_7SteppedE
#[doc(alias = "__ZThn304_N3RBX6Rocket9onSteppedERKNS_7SteppedE")]
#[doc(alias = "non-virtual thunk toRBX::Rocket::onStepped(RBX::Stepped const&)")]
// was: __ZThn304_N3RBX6Rocket9onSteppedERKNS_7SteppedE
// IDA 0x55705c: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55705c() {
}

// 0x5578a0 — __ZN3RBX8BodyGyroC2Ev
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZN3RBX8BodyGyroC2Ev")]
#[doc(alias = "RBX::BodyGyro::BodyGyro(void)")]
// was: __ZN3RBX8BodyGyroC2Ev
// IDA 0x5578a0: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5578a0() {
}

// 0x557c50 — __ZN3RBX8BodyGyro20computeBalanceTorqueEPNS_4BodyES2_
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this, RBX::Body *, RBX::Body *)
#[doc(alias = "__ZN3RBX8BodyGyro20computeBalanceTorqueEPNS_4BodyES2_")]
#[doc(alias = "RBX::BodyGyro::computeBalanceTorque(RBX::Body *,RBX::Body *)")]
// was: __ZN3RBX8BodyGyro20computeBalanceTorqueEPNS_4BodyES2_
// IDA 0x557c50: 274 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_557c50() {
}

// 0x557ff8 — __ZN3RBX8BodyGyro24computeOrientationTorqueEPNS_4BodyES2_
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this, RBX::Body *, RBX::Body *)
#[doc(alias = "__ZN3RBX8BodyGyro24computeOrientationTorqueEPNS_4BodyES2_")]
#[doc(alias = "RBX::BodyGyro::computeOrientationTorque(RBX::Body *,RBX::Body *)")]
// was: __ZN3RBX8BodyGyro24computeOrientationTorqueEPNS_4BodyES2_
// IDA 0x557ff8: 206 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_557ff8() {
}

// 0x5582bc — __ZN3RBX12BodyPositionC2Ev
// type: _DWORD __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZN3RBX12BodyPositionC2Ev")]
#[doc(alias = "RBX::BodyPosition::BodyPosition(void)")]
// was: __ZN3RBX12BodyPositionC2Ev
// IDA 0x5582bc: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5582bc() {
}

// 0x558780 — __ZN3RBX12BodyPosition9onSteppedERKNS_7SteppedE
#[doc(alias = "__ZN3RBX12BodyPosition9onSteppedERKNS_7SteppedE")]
#[doc(alias = "RBX::BodyPosition::onStepped(RBX::Stepped const&)")]
// was: __ZN3RBX12BodyPosition9onSteppedERKNS_7SteppedE
// IDA 0x558780: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558780() {
}

// 0x5588ec — __ZThn304_N3RBX12BodyPosition9onSteppedERKNS_7SteppedE
#[doc(alias = "__ZThn304_N3RBX12BodyPosition9onSteppedERKNS_7SteppedE")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::onStepped(RBX::Stepped const&)")]
// was: __ZThn304_N3RBX12BodyPosition9onSteppedERKNS_7SteppedE
// IDA 0x5588ec: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5588ec() {
}

// 0x5588f8 — __ZN3RBX12BodyVelocityC2Ev
// type: _DWORD __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZN3RBX12BodyVelocityC2Ev")]
#[doc(alias = "RBX::BodyVelocity::BodyVelocity(void)")]
// was: __ZN3RBX12BodyVelocityC2Ev
// IDA 0x5588f8: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5588f8() {
}

// 0x558c34 — __ZN3RBX19BodyAngularVelocityC2Ev
// type: _DWORD __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZN3RBX19BodyAngularVelocityC2Ev")]
#[doc(alias = "RBX::BodyAngularVelocity::BodyAngularVelocity(void)")]
// was: __ZN3RBX19BodyAngularVelocityC2Ev
// IDA 0x558c34: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558c34() {
}

// 0x558f70 — __ZN3RBX9BodyForceC2Ev
// type: _DWORD __fastcall(RBX::BodyForce *__hidden this)
#[doc(alias = "__ZN3RBX9BodyForceC2Ev")]
#[doc(alias = "RBX::BodyForce::BodyForce(void)")]
// was: __ZN3RBX9BodyForceC2Ev
// IDA 0x558f70: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558f70() {
}

// 0x559124 — __ZN3RBX10BodyThrustC2Ev
// type: _DWORD __fastcall(RBX::BodyThrust *__hidden this)
#[doc(alias = "__ZN3RBX10BodyThrustC2Ev")]
#[doc(alias = "RBX::BodyThrust::BodyThrust(void)")]
// was: __ZN3RBX10BodyThrustC2Ev
// IDA 0x559124: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559124() {
}

// 0x559440 — __ZNK3RBX6Rocket18getTargetDangerousEv
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "__ZNK3RBX6Rocket18getTargetDangerousEv")]
#[doc(alias = "RBX::Rocket::getTargetDangerous(void)const")]
// was: __ZNK3RBX6Rocket18getTargetDangerousEv
// IDA 0x559440: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559440() {
}

// 0x559448 — __ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED1Ev
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED1Ev")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED1Ev
// IDA 0x559448: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559448() {
}

// 0x5594a0 — __ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev
// IDA 0x5594a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5594a0() {
}

// 0x5594c4 — __ZNK3RBX4Body14getBranchForceEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "__ZNK3RBX4Body14getBranchForceEv")]
#[doc(alias = "RBX::Body::getBranchForce(void)const")]
// was: __ZNK3RBX4Body14getBranchForceEv
// IDA 0x5594c4: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5594c4() {
}

// 0x559534 — __ZNK3RBX4Body15getBranchTorqueEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "__ZNK3RBX4Body15getBranchTorqueEv")]
#[doc(alias = "RBX::Body::getBranchTorque(void)const")]
// was: __ZNK3RBX4Body15getBranchTorqueEv
// IDA 0x559534: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559534() {
}

// 0x5595ac — __ZN3RBX12BodyPosition12getLastForceEv
// type: _DWORD __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZN3RBX12BodyPosition12getLastForceEv")]
#[doc(alias = "RBX::BodyPosition::getLastForce(void)")]
// was: __ZN3RBX12BodyPosition12getLastForceEv
// IDA 0x5595ac: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5595ac() {
}

// 0x5595e0 — __ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev
// IDA 0x5595e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5595e0() {
}

// 0x559604 — __ZN3RBX12BodyVelocity12getLastForceEv
// type: _DWORD __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZN3RBX12BodyVelocity12getLastForceEv")]
#[doc(alias = "RBX::BodyVelocity::getLastForce(void)")]
// was: __ZN3RBX12BodyVelocity12getLastForceEv
// IDA 0x559604: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559604() {
}

// 0x559638 — __ZN3RBX4Body17getBranchVelocityEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "__ZN3RBX4Body17getBranchVelocityEv")]
#[doc(alias = "RBX::Body::getBranchVelocity(void)")]
// was: __ZN3RBX4Body17getBranchVelocityEv
// IDA 0x559638: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559638() {
}

// 0x5596b0 — __ZN3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZN3RBX12BodyPositionD1Ev")]
#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
// was: __ZN3RBX12BodyPositionD1Ev
// IDA 0x5596b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5596b0() {
}

// 0x5597e0 — __ZN3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZN3RBX12BodyPositionD0Ev")]
#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
// was: __ZN3RBX12BodyPositionD0Ev
// IDA 0x5597e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5597e0() {
}

// 0x559920 — __ZN3RBX12BodyPosition17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::BodyPosition *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX12BodyPosition17onServiceProviderEPNS_15ServiceProviderES2_")]
#[doc(alias = "RBX::BodyPosition::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX12BodyPosition17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x559920: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559920() {
}

// 0x559928 — __ZNK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv
// IDA 0x559928: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559928() {
}

// 0x559938 — __ZNK3RBX9BodyMover12canStepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZNK3RBX9BodyMover12canStepWorldEv")]
#[doc(alias = "RBX::BodyMover::canStepWorld(void)const")]
// was: __ZNK3RBX9BodyMover12canStepWorldEv
// IDA 0x559938: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559938() {
}

// 0x55993c — __ZThn32_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BodyPositionD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn32_N3RBX12BodyPositionD1Ev
// IDA 0x55993c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55993c() {
}

// 0x559a68 — __ZThn32_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BodyPositionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn32_N3RBX12BodyPositionD0Ev
// IDA 0x559a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559a68() {
}

// 0x559ba8 — __ZThn32_NK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv
// IDA 0x559ba8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559ba8() {
}

// 0x559bb8 — __ZThn36_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BodyPositionD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn36_N3RBX12BodyPositionD1Ev
// IDA 0x559bb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559bb8() {
}

// 0x559ce4 — __ZThn36_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BodyPositionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn36_N3RBX12BodyPositionD0Ev
// IDA 0x559ce4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559ce4() {
}

// 0x559e24 — __ZThn92_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12BodyPositionD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn92_N3RBX12BodyPositionD1Ev
// IDA 0x559e24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559e24() {
}

// 0x559f50 — __ZThn92_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12BodyPositionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn92_N3RBX12BodyPositionD0Ev
// IDA 0x559f50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559f50() {
}

// 0x55a090 — __ZThn92_NK3RBX9BodyMover12canStepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "__ZThn92_NK3RBX9BodyMover12canStepWorldEv")]
#[doc(alias = "non-virtual thunk toRBX::BodyMover::canStepWorld(void)const")]
// was: __ZThn92_NK3RBX9BodyMover12canStepWorldEv
// IDA 0x55a090: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a090() {
}

// 0x55a094 — __ZThn124_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn124_N3RBX12BodyPositionD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn124_N3RBX12BodyPositionD1Ev
// IDA 0x55a094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a094() {
}

// 0x55a1c0 — __ZThn124_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn124_N3RBX12BodyPositionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn124_N3RBX12BodyPositionD0Ev
// IDA 0x55a1c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a1c0() {
}

// 0x55a300 — __ZThn244_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn244_N3RBX12BodyPositionD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn244_N3RBX12BodyPositionD1Ev
// IDA 0x55a300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a300() {
}

// 0x55a430 — __ZThn244_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn244_N3RBX12BodyPositionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn244_N3RBX12BodyPositionD0Ev
// IDA 0x55a430: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a430() {
}

// 0x55a574 — __ZThn304_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn304_N3RBX12BodyPositionD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn304_N3RBX12BodyPositionD1Ev
// IDA 0x55a574: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a574() {
}

// 0x55a6a4 — __ZThn304_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "__ZThn304_N3RBX12BodyPositionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
// was: __ZThn304_N3RBX12BodyPositionD0Ev
// IDA 0x55a6a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a6a4() {
}

// 0x55a7e8 — __ZN3RBX6Rocket17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::Rocket *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX6Rocket17onServiceProviderEPNS_15ServiceProviderES2_")]
#[doc(alias = "RBX::Rocket::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX6Rocket17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x55a7e8: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a7e8() {
}

// 0x55a7f0 — __ZNK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv
// IDA 0x55a7f0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a7f0() {
}

// 0x55a800 — __ZThn32_NK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv
// IDA 0x55a800: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a800() {
}
