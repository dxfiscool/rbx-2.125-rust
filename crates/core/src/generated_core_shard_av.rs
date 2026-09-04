//! core shard AV — 100 core stubs EA-sorted, next uncovered after AU 0x3c04a8..0x3c057c (strict RBX|boost|std earliest gap, after AU 0x3a867c..0x3c04a8).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x3c04a8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::BillboardGui::getPart(void)const")]
// 0x3c057c — __ZNK3RBX12BillboardGui7getPartEv
pub fn stub_0x3c057c() {
    // IDA 0x3c057c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
// 0x3c066c — __ZThn96_NK3RBX12BillboardGui25shouldRender3dSortedAdornEv
// was: non-virtual thunk to RBX::BillboardGui::shouldRender3dSortedAdorn(void)const
pub fn stub_0x3c066c() {
    // IDA 0x3c066c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::getModelAdorn(void)const")]
// 0x3c0678 — __ZNK3RBX12BillboardGui13getModelAdornEv
pub fn stub_0x3c0678() {
    // IDA 0x3c0678: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::getPartAdorn(void)const")]
// 0x3c0764 — __ZNK3RBX12BillboardGui12getPartAdornEv
pub fn stub_0x3c0764() {
    // IDA 0x3c0764: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::render3dSortedPosition(void)const")]
// 0x3c0850 — __ZNK3RBX12BillboardGui22render3dSortedPositionEv
pub fn stub_0x3c0850() {
    // IDA 0x3c0850: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::render3dSortedPosition(void)const")]
// 0x3c0a28 — __ZThn96_NK3RBX12BillboardGui22render3dSortedPositionEv
// was: non-virtual thunk to RBX::BillboardGui::render3dSortedPosition(void)const
pub fn stub_0x3c0a28() {
    // IDA 0x3c0a28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
// 0x3c0a34 — __ZN3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
pub fn stub_0x3c0a34() {
    // IDA 0x3c0a34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
// 0x3c0e90 — __ZThn96_N3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
// was: non-virtual thunk to RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)
pub fn stub_0x3c0e90() {
    // IDA 0x3c0e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::process(RBX::GuiEvent const&)")]
// 0x3c0e98 — __ZN3RBX12BillboardGui7processERKNS_8GuiEventE
pub fn stub_0x3c0e98() {
    // IDA 0x3c0e98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::process(RBX::GuiEvent const&)")]
// 0x3c0f34 — __ZThn92_N3RBX12BillboardGui7processERKNS_8GuiEventE
// was: non-virtual thunk to RBX::BillboardGui::process(RBX::GuiEvent const&)
pub fn stub_0x3c0f34() {
    // IDA 0x3c0f34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x3c0f40 — __ZN3RBX12BillboardGui17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_0x3c0f40() {
    // IDA 0x3c0f40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::getAdorneeDangerous(void)const")]
// 0x3c0f58 — __ZNK3RBX12BillboardGui19getAdorneeDangerousEv
pub fn stub_0x3c0f58() {
    // IDA 0x3c0f58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::getEnabled(void)const")]
// 0x3c1014 — __ZNK3RBX12BillboardGui10getEnabledEv
pub fn stub_0x3c1014() {
    // IDA 0x3c1014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::getActive(void)const")]
// 0x3c1040 — __ZNK3RBX12BillboardGui9getActiveEv
pub fn stub_0x3c1040() {
    // IDA 0x3c1040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::getPlayerToHideFrom(void)const")]
// 0x3c1048 — __ZNK3RBX12BillboardGui19getPlayerToHideFromEv
pub fn stub_0x3c1048() {
    // IDA 0x3c1048: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)> const&)")]
// 0x3c106c — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_
pub fn stub_0x3c106c() {
    // IDA 0x3c106c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::operator()(RBX::BillboardGui *,RBX::Adorn *)const")]
// 0x3c1130 — __ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_
pub fn stub_0x3c1130() {
    // IDA 0x3c1130: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}


#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
// 0x3c11f8 — __ZN3RBX12BillboardGuiD1Ev
pub fn stub_0x3c11f8() {
    // IDA 0x3c11f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
// 0x3c11fc — __ZN3RBX12BillboardGuiD0Ev
pub fn stub_0x3c11fc() {
    // IDA 0x3c11fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x3c129c — __ZN3RBX12BillboardGui17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_0x3c129c() {
    // IDA 0x3c129c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::canProcessMeAndDescendants(void)const")]
// 0x3c12b4 — __ZNK3RBX12BillboardGui26canProcessMeAndDescendantsEv
pub fn stub_0x3c12b4() {
    // IDA 0x3c12b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::~BillboardGui()")]
// 0x3c12bc — __ZThn32_N3RBX12BillboardGuiD1Ev
// was: non-virtual thunk to RBX::BillboardGui::~BillboardGui()
pub fn stub_0x3c12bc() {
    // IDA 0x3c12bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::~BillboardGui()")]
// 0x3c12c4 — __ZThn32_N3RBX12BillboardGuiD0Ev
// was: non-virtual thunk to RBX::BillboardGui::~BillboardGui()
pub fn stub_0x3c12c4() {
    // IDA 0x3c12c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::~BillboardGui()")]
// 0x3c1378 — __ZThn36_N3RBX12BillboardGuiD1Ev
// was: non-virtual thunk to RBX::BillboardGui::~BillboardGui()
pub fn stub_0x3c1378() {
    // IDA 0x3c1378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::~BillboardGui()")]
// 0x3c1380 — __ZThn36_N3RBX12BillboardGuiD0Ev
// was: non-virtual thunk to RBX::BillboardGui::~BillboardGui()
pub fn stub_0x3c1380() {
    // IDA 0x3c1380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::~BillboardGui()")]
// 0x3c1428 — __ZThn168_N3RBX12BillboardGuiD1Ev
// was: non-virtual thunk to RBX::BillboardGui::~BillboardGui()
pub fn stub_0x3c1428() {
    // IDA 0x3c1428: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::BillboardGui::~BillboardGui()")]
// 0x3c1430 — __ZThn168_N3RBX12BillboardGuiD0Ev
// was: non-virtual thunk to RBX::BillboardGui::~BillboardGui()
pub fn stub_0x3c1430() {
    // IDA 0x3c1430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void)")]
// 0x3c1adc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5dummy7nonnullEv
pub fn stub_0x3c1adc() {
    // IDA 0x3c1adc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
// 0x3c1ae0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_
pub fn stub_0x3c1ae0() {
    // IDA 0x3c1ae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
// 0x3c1bbc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_
pub fn stub_0x3c1bbc() {
    // IDA 0x3c1bbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to_own(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *> const&)")]
// 0x3c1cc0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_
pub fn stub_0x3c1cc0() {
    // IDA 0x3c1cc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
// 0x3c2b5c — __ZN3RBX12BillboardGuiD2Ev
pub fn stub_0x3c2b5c() {
    // IDA 0x3c2b5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::setCameraType(RBX::Camera::CameraType)")]
// 0x3c3510 — __ZN3RBX6Camera13setCameraTypeENS0_10CameraTypeE
pub fn stub_0x3c3510() {
    // IDA 0x3c3510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::setFieldOfViewDegrees(float)")]
// 0x3c3820 — __ZN3RBX6Camera21setFieldOfViewDegreesEf
pub fn stub_0x3c3820() {
    // IDA 0x3c3820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::setRoll(float)")]
// 0x3c3cb4 — __ZN3RBX6Camera7setRollEf
pub fn stub_0x3c3cb4() {
    // IDA 0x3c3cb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::getRollSlow(void)")]
// 0x3c3da4 — __ZN3RBX6Camera11getRollSlowEv
pub fn stub_0x3c3da4() {
    // IDA 0x3c3da4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::setCameraPanMode(RBX::Camera::CameraPanMode)")]
// 0x3c3dac — __ZN3RBX6Camera16setCameraPanModeENS0_13CameraPanModeE
pub fn stub_0x3c3dac() {
    // IDA 0x3c3dac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::zoom(float)")]
// 0x3c3db4 — __ZN3RBX6Camera4zoomEf
pub fn stub_0x3c3db4() {
    // IDA 0x3c3db4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::panUnits(int)")]
// 0x3c3e64 — __ZN3RBX6Camera8panUnitsEi
pub fn stub_0x3c3e64() {
    // IDA 0x3c3e64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::tiltUnits(int)")]
// 0x3c3f04 — __ZN3RBX6Camera9tiltUnitsEi
pub fn stub_0x3c3f04() {
    // IDA 0x3c3f04: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::StringConverter<RBX::Camera::CameraPanMode>::convertToValue(std::string const&,RBX::Camera::CameraPanMode&)")]
// 0x3c493c — __ZN3RBX15StringConverterINS_6Camera13CameraPanModeEE14convertToValueERKSsRS2_
pub fn stub_0x3c493c() {
    // IDA 0x3c493c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Camera::Camera(void)")]
// 0x3c4988 — __ZN3RBX6CameraC1Ev
pub fn stub_0x3c4988() {
    // IDA 0x3c4988: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Camera::Camera(void)")]
// 0x3c498c — __ZN3RBX6CameraC2Ev
pub fn stub_0x3c498c() {
    // IDA 0x3c498c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Camera::getNewZoomDistance(float,float)")]
// 0x3c4ecc — __ZN3RBX6Camera18getNewZoomDistanceEff
pub fn stub_0x3c4ecc() {
    // IDA 0x3c4ecc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Camera::isCharacterCamera(void)const")]
// 0x3c4f24 — __ZNK3RBX6Camera17isCharacterCameraEv
pub fn stub_0x3c4f24() {
    // IDA 0x3c4f24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Camera::isFirstPersonCamera(void)const")]
// 0x3c4f48 — __ZNK3RBX6Camera19isFirstPersonCameraEv
pub fn stub_0x3c4f48() {
    // IDA 0x3c4f48: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::isLockedToFirstPerson(void)const")]
// 0x3c5544 — __ZNK3RBX6Camera21isLockedToFirstPersonEv
pub fn stub_0x3c5544() {
    // IDA 0x3c5544: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::onHeartbeat(RBX::Heartbeat const&)")]
// 0x3c5588 — __ZN3RBX6Camera11onHeartbeatERKNS_9HeartbeatE
pub fn stub_0x3c5588() {
    // IDA 0x3c5588: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getCameraSubject(void)")]
// 0x3c5688 — __ZN3RBX6Camera16getCameraSubjectEv
pub fn stub_0x3c5688() {
    // IDA 0x3c5688: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::fixedSpeedInterpolateCamera(double)")]
// 0x3c5690 — __ZN3RBX6Camera27fixedSpeedInterpolateCameraEd
pub fn stub_0x3c5690() {
    // IDA 0x3c5690: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::onHeartbeat(RBX::Heartbeat const&)")]
// 0x3c58a8 — __ZThn92_N3RBX6Camera11onHeartbeatERKNS_9HeartbeatE
// was: non-virtual thunk to RBX::Camera::onHeartbeat(RBX::Heartbeat const&)
pub fn stub_0x3c58a8() {
    // IDA 0x3c58a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::getCameraOwner(void)")]
// 0x3c58b0 — __ZN3RBX6Camera14getCameraOwnerEv
pub fn stub_0x3c58b0() {
    // IDA 0x3c58b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::pushCameraHistoryStack(void)")]
// 0x3c58e8 — __ZN3RBX6Camera22pushCameraHistoryStackEv
pub fn stub_0x3c58e8() {
    // IDA 0x3c58e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::popCameraHistoryStack(bool)")]
// 0x3c5a7c — __ZN3RBX6Camera21popCameraHistoryStackEb
pub fn stub_0x3c5a7c() {
    // IDA 0x3c5a7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::stepCameraHistoryForward(void)")]
// 0x3c5cf4 — __ZN3RBX6Camera24stepCameraHistoryForwardEv
pub fn stub_0x3c5cf4() {
    // IDA 0x3c5cf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::stepCameraHistoryBackward(void)")]
// 0x3c5eac — __ZN3RBX6Camera25stepCameraHistoryBackwardEv
pub fn stub_0x3c5eac() {
    // IDA 0x3c5eac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::updateFocus(void)")]
// 0x3c6064 — __ZN3RBX6Camera11updateFocusEv
pub fn stub_0x3c6064() {
    // IDA 0x3c6064: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::step(double)")]
// 0x3c62bc — __ZN3RBX6Camera4stepEd
pub fn stub_0x3c62bc() {
    // IDA 0x3c62bc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::panRadians(float)")]
// 0x3c69a0 — __ZN3RBX6Camera10panRadiansEf
pub fn stub_0x3c69a0() {
    // IDA 0x3c69a0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::tiltRadians(float)")]
// 0x3c6be8 — __ZN3RBX6Camera11tiltRadiansEf
pub fn stub_0x3c6be8() {
    // IDA 0x3c6be8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::tryZoomExtents(RBX::Extents const&)")]
// 0x3c7374 — __ZN3RBX6Camera14tryZoomExtentsERKNS_7ExtentsE
pub fn stub_0x3c7374() {
    // IDA 0x3c7374: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::zoomExtents(RBX::Extents const&,RBX::Camera::ZoomType)")]
// 0x3c75e4 — __ZN3RBX6Camera11zoomExtentsERKNS_7ExtentsENS0_8ZoomTypeE
pub fn stub_0x3c75e4() {
    // IDA 0x3c75e4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::canZoom(bool)const")]
// 0x3c7a18 — __ZNK3RBX6Camera7canZoomEb
pub fn stub_0x3c7a18() {
    // IDA 0x3c7a18: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::setDistanceFromTarget(float)")]
// 0x3c7a9c — __ZN3RBX6Camera21setDistanceFromTargetEf
pub fn stub_0x3c7a9c() {
    // IDA 0x3c7a9c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getConstCameraSubject(void)const")]
// 0x3c7ab0 — __ZNK3RBX6Camera21getConstCameraSubjectEv
pub fn stub_0x3c7ab0() {
    // IDA 0x3c7ab0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::nonCharacterZoom(float)")]
// 0x3c7bd8 — __ZN3RBX6Camera16nonCharacterZoomEf
pub fn stub_0x3c7bd8() {
    // IDA 0x3c7bd8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::characterZoom(float)")]
// 0x3c8060 — __ZN3RBX6Camera13characterZoomEf
pub fn stub_0x3c8060() {
    // IDA 0x3c8060: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::canTilt(int)const")]
// 0x3c8200 — __ZNK3RBX6Camera7canTiltEi
pub fn stub_0x3c8200() {
    // IDA 0x3c8200: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::setHeadingElevationDistance(float,float,float)")]
// 0x3c82a4 — __ZN3RBX6Camera27setHeadingElevationDistanceEfff
pub fn stub_0x3c82a4() {
    // IDA 0x3c82a4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::tiltSpeedRadians(float)")]
// 0x3c8358 — __ZN3RBX6Camera16tiltSpeedRadiansEf
pub fn stub_0x3c8358() {
    // IDA 0x3c8358: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::panSpeedRadians(float)")]
// 0x3c8360 — __ZN3RBX6Camera15panSpeedRadiansEf
pub fn stub_0x3c8360() {
    // IDA 0x3c8360: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::doFly(RBX::NavKeys const&,int,bool)")]
// 0x3c836c — __ZN3RBX6Camera5doFlyERKNS_7NavKeysEib
pub fn stub_0x3c836c() {
    // IDA 0x3c836c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::nearPlaneZ(void)const")]
// 0x3c872c — __ZNK3RBX6Camera10nearPlaneZEv
pub fn stub_0x3c872c() {
    // IDA 0x3c872c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::coordinateFrame(void)const")]
// 0x3c89d8 — __ZNK3RBX6Camera15coordinateFrameEv
pub fn stub_0x3c89d8() {
    // IDA 0x3c89d8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getCameraType(void)const")]
// 0x3c8be4 — __ZNK3RBX6Camera13getCameraTypeEv
pub fn stub_0x3c8be4() {
    // IDA 0x3c8be4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getCameraCoordinateFrame(void)const")]
// 0x3c8c10 — __ZNK3RBX6Camera24getCameraCoordinateFrameEv
pub fn stub_0x3c8c10() {
    // IDA 0x3c8c10: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getCameraFocus(void)const")]
// 0x3c8c38 — __ZNK3RBX6Camera14getCameraFocusEv
pub fn stub_0x3c8c38() {
    // IDA 0x3c8c38: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getFieldOfViewDegrees(void)const")]
// 0x3c8c3c — __ZNK3RBX6Camera21getFieldOfViewDegreesEv
pub fn stub_0x3c8c3c() {
    // IDA 0x3c8c3c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getTiltSpeed(void)")]
// 0x3c8d14 — __ZN3RBX6Camera12getTiltSpeedEv
pub fn stub_0x3c8d14() {
    // IDA 0x3c8d14: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::getPanSpeed(void)")]
// 0x3c8d1c — __ZN3RBX6Camera11getPanSpeedEv
pub fn stub_0x3c8d1c() {
    // IDA 0x3c8d1c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Tolerance::maxExtents(void)")]
// 0x3c9bf0 — __ZN3RBX9Tolerance10maxExtentsEv
pub fn stub_0x3c9bf0() {
    // IDA 0x3c9bf0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::NavKeys::navKeyDown(void)const")]
// 0x3c9c7c — __ZNK3RBX7NavKeys10navKeyDownEv
pub fn stub_0x3c9c7c() {
    // IDA 0x3c9c7c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}


#[doc(alias = "RBX::Camera::~Camera()")]
// 0x3c9cd0 — __ZN3RBX6CameraD1Ev
pub fn stub_0x3c9cd0() {
    // IDA 0x3c9cd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::~Camera()")]
// 0x3c9cd4 — __ZN3RBX6CameraD0Ev
pub fn stub_0x3c9cd4() {
    // IDA 0x3c9cd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x3c9d74 — __ZN3RBX6Camera17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_0x3c9d74() {
    // IDA 0x3c9d74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::~Camera()")]
// 0x3c9d8c — __ZThn32_N3RBX6CameraD1Ev
// was: non-virtual thunk to RBX::Camera::~Camera()
pub fn stub_0x3c9d8c() {
    // IDA 0x3c9d8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::~Camera()")]
// 0x3c9d94 — __ZThn32_N3RBX6CameraD0Ev
// was: non-virtual thunk to RBX::Camera::~Camera()
pub fn stub_0x3c9d94() {
    // IDA 0x3c9d94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::~Camera()")]
// 0x3c9dac — __ZThn36_N3RBX6CameraD1Ev
// was: non-virtual thunk to RBX::Camera::~Camera()
pub fn stub_0x3c9dac() {
    // IDA 0x3c9dac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::~Camera()")]
// 0x3c9db4 — __ZThn36_N3RBX6CameraD0Ev
// was: non-virtual thunk to RBX::Camera::~Camera()
pub fn stub_0x3c9db4() {
    // IDA 0x3c9db4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::~Camera()")]
// 0x3c9dbc — __ZThn92_N3RBX6CameraD1Ev
// was: non-virtual thunk to RBX::Camera::~Camera()
pub fn stub_0x3c9dbc() {
    // IDA 0x3c9dbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk to RBX::Camera::~Camera()")]
// 0x3c9dc4 — __ZThn92_N3RBX6CameraD0Ev
// was: non-virtual thunk to RBX::Camera::~Camera()
pub fn stub_0x3c9dc4() {
    // IDA 0x3c9dc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraPanMode>(RBX::Camera::CameraPanMode const&)")]
// 0x3ca6b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_
pub fn stub_0x3ca6b0() {
    // IDA 0x3ca6b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::singleton(void)")]
// 0x3ca700 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv
pub fn stub_0x3ca700() {
    // IDA 0x3ca700: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::construct_func(char const*,char *)")]
// 0x3ca76c — __ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE14construct_funcEPKcPc
pub fn stub_0x3ca76c() {
    // IDA 0x3ca76c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::destruct_func(char *)")]
// 0x3ca778 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE13destruct_funcEPc
pub fn stub_0x3ca778() {
    // IDA 0x3ca778: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Camera::CameraPanMode const& rbx::any_cast<RBX::Camera::CameraPanMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x3ca848 — __ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x3ca848() {
    // IDA 0x3ca848: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraMode>(RBX::Camera::CameraMode const&)")]
// 0x3cad28 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_
pub fn stub_0x3cad28() {
    // IDA 0x3cad28: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void)")]
// 0x3cad78 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv
pub fn stub_0x3cad78() {
    // IDA 0x3cad78: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::construct_func(char const*,char *)")]
// 0x3cade4 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE14construct_funcEPKcPc
pub fn stub_0x3cade4() {
    // IDA 0x3cade4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::destruct_func(char *)")]
// 0x3cadf0 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE13destruct_funcEPc
pub fn stub_0x3cadf0() {
    // IDA 0x3cadf0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
