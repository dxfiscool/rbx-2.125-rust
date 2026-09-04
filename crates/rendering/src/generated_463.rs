//! rendering shard 463 — 100 stubs 0x6fe994..0x703748 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (49911->50011 distinct, fallback after 0x6fe994).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x6fe994 — __ZN3RBX8Instance17removeAllChildrenEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::removeAllChildren(void)")]
#[doc(alias = "__ZN3RBX8Instance17removeAllChildrenEv")]
// was: __ZN3RBX8Instance17removeAllChildrenEv
// IDA 0x6fe994: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fe994() {
}

// 0x6fea84 — __ZN3RBX8Instance12waitForChildESsN5boost8functionIFvNS1_10shared_ptrIS0_EEEEENS2_IFvSsEEE
// type: int __fastcall(int, std::string *this, int, int)
#[doc(alias = "RBX::Instance::waitForChild(std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX8Instance12waitForChildESsN5boost8functionIFvNS1_10shared_ptrIS0_EEEEENS2_IFvSsEEE")]
// was: __ZN3RBX8Instance12waitForChildESsN5boost8functionIFvNS1_10shared_ptrIS0_EEEEENS2_IFvSsEEE
// IDA 0x6fea84: 343 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fea84() {
}

// 0x6fee38 — __ZN3RBX8Instance15setRobloxLockedEb
// type: _DWORD __fastcall(RBX::Instance *__hidden this, bool)
#[doc(alias = "RBX::Instance::setRobloxLocked(bool)")]
#[doc(alias = "__ZN3RBX8Instance15setRobloxLockedEb")]
// was: __ZN3RBX8Instance15setRobloxLockedEb
// IDA 0x6fee38: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fee38() {
}

// 0x6fee6c — __ZN3RBX8Instance11createChildERKNS_4NameENS_11CreatorRoleE
// type: 
#[doc(alias = "RBX::Instance::createChild(RBX::Name const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance11createChildERKNS_4NameENS_11CreatorRoleE")]
// was: __ZN3RBX8Instance11createChildERKNS_4NameENS_11CreatorRoleE
// IDA 0x6fee6c: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fee6c() {
}

// 0x6fee7c — __ZN3RBX8Instance9readChildEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
// type: int __fastcall(int, XmlElement *this, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Instance::readChild(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance9readChildEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE")]
// was: __ZN3RBX8Instance9readChildEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
// IDA 0x6fee7c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fee7c() {
}

// 0x6fefd0 — __ZN3RBX8Instance4readEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
// type: int __fastcall(int, XmlElement *this)
#[doc(alias = "RBX::Instance::read(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance4readEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE")]
// was: __ZN3RBX8Instance4readEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
// IDA 0x6fefd0: 60 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fefd0() {
}

// 0x6ff070 — __ZN3RBX8Instance12readChildrenEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
// type: int __fastcall(int, XmlElement *this)
#[doc(alias = "RBX::Instance::readChildren(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance12readChildrenEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE")]
// was: __ZN3RBX8Instance12readChildrenEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
// IDA 0x6ff070: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff070() {
}

// 0x6ff0b8 — __ZN3RBX8Instance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
// type: 
#[doc(alias = "RBX::Instance::readProperty(XmlElement const*,RBX::IReferenceBinder &)")]
#[doc(alias = "__ZN3RBX8Instance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZN3RBX8Instance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6ff0b8: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff0b8() {
}

// 0x6ff290 — __ZN3RBX8Instance14readPropertiesEPK10XmlElementRNS_16IReferenceBinderE
// type: 
#[doc(alias = "RBX::Instance::readProperties(XmlElement const*,RBX::IReferenceBinder &)")]
#[doc(alias = "__ZN3RBX8Instance14readPropertiesEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZN3RBX8Instance14readPropertiesEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6ff290: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff290() {
}

// 0x6ff2b0 — __ZN3RBX8Instance13writeChildrenEP10XmlElementRKN5boost8functionIFbPS0_EEENS_11CreatorRoleENS0_10SaveFilterE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Instance::writeChildren(XmlElement *,boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole,RBX::Instance::SaveFilter)")]
#[doc(alias = "__ZN3RBX8Instance13writeChildrenEP10XmlElementRKN5boost8functionIFbPS0_EEENS_11CreatorRoleENS0_10SaveFilterE")]
// was: __ZN3RBX8Instance13writeChildrenEP10XmlElementRKN5boost8functionIFbPS0_EEENS_11CreatorRoleENS0_10SaveFilterE
// IDA 0x6ff2b0: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff2b0() {
}

// 0x6ff3e0 — __ZNK3RBX8Instance15writePropertiesEP10XmlElement
// type: _DWORD __fastcall(RBX::Instance *__hidden this, XmlElement *)
#[doc(alias = "RBX::Instance::writeProperties(XmlElement *)const")]
#[doc(alias = "__ZNK3RBX8Instance15writePropertiesEP10XmlElement")]
// was: __ZNK3RBX8Instance15writePropertiesEP10XmlElement
// IDA 0x6ff3e0: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff3e0() {
}

// 0x6ff48c — __ZN3RBX8Instance8writeXmlERKN5boost8functionIFbPS0_EEENS_11CreatorRoleE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, void *, void *, int, int, int, int)
#[doc(alias = "RBX::Instance::writeXml(boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance8writeXmlERKN5boost8functionIFbPS0_EEENS_11CreatorRoleE")]
// was: __ZN3RBX8Instance8writeXmlERKN5boost8functionIFbPS0_EEENS_11CreatorRoleE
// IDA 0x6ff48c: 285 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff48c() {
}

// 0x6ff77c — __ZNK3RBX8Instance21getPersistentDataCostEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getPersistentDataCost(void)const")]
#[doc(alias = "__ZNK3RBX8Instance21getPersistentDataCostEv")]
// was: __ZNK3RBX8Instance21getPersistentDataCostEv
// IDA 0x6ff77c: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff77c() {
}

// 0x6ff888 — __ZN3RBXL16computeChildCostEN5boost10shared_ptrINS_8InstanceEEEPi
// type: 
#[doc(alias = "RBX::computeChildCost(boost::shared_ptr<RBX::Instance>,int *)")]
#[doc(alias = "__ZN3RBXL16computeChildCostEN5boost10shared_ptrINS_8InstanceEEEPi")]
// was: __ZN3RBXL16computeChildCostEN5boost10shared_ptrINS_8InstanceEEEPi
// IDA 0x6ff888: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff888() {
}

// 0x6ff8a0 — __ZN3RBX8Instance14onChildChangedEPS0_RKNS_15PropertyChangedE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Instance::onChildChanged(RBX::Instance*,RBX::PropertyChanged const&)")]
#[doc(alias = "__ZN3RBX8Instance14onChildChangedEPS0_RKNS_15PropertyChangedE")]
// was: __ZN3RBX8Instance14onChildChangedEPS0_RKNS_15PropertyChangedE
// IDA 0x6ff8a0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff8a0() {
}

// 0x6ff8b0 — __ZNK3RBX8Instance14findChildIndexEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::findChildIndex(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance14findChildIndexEPKS0_")]
// was: __ZNK3RBX8Instance14findChildIndexEPKS0_
// IDA 0x6ff8b0: 142 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ff8b0() {
}

// 0x6ffa58 — __ZN3RBX8Instance29findFirstChildByNameRecursiveERKSs
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const std::string *)
#[doc(alias = "RBX::Instance::findFirstChildByNameRecursive(std::string const&)")]
#[doc(alias = "__ZN3RBX8Instance29findFirstChildByNameRecursiveERKSs")]
// was: __ZN3RBX8Instance29findFirstChildByNameRecursiveERKSs
// IDA 0x6ffa58: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffa58() {
}

// 0x6ffa9c — __ZNK3RBX8Instance25findConstFirstChildByNameERKSs
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const std::string *)
#[doc(alias = "RBX::Instance::findConstFirstChildByName(std::string const&)const")]
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildByNameERKSs")]
// was: __ZNK3RBX8Instance25findConstFirstChildByNameERKSs
// IDA 0x6ffa9c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffa9c() {
}

// 0x6ffae0 — __ZNK3RBX8Instance19findFirstAncestorOfEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::findFirstAncestorOf(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance19findFirstAncestorOfEPKS0_")]
// was: __ZNK3RBX8Instance19findFirstAncestorOfEPKS0_
// IDA 0x6ffae0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffae0() {
}

// 0x6ffb48 — __ZNK3RBX8Instance13securityCheckEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::securityCheck(void)const")]
#[doc(alias = "__ZNK3RBX8Instance13securityCheckEv")]
// was: __ZNK3RBX8Instance13securityCheckEv
// IDA 0x6ffb48: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffb48() {
}

// 0x6ffb68 — __ZNK3RBX8Instance13securityCheckERNS_8Security7ContextE
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Security::Context *)
#[doc(alias = "RBX::Instance::securityCheck(RBX::Security::Context &)const")]
#[doc(alias = "__ZNK3RBX8Instance13securityCheckERNS_8Security7ContextE")]
// was: __ZNK3RBX8Instance13securityCheckERNS_8Security7ContextE
// IDA 0x6ffb68: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffb68() {
}

// 0x6ffb84 — __ZNK3RBX8Instance17verifySetAncestorEPKS0_S2_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Instance::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance17verifySetAncestorEPKS0_S2_")]
// was: __ZNK3RBX8Instance17verifySetAncestorEPKS0_S2_
// IDA 0x6ffb84: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffb84() {
}

// 0x6ffc74 — __ZNK3RBX8Instance19verifyAddDescendantEPKS0_S2_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Instance::verifyAddDescendant(RBX::Instance const*,RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance19verifyAddDescendantEPKS0_S2_")]
// was: __ZNK3RBX8Instance19verifyAddDescendantEPKS0_S2_
// IDA 0x6ffc74: 18 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffc74() {
}

// 0x6ffc98 — __ZN3RBX8Instance17setParentInternalEPS0_b
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Instance *, bool)
#[doc(alias = "RBX::Instance::setParentInternal(RBX::Instance*,bool)")]
#[doc(alias = "__ZN3RBX8Instance17setParentInternalEPS0_b")]
// was: __ZN3RBX8Instance17setParentInternalEPS0_b
// IDA 0x6ffc98: 1081 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ffc98() {
}

// 0x70086c — __ZNK3RBX8Instance11getFullNameEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getFullName(void)const")]
#[doc(alias = "__ZNK3RBX8Instance11getFullNameEv")]
// was: __ZNK3RBX8Instance11getFullNameEv
// IDA 0x70086c: 201 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_70086c() {
}

// 0x700ab8 — __ZNK3RBX8Instance8containsEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::contains(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance8containsEPKS0_")]
// was: __ZNK3RBX8Instance8containsEPKS0_
// IDA 0x700ab8: 10 insns (B..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_700ab8() {
}

// 0x700acc — __ZN3RBX8Instance24signalDescendantRemovingERKN5boost10shared_ptrIS0_EEPS0_S6_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Instance::signalDescendantRemoving(boost::shared_ptr<RBX::Instance> const&,RBX::Instance*,RBX::Instance*)")]
#[doc(alias = "__ZN3RBX8Instance24signalDescendantRemovingERKN5boost10shared_ptrIS0_EEPS0_S6_")]
// was: __ZN3RBX8Instance24signalDescendantRemovingERKN5boost10shared_ptrIS0_EEPS0_S6_
// IDA 0x700acc: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_700acc() {
}

// 0x700bf8 — __ZN3RBX8Instance21signalDescendantAddedEPS0_S1_S1_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Instance *, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::Instance::signalDescendantAdded(RBX::Instance*,RBX::Instance*,RBX::Instance*)")]
#[doc(alias = "__ZN3RBX8Instance21signalDescendantAddedEPS0_S1_S1_")]
// was: __ZN3RBX8Instance21signalDescendantAddedEPS0_S1_S1_
// IDA 0x700bf8: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_700bf8() {
}

// 0x700d28 — __ZN3RBX8Instance17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Instance::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX8Instance17onAncestorChangedERKNS_15AncestorChangedE")]
// was: __ZN3RBX8Instance17onAncestorChangedERKNS_15AncestorChangedE
// IDA 0x700d28: 263 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_700d28() {
}

// 0x700fcc — __ZN3RBX8Instance17onDescendantAddedEPS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Instance::onDescendantAdded(RBX::Instance*)")]
#[doc(alias = "__ZN3RBX8Instance17onDescendantAddedEPS0_")]
// was: __ZN3RBX8Instance17onDescendantAddedEPS0_
// IDA 0x700fcc: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_700fcc() {
}

// 0x7010a8 — __ZNK3RBX8Instance12onDemandReadEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::onDemandRead(void)const")]
#[doc(alias = "__ZNK3RBX8Instance12onDemandReadEv")]
// was: __ZNK3RBX8Instance12onDemandReadEv
// IDA 0x7010a8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7010a8() {
}

// 0x7010ac — __ZN3RBX8Instance13onDemandWriteEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::onDemandWrite(void)")]
#[doc(alias = "__ZN3RBX8Instance13onDemandWriteEv")]
// was: __ZN3RBX8Instance13onDemandWriteEv
// IDA 0x7010ac: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7010ac() {
}

// 0x70112c — __ZN3RBX8Instance20onDescendantRemovingERKN5boost10shared_ptrIS0_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Instance::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX8Instance20onDescendantRemovingERKN5boost10shared_ptrIS0_EE")]
// was: __ZN3RBX8Instance20onDescendantRemovingERKN5boost10shared_ptrIS0_EE
// IDA 0x70112c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_70112c() {
}

// 0x701130 — __ZN3RBX8Instance12toNewXmlRootEPS0_NS_11CreatorRoleE
// type: 
#[doc(alias = "RBX::Instance::toNewXmlRoot(RBX::Instance*,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance12toNewXmlRootEPS0_NS_11CreatorRoleE")]
// was: __ZN3RBX8Instance12toNewXmlRootEPS0_NS_11CreatorRoleE
// IDA 0x701130: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701130() {
}

// 0x701228 — __ZN3RBXL9isInScopeEPNS_8InstanceES1_
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::isInScope(RBX::Instance *,RBX::Instance *)")]
#[doc(alias = "__ZN3RBXL9isInScopeEPNS_8InstanceES1_")]
// was: __ZN3RBXL9isInScopeEPNS_8InstanceES1_
// IDA 0x701228: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701228() {
}

// 0x701240 — __ZN3RBX8Instance5cloneENS_11CreatorRoleE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Instance::clone(RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX8Instance5cloneENS_11CreatorRoleE")]
// was: __ZN3RBX8Instance5cloneENS_11CreatorRoleE
// IDA 0x701240: 211 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701240() {
}

// 0x701468 — __ZN3RBX14countInstancesEN5boost10shared_ptrINS_8InstanceEEEPi
// type: 
#[doc(alias = "RBX::countInstances(boost::shared_ptr<RBX::Instance>,int *)")]
#[doc(alias = "__ZN3RBX14countInstancesEN5boost10shared_ptrINS_8InstanceEEEPi")]
// was: __ZN3RBX14countInstancesEN5boost10shared_ptrINS_8InstanceEEEPi
// IDA 0x701468: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701468() {
}

// 0x701470 — __ZN3RBX8Instance8luaCloneEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::luaClone(void)")]
#[doc(alias = "__ZN3RBX8Instance8luaCloneEv")]
// was: __ZN3RBX8Instance8luaCloneEv
// IDA 0x701470: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701470() {
}

// 0x701600 — __ZN3RBX8InstanceC2EPNS_10FWInstanceE
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::FWInstance *)
#[doc(alias = "RBX::Instance::Instance(RBX::FWInstance *)")]
#[doc(alias = "__ZN3RBX8InstanceC2EPNS_10FWInstanceE")]
// was: __ZN3RBX8InstanceC2EPNS_10FWInstanceE
// IDA 0x701600: 390 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701600() {
}

// 0x701a24 — __ZN3RBX8InstanceC2EPKcPNS_10FWInstanceE
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const char *, RBX::FWInstance *)
#[doc(alias = "RBX::Instance::Instance(char const*,RBX::FWInstance *)")]
#[doc(alias = "__ZN3RBX8InstanceC2EPKcPNS_10FWInstanceE")]
// was: __ZN3RBX8InstanceC2EPKcPNS_10FWInstanceE
// IDA 0x701a24: 451 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701a24() {
}

// 0x701ef4 — __ZNK3RBX8Instance2fwEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::fw(void)const")]
#[doc(alias = "__ZNK3RBX8Instance2fwEv")]
// was: __ZNK3RBX8Instance2fwEv
// IDA 0x701ef4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_701ef4() {
}

// 0x701ef8 — __ZN3RBX8InstanceD0Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::~Instance()")]
#[doc(alias = "__ZN3RBX8InstanceD0Ev")]
// was: __ZN3RBX8InstanceD0Ev
// IDA 0x701ef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_701ef8() {
}

// 0x701f98 — __ZN3RBX8InstanceD1Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::~Instance()")]
#[doc(alias = "__ZN3RBX8InstanceD1Ev")]
// was: __ZN3RBX8InstanceD1Ev
// IDA 0x701f98: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_701f98() {
}

// 0x701f9c — __ZThn32_N3RBX8InstanceD0Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Instance::~Instance()")]
#[doc(alias = "__ZThn32_N3RBX8InstanceD0Ev")]
// was: __ZThn32_N3RBX8InstanceD0Ev
// IDA 0x701f9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_701f9c() {
}

// 0x701fa4 — __ZThn36_N3RBX8InstanceD0Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Instance::~Instance()")]
#[doc(alias = "__ZThn36_N3RBX8InstanceD0Ev")]
// was: __ZThn36_N3RBX8InstanceD0Ev
// IDA 0x701fa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_701fa4() {
}

// 0x701fac — __ZN3RBX8InstanceD2Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::~Instance()")]
#[doc(alias = "__ZN3RBX8InstanceD2Ev")]
// was: __ZN3RBX8InstanceD2Ev
// IDA 0x701fac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_701fac() {
}

// 0x7023a8 — __ZThn32_N3RBX8InstanceD1Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Instance::~Instance()")]
#[doc(alias = "__ZThn32_N3RBX8InstanceD1Ev")]
// was: __ZThn32_N3RBX8InstanceD1Ev
// IDA 0x7023a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7023a8() {
}

// 0x7023b0 — __ZThn36_N3RBX8InstanceD1Ev
// type: void __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Instance::~Instance()")]
#[doc(alias = "__ZThn36_N3RBX8InstanceD1Ev")]
// was: __ZThn36_N3RBX8InstanceD1Ev
// IDA 0x7023b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7023b0() {
}

// 0x7023b8 — __ZN3RBX8Instance7destroyEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::destroy(void)")]
#[doc(alias = "__ZN3RBX8Instance7destroyEv")]
// was: __ZN3RBX8Instance7destroyEv
// IDA 0x7023b8: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7023b8() {
}

// 0x7025bc — __ZN3RBX8Instance16setAndLockParentEPS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Instance::setAndLockParent(RBX::Instance*)")]
#[doc(alias = "__ZN3RBX8Instance16setAndLockParentEPS0_")]
// was: __ZN3RBX8Instance16setAndLockParentEPS0_
// IDA 0x7025bc: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7025bc() {
}

// 0x702778 — __ZN3RBX8Instance7setNameERKSs
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const std::string *)
#[doc(alias = "RBX::Instance::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX8Instance7setNameERKSs")]
// was: __ZN3RBX8Instance7setNameERKSs
// IDA 0x702778: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702778() {
}

// 0x7028ec — __ZN3RBX8Instance20raisePropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Instance::raisePropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX8Instance20raisePropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
// was: __ZN3RBX8Instance20raisePropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x7028ec: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7028ec() {
}

// 0x7029f4 — __ZN3RBX8Instance15setIsArchivableEb
// type: _DWORD __fastcall(RBX::Instance *__hidden this, bool)
#[doc(alias = "RBX::Instance::setIsArchivable(bool)")]
#[doc(alias = "__ZN3RBX8Instance15setIsArchivableEb")]
// was: __ZN3RBX8Instance15setIsArchivableEb
// IDA 0x7029f4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7029f4() {
}

// 0x702a28 — __ZN3RBX8Instance9predeleteEPS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Instance::predelete(RBX::Instance*)")]
#[doc(alias = "__ZN3RBX8Instance9predeleteEPS0_")]
// was: __ZN3RBX8Instance9predeleteEPS0_
// IDA 0x702a28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_702a28() {
}

// 0x702a2c — __ZN3RBX8Instance9predeleteEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::predelete(void)")]
#[doc(alias = "__ZN3RBX8Instance9predeleteEv")]
// was: __ZN3RBX8Instance9predeleteEv
// IDA 0x702a2c: 252 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702a2c() {
}

// 0x702cc4 — __ZNK3RBX8Instance12askSetParentEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance12askSetParentEPKS0_")]
// was: __ZNK3RBX8Instance12askSetParentEPKS0_
// IDA 0x702cc4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702cc4() {
}

// 0x702cc8 — __ZNK3RBX8Instance15askForbidParentEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::askForbidParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance15askForbidParentEPKS0_")]
// was: __ZNK3RBX8Instance15askForbidParentEPKS0_
// IDA 0x702cc8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702cc8() {
}

// 0x702ccc — __ZNK3RBX8Instance11askAddChildEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance11askAddChildEPKS0_")]
// was: __ZNK3RBX8Instance11askAddChildEPKS0_
// IDA 0x702ccc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702ccc() {
}

// 0x702cd0 — __ZNK3RBX8Instance14askForbidChildEPKS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::askForbidChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Instance14askForbidChildEPKS0_")]
// was: __ZNK3RBX8Instance14askForbidChildEPKS0_
// IDA 0x702cd0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702cd0() {
}

// 0x702cd4 — __ZN3RBX8Instance15promoteChildrenEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::promoteChildren(void)")]
#[doc(alias = "__ZN3RBX8Instance15promoteChildrenEv")]
// was: __ZN3RBX8Instance15promoteChildrenEv
// IDA 0x702cd4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702cd4() {
}

// 0x702d08 — __ZN3RBX8Instance12initOnDemandEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::initOnDemand(void)")]
#[doc(alias = "__ZN3RBX8Instance12initOnDemandEv")]
// was: __ZN3RBX8Instance12initOnDemandEv
// IDA 0x702d08: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702d08() {
}

// 0x702dc8 — __ZN3RBX10FWInstanceC2Ev
// type: _DWORD __fastcall(RBX::FWInstance *__hidden this)
#[doc(alias = "RBX::FWInstance::FWInstance(void)")]
#[doc(alias = "__ZN3RBX10FWInstanceC2Ev")]
// was: __ZN3RBX10FWInstanceC2Ev
// IDA 0x702dc8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702dc8() {
}

// 0x702eb4 — __ZNK3RBX10FWInstanceeqERKS0_
// type: 
#[doc(alias = "RBX::FWInstance::operator==(RBX::FWInstance const&)const")]
#[doc(alias = "__ZNK3RBX10FWInstanceeqERKS0_")]
// was: __ZNK3RBX10FWInstanceeqERKS0_
// IDA 0x702eb4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702eb4() {
}

// 0x702efc — __ZN3RBX10hash_valueERKNS_10FWInstanceE
// type: 
#[doc(alias = "RBX::hash_value(RBX::FWInstance const&)")]
#[doc(alias = "__ZN3RBX10hash_valueERKNS_10FWInstanceE")]
// was: __ZN3RBX10hash_valueERKNS_10FWInstanceE
// IDA 0x702efc: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702efc() {
}

// 0x702f60 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0
// type: 
#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0")]
#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0")]
// was: __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0
// IDA 0x702f60: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_702f60() {
}

// 0x7030c8 — __ZNK3RBX8Instance15getIsArchivableEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getIsArchivable(void)const")]
#[doc(alias = "__ZNK3RBX8Instance15getIsArchivableEv")]
// was: __ZNK3RBX8Instance15getIsArchivableEv
// IDA 0x7030c8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7030c8() {
}

// 0x7030d0 — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceEbED1Ev
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceEbED1Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8InstanceEbED1Ev
// IDA 0x7030d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7030d0() {
}

// 0x7030f4 — __ZN3RBX10Reflection13DescribedBase3isAESs
// type: 
#[doc(alias = "RBX::Reflection::DescribedBase::isA(std::string)")]
#[doc(alias = "__ZN3RBX10Reflection13DescribedBase3isAESs")]
// was: __ZN3RBX10Reflection13DescribedBase3isAESs
// IDA 0x7030f4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7030f4() {
}

// 0x703104 — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFbSsELi1EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,bool ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFbSsELi1EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFbSsELi1EED1Ev
// IDA 0x703104: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703104() {
}

// 0x703144 — __ZN3RBX8Instance21findFirstChildByName2ESsb
// type: 
#[doc(alias = "RBX::Instance::findFirstChildByName2(std::string,bool)")]
#[doc(alias = "__ZN3RBX8Instance21findFirstChildByName2ESsb")]
// was: __ZN3RBX8Instance21findFirstChildByName2ESsb
// IDA 0x703144: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703144() {
}

// 0x703168 — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsbELi2EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string,bool),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsbELi2EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsbELi2EED1Ev
// IDA 0x703168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703168() {
}

// 0x7031b0 — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EEvELi0EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EEvELi0EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EEvELi0EED1Ev
// IDA 0x7031b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7031b0() {
}

// 0x7031d4 — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFvvELi0EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFvvELi0EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFvvELi0EED1Ev
// IDA 0x7031d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7031d4() {
}

// 0x7031f8 — __ZN3RBX8Instance12getChildren2Ev
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getChildren2(void)")]
#[doc(alias = "__ZN3RBX8Instance12getChildren2Ev")]
// was: __ZN3RBX8Instance12getChildren2Ev
// IDA 0x7031f8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7031f8() {
}

// 0x70320c — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIKSt6vectorINS4_IS2_EESaIS6_EEEEvELi0EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIKSt6vectorINS4_IS2_EESaIS6_EEEEvELi0EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFN5boost10shared_ptrIKSt6vectorINS4_IS2_EESaIS6_EEEEvELi0EED1Ev
// IDA 0x70320c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_70320c() {
}

// 0x703230 — __ZN3RBX8Instance24getFullNameForReflectionEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getFullNameForReflection(void)")]
#[doc(alias = "__ZN3RBX8Instance24getFullNameForReflectionEv")]
// was: __ZN3RBX8Instance24getFullNameForReflectionEv
// IDA 0x703230: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703230() {
}

// 0x70323c — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFSsvELi0EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,std::string ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFSsvELi0EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFSsvELi0EED1Ev
// IDA 0x70323c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_70323c() {
}

// 0x703260 — __ZN3RBX8Instance15isDescendantOf2EN5boost10shared_ptrIS0_EE
// type: 
#[doc(alias = "RBX::Instance::isDescendantOf2(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Instance15isDescendantOf2EN5boost10shared_ptrIS0_EE")]
// was: __ZN3RBX8Instance15isDescendantOf2EN5boost10shared_ptrIS0_EE
// IDA 0x703260: 10 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703260() {
}

// 0x703274 — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFbN5boost10shared_ptrIS2_EEELi1EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,bool ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFbN5boost10shared_ptrIS2_EEELi1EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFbN5boost10shared_ptrIS2_EEELi1EED1Ev
// IDA 0x703274: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703274() {
}

// 0x703380 — __ZN3RBX8Instance13isAncestorOf2EN5boost10shared_ptrIS0_EE
// type: 
#[doc(alias = "RBX::Instance::isAncestorOf2(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Instance13isAncestorOf2EN5boost10shared_ptrIS0_EE")]
// was: __ZN3RBX8Instance13isAncestorOf2EN5boost10shared_ptrIS0_EE
// IDA 0x703380: 10 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703380() {
}

// 0x703394 — __ZN3RBX8Instance18getReadableDebugIdEi
// type: _DWORD __fastcall(RBX::Instance *__hidden this, int)
#[doc(alias = "RBX::Instance::getReadableDebugId(int)")]
#[doc(alias = "__ZN3RBX8Instance18getReadableDebugIdEi")]
// was: __ZN3RBX8Instance18getReadableDebugIdEi
// IDA 0x703394: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703394() {
}

// 0x7033a0 — __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFSsiELi1EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Instance,std::string ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFSsiELi1EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8InstanceEFSsiELi1EED1Ev
// IDA 0x7033a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7033a0() {
}

// 0x7033e0 — __ZNK3RBX8Instance15getClassNameStrEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getClassNameStr(void)const")]
#[doc(alias = "__ZNK3RBX8Instance15getClassNameStrEv")]
// was: __ZNK3RBX8Instance15getClassNameStrEv
// IDA 0x7033e0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7033e0() {
}

// 0x7033fc — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsED1Ev
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsED1Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsED1Ev
// IDA 0x7033fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7033fc() {
}

// 0x703420 — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiED1Ev
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiED1Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiED1Ev
// IDA 0x703420: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703420() {
}

// 0x703444 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED1Ev")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED1Ev
// IDA 0x703444: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703444() {
}

// 0x703484 — __ZNK3RBX8Instance7getNameEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getName(void)const")]
#[doc(alias = "__ZNK3RBX8Instance7getNameEv")]
// was: __ZNK3RBX8Instance7getNameEv
// IDA 0x703484: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703484() {
}

// 0x70348c — __ZNK3RBX8Instance18getParentDangerousEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getParentDangerous(void)const")]
#[doc(alias = "__ZNK3RBX8Instance18getParentDangerousEv")]
// was: __ZNK3RBX8Instance18getParentDangerousEv
// IDA 0x70348c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_70348c() {
}

// 0x703490 — __ZN3RBX8Instance9setParentEPS0_
// type: _DWORD __fastcall(RBX::Instance *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Instance::setParent(RBX::Instance*)")]
#[doc(alias = "__ZN3RBX8Instance9setParentEPS0_")]
// was: __ZN3RBX8Instance9setParentEPS0_
// IDA 0x703490: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703490() {
}

// 0x703498 — __ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED1Ev
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED1Ev")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED1Ev
// IDA 0x703498: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703498() {
}

// 0x7034c4 — __ZNK3RBX8Instance15getRobloxLockedEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getRobloxLocked(void)const")]
#[doc(alias = "__ZNK3RBX8Instance15getRobloxLockedEv")]
// was: __ZNK3RBX8Instance15getRobloxLockedEv
// IDA 0x7034c4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7034c4() {
}

// 0x7034cc — __ZN3RBX8Instance27getOrCreateChildAddedSignalEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getOrCreateChildAddedSignal(void)")]
#[doc(alias = "__ZN3RBX8Instance27getOrCreateChildAddedSignalEv")]
// was: __ZN3RBX8Instance27getOrCreateChildAddedSignalEv
// IDA 0x7034cc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7034cc() {
}

// 0x7034d8 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED1Ev
// type: 
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED1Ev")]
// was: __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED1Ev
// IDA 0x7034d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7034d8() {
}

// 0x7034fc — __ZN3RBX8Instance29getOrCreateChildRemovedSignalEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getOrCreateChildRemovedSignal(void)")]
#[doc(alias = "__ZN3RBX8Instance29getOrCreateChildRemovedSignalEv")]
// was: __ZN3RBX8Instance29getOrCreateChildRemovedSignalEv
// IDA 0x7034fc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7034fc() {
}

// 0x703508 — __ZN3RBX8Instance32getOrCreateDescendantAddedSignalEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getOrCreateDescendantAddedSignal(void)")]
#[doc(alias = "__ZN3RBX8Instance32getOrCreateDescendantAddedSignalEv")]
// was: __ZN3RBX8Instance32getOrCreateDescendantAddedSignalEv
// IDA 0x703508: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703508() {
}

// 0x703514 — __ZN3RBX8Instance35getOrCreateDescendantRemovingSignalEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getOrCreateDescendantRemovingSignal(void)")]
#[doc(alias = "__ZN3RBX8Instance35getOrCreateDescendantRemovingSignalEv")]
// was: __ZN3RBX8Instance35getOrCreateDescendantRemovingSignalEv
// IDA 0x703514: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703514() {
}

// 0x703520 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED1Ev
// type: 
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED1Ev")]
// was: __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED1Ev
// IDA 0x703520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703520() {
}

// 0x703544 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED1Ev
// type: 
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED1Ev")]
// was: __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED1Ev
// IDA 0x703544: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_703544() {
}

// 0x703568 — __ZN3RBX22AbstractFactoryProductINS_8InstanceEE6createERKNS_4NameENS_11CreatorRoleE
// type: 
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::create(RBX::Name const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX22AbstractFactoryProductINS_8InstanceEE6createERKNS_4NameENS_11CreatorRoleE")]
// was: __ZN3RBX22AbstractFactoryProductINS_8InstanceEE6createERKNS_4NameENS_11CreatorRoleE
// IDA 0x703568: 177 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703568() {
}

// 0x703748 — __ZN10Serializer13canWriteChildEN5boost10shared_ptrIN3RBX8InstanceEEENS3_10SaveFilterE
// type: 
#[doc(alias = "Serializer::canWriteChild(boost::shared_ptr<RBX::Instance>,RBX::Instance::SaveFilter)")]
#[doc(alias = "__ZN10Serializer13canWriteChildEN5boost10shared_ptrIN3RBX8InstanceEEENS3_10SaveFilterE")]
// was: __ZN10Serializer13canWriteChildEN5boost10shared_ptrIN3RBX8InstanceEEENS3_10SaveFilterE
// IDA 0x703748: 160 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_703748() {
}
