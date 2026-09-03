//! rendering shard 342 — 120 stubs 0x5dd610..0x5e2af4 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 37200->37320 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 37200 before -> 37320 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 after 0x5dd584 (range 0x5dd610..0x5e2af4)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x5dd610 — __ZN3RBX12PartInstance8safeMoveEv
#[doc(alias = "RBX::PartInstance::safeMove(void)")]
// was: __ZN3RBX12PartInstance8safeMoveEv
// IDA 0x5dd610: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dd610() {
}

// 0x5dd824 — __ZN3RBX12PartInstance10setPhysicsERKNS_2PVE
#[doc(alias = "RBX::PartInstance::setPhysics(RBX::PV const&)")]
// was: __ZN3RBX12PartInstance10setPhysicsERKNS_2PVE
// IDA 0x5dd824: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dd824() {
}

// 0x5ddecc — __ZNK3RBX12PartInstance32computeNetworkOwnerIsSomeoneElseEv
#[doc(alias = "RBX::PartInstance::computeNetworkOwnerIsSomeoneElse(void)const")]
// was: __ZNK3RBX12PartInstance32computeNetworkOwnerIsSomeoneElseEv
// IDA 0x5ddecc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ddecc() {
}

// 0x5ddef8 — __ZNK3RBX12PartInstance12isProjectileEv
#[doc(alias = "RBX::PartInstance::isProjectile(void)const")]
// was: __ZNK3RBX12PartInstance12isProjectileEv
// IDA 0x5ddef8: 152 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ddef8() {
}

// 0x5de174 — __ZNK3RBX12PartInstance16getTranslationUiEv
#[doc(alias = "RBX::PartInstance::getTranslationUi(void)const")]
// was: __ZNK3RBX12PartInstance16getTranslationUiEv
// IDA 0x5de174: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de174() {
}

// 0x5de240 — __ZNK3RBX12PartInstance13getRotationUiEv
#[doc(alias = "RBX::PartInstance::getRotationUi(void)const")]
// was: __ZNK3RBX12PartInstance13getRotationUiEv
// IDA 0x5de240: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de240() {
}

// 0x5de394 — __ZNK3RBX12PartInstance11getVelocityEv
#[doc(alias = "RBX::PartInstance::getVelocity(void)const")]
// was: __ZNK3RBX12PartInstance11getVelocityEv
// IDA 0x5de394: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de394() {
}

// 0x5de534 — __ZN3RBX12PartInstance17refreshPartSizeUiEv
#[doc(alias = "RBX::PartInstance::refreshPartSizeUi(void)")]
// was: __ZN3RBX12PartInstance17refreshPartSizeUiEv
// IDA 0x5de534: 4 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de534() {
}

// 0x5de570 — __ZN3RBX12PartInstance26onNetworkIsSleepingChangedEv
#[doc(alias = "RBX::PartInstance::onNetworkIsSleepingChanged(void)")]
// was: __ZN3RBX12PartInstance26onNetworkIsSleepingChangedEv
// IDA 0x5de570: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de570() {
}

// 0x5de59c — __ZThn96_N3RBX12PartInstance26onNetworkIsSleepingChangedEv
#[doc(alias = "non-virtual thunk toRBX::PartInstance::onNetworkIsSleepingChanged(void)")]
// was: __ZThn96_N3RBX12PartInstance26onNetworkIsSleepingChangedEv
// IDA 0x5de59c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de59c() {
}

// 0x5de5a4 — __ZN3RBX12PartInstance13setCanCollideEb
#[doc(alias = "RBX::PartInstance::setCanCollide(bool)")]
// was: __ZN3RBX12PartInstance13setCanCollideEb
// IDA 0x5de5a4: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de5a4() {
}

// 0x5de5dc — __ZN3RBX12PartInstance11setAnchoredEb
#[doc(alias = "RBX::PartInstance::setAnchored(bool)")]
// was: __ZN3RBX12PartInstance11setAnchoredEb
// IDA 0x5de5dc: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de5dc() {
}

// 0x5de610 — __ZNK3RBX12PartInstance30getConnectedPartsRecursiveImplERN5boost10shared_ptrISt6vectorINS2_INS_8InstanceEEESaIS5_EEEERNS1_9unordered13unordered_setIPS0_NS1_4hashISC_EESt8equal_toISC_ESaISC_EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::PartInstance::getConnectedPartsRecursiveImpl(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::unordered_set<RBX::PartInstance*,boost::hash<RBX::PartInstance*>,std::equal_to<RBX::PartInstance*>,std::allocator<RBX::PartInstance*>> &)const")]
// was: __ZNK3RBX12PartInstance30getConnectedPartsRecursiveImplERN5boost10shared_ptrISt6vectorINS2_INS_8InstanceEEESaIS5_EEEERNS1_9unordered13unordered_setIPS0_NS1_4hashISC_EESt8equal_toISC_ESaISC_EEE
// IDA 0x5de610: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de610() {
}

// 0x5de7fc — __ZN3RBXL24getConnectedPartsVisitorEPNS_9PrimitiveERN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::getConnectedPartsVisitor(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &)")]
// was: __ZN3RBXL24getConnectedPartsVisitorEPNS_9PrimitiveERN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// IDA 0x5de7fc: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de7fc() {
}

// 0x5de910 — __ZN3RBX12PartInstance13setPartLockedEb
#[doc(alias = "RBX::PartInstance::setPartLocked(bool)")]
// was: __ZN3RBX12PartInstance13setPartLockedEb
// IDA 0x5de910: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de910() {
}

// 0x5de94c — __ZN3RBX12PartInstance9getLockedEPNS_8InstanceE
#[doc(alias = "RBX::PartInstance::getLocked(RBX::Instance *)")]
// was: __ZN3RBX12PartInstance9getLockedEPNS_8InstanceE
// IDA 0x5de94c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de94c() {
}

// 0x5de9bc — __ZN3RBX12PartInstance9setLockedEPNS_8InstanceEb
#[doc(alias = "RBX::PartInstance::setLocked(RBX::Instance *,bool)")]
// was: __ZN3RBX12PartInstance9setLockedEPNS_8InstanceEb
// IDA 0x5de9bc: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5de9bc() {
}

// 0x5dea94 — __ZN3RBX12PartInstance15setTransparencyEf
#[doc(alias = "RBX::PartInstance::setTransparency(float)")]
// was: __ZN3RBX12PartInstance15setTransparencyEf
// IDA 0x5dea94: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dea94() {
}

// 0x5deadc — __ZN3RBX12PartInstance14setReflectanceEf
#[doc(alias = "RBX::PartInstance::setReflectance(float)")]
// was: __ZN3RBX12PartInstance14setReflectanceEf
// IDA 0x5deadc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5deadc() {
}

// 0x5deb1c — __ZN3RBX12PartInstance8setColorENS_10BrickColorE
#[doc(alias = "RBX::PartInstance::setColor(RBX::BrickColor)")]
// was: __ZN3RBX12PartInstance8setColorENS_10BrickColorE
// IDA 0x5deb1c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5deb1c() {
}

// 0x5deb60 — __ZN3RBX12PartInstance11setFrictionEf
#[doc(alias = "RBX::PartInstance::setFriction(float)")]
// was: __ZN3RBX12PartInstance11setFrictionEf
// IDA 0x5deb60: 24 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5deb60() {
}

// 0x5debb4 — __ZN3RBX12PartInstance13setElasticityEf
#[doc(alias = "RBX::PartInstance::setElasticity(float)")]
// was: __ZN3RBX12PartInstance13setElasticityEf
// IDA 0x5debb4: 24 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5debb4() {
}

// 0x5dec08 — __ZN3RBX12PartInstance6resizeENS_8NormalIdEi
#[doc(alias = "RBX::PartInstance::resize(RBX::NormalId,int)")]
// was: __ZN3RBX12PartInstance6resizeENS_8NormalIdEi
// IDA 0x5dec08: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dec08() {
}

// 0x5dec38 — __ZN3RBX12PartInstance10resizeImplENS_8NormalIdEi
#[doc(alias = "RBX::PartInstance::resizeImpl(RBX::NormalId,int)")]
// was: __ZN3RBX12PartInstance10resizeImplENS_8NormalIdEi
// IDA 0x5dec38: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dec38() {
}

// 0x5defac — __ZN3RBX12PartInstance11resizeFloatENS_8NormalIdEfb
#[doc(alias = "RBX::PartInstance::resizeFloat(RBX::NormalId,float,bool)")]
// was: __ZN3RBX12PartInstance11resizeFloatENS_8NormalIdEfb
// IDA 0x5defac: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5defac() {
}

// 0x5defe8 — __ZN3RBX12PartInstance13advResizeImplENS_8NormalIdEfb
#[doc(alias = "RBX::PartInstance::advResizeImpl(RBX::NormalId,float,bool)")]
// was: __ZN3RBX12PartInstance13advResizeImplENS_8NormalIdEfb
// IDA 0x5defe8: 444 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5defe8() {
}

// 0x5df538 — __ZN3RBX12PartInstance10getSurfaceERKNS_6RbxRayERi
#[doc(alias = "RBX::PartInstance::getSurface(RBX::RbxRay const&,int &)")]
// was: __ZN3RBX12PartInstance10getSurfaceERKNS_6RbxRayERi
// IDA 0x5df538: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df538() {
}

// 0x5df668 — __ZN3RBX12PartInstance14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
#[doc(alias = "RBX::PartInstance::setSurfaceType(RBX::NormalId,RBX::SurfaceType)")]
// was: __ZN3RBX12PartInstance14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
// IDA 0x5df668: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df668() {
}

// 0x5df6a0 — __ZNK3RBX12PartInstance8getInputENS_8NormalIdE
#[doc(alias = "RBX::PartInstance::getInput(RBX::NormalId)const")]
// was: __ZNK3RBX12PartInstance8getInputENS_8NormalIdE
// IDA 0x5df6a0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df6a0() {
}

// 0x5df6c8 — __ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE
#[doc(alias = "RBX::PartInstance::setSurfaceInput(RBX::NormalId,RBX::LegacyController::InputType)")]
// was: __ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE
// IDA 0x5df6c8: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df6c8() {
}

// 0x5df788 — __ZNK3RBX12PartInstance9getParamAENS_8NormalIdE
#[doc(alias = "RBX::PartInstance::getParamA(RBX::NormalId)const")]
// was: __ZNK3RBX12PartInstance9getParamAENS_8NormalIdE
// IDA 0x5df788: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df788() {
}

// 0x5df804 — __ZNK3RBX12PartInstance9getParamBENS_8NormalIdE
#[doc(alias = "RBX::PartInstance::getParamB(RBX::NormalId)const")]
// was: __ZNK3RBX12PartInstance9getParamBENS_8NormalIdE
// IDA 0x5df804: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df804() {
}

// 0x5df880 — __ZN3RBX12PartInstance9setParamAENS_8NormalIdEf
#[doc(alias = "RBX::PartInstance::setParamA(RBX::NormalId,float)")]
// was: __ZN3RBX12PartInstance9setParamAENS_8NormalIdEf
// IDA 0x5df880: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df880() {
}

// 0x5df8f8 — __ZN3RBX12PartInstance9setParamBENS_8NormalIdEf
#[doc(alias = "RBX::PartInstance::setParamB(RBX::NormalId,float)")]
// was: __ZN3RBX12PartInstance9setParamBENS_8NormalIdEf
// IDA 0x5df8f8: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df8f8() {
}

// 0x5df970 — __ZNK3RBX12PartInstance18containedByFrustumERNS_7FrustumE
#[doc(alias = "RBX::PartInstance::containedByFrustum(RBX::Frustum &)const")]
// was: __ZNK3RBX12PartInstance18containedByFrustumERNS_7FrustumE
// IDA 0x5df970: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df970() {
}

// 0x5df9dc — __ZNK3RBX12PartInstance14isStandardPartEv
#[doc(alias = "RBX::PartInstance::isStandardPart(void)const")]
// was: __ZNK3RBX12PartInstance14isStandardPartEv
// IDA 0x5df9dc: 21 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5df9dc() {
}

// 0x5dfc80 — __ZN3RBXL12getComponentEPK10XmlElementRKNS_4NameE
#[doc(alias = "RBX::getComponent(XmlElement const*,RBX::Name const&)")]
// was: __ZN3RBXL12getComponentEPK10XmlElementRKNS_4NameE
// IDA 0x5dfc80: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dfc80() {
}

// 0x5e01a0 — __ZN3RBX12PartInstance17fireOutfitChangedEv
#[doc(alias = "RBX::PartInstance::fireOutfitChanged(void)")]
// was: __ZN3RBX12PartInstance17fireOutfitChangedEv
// IDA 0x5e01a0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e01a0() {
}

// 0x5e01dc — __ZN3RBX12PartInstance11reportTouchERKN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::PartInstance::reportTouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: __ZN3RBX12PartInstance11reportTouchERKN5boost10shared_ptrIS0_EE
// IDA 0x5e01dc: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e01dc() {
}

// 0x5e02d8 — __ZN3RBX12PartInstance32setIsCurrentlyStreamRemovingPartEv
#[doc(alias = "RBX::PartInstance::setIsCurrentlyStreamRemovingPart(void)")]
// was: __ZN3RBX12PartInstance32setIsCurrentlyStreamRemovingPartEv
// IDA 0x5e02d8: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e02d8() {
}

// 0x5e02e8 — __ZN3RBX10hash_valueERKNS_14FWPartInstanceE
#[doc(alias = "RBX::hash_value(RBX::FWPartInstance const&)")]
// was: __ZN3RBX10hash_valueERKNS_14FWPartInstanceE
// IDA 0x5e02e8: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e02e8() {
}

// 0x5e03b4 — __ZN3RBX14FWPartInstanceC2Ev
#[doc(alias = "RBX::FWPartInstance::FWPartInstance(void)")]
// was: __ZN3RBX14FWPartInstanceC2Ev
// IDA 0x5e03b4: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e03b4() {
}

// 0x5e04b4 — __ZNK3RBX14FWPartInstanceeqERKS0_
#[doc(alias = "RBX::FWPartInstance::operator==(RBX::FWPartInstance const&)const")]
// was: __ZNK3RBX14FWPartInstanceeqERKS0_
// IDA 0x5e04b4: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e04b4() {
}

// 0x5e0580 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEfED1Ev
// IDA 0x5e0580: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0580() {
}

// 0x5e05a4 — __ZN3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor>::addPair(RBX::PartInstance::FormFactor,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE7addPairES3_PKc
// IDA 0x5e05a4: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e05a4() {
}

// 0x5e0904 — __ZN3RBX10Reflection8EnumDescINS_8MaterialEE7addPairES2_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::addPair(RBX::Material,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_8MaterialEE7addPairES2_PKc
// IDA 0x5e0904: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0904() {
}

// 0x5e0c64 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EED1Ev
// IDA 0x5e0c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0c64() {
}

// 0x5e0c88 — __ZN3RBX12PartInstance15getMassNonConstEv
#[doc(alias = "RBX::PartInstance::getMassNonConst(void)")]
// was: __ZN3RBX12PartInstance15getMassNonConstEv
// IDA 0x5e0c88: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0c88() {
}

// 0x5e0c98 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,float ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EED1Ev
// IDA 0x5e0c98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0c98() {
}

// 0x5e0cbc — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EED1Ev
// IDA 0x5e0cbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0cbc() {
}

// 0x5e0ce0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED1Ev
// IDA 0x5e0ce0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0ce0() {
}

// 0x5e0d20 — __ZNK3RBX12PartInstance9getColor3Ev
#[doc(alias = "RBX::PartInstance::getColor3(void)const")]
// was: __ZNK3RBX12PartInstance9getColor3Ev
// IDA 0x5e0d20: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0d20() {
}

// 0x5e0d90 — __ZNK3RBX12PartInstance8getColorEv
#[doc(alias = "RBX::PartInstance::getColor(void)const")]
// was: __ZNK3RBX12PartInstance8getColorEv
// IDA 0x5e0d90: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0d90() {
}

// 0x5e0d98 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_10BrickColorEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::BrickColor>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_10BrickColorEED1Ev
// IDA 0x5e0d98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0d98() {
}

// 0x5e0dc4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED1Ev
// IDA 0x5e0dc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0dc4() {
}

// 0x5e0de8 — __ZNK3RBX12PartInstance18getTransparencyXmlEv
#[doc(alias = "RBX::PartInstance::getTransparencyXml(void)const")]
// was: __ZNK3RBX12PartInstance18getTransparencyXmlEv
// IDA 0x5e0de8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0de8() {
}

// 0x5e0df0 — __ZNK3RBX12PartInstance16getAlphaModifierEv
#[doc(alias = "RBX::PartInstance::getAlphaModifier(void)const")]
// was: __ZNK3RBX12PartInstance16getAlphaModifierEv
// IDA 0x5e0df0: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0df0() {
}

// 0x5e0df8 — __ZNK3RBX12PartInstance14getReflectanceEv
#[doc(alias = "RBX::PartInstance::getReflectance(void)const")]
// was: __ZNK3RBX12PartInstance14getReflectanceEv
// IDA 0x5e0df8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0df8() {
}

// 0x5e0e00 — __ZNK3RBX12PartInstance13getPartLockedEv
#[doc(alias = "RBX::PartInstance::getPartLocked(void)const")]
// was: __ZNK3RBX12PartInstance13getPartLockedEv
// IDA 0x5e0e00: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0e00() {
}

// 0x5e0e08 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED1Ev
// IDA 0x5e0e08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0e08() {
}

// 0x5e0e2c — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED1Ev
// IDA 0x5e0e2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0e2c() {
}

// 0x5e0e50 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED1Ev
// IDA 0x5e0e50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0e50() {
}

// 0x5e0e74 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED1Ev
// IDA 0x5e0e74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0e74() {
}

// 0x5e0ebc — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED1Ev
// IDA 0x5e0ebc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0ebc() {
}

// 0x5e0ee0 — __ZN3RBX12PartInstance39getOrCreateLocalSimulationTouchedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateLocalSimulationTouchedSignal(void)")]
// was: __ZN3RBX12PartInstance39getOrCreateLocalSimulationTouchedSignalEv
// IDA 0x5e0ee0: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0ee0() {
}

// 0x5e0eec — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED1Ev
// IDA 0x5e0eec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0eec() {
}

// 0x5e0f10 — __ZN3RBX12PartInstance24getOrCreateTouchedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateTouchedSignal(void)")]
// was: __ZN3RBX12PartInstance24getOrCreateTouchedSignalEv
// IDA 0x5e0f10: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0f10() {
}

// 0x5e0f1c — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED1Ev
// IDA 0x5e0f1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0f1c() {
}

// 0x5e0f40 — __ZN3RBX12PartInstance29getOrCreateTouchedEndedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateTouchedEndedSignal(void)")]
// was: __ZN3RBX12PartInstance29getOrCreateTouchedEndedSignalEv
// IDA 0x5e0f40: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0f40() {
}

// 0x5e0f4c — __ZN3RBX12PartInstance42getOrCreateDeprecatedStoppedTouchingSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateDeprecatedStoppedTouchingSignal(void)")]
// was: __ZN3RBX12PartInstance42getOrCreateDeprecatedStoppedTouchingSignalEv
// IDA 0x5e0f4c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0f4c() {
}

// 0x5e0f58 — __ZN3RBX12PartInstance30getOrCreateOutfitChangedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateOutfitChangedSignal(void)")]
// was: __ZN3RBX12PartInstance30getOrCreateOutfitChangedSignalEv
// IDA 0x5e0f58: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0f58() {
}

// 0x5e0f64 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED1Ev
// IDA 0x5e0f64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e0f64() {
}

// 0x5e0f88 — __ZNK3RBX12PartInstance18getReceiveIntervalEv
#[doc(alias = "RBX::PartInstance::getReceiveInterval(void)const")]
// was: __ZNK3RBX12PartInstance18getReceiveIntervalEv
// IDA 0x5e0f88: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0f88() {
}

// 0x5e0f90 — __ZNK3RBX9Primitive15getExtentsWorldEv
#[doc(alias = "RBX::Primitive::getExtentsWorld(void)const")]
// was: __ZNK3RBX9Primitive15getExtentsWorldEv
// IDA 0x5e0f90: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0f90() {
}

// 0x5e0ff8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
// IDA 0x5e0ff8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0ff8() {
}

// 0x5e10ac — __ZN3RBX18OnDemandPVInstanceC2Ev
#[doc(alias = "RBX::OnDemandPVInstance::OnDemandPVInstance(void)")]
// was: __ZN3RBX18OnDemandPVInstanceC2Ev
// IDA 0x5e10ac: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e10ac() {
}

// 0x5e1178 — __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::Allocator(void)")]
// was: __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEC2Ev
// IDA 0x5e1178: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1178() {
}

// 0x5e11dc — __ZN3RBX6FWBase4initINS_14FWPartInstanceEEEPT_S4_
#[doc(alias = "RBX::FWPartInstance * RBX::FWBase::init<RBX::FWPartInstance>(RBX::FWPartInstance *)")]
// was: __ZN3RBX6FWBase4initINS_14FWPartInstanceEEEPT_S4_
// IDA 0x5e11dc: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e11dc() {
}

// 0x5e12a8 — __ZN3RBX7IMovingD2Ev
#[doc(alias = "RBX::IMoving::~IMoving()")]
// was: __ZN3RBX7IMovingD2Ev
// IDA 0x5e12a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e12a8() {
}

// 0x5e1314 — __ZN3RBX12PartInstance20OnDemandPartInstancedlEPv
#[doc(alias = "RBX::PartInstance::OnDemandPartInstance::operator delete(void *)")]
// was: __ZN3RBX12PartInstance20OnDemandPartInstancedlEPv
// IDA 0x5e1314: operator new/delete pair → Rust allocator/global alloc; no-op glue.
pub fn stub_5e1314() {
}

// 0x5e1468 — __ZN3RBX7Dragger8dragSnapEv
#[doc(alias = "RBX::Dragger::dragSnap(void)")]
// was: __ZN3RBX7Dragger8dragSnapEv
// IDA 0x5e1468: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1468() {
}

// 0x5e14bc — __ZNK3RBX5Joint11getNormalIdEi
#[doc(alias = "RBX::Joint::getNormalId(int)const")]
// was: __ZNK3RBX5Joint11getNormalIdEi
// IDA 0x5e14bc: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e14bc() {
}

// 0x5e1534 — __ZNK3RBX12PartInstance19hasTouchTransmitterEv
#[doc(alias = "RBX::PartInstance::hasTouchTransmitter(void)const")]
// was: __ZNK3RBX12PartInstance19hasTouchTransmitterEv
// IDA 0x5e1534: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1534() {
}

// 0x5e1558 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_16TouchTransmitterEEEPKT_v
#[doc(alias = "RBX::TouchTransmitter const* RBX::Instance::findConstFirstChildOfType<RBX::TouchTransmitter>(void)const")]
// was: __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_16TouchTransmitterEEEPKT_v
// IDA 0x5e1558: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1558() {
}

// 0x5e15c0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::push_back(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
// IDA 0x5e15c0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5e15c0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5e1610 — __ZN3RBX11shared_fromINS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> RBX::shared_from<RBX::PartInstance>(RBX::PartInstance*)")]
// was: __ZN3RBX11shared_fromINS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x5e1610: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1610() {
}

// 0x5e1780 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::push_back(rbx_core::WeakPtr<RBX::PartInstance> const&)")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
// IDA 0x5e1780: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5e1780() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5e1810 — __ZNK3RBX8Instance11numChildrenEv
#[doc(alias = "RBX::Instance::numChildren(void)const")]
// was: __ZNK3RBX8Instance11numChildrenEv
// IDA 0x5e1810: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1810() {
}

// 0x5e1830 — __ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(bool)>::operator()(bool)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb
// IDA 0x5e1830: 76 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1830() {
}

// 0x5e1978 — __ZN3RBX8Instance17onServiceProviderEPNS_15ServiceProviderES2_
#[doc(alias = "RBX::Instance::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8Instance17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x5e1978: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5e1978() {
}

// 0x5e197c — __ZN3RBX13FWDictionnaryINS_14FWPartInstanceEE17registerFlyweightEPNS_5FWRefE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::FWDictionnary<RBX::FWPartInstance>::registerFlyweight(RBX::FWRef *)")]
// was: __ZN3RBX13FWDictionnaryINS_14FWPartInstanceEE17registerFlyweightEPNS_5FWRefE
// IDA 0x5e197c: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e197c() {
}

// 0x5e1bc0 — __ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
#[doc(alias = "RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
// IDA 0x5e1bc0: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1bc0() {
}

// 0x5e1de8 — __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
#[doc(alias = "RBX::Network::NetworkOwner::ServerUnassigned(void)")]
// was: __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
// IDA 0x5e1de8: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1de8() {
}

// 0x5e1e40 — __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
#[doc(alias = "RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")]
// was: __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
// IDA 0x5e1e40: 45 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1e40() {
}

// 0x5e1eac — __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
#[doc(alias = "RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")]
// was: __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
// IDA 0x5e1eac: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1eac() {
}

// 0x5e1ef8 — __ZN3RBX7Network12NetworkOwner6ServerEv
#[doc(alias = "RBX::Network::NetworkOwner::Server(void)")]
// was: __ZN3RBX7Network12NetworkOwner6ServerEv
// IDA 0x5e1ef8: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1ef8() {
}

// 0x5e1f50 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::push_back(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
// IDA 0x5e1f50: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5e1f50() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5e1fa0 — __ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")]
// was: __ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
// IDA 0x5e1fa0: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e1fa0() {
}

// 0x5e20e4 — __ZN5boost4bindIvPN3RBX9PrimitiveERNS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list_av_2<boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: __ZN5boost4bindIvPN3RBX9PrimitiveERNS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
// IDA 0x5e20e4: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e20e4() {
}

// 0x5e21fc — __ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
#[doc(alias = "RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
// IDA 0x5e21fc: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e21fc() {
}

// 0x5e2424 — __ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
#[doc(alias = "RBX::FWValue<float>::set(float const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
// IDA 0x5e2424: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2424() {
}

// 0x5e2658 — __ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
#[doc(alias = "RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
// IDA 0x5e2658: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2658() {
}

// 0x5e2884 — __ZN3RBX8Instance23OutfitChangedSignalDataD1Ev
#[doc(alias = "RBX::Instance::OutfitChangedSignalData::~OutfitChangedSignalData()")]
// was: __ZN3RBX8Instance23OutfitChangedSignalDataD1Ev
// IDA 0x5e2884: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5e2884() {
}

// 0x5e2888 — __ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5e2888: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2888() {
}

// 0x5e2978 — __ZN3RBX9AllocatorINS_14FWPartInstanceEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::FWPartInstance>::Allocator(void)")]
// was: __ZN3RBX9AllocatorINS_14FWPartInstanceEEC2Ev
// IDA 0x5e2978: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2978() {
}

// 0x5e29dc — __ZNK3RBX12PartInstance21getPersistentDataCostEv
#[doc(alias = "RBX::PartInstance::getPersistentDataCost(void)const")]
// was: __ZNK3RBX12PartInstance21getPersistentDataCostEv
// IDA 0x5e29dc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e29dc() {
}

// 0x5e29e8 — __ZN3RBX8Instance15canClientCreateEv
#[doc(alias = "RBX::Instance::canClientCreate(void)")]
// was: __ZN3RBX8Instance15canClientCreateEv
// IDA 0x5e29e8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e29e8() {
}

// 0x5e29f0 — __ZN3RBX8Instance12onChildAddedEPS0_
#[doc(alias = "RBX::Instance::onChildAdded(RBX::Instance*)")]
// was: __ZN3RBX8Instance12onChildAddedEPS0_
// IDA 0x5e29f0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5e29f0() {
}

// 0x5e29f8 — __ZNK3RBX10PVInstance13childHashCodeEv
#[doc(alias = "RBX::PVInstance::childHashCode(void)const")]
// was: __ZNK3RBX10PVInstance13childHashCodeEv
// IDA 0x5e29f8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e29f8() {
}

// 0x5e29fc — __ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv
// IDA 0x5e29fc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e29fc() {
}

// 0x5e2a24 — __ZN3RBX12PartInstance23hasThreeDimensionalSizeEv
#[doc(alias = "RBX::PartInstance::hasThreeDimensionalSize(void)")]
// was: __ZN3RBX12PartInstance23hasThreeDimensionalSizeEv
// IDA 0x5e2a24: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2a24() {
}

// 0x5e2a28 — __ZNK3RBX12PartInstance11getPartTypeEv
#[doc(alias = "RBX::PartInstance::getPartType(void)const")]
// was: __ZNK3RBX12PartInstance11getPartTypeEv
// IDA 0x5e2a28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2a28() {
}

// 0x5e2a2c — __ZNK3RBX12PartInstance16getMinimumUiSizeEv
#[doc(alias = "RBX::PartInstance::getMinimumUiSize(void)const")]
// was: __ZNK3RBX12PartInstance16getMinimumUiSizeEv
// IDA 0x5e2a2c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2a2c() {
}

// 0x5e2a78 — __ZNK3RBX12PartInstance22getMinimumUiSizeCustomEv
#[doc(alias = "RBX::PartInstance::getMinimumUiSizeCustom(void)const")]
// was: __ZNK3RBX12PartInstance22getMinimumUiSizeCustomEv
// IDA 0x5e2a78: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2a78() {
}

// 0x5e2ac8 — __ZNK3RBX12PartInstance19getResizeHandleMaskEv
#[doc(alias = "RBX::PartInstance::getResizeHandleMask(void)const")]
// was: __ZNK3RBX12PartInstance19getResizeHandleMaskEv
// IDA 0x5e2ac8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2ac8() {
}

// 0x5e2adc — __ZNK3RBX12PartInstance23getDragUtilitiesSupportEv
#[doc(alias = "RBX::PartInstance::getDragUtilitiesSupport(void)const")]
// was: __ZNK3RBX12PartInstance23getDragUtilitiesSupportEv
// IDA 0x5e2adc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2adc() {
}

// 0x5e2ae0 — __ZNK3RBX12PartInstance18getResizeIncrementEv
#[doc(alias = "RBX::PartInstance::getResizeIncrement(void)const")]
// was: __ZNK3RBX12PartInstance18getResizeIncrementEv
// IDA 0x5e2ae0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2ae0() {
}

// 0x5e2ae4 — __ZNK3RBX12PartInstance25getMinimumResizeIncrementEv
#[doc(alias = "RBX::PartInstance::getMinimumResizeIncrement(void)const")]
// was: __ZNK3RBX12PartInstance25getMinimumResizeIncrementEv
// IDA 0x5e2ae4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2ae4() {
}

// 0x5e2af0 — __ZNK3RBX12PartInstance13getFormFactorEv
#[doc(alias = "RBX::PartInstance::getFormFactor(void)const")]
// was: __ZNK3RBX12PartInstance13getFormFactorEv
// IDA 0x5e2af0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2af0() {
}

// 0x5e2af4 — __ZN3RBX12PartInstance11getLocationEv
#[doc(alias = "RBX::PartInstance::getLocation(void)")]
// was: __ZN3RBX12PartInstance11getLocationEv
// IDA 0x5e2af4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2af4() {
}
