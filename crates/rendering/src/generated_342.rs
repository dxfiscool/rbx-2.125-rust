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
pub fn stub_5dd610() -> ! {
    todo!("0x5dd610 RBX::PartInstance::safeMove(void)")
}

// 0x5dd824 — __ZN3RBX12PartInstance10setPhysicsERKNS_2PVE
#[doc(alias = "RBX::PartInstance::setPhysics(RBX::PV const&)")]
// was: __ZN3RBX12PartInstance10setPhysicsERKNS_2PVE
pub fn stub_5dd824() -> ! {
    todo!("0x5dd824 RBX::PartInstance::setPhysics(RBX::PV const&)")
}

// 0x5ddecc — __ZNK3RBX12PartInstance32computeNetworkOwnerIsSomeoneElseEv
#[doc(alias = "RBX::PartInstance::computeNetworkOwnerIsSomeoneElse(void)const")]
// was: __ZNK3RBX12PartInstance32computeNetworkOwnerIsSomeoneElseEv
pub fn stub_5ddecc() -> ! {
    todo!("0x5ddecc RBX::PartInstance::computeNetworkOwnerIsSomeoneElse(void)const")
}

// 0x5ddef8 — __ZNK3RBX12PartInstance12isProjectileEv
#[doc(alias = "RBX::PartInstance::isProjectile(void)const")]
// was: __ZNK3RBX12PartInstance12isProjectileEv
pub fn stub_5ddef8() -> ! {
    todo!("0x5ddef8 RBX::PartInstance::isProjectile(void)const")
}

// 0x5de174 — __ZNK3RBX12PartInstance16getTranslationUiEv
#[doc(alias = "RBX::PartInstance::getTranslationUi(void)const")]
// was: __ZNK3RBX12PartInstance16getTranslationUiEv
pub fn stub_5de174() -> ! {
    todo!("0x5de174 RBX::PartInstance::getTranslationUi(void)const")
}

// 0x5de240 — __ZNK3RBX12PartInstance13getRotationUiEv
#[doc(alias = "RBX::PartInstance::getRotationUi(void)const")]
// was: __ZNK3RBX12PartInstance13getRotationUiEv
pub fn stub_5de240() -> ! {
    todo!("0x5de240 RBX::PartInstance::getRotationUi(void)const")
}

// 0x5de394 — __ZNK3RBX12PartInstance11getVelocityEv
#[doc(alias = "RBX::PartInstance::getVelocity(void)const")]
// was: __ZNK3RBX12PartInstance11getVelocityEv
pub fn stub_5de394() -> ! {
    todo!("0x5de394 RBX::PartInstance::getVelocity(void)const")
}

// 0x5de534 — __ZN3RBX12PartInstance17refreshPartSizeUiEv
#[doc(alias = "RBX::PartInstance::refreshPartSizeUi(void)")]
// was: __ZN3RBX12PartInstance17refreshPartSizeUiEv
pub fn stub_5de534() -> ! {
    todo!("0x5de534 RBX::PartInstance::refreshPartSizeUi(void)")
}

// 0x5de570 — __ZN3RBX12PartInstance26onNetworkIsSleepingChangedEv
#[doc(alias = "RBX::PartInstance::onNetworkIsSleepingChanged(void)")]
// was: __ZN3RBX12PartInstance26onNetworkIsSleepingChangedEv
pub fn stub_5de570() -> ! {
    todo!("0x5de570 RBX::PartInstance::onNetworkIsSleepingChanged(void)")
}

// 0x5de59c — __ZThn96_N3RBX12PartInstance26onNetworkIsSleepingChangedEv
#[doc(alias = "`non-virtual thunk toRBX::PartInstance::onNetworkIsSleepingChanged(void)")]
// was: __ZThn96_N3RBX12PartInstance26onNetworkIsSleepingChangedEv
pub fn stub_5de59c() -> ! {
    todo!("0x5de59c `non-virtual thunk toRBX::PartInstance::onNetworkIsSleepingChanged(void)")
}

// 0x5de5a4 — __ZN3RBX12PartInstance13setCanCollideEb
#[doc(alias = "RBX::PartInstance::setCanCollide(bool)")]
// was: __ZN3RBX12PartInstance13setCanCollideEb
pub fn stub_5de5a4() -> ! {
    todo!("0x5de5a4 RBX::PartInstance::setCanCollide(bool)")
}

// 0x5de5dc — __ZN3RBX12PartInstance11setAnchoredEb
#[doc(alias = "RBX::PartInstance::setAnchored(bool)")]
// was: __ZN3RBX12PartInstance11setAnchoredEb
pub fn stub_5de5dc() -> ! {
    todo!("0x5de5dc RBX::PartInstance::setAnchored(bool)")
}

// 0x5de610 — __ZNK3RBX12PartInstance30getConnectedPartsRecursiveImplERN5boost10shared_ptrISt6vectorINS2_INS_8InstanceEEESaIS5_EEEERNS1_9unordered13unordered_setIPS0_NS1_4hashISC_EESt8equal_toISC_ESaISC_EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::PartInstance::getConnectedPartsRecursiveImpl(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &,boost::unordered::unordered_set<RBX::PartInstance*,boost::hash<RBX::PartInstance*>,std::equal_to<RBX::PartInstance*>,std::allocator<RBX::PartInstance*>> &)const")]
// was: __ZNK3RBX12PartInstance30getConnectedPartsRecursiveImplERN5boost10shared_ptrISt6vectorINS2_INS_8InstanceEEESaIS5_EEEERNS1_9unordered13unordered_setIPS0_NS1_4hashISC_EESt8equal_toISC_ESaISC_EEE
pub fn stub_5de610() -> ! {
    todo!("0x5de610 RBX::PartInstance::getConnectedPartsRecursiveImpl(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &,boost::unordered::unordered_set<RBX::PartInstance*,boost::hash<RBX::PartInstance*>,std::equal_to<RBX::PartInstance*>,std::allocator<RBX::PartInstance*>> &)const")
}

// 0x5de7fc — __ZN3RBXL24getConnectedPartsVisitorEPNS_9PrimitiveERN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::getConnectedPartsVisitor(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &)")]
// was: __ZN3RBXL24getConnectedPartsVisitorEPNS_9PrimitiveERN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
pub fn stub_5de7fc() -> ! {
    todo!("0x5de7fc RBX::getConnectedPartsVisitor(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &)")
}

// 0x5de910 — __ZN3RBX12PartInstance13setPartLockedEb
#[doc(alias = "RBX::PartInstance::setPartLocked(bool)")]
// was: __ZN3RBX12PartInstance13setPartLockedEb
pub fn stub_5de910() -> ! {
    todo!("0x5de910 RBX::PartInstance::setPartLocked(bool)")
}

// 0x5de94c — __ZN3RBX12PartInstance9getLockedEPNS_8InstanceE
#[doc(alias = "RBX::PartInstance::getLocked(RBX::Instance *)")]
// was: __ZN3RBX12PartInstance9getLockedEPNS_8InstanceE
pub fn stub_5de94c() -> ! {
    todo!("0x5de94c RBX::PartInstance::getLocked(RBX::Instance *)")
}

// 0x5de9bc — __ZN3RBX12PartInstance9setLockedEPNS_8InstanceEb
#[doc(alias = "RBX::PartInstance::setLocked(RBX::Instance *,bool)")]
// was: __ZN3RBX12PartInstance9setLockedEPNS_8InstanceEb
pub fn stub_5de9bc() -> ! {
    todo!("0x5de9bc RBX::PartInstance::setLocked(RBX::Instance *,bool)")
}

// 0x5dea94 — __ZN3RBX12PartInstance15setTransparencyEf
#[doc(alias = "RBX::PartInstance::setTransparency(float)")]
// was: __ZN3RBX12PartInstance15setTransparencyEf
pub fn stub_5dea94() -> ! {
    todo!("0x5dea94 RBX::PartInstance::setTransparency(float)")
}

// 0x5deadc — __ZN3RBX12PartInstance14setReflectanceEf
#[doc(alias = "RBX::PartInstance::setReflectance(float)")]
// was: __ZN3RBX12PartInstance14setReflectanceEf
pub fn stub_5deadc() -> ! {
    todo!("0x5deadc RBX::PartInstance::setReflectance(float)")
}

// 0x5deb1c — __ZN3RBX12PartInstance8setColorENS_10BrickColorE
#[doc(alias = "RBX::PartInstance::setColor(RBX::BrickColor)")]
// was: __ZN3RBX12PartInstance8setColorENS_10BrickColorE
pub fn stub_5deb1c() -> ! {
    todo!("0x5deb1c RBX::PartInstance::setColor(RBX::BrickColor)")
}

// 0x5deb60 — __ZN3RBX12PartInstance11setFrictionEf
#[doc(alias = "RBX::PartInstance::setFriction(float)")]
// was: __ZN3RBX12PartInstance11setFrictionEf
pub fn stub_5deb60() -> ! {
    todo!("0x5deb60 RBX::PartInstance::setFriction(float)")
}

// 0x5debb4 — __ZN3RBX12PartInstance13setElasticityEf
#[doc(alias = "RBX::PartInstance::setElasticity(float)")]
// was: __ZN3RBX12PartInstance13setElasticityEf
pub fn stub_5debb4() -> ! {
    todo!("0x5debb4 RBX::PartInstance::setElasticity(float)")
}

// 0x5dec08 — __ZN3RBX12PartInstance6resizeENS_8NormalIdEi
#[doc(alias = "RBX::PartInstance::resize(RBX::NormalId,int)")]
// was: __ZN3RBX12PartInstance6resizeENS_8NormalIdEi
pub fn stub_5dec08() -> ! {
    todo!("0x5dec08 RBX::PartInstance::resize(RBX::NormalId,int)")
}

// 0x5dec38 — __ZN3RBX12PartInstance10resizeImplENS_8NormalIdEi
#[doc(alias = "RBX::PartInstance::resizeImpl(RBX::NormalId,int)")]
// was: __ZN3RBX12PartInstance10resizeImplENS_8NormalIdEi
pub fn stub_5dec38() -> ! {
    todo!("0x5dec38 RBX::PartInstance::resizeImpl(RBX::NormalId,int)")
}

// 0x5defac — __ZN3RBX12PartInstance11resizeFloatENS_8NormalIdEfb
#[doc(alias = "RBX::PartInstance::resizeFloat(RBX::NormalId,float,bool)")]
// was: __ZN3RBX12PartInstance11resizeFloatENS_8NormalIdEfb
pub fn stub_5defac() -> ! {
    todo!("0x5defac RBX::PartInstance::resizeFloat(RBX::NormalId,float,bool)")
}

// 0x5defe8 — __ZN3RBX12PartInstance13advResizeImplENS_8NormalIdEfb
#[doc(alias = "RBX::PartInstance::advResizeImpl(RBX::NormalId,float,bool)")]
// was: __ZN3RBX12PartInstance13advResizeImplENS_8NormalIdEfb
pub fn stub_5defe8() -> ! {
    todo!("0x5defe8 RBX::PartInstance::advResizeImpl(RBX::NormalId,float,bool)")
}

// 0x5df538 — __ZN3RBX12PartInstance10getSurfaceERKNS_6RbxRayERi
#[doc(alias = "RBX::PartInstance::getSurface(RBX::RbxRay const&,int &)")]
// was: __ZN3RBX12PartInstance10getSurfaceERKNS_6RbxRayERi
pub fn stub_5df538() -> ! {
    todo!("0x5df538 RBX::PartInstance::getSurface(RBX::RbxRay const&,int &)")
}

// 0x5df668 — __ZN3RBX12PartInstance14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
#[doc(alias = "RBX::PartInstance::setSurfaceType(RBX::NormalId,RBX::SurfaceType)")]
// was: __ZN3RBX12PartInstance14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
pub fn stub_5df668() -> ! {
    todo!("0x5df668 RBX::PartInstance::setSurfaceType(RBX::NormalId,RBX::SurfaceType)")
}

// 0x5df6a0 — __ZNK3RBX12PartInstance8getInputENS_8NormalIdE
#[doc(alias = "RBX::PartInstance::getInput(RBX::NormalId)const")]
// was: __ZNK3RBX12PartInstance8getInputENS_8NormalIdE
pub fn stub_5df6a0() -> ! {
    todo!("0x5df6a0 RBX::PartInstance::getInput(RBX::NormalId)const")
}

// 0x5df6c8 — __ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE
#[doc(alias = "RBX::PartInstance::setSurfaceInput(RBX::NormalId,RBX::LegacyController::InputType)")]
// was: __ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE
pub fn stub_5df6c8() -> ! {
    todo!("0x5df6c8 RBX::PartInstance::setSurfaceInput(RBX::NormalId,RBX::LegacyController::InputType)")
}

// 0x5df788 — __ZNK3RBX12PartInstance9getParamAENS_8NormalIdE
#[doc(alias = "RBX::PartInstance::getParamA(RBX::NormalId)const")]
// was: __ZNK3RBX12PartInstance9getParamAENS_8NormalIdE
pub fn stub_5df788() -> ! {
    todo!("0x5df788 RBX::PartInstance::getParamA(RBX::NormalId)const")
}

// 0x5df804 — __ZNK3RBX12PartInstance9getParamBENS_8NormalIdE
#[doc(alias = "RBX::PartInstance::getParamB(RBX::NormalId)const")]
// was: __ZNK3RBX12PartInstance9getParamBENS_8NormalIdE
pub fn stub_5df804() -> ! {
    todo!("0x5df804 RBX::PartInstance::getParamB(RBX::NormalId)const")
}

// 0x5df880 — __ZN3RBX12PartInstance9setParamAENS_8NormalIdEf
#[doc(alias = "RBX::PartInstance::setParamA(RBX::NormalId,float)")]
// was: __ZN3RBX12PartInstance9setParamAENS_8NormalIdEf
pub fn stub_5df880() -> ! {
    todo!("0x5df880 RBX::PartInstance::setParamA(RBX::NormalId,float)")
}

// 0x5df8f8 — __ZN3RBX12PartInstance9setParamBENS_8NormalIdEf
#[doc(alias = "RBX::PartInstance::setParamB(RBX::NormalId,float)")]
// was: __ZN3RBX12PartInstance9setParamBENS_8NormalIdEf
pub fn stub_5df8f8() -> ! {
    todo!("0x5df8f8 RBX::PartInstance::setParamB(RBX::NormalId,float)")
}

// 0x5df970 — __ZNK3RBX12PartInstance18containedByFrustumERNS_7FrustumE
#[doc(alias = "RBX::PartInstance::containedByFrustum(RBX::Frustum &)const")]
// was: __ZNK3RBX12PartInstance18containedByFrustumERNS_7FrustumE
pub fn stub_5df970() -> ! {
    todo!("0x5df970 RBX::PartInstance::containedByFrustum(RBX::Frustum &)const")
}

// 0x5df9dc — __ZNK3RBX12PartInstance14isStandardPartEv
#[doc(alias = "RBX::PartInstance::isStandardPart(void)const")]
// was: __ZNK3RBX12PartInstance14isStandardPartEv
pub fn stub_5df9dc() -> ! {
    todo!("0x5df9dc RBX::PartInstance::isStandardPart(void)const")
}

// 0x5dfc80 — __ZN3RBXL12getComponentEPK10XmlElementRKNS_4NameE
#[doc(alias = "RBX::getComponent(XmlElement const*,RBX::Name const&)")]
// was: __ZN3RBXL12getComponentEPK10XmlElementRKNS_4NameE
pub fn stub_5dfc80() -> ! {
    todo!("0x5dfc80 RBX::getComponent(XmlElement const*,RBX::Name const&)")
}

// 0x5e01a0 — __ZN3RBX12PartInstance17fireOutfitChangedEv
#[doc(alias = "RBX::PartInstance::fireOutfitChanged(void)")]
// was: __ZN3RBX12PartInstance17fireOutfitChangedEv
pub fn stub_5e01a0() -> ! {
    todo!("0x5e01a0 RBX::PartInstance::fireOutfitChanged(void)")
}

// 0x5e01dc — __ZN3RBX12PartInstance11reportTouchERKN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::PartInstance::reportTouch(boost::shared_ptr<RBX::PartInstance> const&)")]
// was: __ZN3RBX12PartInstance11reportTouchERKN5boost10shared_ptrIS0_EE
pub fn stub_5e01dc() -> ! {
    todo!("0x5e01dc RBX::PartInstance::reportTouch(boost::shared_ptr<RBX::PartInstance> const&)")
}

// 0x5e02d8 — __ZN3RBX12PartInstance32setIsCurrentlyStreamRemovingPartEv
#[doc(alias = "RBX::PartInstance::setIsCurrentlyStreamRemovingPart(void)")]
// was: __ZN3RBX12PartInstance32setIsCurrentlyStreamRemovingPartEv
pub fn stub_5e02d8() -> ! {
    todo!("0x5e02d8 RBX::PartInstance::setIsCurrentlyStreamRemovingPart(void)")
}

// 0x5e02e8 — __ZN3RBX10hash_valueERKNS_14FWPartInstanceE
#[doc(alias = "RBX::hash_value(RBX::FWPartInstance const&)")]
// was: __ZN3RBX10hash_valueERKNS_14FWPartInstanceE
pub fn stub_5e02e8() -> ! {
    todo!("0x5e02e8 RBX::hash_value(RBX::FWPartInstance const&)")
}

// 0x5e03b4 — __ZN3RBX14FWPartInstanceC2Ev
#[doc(alias = "RBX::FWPartInstance::FWPartInstance(void)")]
// was: __ZN3RBX14FWPartInstanceC2Ev
pub fn stub_5e03b4() -> ! {
    todo!("0x5e03b4 RBX::FWPartInstance::FWPartInstance(void)")
}

// 0x5e04b4 — __ZNK3RBX14FWPartInstanceeqERKS0_
#[doc(alias = "RBX::FWPartInstance::operator==(RBX::FWPartInstance const&)const")]
// was: __ZNK3RBX14FWPartInstanceeqERKS0_
pub fn stub_5e04b4() -> ! {
    todo!("0x5e04b4 RBX::FWPartInstance::operator==(RBX::FWPartInstance const&)const")
}

// 0x5e0580 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEfED1Ev
pub fn stub_5e0580() -> ! {
    todo!("0x5e0580 RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::~PropDescriptor()")
}

// 0x5e05a4 — __ZN3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor>::addPair(RBX::PartInstance::FormFactor,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE7addPairES3_PKc
pub fn stub_5e05a4() -> ! {
    todo!("0x5e05a4 RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor>::addPair(RBX::PartInstance::FormFactor,char const*)")
}

// 0x5e0904 — __ZN3RBX10Reflection8EnumDescINS_8MaterialEE7addPairES2_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::addPair(RBX::Material,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_8MaterialEE7addPairES2_PKc
pub fn stub_5e0904() -> ! {
    todo!("0x5e0904 RBX::Reflection::EnumDesc<RBX::Material>::addPair(RBX::Material,char const*)")
}

// 0x5e0c64 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EED1Ev
pub fn stub_5e0c64() -> ! {
    todo!("0x5e0c64 RBX::Reflection::BoundFuncDesc<RBX::PartInstance,void ()(void),0>::~BoundFuncDesc()")
}

// 0x5e0c88 — __ZN3RBX12PartInstance15getMassNonConstEv
#[doc(alias = "RBX::PartInstance::getMassNonConst(void)")]
// was: __ZN3RBX12PartInstance15getMassNonConstEv
pub fn stub_5e0c88() -> ! {
    todo!("0x5e0c88 RBX::PartInstance::getMassNonConst(void)")
}

// 0x5e0c98 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,float ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EED1Ev
pub fn stub_5e0c98() -> ! {
    todo!("0x5e0c98 RBX::Reflection::BoundFuncDesc<RBX::PartInstance,float ()(void),0>::~BoundFuncDesc()")
}

// 0x5e0cbc — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EED1Ev
pub fn stub_5e0cbc() -> ! {
    todo!("0x5e0cbc RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(void),0>::~BoundFuncDesc()")
}

// 0x5e0ce0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED1Ev
pub fn stub_5e0ce0() -> ! {
    todo!("0x5e0ce0 RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()")
}

// 0x5e0d20 — __ZNK3RBX12PartInstance9getColor3Ev
#[doc(alias = "RBX::PartInstance::getColor3(void)const")]
// was: __ZNK3RBX12PartInstance9getColor3Ev
pub fn stub_5e0d20() -> ! {
    todo!("0x5e0d20 RBX::PartInstance::getColor3(void)const")
}

// 0x5e0d90 — __ZNK3RBX12PartInstance8getColorEv
#[doc(alias = "RBX::PartInstance::getColor(void)const")]
// was: __ZNK3RBX12PartInstance8getColorEv
pub fn stub_5e0d90() -> ! {
    todo!("0x5e0d90 RBX::PartInstance::getColor(void)const")
}

// 0x5e0d98 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_10BrickColorEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::BrickColor>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_10BrickColorEED1Ev
pub fn stub_5e0d98() -> ! {
    todo!("0x5e0d98 RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::BrickColor>::~PropDescriptor()")
}

// 0x5e0dc4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED1Ev
pub fn stub_5e0dc4() -> ! {
    todo!("0x5e0dc4 RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::~EnumPropDescriptor()")
}

// 0x5e0de8 — __ZNK3RBX12PartInstance18getTransparencyXmlEv
#[doc(alias = "RBX::PartInstance::getTransparencyXml(void)const")]
// was: __ZNK3RBX12PartInstance18getTransparencyXmlEv
pub fn stub_5e0de8() -> ! {
    todo!("0x5e0de8 RBX::PartInstance::getTransparencyXml(void)const")
}

// 0x5e0df0 — __ZNK3RBX12PartInstance16getAlphaModifierEv
#[doc(alias = "RBX::PartInstance::getAlphaModifier(void)const")]
// was: __ZNK3RBX12PartInstance16getAlphaModifierEv
pub fn stub_5e0df0() -> ! {
    todo!("0x5e0df0 RBX::PartInstance::getAlphaModifier(void)const")
}

// 0x5e0df8 — __ZNK3RBX12PartInstance14getReflectanceEv
#[doc(alias = "RBX::PartInstance::getReflectance(void)const")]
// was: __ZNK3RBX12PartInstance14getReflectanceEv
pub fn stub_5e0df8() -> ! {
    todo!("0x5e0df8 RBX::PartInstance::getReflectance(void)const")
}

// 0x5e0e00 — __ZNK3RBX12PartInstance13getPartLockedEv
#[doc(alias = "RBX::PartInstance::getPartLocked(void)const")]
// was: __ZNK3RBX12PartInstance13getPartLockedEv
pub fn stub_5e0e00() -> ! {
    todo!("0x5e0e00 RBX::PartInstance::getPartLocked(void)const")
}

// 0x5e0e08 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED1Ev
pub fn stub_5e0e08() -> ! {
    todo!("0x5e0e08 RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::~PropDescriptor()")
}

// 0x5e0e2c — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED1Ev
pub fn stub_5e0e2c() -> ! {
    todo!("0x5e0e2c RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::~PropDescriptor()")
}

// 0x5e0e50 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED1Ev
pub fn stub_5e0e50() -> ! {
    todo!("0x5e0e50 RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::~PropDescriptor()")
}

// 0x5e0e74 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED1Ev
pub fn stub_5e0e74() -> ! {
    todo!("0x5e0e74 RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::~BoundFuncDesc()")
}

// 0x5e0ebc — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED1Ev
pub fn stub_5e0ebc() -> ! {
    todo!("0x5e0ebc RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::~PropDescriptor()")
}

// 0x5e0ee0 — __ZN3RBX12PartInstance39getOrCreateLocalSimulationTouchedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateLocalSimulationTouchedSignal(void)")]
// was: __ZN3RBX12PartInstance39getOrCreateLocalSimulationTouchedSignalEv
pub fn stub_5e0ee0() -> ! {
    todo!("0x5e0ee0 RBX::PartInstance::getOrCreateLocalSimulationTouchedSignal(void)")
}

// 0x5e0eec — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED1Ev
pub fn stub_5e0eec() -> ! {
    todo!("0x5e0eec RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()")
}

// 0x5e0f10 — __ZN3RBX12PartInstance24getOrCreateTouchedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateTouchedSignal(void)")]
// was: __ZN3RBX12PartInstance24getOrCreateTouchedSignalEv
pub fn stub_5e0f10() -> ! {
    todo!("0x5e0f10 RBX::PartInstance::getOrCreateTouchedSignal(void)")
}

// 0x5e0f1c — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED1Ev
pub fn stub_5e0f1c() -> ! {
    todo!("0x5e0f1c RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()")
}

// 0x5e0f40 — __ZN3RBX12PartInstance29getOrCreateTouchedEndedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateTouchedEndedSignal(void)")]
// was: __ZN3RBX12PartInstance29getOrCreateTouchedEndedSignalEv
pub fn stub_5e0f40() -> ! {
    todo!("0x5e0f40 RBX::PartInstance::getOrCreateTouchedEndedSignal(void)")
}

// 0x5e0f4c — __ZN3RBX12PartInstance42getOrCreateDeprecatedStoppedTouchingSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateDeprecatedStoppedTouchingSignal(void)")]
// was: __ZN3RBX12PartInstance42getOrCreateDeprecatedStoppedTouchingSignalEv
pub fn stub_5e0f4c() -> ! {
    todo!("0x5e0f4c RBX::PartInstance::getOrCreateDeprecatedStoppedTouchingSignal(void)")
}

// 0x5e0f58 — __ZN3RBX12PartInstance30getOrCreateOutfitChangedSignalEv
#[doc(alias = "RBX::PartInstance::getOrCreateOutfitChangedSignal(void)")]
// was: __ZN3RBX12PartInstance30getOrCreateOutfitChangedSignalEv
pub fn stub_5e0f58() -> ! {
    todo!("0x5e0f58 RBX::PartInstance::getOrCreateOutfitChangedSignal(void)")
}

// 0x5e0f64 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED1Ev
pub fn stub_5e0f64() -> ! {
    todo!("0x5e0f64 RBX::Reflection::EventDesc<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::~EventDesc()")
}

// 0x5e0f88 — __ZNK3RBX12PartInstance18getReceiveIntervalEv
#[doc(alias = "RBX::PartInstance::getReceiveInterval(void)const")]
// was: __ZNK3RBX12PartInstance18getReceiveIntervalEv
pub fn stub_5e0f88() -> ! {
    todo!("0x5e0f88 RBX::PartInstance::getReceiveInterval(void)const")
}

// 0x5e0f90 — __ZNK3RBX9Primitive15getExtentsWorldEv
#[doc(alias = "RBX::Primitive::getExtentsWorld(void)const")]
// was: __ZNK3RBX9Primitive15getExtentsWorldEv
pub fn stub_5e0f90() -> ! {
    todo!("0x5e0f90 RBX::Primitive::getExtentsWorld(void)const")
}

// 0x5e0ff8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
pub fn stub_5e0ff8() -> ! {
    todo!("0x5e0ff8 boost::shared_ptr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)")
}

// 0x5e10ac — __ZN3RBX18OnDemandPVInstanceC2Ev
#[doc(alias = "RBX::OnDemandPVInstance::OnDemandPVInstance(void)")]
// was: __ZN3RBX18OnDemandPVInstanceC2Ev
pub fn stub_5e10ac() -> ! {
    todo!("0x5e10ac RBX::OnDemandPVInstance::OnDemandPVInstance(void)")
}

// 0x5e1178 — __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::Allocator(void)")]
// was: __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEC2Ev
pub fn stub_5e1178() -> ! {
    todo!("0x5e1178 RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::Allocator(void)")
}

// 0x5e11dc — __ZN3RBX6FWBase4initINS_14FWPartInstanceEEEPT_S4_
#[doc(alias = "RBX::FWPartInstance * RBX::FWBase::init<RBX::FWPartInstance>(RBX::FWPartInstance *)")]
// was: __ZN3RBX6FWBase4initINS_14FWPartInstanceEEEPT_S4_
pub fn stub_5e11dc() -> ! {
    todo!("0x5e11dc RBX::FWPartInstance * RBX::FWBase::init<RBX::FWPartInstance>(RBX::FWPartInstance *)")
}

// 0x5e12a8 — __ZN3RBX7IMovingD2Ev
#[doc(alias = "RBX::IMoving::~IMoving()")]
// was: __ZN3RBX7IMovingD2Ev
pub fn stub_5e12a8() -> ! {
    todo!("0x5e12a8 RBX::IMoving::~IMoving()")
}

// 0x5e1314 — __ZN3RBX12PartInstance20OnDemandPartInstancedlEPv
#[doc(alias = "RBX::PartInstance::OnDemandPartInstance::operator delete(void *)")]
// was: __ZN3RBX12PartInstance20OnDemandPartInstancedlEPv
pub fn stub_5e1314() -> ! {
    todo!("0x5e1314 RBX::PartInstance::OnDemandPartInstance::operator delete(void *)")
}

// 0x5e1468 — __ZN3RBX7Dragger8dragSnapEv
#[doc(alias = "RBX::Dragger::dragSnap(void)")]
// was: __ZN3RBX7Dragger8dragSnapEv
pub fn stub_5e1468() -> ! {
    todo!("0x5e1468 RBX::Dragger::dragSnap(void)")
}

// 0x5e14bc — __ZNK3RBX5Joint11getNormalIdEi
#[doc(alias = "RBX::Joint::getNormalId(int)const")]
// was: __ZNK3RBX5Joint11getNormalIdEi
pub fn stub_5e14bc() -> ! {
    todo!("0x5e14bc RBX::Joint::getNormalId(int)const")
}

// 0x5e1534 — __ZNK3RBX12PartInstance19hasTouchTransmitterEv
#[doc(alias = "RBX::PartInstance::hasTouchTransmitter(void)const")]
// was: __ZNK3RBX12PartInstance19hasTouchTransmitterEv
pub fn stub_5e1534() -> ! {
    todo!("0x5e1534 RBX::PartInstance::hasTouchTransmitter(void)const")
}

// 0x5e1558 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_16TouchTransmitterEEEPKT_v
#[doc(alias = "RBX::TouchTransmitter const* RBX::Instance::findConstFirstChildOfType<RBX::TouchTransmitter>(void)const")]
// was: __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_16TouchTransmitterEEEPKT_v
pub fn stub_5e1558() -> ! {
    todo!("0x5e1558 RBX::TouchTransmitter const* RBX::Instance::findConstFirstChildOfType<RBX::TouchTransmitter>(void)const")
}

// 0x5e15c0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::push_back(boost::shared_ptr<RBX::PartInstance> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
pub fn stub_5e15c0() -> ! {
    todo!("0x5e15c0 std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::push_back(boost::shared_ptr<RBX::PartInstance> const&)")
}

// 0x5e1610 — __ZN3RBX11shared_fromINS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "boost::shared_ptr<RBX::PartInstance> RBX::shared_from<RBX::PartInstance>(RBX::PartInstance*)")]
// was: __ZN3RBX11shared_fromINS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_5e1610() -> ! {
    todo!("0x5e1610 boost::shared_ptr<RBX::PartInstance> RBX::shared_from<RBX::PartInstance>(RBX::PartInstance*)")
}

// 0x5e1780 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::push_back(boost::weak_ptr<RBX::PartInstance> const&)")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
pub fn stub_5e1780() -> ! {
    todo!("0x5e1780 std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::push_back(boost::weak_ptr<RBX::PartInstance> const&)")
}

// 0x5e1810 — __ZNK3RBX8Instance11numChildrenEv
#[doc(alias = "RBX::Instance::numChildren(void)const")]
// was: __ZNK3RBX8Instance11numChildrenEv
pub fn stub_5e1810() -> ! {
    todo!("0x5e1810 RBX::Instance::numChildren(void)const")
}

// 0x5e1830 — __ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(bool)>::operator()(bool)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb
pub fn stub_5e1830() -> ! {
    todo!("0x5e1830 rbx::signals::signal_with_args<1,void ()(bool)>::operator()(bool)")
}

// 0x5e1978 — __ZN3RBX8Instance17onServiceProviderEPNS_15ServiceProviderES2_
#[doc(alias = "RBX::Instance::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8Instance17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_5e1978() -> ! {
    todo!("0x5e1978 RBX::Instance::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x5e197c — __ZN3RBX13FWDictionnaryINS_14FWPartInstanceEE17registerFlyweightEPNS_5FWRefE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::FWDictionnary<RBX::FWPartInstance>::registerFlyweight(RBX::FWRef *)")]
// was: __ZN3RBX13FWDictionnaryINS_14FWPartInstanceEE17registerFlyweightEPNS_5FWRefE
pub fn stub_5e197c() -> ! {
    todo!("0x5e197c RBX::FWDictionnary<RBX::FWPartInstance>::registerFlyweight(RBX::FWRef *)")
}

// 0x5e1bc0 — __ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
#[doc(alias = "RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
pub fn stub_5e1bc0() -> ! {
    todo!("0x5e1bc0 RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")
}

// 0x5e1de8 — __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
#[doc(alias = "RBX::Network::NetworkOwner::ServerUnassigned(void)")]
// was: __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
pub fn stub_5e1de8() -> ! {
    todo!("0x5e1de8 RBX::Network::NetworkOwner::ServerUnassigned(void)")
}

// 0x5e1e40 — __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
#[doc(alias = "RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")]
// was: __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
pub fn stub_5e1e40() -> ! {
    todo!("0x5e1e40 RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")
}

// 0x5e1eac — __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
#[doc(alias = "RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")]
// was: __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
pub fn stub_5e1eac() -> ! {
    todo!("0x5e1eac RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")
}

// 0x5e1ef8 — __ZN3RBX7Network12NetworkOwner6ServerEv
#[doc(alias = "RBX::Network::NetworkOwner::Server(void)")]
// was: __ZN3RBX7Network12NetworkOwner6ServerEv
pub fn stub_5e1ef8() -> ! {
    todo!("0x5e1ef8 RBX::Network::NetworkOwner::Server(void)")
}

// 0x5e1f50 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::push_back(boost::shared_ptr<RBX::Instance> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
pub fn stub_5e1f50() -> ! {
    todo!("0x5e1f50 std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::push_back(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x5e1fa0 — __ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>)")]
// was: __ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
pub fn stub_5e1fa0() -> ! {
    todo!("0x5e1fa0 void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>)")
}

// 0x5e20e4 — __ZN5boost4bindIvPN3RBX9PrimitiveERNS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list_av_2<boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &,boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
// was: __ZN5boost4bindIvPN3RBX9PrimitiveERNS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
pub fn stub_5e20e4() -> ! {
    todo!("0x5e20e4 boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list_av_2<boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &,boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")
}

// 0x5e21fc — __ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
#[doc(alias = "RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
pub fn stub_5e21fc() -> ! {
    todo!("0x5e21fc RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")
}

// 0x5e2424 — __ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
#[doc(alias = "RBX::FWValue<float>::set(float const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
pub fn stub_5e2424() -> ! {
    todo!("0x5e2424 RBX::FWValue<float>::set(float const&,RBX::FWRef *)")
}

// 0x5e2658 — __ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
#[doc(alias = "RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")]
// was: __ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
pub fn stub_5e2658() -> ! {
    todo!("0x5e2658 RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")
}

// 0x5e2884 — __ZN3RBX8Instance23OutfitChangedSignalDataD1Ev
#[doc(alias = "RBX::Instance::OutfitChangedSignalData::~OutfitChangedSignalData()")]
// was: __ZN3RBX8Instance23OutfitChangedSignalDataD1Ev
pub fn stub_5e2884() -> ! {
    todo!("0x5e2884 RBX::Instance::OutfitChangedSignalData::~OutfitChangedSignalData()")
}

// 0x5e2888 — __ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartInstance::TouchedSignal::operator()(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5e2888() -> ! {
    todo!("0x5e2888 RBX::PartInstance::TouchedSignal::operator()(boost::shared_ptr<RBX::Instance>)")
}

// 0x5e2978 — __ZN3RBX9AllocatorINS_14FWPartInstanceEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::FWPartInstance>::Allocator(void)")]
// was: __ZN3RBX9AllocatorINS_14FWPartInstanceEEC2Ev
pub fn stub_5e2978() -> ! {
    todo!("0x5e2978 RBX::Allocator<RBX::FWPartInstance>::Allocator(void)")
}

// 0x5e29dc — __ZNK3RBX12PartInstance21getPersistentDataCostEv
#[doc(alias = "RBX::PartInstance::getPersistentDataCost(void)const")]
// was: __ZNK3RBX12PartInstance21getPersistentDataCostEv
pub fn stub_5e29dc() -> ! {
    todo!("0x5e29dc RBX::PartInstance::getPersistentDataCost(void)const")
}

// 0x5e29e8 — __ZN3RBX8Instance15canClientCreateEv
#[doc(alias = "RBX::Instance::canClientCreate(void)")]
// was: __ZN3RBX8Instance15canClientCreateEv
pub fn stub_5e29e8() -> ! {
    todo!("0x5e29e8 RBX::Instance::canClientCreate(void)")
}

// 0x5e29f0 — __ZN3RBX8Instance12onChildAddedEPS0_
#[doc(alias = "RBX::Instance::onChildAdded(RBX::Instance*)")]
// was: __ZN3RBX8Instance12onChildAddedEPS0_
pub fn stub_5e29f0() -> ! {
    todo!("0x5e29f0 RBX::Instance::onChildAdded(RBX::Instance*)")
}

// 0x5e29f8 — __ZNK3RBX10PVInstance13childHashCodeEv
#[doc(alias = "RBX::PVInstance::childHashCode(void)const")]
// was: __ZNK3RBX10PVInstance13childHashCodeEv
pub fn stub_5e29f8() -> ! {
    todo!("0x5e29f8 RBX::PVInstance::childHashCode(void)const")
}

// 0x5e29fc — __ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv
pub fn stub_5e29fc() -> ! {
    todo!("0x5e29fc __ZNK3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEE12getClassNameEv")
}

// 0x5e2a24 — __ZN3RBX12PartInstance23hasThreeDimensionalSizeEv
#[doc(alias = "RBX::PartInstance::hasThreeDimensionalSize(void)")]
// was: __ZN3RBX12PartInstance23hasThreeDimensionalSizeEv
pub fn stub_5e2a24() -> ! {
    todo!("0x5e2a24 RBX::PartInstance::hasThreeDimensionalSize(void)")
}

// 0x5e2a28 — __ZNK3RBX12PartInstance11getPartTypeEv
#[doc(alias = "RBX::PartInstance::getPartType(void)const")]
// was: __ZNK3RBX12PartInstance11getPartTypeEv
pub fn stub_5e2a28() -> ! {
    todo!("0x5e2a28 RBX::PartInstance::getPartType(void)const")
}

// 0x5e2a2c — __ZNK3RBX12PartInstance16getMinimumUiSizeEv
#[doc(alias = "RBX::PartInstance::getMinimumUiSize(void)const")]
// was: __ZNK3RBX12PartInstance16getMinimumUiSizeEv
pub fn stub_5e2a2c() -> ! {
    todo!("0x5e2a2c RBX::PartInstance::getMinimumUiSize(void)const")
}

// 0x5e2a78 — __ZNK3RBX12PartInstance22getMinimumUiSizeCustomEv
#[doc(alias = "RBX::PartInstance::getMinimumUiSizeCustom(void)const")]
// was: __ZNK3RBX12PartInstance22getMinimumUiSizeCustomEv
pub fn stub_5e2a78() -> ! {
    todo!("0x5e2a78 RBX::PartInstance::getMinimumUiSizeCustom(void)const")
}

// 0x5e2ac8 — __ZNK3RBX12PartInstance19getResizeHandleMaskEv
#[doc(alias = "RBX::PartInstance::getResizeHandleMask(void)const")]
// was: __ZNK3RBX12PartInstance19getResizeHandleMaskEv
pub fn stub_5e2ac8() -> ! {
    todo!("0x5e2ac8 RBX::PartInstance::getResizeHandleMask(void)const")
}

// 0x5e2adc — __ZNK3RBX12PartInstance23getDragUtilitiesSupportEv
#[doc(alias = "RBX::PartInstance::getDragUtilitiesSupport(void)const")]
// was: __ZNK3RBX12PartInstance23getDragUtilitiesSupportEv
pub fn stub_5e2adc() -> ! {
    todo!("0x5e2adc RBX::PartInstance::getDragUtilitiesSupport(void)const")
}

// 0x5e2ae0 — __ZNK3RBX12PartInstance18getResizeIncrementEv
#[doc(alias = "RBX::PartInstance::getResizeIncrement(void)const")]
// was: __ZNK3RBX12PartInstance18getResizeIncrementEv
pub fn stub_5e2ae0() -> ! {
    todo!("0x5e2ae0 RBX::PartInstance::getResizeIncrement(void)const")
}

// 0x5e2ae4 — __ZNK3RBX12PartInstance25getMinimumResizeIncrementEv
#[doc(alias = "RBX::PartInstance::getMinimumResizeIncrement(void)const")]
// was: __ZNK3RBX12PartInstance25getMinimumResizeIncrementEv
pub fn stub_5e2ae4() -> ! {
    todo!("0x5e2ae4 RBX::PartInstance::getMinimumResizeIncrement(void)const")
}

// 0x5e2af0 — __ZNK3RBX12PartInstance13getFormFactorEv
#[doc(alias = "RBX::PartInstance::getFormFactor(void)const")]
// was: __ZNK3RBX12PartInstance13getFormFactorEv
pub fn stub_5e2af0() -> ! {
    todo!("0x5e2af0 RBX::PartInstance::getFormFactor(void)const")
}

// 0x5e2af4 — __ZN3RBX12PartInstance11getLocationEv
#[doc(alias = "RBX::PartInstance::getLocation(void)")]
// was: __ZN3RBX12PartInstance11getLocationEv
pub fn stub_5e2af4() -> ! {
    todo!("0x5e2af4 RBX::PartInstance::getLocation(void)")
}
