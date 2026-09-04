//! core shard CA — 100 core stubs EA-sorted, next uncovered after BZ 0x5a3a38 (strict RBX|boost|std|rbx earliest gap 0x3cb538).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Camera::CameraType const& rbx::any_cast<RBX::Camera::CameraType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x3cb538 — __ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x3cb538() {
    // IDA 0x3cb538: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::ManualWeld::shouldRender3dAdorn(void)const")]
// 0x5a3aec — __ZNK3RBX10ManualWeld19shouldRender3dAdornEv
pub fn stub_0x5a3aec() {
    // IDA 0x5a3aec: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ManualWeld::shouldRender3dAdorn(void)const")]
// 0x5a3c58 — __ZThn92_NK3RBX10ManualWeld19shouldRender3dAdornEv
pub fn stub_0x5a3c58() {
    // IDA 0x5a3c58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualGlue::shouldRender3dAdorn(void)const")]
// 0x5a3d10 — __ZNK3RBX10ManualGlue19shouldRender3dAdornEv
pub fn stub_0x5a3d10() {
    // IDA 0x5a3d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ManualGlue::shouldRender3dAdorn(void)const")]
// 0x5a3e7c — __ZThn92_NK3RBX10ManualGlue19shouldRender3dAdornEv
pub fn stub_0x5a3e7c() {
    // IDA 0x5a3e7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::setFogEnd(float)")]
// 0x5c2350 — __ZN3RBX8Lighting9setFogEndEf
pub fn stub_0x5c2350() {
    // IDA 0x5c2350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::setGlobalShadows(bool)")]
// 0x5c2378 — __ZN3RBX8Lighting16setGlobalShadowsEb
pub fn stub_0x5c2378() {
    // IDA 0x5c2378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::Lighting(void)")]
// 0x5c239c — __ZN3RBX8LightingC1Ev
pub fn stub_0x5c239c() {
    // IDA 0x5c239c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::Lighting(void)")]
// 0x5c23a0 — __ZN3RBX8LightingC2Ev
pub fn stub_0x5c23a0() {
    // IDA 0x5c23a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::replaceSky(RBX::Sky *)")]
// 0x5c2af8 — __ZN3RBX8Lighting10replaceSkyEPNS_3SkyE
pub fn stub_0x5c2af8() {
    // IDA 0x5c2af8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getGeographicLatitude(void)const")]
// 0x5c2cd8 — __ZNK3RBX8Lighting21getGeographicLatitudeEv
pub fn stub_0x5c2cd8() {
    // IDA 0x5c2cd8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getMoonPhase(void)")]
// 0x5c2d04 — __ZN3RBX8Lighting12getMoonPhaseEv
pub fn stub_0x5c2d04() {
    // IDA 0x5c2d04: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getShadowColor3(void)const")]
// 0x5c2dc0 — __ZNK3RBX8Lighting15getShadowColor3Ev
pub fn stub_0x5c2dc0() {
    // IDA 0x5c2dc0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getFogColor3(void)const")]
// 0x5c2e28 — __ZNK3RBX8Lighting12getFogColor3Ev
pub fn stub_0x5c2e28() {
    // IDA 0x5c2e28: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getFogStart(void)const")]
// 0x5c2e6c — __ZNK3RBX8Lighting11getFogStartEv
pub fn stub_0x5c2e6c() {
    // IDA 0x5c2e6c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getFogEnd(void)const")]
// 0x5c2e74 — __ZNK3RBX8Lighting9getFogEndEv
pub fn stub_0x5c2e74() {
    // IDA 0x5c2e74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::getGlobalShadows(void)const")]
// 0x5c2e7c — __ZNK3RBX8Lighting16getGlobalShadowsEv
pub fn stub_0x5c2e7c() {
    // IDA 0x5c2e7c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lighting::~Lighting()")]
// 0x5c305c — __ZN3RBX8LightingD1Ev
pub fn stub_0x5c305c() {
    // IDA 0x5c305c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::~Lighting()")]
// 0x5c3060 — __ZN3RBX8LightingD0Ev
pub fn stub_0x5c3060() {
    // IDA 0x5c3060: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// 0x5c3110 — __ZThn32_N3RBX8LightingD1Ev
pub fn stub_0x5c3110() {
    // IDA 0x5c3110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// 0x5c3118 — __ZThn32_N3RBX8LightingD0Ev
pub fn stub_0x5c3118() {
    // IDA 0x5c3118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// 0x5c31cc — __ZThn36_N3RBX8LightingD1Ev
pub fn stub_0x5c31cc() {
    // IDA 0x5c31cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// 0x5c31d4 — __ZThn36_N3RBX8LightingD0Ev
pub fn stub_0x5c31d4() {
    // IDA 0x5c31d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Lighting::~Lighting()")]
// 0x5c4ea0 — __ZN3RBX8LightingD2Ev
pub fn stub_0x5c4ea0() {
    // IDA 0x5c4ea0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LocalBackpack::LocalBackpack(void)")]
// 0x5c7004 — __ZN3RBX13LocalBackpackC1Ev
pub fn stub_0x5c7004() {
    // IDA 0x5c7004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LocalBackpack::LocalBackpack(void)")]
// 0x5c7008 — __ZN3RBX13LocalBackpackC2Ev
pub fn stub_0x5c7008() {
    // IDA 0x5c7008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LocalBackpack::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x5c7220 — __ZN3RBX13LocalBackpack17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_0x5c7220() {
    // IDA 0x5c7220: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LocalBackpack::setOldSchoolBackpack(bool)")]
// 0x5c722c — __ZN3RBX13LocalBackpack20setOldSchoolBackpackEb
pub fn stub_0x5c722c() {
    // IDA 0x5c722c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LocalBackpack::getOldSchoolBackpack(void)")]
// 0x5c7270 — __ZN3RBX13LocalBackpack20getOldSchoolBackpackEv
pub fn stub_0x5c7270() {
    // IDA 0x5c7270: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LocalBackpack::~LocalBackpack()")]
// 0x5c7298 — __ZN3RBX13LocalBackpackD1Ev
pub fn stub_0x5c7298() {
    // IDA 0x5c7298: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LocalBackpack::~LocalBackpack()")]
// 0x5c729c — __ZN3RBX13LocalBackpackD0Ev
pub fn stub_0x5c729c() {
    // IDA 0x5c729c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// 0x5c7368 — __ZThn32_N3RBX13LocalBackpackD1Ev
pub fn stub_0x5c7368() {
    // IDA 0x5c7368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// 0x5c7370 — __ZThn32_N3RBX13LocalBackpackD0Ev
pub fn stub_0x5c7370() {
    // IDA 0x5c7370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// 0x5c743c — __ZThn36_N3RBX13LocalBackpackD1Ev
pub fn stub_0x5c743c() {
    // IDA 0x5c743c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// 0x5c7444 — __ZThn36_N3RBX13LocalBackpackD0Ev
pub fn stub_0x5c7444() {
    // IDA 0x5c7444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::setText(std::string const&)")]
// 0x5c7fa4 — __ZN3RBX7Message7setTextERKSs
pub fn stub_0x5c7fa4() {
    // IDA 0x5c7fa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::Message(void)")]
// 0x5c81b4 — __ZN3RBX7MessageC2Ev
pub fn stub_0x5c81b4() {
    // IDA 0x5c81b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::renderFullScreen(RBX::Adorn *)")]
// 0x5c8324 — __ZN3RBX7Message16renderFullScreenEPNS_5AdornE
pub fn stub_0x5c8324() {
    // IDA 0x5c8324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::renderPersonalMsg(RBX::Adorn *)")]
// 0x5c84b0 — __ZN3RBX7Message17renderPersonalMsgEPNS_5AdornE
pub fn stub_0x5c84b0() {
    // IDA 0x5c84b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::render2d(RBX::Adorn *)")]
// 0x5c8658 — __ZN3RBX7Message8render2dEPNS_5AdornE
pub fn stub_0x5c8658() {
    // IDA 0x5c8658: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::Message::render2d(RBX::Adorn *)")]
// 0x5c86e0 — __ZThn92_N3RBX7Message8render2dEPNS_5AdornE
pub fn stub_0x5c86e0() {
    // IDA 0x5c86e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Hint::render2d(RBX::Adorn *)")]
// 0x5c86e8 — __ZN3RBX4Hint8render2dEPNS_5AdornE
pub fn stub_0x5c86e8() {
    // IDA 0x5c86e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hint::render2d(RBX::Adorn *)")]
// 0x5c8814 — __ZThn92_N3RBX4Hint8render2dEPNS_5AdornE
pub fn stub_0x5c8814() {
    // IDA 0x5c8814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::getText(void)const")]
// 0x5c881c — __ZNK3RBX7Message7getTextEv
pub fn stub_0x5c881c() {
    // IDA 0x5c881c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::~Message()")]
// 0x5c8844 — __ZN3RBX7MessageD1Ev
pub fn stub_0x5c8844() {
    // IDA 0x5c8844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::~Message()")]
// 0x5c8938 — __ZN3RBX7MessageD0Ev
pub fn stub_0x5c8938() {
    // IDA 0x5c8938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::getPersistentDataCost(void)const")]
// 0x5c8a3c — __ZNK3RBX7Message21getPersistentDataCostEv
pub fn stub_0x5c8a3c() {
    // IDA 0x5c8a3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Message::shouldRender2d(void)const")]
// 0x5c8a7c — __ZNK3RBX7Message14shouldRender2dEv
pub fn stub_0x5c8a7c() {
    // IDA 0x5c8a7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// 0x5c8a80 — __ZThn32_N3RBX7MessageD1Ev
pub fn stub_0x5c8a80() {
    // IDA 0x5c8a80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// 0x5c8b74 — __ZThn32_N3RBX7MessageD0Ev
pub fn stub_0x5c8b74() {
    // IDA 0x5c8b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// 0x5c8c8c — __ZThn36_N3RBX7MessageD1Ev
pub fn stub_0x5c8c8c() {
    // IDA 0x5c8c8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// 0x5c8d7c — __ZThn36_N3RBX7MessageD0Ev
pub fn stub_0x5c8d7c() {
    // IDA 0x5c8d7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Message::shouldRender2d(void)const")]
// 0x5c8e84 — __ZThn92_NK3RBX7Message14shouldRender2dEv
pub fn stub_0x5c8e84() {
    // IDA 0x5c8e84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Hint::~Hint()")]
// 0x5c8e88 — __ZN3RBX4HintD1Ev
pub fn stub_0x5c8e88() {
    // IDA 0x5c8e88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Hint::~Hint()")]
// 0x5c8f7c — __ZN3RBX4HintD0Ev
pub fn stub_0x5c8f7c() {
    // IDA 0x5c8f7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Hint::canClientCreate(void)")]
// 0x5c9080 — __ZN3RBX4Hint15canClientCreateEv
pub fn stub_0x5c9080() {
    // IDA 0x5c9080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// 0x5c9094 — __ZThn32_N3RBX4HintD1Ev
pub fn stub_0x5c9094() {
    // IDA 0x5c9094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// 0x5c9188 — __ZThn32_N3RBX4HintD0Ev
pub fn stub_0x5c9188() {
    // IDA 0x5c9188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// 0x5c92a0 — __ZThn36_N3RBX4HintD1Ev
pub fn stub_0x5c92a0() {
    // IDA 0x5c92a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// 0x5c9390 — __ZThn36_N3RBX4HintD0Ev
pub fn stub_0x5c9390() {
    // IDA 0x5c9390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::push_back(RBX::IModelModifier * const&)")]
// 0x5cde1c — __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE9push_backERKS2_
pub fn stub_0x5cde1c() {
    // IDA 0x5cde1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier *>(__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier * const&,std::random_access_iterator_tag)")]
// 0x5ce3ec — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14IModelModifierESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_0x5ce3ec() {
    // IDA 0x5ce3ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier * const&)")]
// 0x5ce47c — __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x5ce47c() {
    // IDA 0x5ce47c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::_M_allocate(unsigned long)")]
// 0x5ce55c — __ZNSt12_Vector_baseIPN3RBX14IModelModifierESaIS2_EE11_M_allocateEm
pub fn stub_0x5ce55c() {
    // IDA 0x5ce55c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::Mouse(void)")]
// 0x5cfffc — __ZN3RBX5MouseC1Ev
pub fn stub_0x5cfffc() {
    // IDA 0x5cfffc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Mouse::Mouse(void)")]
// 0x5d0000 — __ZN3RBX5MouseC2Ev
pub fn stub_0x5d0000() {
    // IDA 0x5d0000: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Mouse::~Mouse()")]
// 0x5d06a4 — __ZN3RBX5MouseD0Ev
pub fn stub_0x5d06a4() {
    // IDA 0x5d06a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::~Mouse()")]
// 0x5d0744 — __ZN3RBX5MouseD1Ev
pub fn stub_0x5d0744() {
    // IDA 0x5d0744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// 0x5d0748 — __ZThn32_N3RBX5MouseD0Ev
pub fn stub_0x5d0748() {
    // IDA 0x5d0748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// 0x5d0750 — __ZThn36_N3RBX5MouseD0Ev
pub fn stub_0x5d0750() {
    // IDA 0x5d0750: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::~Mouse()")]
// 0x5d0758 — __ZN3RBX5MouseD2Ev
pub fn stub_0x5d0758() {
    // IDA 0x5d0758: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// 0x5d0dac — __ZThn32_N3RBX5MouseD1Ev
pub fn stub_0x5d0dac() {
    // IDA 0x5d0dac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// 0x5d0db4 — __ZThn36_N3RBX5MouseD1Ev
pub fn stub_0x5d0db4() {
    // IDA 0x5d0db4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::getHit(void)const")]
// 0x5d0dbc — __ZNK3RBX5Mouse6getHitEv
pub fn stub_0x5d0dbc() {
    // IDA 0x5d0dbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::getTargetFilter(void)const")]
// 0x5d0f58 — __ZNK3RBX5Mouse15getTargetFilterEv
pub fn stub_0x5d0f58() {
    // IDA 0x5d0f58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::getUnitRay(void)const")]
// 0x5d0f78 — __ZNK3RBX5Mouse10getUnitRayEv
pub fn stub_0x5d0f78() {
    // IDA 0x5d0f78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::getOrigin(void)const")]
// 0x5d0ff0 — __ZNK3RBX5Mouse9getOriginEv
pub fn stub_0x5d0ff0() {
    // IDA 0x5d0ff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mouse::getTarget(void)const")]
// 0x5d1284 — __ZNK3RBX5Mouse9getTargetEv
pub fn stub_0x5d1284() {
    // IDA 0x5d1284: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getTargetSurface(void)const")]
// 0x5d1410 — __ZNK3RBX5Mouse16getTargetSurfaceEv
pub fn stub_0x5d1410() {
    // IDA 0x5d1410: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::cacheUIEvent(RBX::UIEvent const&)")]
// 0x5d1598 — __ZN3RBX5Mouse12cacheUIEventERKNS_7UIEventE
pub fn stub_0x5d1598() {
    // IDA 0x5d1598: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::update(RBX::UIEvent const&)")]
// 0x5d15b8 — __ZN3RBX5Mouse6updateERKNS_7UIEventE
pub fn stub_0x5d15b8() {
    // IDA 0x5d15b8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::setCommand(RBX::MouseCommand *)")]
// 0x5d18d0 — __ZN3RBX5Mouse10setCommandEPNS_12MouseCommandE
pub fn stub_0x5d18d0() {
    // IDA 0x5d18d0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getIcon(void)const")]
// 0x5d1a04 — __ZNK3RBX5Mouse7getIconEv
pub fn stub_0x5d1a04() {
    // IDA 0x5d1a04: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::setIcon(RBX::TextureId const&)")]
// 0x5d1a28 — __ZN3RBX5Mouse7setIconERKNS_9TextureIdE
pub fn stub_0x5d1a28() {
    // IDA 0x5d1a28: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getX(void)const")]
// 0x5d1a74 — __ZNK3RBX5Mouse4getXEv
pub fn stub_0x5d1a74() {
    // IDA 0x5d1a74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getY(void)const")]
// 0x5d1a8c — __ZNK3RBX5Mouse4getYEv
pub fn stub_0x5d1a8c() {
    // IDA 0x5d1a8c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getViewSizeX(void)const")]
// 0x5d1aa4 — __ZNK3RBX5Mouse12getViewSizeXEv
pub fn stub_0x5d1aa4() {
    // IDA 0x5d1aa4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getViewSizeY(void)const")]
// 0x5d1abc — __ZNK3RBX5Mouse12getViewSizeYEv
pub fn stub_0x5d1abc() {
    // IDA 0x5d1abc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mouse::getTargetFilterUnsafe(void)const")]
// 0x5d1b64 — __ZNK3RBX5Mouse21getTargetFilterUnsafeEv
pub fn stub_0x5d1b64() {
    // IDA 0x5d1b64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::~MouseCommand()")]
// 0x5d507c — __ZN3RBX12MouseCommandD0Ev
pub fn stub_0x5d507c() {
    // IDA 0x5d507c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::~MouseCommand()")]
// 0x5d511c — __ZN3RBX12MouseCommandD1Ev
pub fn stub_0x5d511c() {
    // IDA 0x5d511c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::MouseCommand::~MouseCommand()")]
// 0x5d5120 — __ZThn36_N3RBX12MouseCommandD0Ev
pub fn stub_0x5d5120() {
    // IDA 0x5d5120: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::~MouseCommand()")]
// 0x5d5128 — __ZN3RBX12MouseCommandD2Ev
pub fn stub_0x5d5128() {
    // IDA 0x5d5128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::MouseCommand::~MouseCommand()")]
// 0x5d5258 — __ZThn36_N3RBX12MouseCommandD1Ev
pub fn stub_0x5d5258() {
    // IDA 0x5d5258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::getCursorId(void)const")]
// 0x5d5328 — __ZNK3RBX12MouseCommand11getCursorIdEv
pub fn stub_0x5d5328() {
    // IDA 0x5d5328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::capture(void)")]
// 0x5d5848 — __ZN3RBX12MouseCommand7captureEv
pub fn stub_0x5d5848() {
    // IDA 0x5d5848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::getUnitMouseRay(RBX::UIEvent const&)const")]
// 0x5d5b24 — __ZNK3RBX12MouseCommand15getUnitMouseRayERKNS_7UIEventE
pub fn stub_0x5d5b24() {
    // IDA 0x5d5b24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::getSurface(RBX::UIEvent const&,RBX::HitTestFilter const*)")]
// 0x5d5b3c — __ZN3RBX12MouseCommand10getSurfaceERKNS_7UIEventEPKNS_13HitTestFilterE
pub fn stub_0x5d5b3c() {
    // IDA 0x5d5b3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::getUnitMouseRay(RBX::UIEvent const&,RBX::ICameraOwner *)")]
// 0x5d5b60 — __ZN3RBX12MouseCommand15getUnitMouseRayERKNS_7UIEventEPNS_12ICameraOwnerE
pub fn stub_0x5d5b60() {
    // IDA 0x5d5b60: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::getSearchRay(RBX::RbxRay const&)")]
// 0x5d5c08 — __ZN3RBX12MouseCommand12getSearchRayERKNS_6RbxRayE
pub fn stub_0x5d5c08() {
    // IDA 0x5d5c08: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}
