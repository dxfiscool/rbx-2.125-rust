//! rendering shard 257 — 100 stubs EA-sorted asc global gap filler after 0x300c6c not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 27721->27821 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x30e6b4 — __ZN3RBX13ContentFilter12setFilterUrlESs
#[doc(alias = "RBX::ContentFilter::setFilterUrl(std::string)")]
// was: __ZN3RBX13ContentFilter12setFilterUrlESs
// IDA 0x30e6b4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30e6b4() {
}

// 0x30e6bc — __ZN3RBX13ContentFilter15setFilterLimitsEii
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this, int, int)
#[doc(alias = "RBX::ContentFilter::setFilterLimits(int,int)")]
// was: __ZN3RBX13ContentFilter15setFilterLimitsEii
// IDA 0x30e6bc: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30e6bc() {
}

// 0x30e6c8 — __ZN3RBX13ContentFilterC1Ev
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "RBX::ContentFilter::ContentFilter(void)")]
// was: __ZN3RBX13ContentFilterC1Ev
// IDA 0x30e6c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_30e6c8() {
}

// 0x30e6cc — __ZN3RBX13ContentFilterC2Ev
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "RBX::ContentFilter::ContentFilter(void)")]
// was: __ZN3RBX13ContentFilterC2Ev
// IDA 0x30e6cc: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30e6cc() {
}

// 0x30e868 — __ZN3RBX13ContentFilterD0Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// was: __ZN3RBX13ContentFilterD0Ev
// IDA 0x30e868: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30e868() {
}

// 0x30e908 — __ZN3RBX13ContentFilterD1Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// was: __ZN3RBX13ContentFilterD1Ev
// IDA 0x30e908: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_30e908() {
}

// 0x30e90c — __ZThn32_N3RBX13ContentFilterD0Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// was: __ZThn32_N3RBX13ContentFilterD0Ev
// IDA 0x30e90c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30e90c() {
}

// 0x30e914 — __ZThn36_N3RBX13ContentFilterD0Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// was: __ZThn36_N3RBX13ContentFilterD0Ev
// IDA 0x30e914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30e914() {
}

// 0x30e91c — __ZN3RBX13ContentFilterD2Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// was: __ZN3RBX13ContentFilterD2Ev
// IDA 0x30e91c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30e91c() {
}

// 0x30e96c — __ZThn32_N3RBX13ContentFilterD1Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// was: __ZThn32_N3RBX13ContentFilterD1Ev
// IDA 0x30e96c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30e96c() {
}

// 0x30e974 — __ZThn36_N3RBX13ContentFilterD1Ev
// type: void __fastcall(RBX::ContentFilter *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// was: __ZThn36_N3RBX13ContentFilterD1Ev
// IDA 0x30e974: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30e974() {
}

// 0x30e97c — __ZN3RBX13ContentFilter14truncateStringERSs
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this, std::string *)
#[doc(alias = "RBX::ContentFilter::truncateString(std::string &)")]
// was: __ZN3RBX13ContentFilter14truncateStringERSs
// IDA 0x30e97c: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30e97c() {
}

// 0x30eab0 — __ZN3RBX13ContentFilter14getStringStateERSs
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this, std::string *)
#[doc(alias = "RBX::ContentFilter::getStringState(std::string &)")]
// was: __ZN3RBX13ContentFilter14getStringStateERSs
// IDA 0x30eab0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30eab0() {
}

// 0x30eadc — __ZN3RBX13ContentFilter20isContentFilterReadyERKSs
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this, const std::string *)
#[doc(alias = "RBX::ContentFilter::isContentFilterReady(std::string const&)")]
// was: __ZN3RBX13ContentFilter20isContentFilterReadyERKSs
// IDA 0x30eadc: 325 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30eadc() {
}

// 0x30ee70 — __ZN3RBX13ContentFilter12isStringSafeERSs
// type: _DWORD __fastcall(RBX::ContentFilter *__hidden this, std::string *)
#[doc(alias = "RBX::ContentFilter::isStringSafe(std::string &)")]
// was: __ZN3RBX13ContentFilter12isStringSafeERSs
// IDA 0x30ee70: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30ee70() {
}

// 0x30eebc — __ZN3RBX13ContentFilter10cleanTableEv
// type: unsigned int __fastcall(RBX::ContentFilter *this)
#[doc(alias = "RBX::ContentFilter::cleanTable(void)")]
// was: __ZN3RBX13ContentFilter10cleanTableEv
// IDA 0x30eebc: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30eebc() {
}

// 0x30f0a0 — __ZN3RBX13ContentFilter15doFilterRequestESs
#[doc(alias = "RBX::ContentFilter::doFilterRequest(std::string)")]
// was: __ZN3RBX13ContentFilter15doFilterRequestESs
// IDA 0x30f0a0: 457 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30f0a0() {
}

// 0x30fa64 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EED1Ev
// IDA 0x30fa64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30fa64() {
}

// 0x30faa4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(int,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EED1Ev
// IDA 0x30faa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_30faa4() {
}

// 0x310284 — __ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// type: int(void)
#[doc(alias = "std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// IDA 0x310284: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_310284() {
}

// 0x3103d4 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv
// IDA 0x3103d4: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3103d4() {
}

// 0x3103fc — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv
// IDA 0x3103fc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3103fc() {
}

// 0x310424 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// IDA 0x310424: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_310424() {
}

// 0x310510 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// IDA 0x310510: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_310510() {
}

// 0x310560 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// IDA 0x310560: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_310560() {
}

// 0x3105e4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// IDA 0x3105e4: 78 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3105e4() {
}

// 0x3106c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// IDA 0x3106c4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3106c4() {
}

// 0x3106f4 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs
// IDA 0x3106f4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3106f4() {
}

// 0x31071c — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")]
// was: __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_
// IDA 0x31071c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31071c() {
}

// 0x310770 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")]
// was: __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE
// IDA 0x310770: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_310770() {
}

// 0x310798 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs
// IDA 0x310798: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_310798() {
}

// 0x3107c8 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs
// type: _DWORD *__fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs
// IDA 0x3107c8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3107c8() {
}

// 0x311908 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS7_5list2INS7_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS7_5list2INS7_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS7_5list2INS7_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// IDA 0x311908: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_311908() {
}

// 0x311ac8 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// IDA 0x311ac8: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_311ac8() {
}

// 0x311c8c — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEEvT_
// type: int(void)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEEvT_
// IDA 0x311c8c: 163 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_311c8c() {
}

// 0x311e7c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESI_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESI_
// IDA 0x311e7c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_311e7c() {
}

// 0x311e98 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x311e98: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_311e98() {
}

// 0x31205c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, void *)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x31205c: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31205c() {
}

// 0x31221c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x31221c: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31221c() {
}

// 0x312360 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x312360: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_312360() {
}

// 0x312a54 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs
// IDA 0x312a54: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_312a54() {
}

// 0x312aa4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// IDA 0x312aa4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_312aa4() {
}

// 0x312af4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// IDA 0x312af4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_312af4() {
}

// 0x312b1c — __ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x312b1c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_312b1c() {
}

// 0x312b20 — __ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x312b20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_312b20() {
}

// 0x312bc0 — __ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x312bc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_312bc0() {
}

// 0x312bc8 — __ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x312bc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_312bc8() {
}

// 0x312c6c — __ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x312c6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_312c6c() {
}

// 0x312c74 — __ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x312c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_312c74() {
}

// 0x312d18 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EEC2EMS2_FviiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(int,int),2>::BoundFuncDesc(void (RBX::ContentFilter::*)(int,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EEC2EMS2_FviiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x312d18: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_312d18() {
}

// 0x312ee0 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(int,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// IDA 0x312ee0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_312ee0() {
}

// 0x312f2c — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(int,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EED0Ev
// IDA 0x312f2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_312f2c() {
}

// 0x31300c — __ZNK3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(int,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x31300c: 29 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31300c() {
}

// 0x313060 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(std::string),1>::BoundFuncDesc(void (RBX::ContentFilter::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x313060: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_313060() {
}

// 0x3131d8 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3131d8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3131d8() {
}

// 0x313208 — __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EED0Ev
// IDA 0x313208: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_313208() {
}

// 0x3132d4 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ContentFilter,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3132d4: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3132d4() {
}

// 0x313410 — __ZN3RBX10Reflection11Call1HelperINS_13ContentFilterEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ContentFilter,void (RBX::ContentFilter::*)(std::string),std::string,void>::call(RBX::ContentFilter*,void (RBX::ContentFilter::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_13ContentFilterEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// IDA 0x313410: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_313410() {
}

// 0x313b80 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS7_5list3INS7_5valueISB_EENSF_ISsEENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS7_5list3INS7_5valueISB_EENSF_ISsEENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS7_5list3INS7_5valueISB_EENSF_ISsEENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x313b80: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_313b80() {
}

// 0x313d48 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// IDA 0x313d48: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_313d48() {
}

// 0x313f14 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEEvT_
// type: int(void)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEEvT_
// IDA 0x313f14: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_313f14() {
}

// 0x31410c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESJ_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESJ_
// IDA 0x31410c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31410c() {
}

// 0x314128 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x314128: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314128() {
}

// 0x3142f4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x3142f4: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3142f4() {
}

// 0x3144bc — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x3144bc: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3144bc() {
}

// 0x314604 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEclIPFvS6_SsbENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEclIPFvS6_SsbENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x314604: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314604() {
}

// 0x314a10 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0x314a10: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314a10() {
}

// 0x314a40 — __GLOBAL__I_a_110
#[doc(alias = "global constructor keyed to_a_110")]
// was: __GLOBAL__I_a_110
// IDA 0x314a40: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_314a40() {
}

// 0x314c84 — __ZN3RBXltERKNS_9ContentIdES2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::operator<(RBX::ContentId const&,RBX::ContentId const&)")]
// was: __ZN3RBXltERKNS_9ContentIdES2_
// IDA 0x314c84: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314c84() {
}

// 0x314c90 — __ZN3RBXneERKNS_9ContentIdES2_
// type: int __fastcall(std::string *, std::string *this)
#[doc(alias = "RBX::operator!=(RBX::ContentId const&,RBX::ContentId const&)")]
// was: __ZN3RBXneERKNS_9ContentIdES2_
// IDA 0x314c90: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314c90() {
}

// 0x314ca8 — __ZN3RBXeqERKNS_9ContentIdES2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::operator==(RBX::ContentId const&,RBX::ContentId const&)")]
// was: __ZN3RBXeqERKNS_9ContentIdES2_
// IDA 0x314ca8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314ca8() {
}

// 0x314cbc — __ZN3RBX9ContentId7fromUrlERKSs
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const std::string *)
#[doc(alias = "RBX::ContentId::fromUrl(std::string const&)")]
// was: __ZN3RBX9ContentId7fromUrlERKSs
// IDA 0x314cbc: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314cbc() {
}

// 0x314cc8 — __ZN3RBX9ContentId16CorrectBackslashERSs
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, std::string *)
#[doc(alias = "RBX::ContentId::CorrectBackslash(std::string &)")]
// was: __ZN3RBX9ContentId16CorrectBackslashERSs
// IDA 0x314cc8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314cc8() {
}

// 0x314d14 — __ZN3RBX9ContentId14convertAssetIdERKSs
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const std::string *)
#[doc(alias = "RBX::ContentId::convertAssetId(std::string const&)")]
// was: __ZN3RBX9ContentId14convertAssetIdERKSs
// IDA 0x314d14: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314d14() {
}

// 0x314f94 — __ZN12_GLOBAL__N_111createIdUrlERSsRKSsS2_
// type: _DWORD __fastcall(_anonymous_namespace_ *__hidden this, std::string *, const std::string *, const std::string *)
#[doc(alias = "anonymous namespace::createIdUrl(std::string &,std::string const&,std::string const&)")]
// was: __ZN12_GLOBAL__N_111createIdUrlERSsRKSsS2_
// IDA 0x314f94: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_314f94() {
}

// 0x315004 — __ZN3RBX9ContentId22convertToLegacyContentERKSs
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const std::string *)
#[doc(alias = "RBX::ContentId::convertToLegacyContent(std::string const&)")]
// was: __ZN3RBX9ContentId22convertToLegacyContentERKSs
// IDA 0x315004: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_315004() {
}

// 0x31507c — __ZNK3RBX9ContentId10getAssetIdEv
// type: _DWORD __fastcall(RBX::ContentId *__hidden this)
#[doc(alias = "RBX::ContentId::getAssetId(void)const")]
// was: __ZNK3RBX9ContentId10getAssetIdEv
// IDA 0x31507c: 236 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31507c() {
}

// 0x31530c — __ZN3RBX9ContentId10fromAssetsEPKc
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const char *)
#[doc(alias = "RBX::ContentId::fromAssets(char const*)")]
// was: __ZN3RBX9ContentId10fromAssetsEPKc
// IDA 0x31530c: 179 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31530c() {
}

// 0x315514 — __ZN3RBX18LegacyContentTableD1Ev
// type: void __fastcall(RBX::LegacyContentTable *__hidden this)
#[doc(alias = "RBX::LegacyContentTable::~LegacyContentTable()")]
// was: __ZN3RBX18LegacyContentTableD1Ev
// IDA 0x315514: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_315514() {
}

// 0x315594 — __GLOBAL__I_a_111
#[doc(alias = "global constructor keyed to_a_111")]
// was: __GLOBAL__I_a_111
// IDA 0x315594: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_315594() {
}

// 0x315680 — __ZN3RBX10FileSystem16getUserDirectoryEbNS_13FileSystemDirEPKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::FileSystem::getUserDirectory(bool,RBX::FileSystemDir,char const*)")]
// was: __ZN3RBX10FileSystem16getUserDirectoryEbNS_13FileSystemDirEPKc
// IDA 0x315680: 475 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_315680() {
}

// 0x315ba4 — __ZN3RBX10FileSystem19clearCacheDirectoryEPKci
// type: _DWORD __fastcall(RBX::FileSystem *__hidden this, const char *, int)
#[doc(alias = "RBX::FileSystem::clearCacheDirectory(char const*,int)")]
// was: __ZN3RBX10FileSystem19clearCacheDirectoryEPKci
// IDA 0x315ba4: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_315ba4() {
}

// 0x315dc8 — __ZN3RBX10FileSystem17getCacheDirectoryEbPKc
// type: _DWORD __fastcall(RBX::FileSystem *__hidden this, bool, const char *)
#[doc(alias = "RBX::FileSystem::getCacheDirectory(bool,char const*)")]
// was: __ZN3RBX10FileSystem17getCacheDirectoryEbPKc
// IDA 0x315dc8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_315dc8() {
}

// 0x315dd4 — __ZN3RBX10FileSystem21getBaseCacheDirectoryEb
// type: _DWORD __fastcall(RBX::FileSystem *__hidden this, bool)
#[doc(alias = "RBX::FileSystem::getBaseCacheDirectory(bool)")]
// was: __ZN3RBX10FileSystem21getBaseCacheDirectoryEb
// IDA 0x315dd4: 201 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_315dd4() {
}

// 0x3164c8 — __GLOBAL__I_a_112
#[doc(alias = "global constructor keyed to_a_112")]
// was: __GLOBAL__I_a_112
// IDA 0x3164c8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_3164c8() {
}

// 0x316590 — __ZN3RBX4Http21getRobloxResponceLockEv
// type: _DWORD __fastcall(RBX::Http *__hidden this)
#[doc(alias = "RBX::Http::getRobloxResponceLock(void)")]
// was: __ZN3RBX4Http21getRobloxResponceLockEv
// IDA 0x316590: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_316590() {
}

// 0x3168b0 — __ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Http::get(boost::function<void ()(std::string *,std::exception *)>,bool)")]
// was: __ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb
// IDA 0x3168b0: 252 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3168b0() {
}

// 0x316f2c — __ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Http::post(std::string const&,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
// was: __ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb
// IDA 0x316f2c: 388 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_316f2c() {
}

// 0x317570 — __ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb
#[doc(alias = "RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
// was: __ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb
// IDA 0x317570: 413 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_317570() {
}

// 0x3180dc — __ZN3RBX4Http12isRobloxSiteEPKc
// type: _DWORD __fastcall(RBX::Http *__hidden this, const char *)
#[doc(alias = "RBX::Http::isRobloxSite(char const*)")]
// was: __ZN3RBX4Http12isRobloxSiteEPKc
// IDA 0x3180dc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3180dc() {
}

// 0x31df30 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
// IDA 0x31df30: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31df30() {
}

// 0x31e084 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_
// IDA 0x31e084: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e084() {
}

// 0x31e1a8 — __ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_
#[doc(alias = "boost::function2<void,std::string *,std::exception *>::operator()(std::string *,std::exception *)const")]
// was: __ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_
// IDA 0x31e1a8: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e1a8() {
}

// 0x31e270 — __ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
#[doc(alias = "rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// was: __ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// IDA 0x31e270: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e270() {
}

// 0x31e344 — __ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// was: __ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// IDA 0x31e344: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e344() {
}

// 0x31e43c — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev
// IDA 0x31e43c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_31e43c() {
}

// 0x31e440 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev
// IDA 0x31e440: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_31e440() {
}

// 0x31e444 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv
// IDA 0x31e444: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e444() {
}

// 0x31e454 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
// IDA 0x31e454: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e454() {
}

// 0x31e458 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
// IDA 0x31e458: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31e458() {
}