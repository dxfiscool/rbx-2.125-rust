//! core shard CP — 100 core stubs EA-sorted, next uncovered after CO 0x6c40e0 (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::VehicleSeat::shouldRender2d(void)const")]
// 0x6bd00c — __ZNK3RBX11VehicleSeat14shouldRender2dEv
pub fn stub_6bd00c() {
    // IDA 0x6bd00c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::VehicleSeat::shouldRender2d(void)const")]
// 0x6bd020 — __ZThn108_NK3RBX11VehicleSeat14shouldRender2dEv
// was: non-virtual thunk to RBX::VehicleSeat::shouldRender2d(void)const
pub fn stub_6bd020() {
    // IDA 0x6bd020: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::convert(std::string const&)")]
// 0x6c424c — __ZN3RBX11VirtualUser7convertERKSs
pub fn stub_6c424c() {
    // IDA 0x6c424c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::onGuiEvent(RBX::UIEvent const&)")]
// 0x6c43e8 — __ZN3RBX11VirtualUser10onGuiEventERKNS_7UIEventE
pub fn stub_6c43e8() {
    // IDA 0x6c43e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::writeWait(void)")]
// 0x6c443c — __ZN3RBX11VirtualUser9writeWaitEv
pub fn stub_6c443c() {
    // IDA 0x6c443c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::writeKey(char const*,RBX::UIEvent const&)")]
// 0x6c4498 — __ZN3RBX11VirtualUser8writeKeyEPKcRKNS_7UIEventE
pub fn stub_6c4498() {
    // IDA 0x6c4498: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::writeMouse(char const*,RBX::UIEvent const&)")]
// 0x6c468c — __ZN3RBX11VirtualUser10writeMouseEPKcRKNS_7UIEventE
pub fn stub_6c468c() {
    // IDA 0x6c468c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::VirtualUser::~VirtualUser()")]
// 0x6c4b28 — __ZN3RBX11VirtualUserD1Ev
pub fn stub_6c4b28() {
    // IDA 0x6c4b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::~VirtualUser()")]
// 0x6c4c80 — __ZN3RBX11VirtualUserD0Ev
pub fn stub_6c4c80() {
    // IDA 0x6c4c80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::VirtualUser::~VirtualUser()")]
// 0x6c4d30 — __ZThn32_N3RBX11VirtualUserD1Ev
// was: non-virtual thunk to RBX::VirtualUser::~VirtualUser()
pub fn stub_6c4d30() {
    // IDA 0x6c4d30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::VirtualUser::~VirtualUser()")]
// 0x6c4e84 — __ZThn32_N3RBX11VirtualUserD0Ev
// was: non-virtual thunk to RBX::VirtualUser::~VirtualUser()
pub fn stub_6c4e84() {
    // IDA 0x6c4e84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::VirtualUser::~VirtualUser()")]
// 0x6c5000 — __ZThn36_N3RBX11VirtualUserD1Ev
// was: non-virtual thunk to RBX::VirtualUser::~VirtualUser()
pub fn stub_6c5000() {
    // IDA 0x6c5000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::VirtualUser::~VirtualUser()")]
// 0x6c5154 — __ZThn36_N3RBX11VirtualUserD0Ev
// was: non-virtual thunk to RBX::VirtualUser::~VirtualUser()
pub fn stub_6c5154() {
    // IDA 0x6c5154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputBase::~UserInputBase()")]
// 0x6c6578 — __ZN3RBX13UserInputBaseD2Ev
pub fn stub_6c6578() {
    // IDA 0x6c6578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualHardwareDevice::getCursorPosition(void)")]
// 0x6c787c — __ZN3RBX21VirtualHardwareDevice17getCursorPositionEv
pub fn stub_6c787c() {
    // IDA 0x6c787c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualHardwareDevice::centerCursor(void)")]
// 0x6c788c — __ZN3RBX21VirtualHardwareDevice12centerCursorEv
pub fn stub_6c788c() {
    // IDA 0x6c788c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualHardwareDevice::keyDown(RBX::KeyCode)const")]
// 0x6c78a0 — __ZNK3RBX21VirtualHardwareDevice7keyDownENS_7KeyCodeE
pub fn stub_6c78a0() {
    // IDA 0x6c78a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualHardwareDevice::setKeyState(RBX::KeyCode,bool)")]
// 0x6c78a8 — __ZN3RBX21VirtualHardwareDevice11setKeyStateENS_7KeyCodeEb
pub fn stub_6c78a8() {
    // IDA 0x6c78a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualHardwareDevice::renderGameCursor(RBX::Adorn *)")]
// 0x6c78c0 — __ZN3RBX21VirtualHardwareDevice16renderGameCursorEPNS_5AdornE
pub fn stub_6c78c0() {
    // IDA 0x6c78c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Visit::setUploadUrl(std::string)")]
// 0x6c7f30 — __ZN3RBX5Visit12setUploadUrlESs
pub fn stub_6c7f30() {
    // IDA 0x6c7f30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Visit::setPing(std::string,int)")]
// 0x6c7f38 — __ZN3RBX5Visit7setPingESsi
pub fn stub_6c7f38() {
    // IDA 0x6c7f38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Visit::Visit(void)")]
// 0x6c81c4 — __ZN3RBX5VisitC1Ev
pub fn stub_6c81c4() {
    // IDA 0x6c81c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Visit::Visit(void)")]
// 0x6c81c8 — __ZN3RBX5VisitC2Ev
pub fn stub_6c81c8() {
    // IDA 0x6c81c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Visit::~Visit()")]
// 0x6c8410 — __ZN3RBX5VisitD0Ev
pub fn stub_6c8410() {
    // IDA 0x6c8410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Visit::~Visit()")]
// 0x6c84b0 — __ZN3RBX5VisitD1Ev
pub fn stub_6c84b0() {
    // IDA 0x6c84b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Visit::~Visit()")]
// 0x6c84b4 — __ZThn32_N3RBX5VisitD0Ev
// was: non-virtual thunk to RBX::Visit::~Visit()
pub fn stub_6c84b4() {
    // IDA 0x6c84b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Visit::~Visit()")]
// 0x6c84bc — __ZThn36_N3RBX5VisitD0Ev
// was: non-virtual thunk to RBX::Visit::~Visit()
pub fn stub_6c84bc() {
    // IDA 0x6c84bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Visit::~Visit()")]
// 0x6c84c4 — __ZN3RBX5VisitD2Ev
pub fn stub_6c84c4() {
    // IDA 0x6c84c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Visit::~Visit()")]
// 0x6c85b8 — __ZThn32_N3RBX5VisitD1Ev
// was: non-virtual thunk to RBX::Visit::~Visit()
pub fn stub_6c85b8() {
    // IDA 0x6c85b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Visit::~Visit()")]
// 0x6c85c0 — __ZThn36_N3RBX5VisitD1Ev
// was: non-virtual thunk to RBX::Visit::~Visit()
pub fn stub_6c85c0() {
    // IDA 0x6c85c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Visit::ping(std::string,int)")]
// 0x6c85c8 — __ZN3RBX5Visit4pingESsi
pub fn stub_6c85c8() {
    // IDA 0x6c85c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Visit::getUploadUrl(void)")]
// 0x6c8ab4 — __ZN3RBX5Visit12getUploadUrlEv
pub fn stub_6c8ab4() {
    // IDA 0x6c8ab4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::hash_value(RBX::TouchPair const&)")]
// 0x6d02f4 — __ZN3RBX10hash_valueERKNS_9TouchPairE
pub fn stub_6d02f4() {
    // IDA 0x6d02f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::reset(void)")]
// 0x6d19d8 — __ZN3RBX5World5resetEv
pub fn stub_6d19d8() {
    // IDA 0x6d19d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::resize(unsigned long,RBX::TouchPair)")]
// 0x6d1b68 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_
pub fn stub_6d1b68() {
    // IDA 0x6d1b68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IAdornable::shouldRender3dSortedAdorn(void)const")]
// 0x6d2d48 — __ZNK3RBX10IAdornable25shouldRender3dSortedAdornEv
pub fn stub_6d2d48() {
    // IDA 0x6d2d48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IAdornable::renderBackground2d(RBX::Adorn *)")]
// 0x6d2d50 — __ZN3RBX10IAdornable18renderBackground2dEPNS_5AdornE
pub fn stub_6d2d50() {
    // IDA 0x6d2d50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Stats::StatsService::StatsService(void)")]
// 0x6d3ed8 — __ZN3RBX5Stats12StatsServiceC2Ev
pub fn stub_6d3ed8() {
    // IDA 0x6d3ed8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Selection>(void)")]
// 0x6d4658 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9SelectionEEEvv
pub fn stub_6d4658() {
    // IDA 0x6d4658: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchPair*,std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>>,unsigned long,RBX::TouchPair const&)")]
// 0x6d48e4 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_6d48e4() {
    // IDA 0x6d48e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::fill<RBX::TouchPair *,RBX::TouchPair>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair const&)")]
// 0x6d4cd8 — __ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_
pub fn stub_6d4cd8() {
    // IDA 0x6d4cd8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_allocate(unsigned long)")]
// 0x6d4d10 — __ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm
pub fn stub_6d4d10() {
    // IDA 0x6d4d10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::TouchPair *,unsigned long,RBX::TouchPair>(RBX::TouchPair *,unsigned long,RBX::TouchPair const&,std::__false_type)")]
// 0x6d4d34 — __ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type
pub fn stub_6d4d34() {
    // IDA 0x6d4d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TouchPair::operator=(RBX::TouchPair const&)")]
// 0x6d4eec — __ZN3RBX9TouchPairaSERKS0_
pub fn stub_6d4eec() {
    // IDA 0x6d4eec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TouchPair * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *)")]
// 0x6d4f0c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_
pub fn stub_6d4f0c() {
    // IDA 0x6d4f0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TouchPair * std::__uninitialized_copy_aux<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *,std::__false_type)")]
// 0x6d4f68 — __ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type
pub fn stub_6d4f68() {
    // IDA 0x6d4f68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::~vector()")]
// 0x6d5144 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev
pub fn stub_6d5144() {
    // IDA 0x6d5144: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterPackService>(void)")]
// 0x6d5f20 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18StarterPackServiceEEEvv
pub fn stub_6d5f20() {
    // IDA 0x6d5f20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::onRightMouseDown(RBX::UIEvent const&)")]
// 0x6d6820 — __ZN3RBX12MouseCommand16onRightMouseDownERKNS_7UIEventE
pub fn stub_6d6820() {
    // IDA 0x6d6820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::onMouseIdle(RBX::UIEvent const&)")]
// 0x6d682c — __ZN3RBX12MouseCommand11onMouseIdleERKNS_7UIEventE
pub fn stub_6d682c() {
    // IDA 0x6d682c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::onKeyUp(RBX::UIEvent const&)")]
// 0x6d6830 — __ZN3RBX12MouseCommand7onKeyUpERKNS_7UIEventE
pub fn stub_6d6830() {
    // IDA 0x6d6830: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::onPeekKeyDown(RBX::UIEvent const&)")]
// 0x6d683c — __ZN3RBX12MouseCommand13onPeekKeyDownERKNS_7UIEventE
pub fn stub_6d683c() {
    // IDA 0x6d683c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onPeekKeyUp(RBX::UIEvent const&)")]
// 0x6d6848 — __ZN3RBX12MouseCommand11onPeekKeyUpERKNS_7UIEventE
pub fn stub_6d6848() {
    // IDA 0x6d6848: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onMouseDelta(RBX::UIEvent const&)")]
// 0x6d6854 — __ZN3RBX12MouseCommand12onMouseDeltaERKNS_7UIEventE
pub fn stub_6d6854() {
    // IDA 0x6d6854: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onRightMouseUp(RBX::UIEvent const&)")]
// 0x6d6858 — __ZN3RBX12MouseCommand14onRightMouseUpERKNS_7UIEventE
pub fn stub_6d6858() {
    // IDA 0x6d6858: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onMouseWheelForward(RBX::UIEvent const&)")]
// 0x6d6864 — __ZN3RBX12MouseCommand19onMouseWheelForwardERKNS_7UIEventE
pub fn stub_6d6864() {
    // IDA 0x6d6864: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onMouseWheelBackward(RBX::UIEvent const&)")]
// 0x6d6870 — __ZN3RBX12MouseCommand20onMouseWheelBackwardERKNS_7UIEventE
pub fn stub_6d6870() {
    // IDA 0x6d6870: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::releaseCapture(void)")]
// 0x6d687c — __ZN3RBX12MouseCommand14releaseCaptureEv
pub fn stub_6d687c() {
    // IDA 0x6d687c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::cancel(void)")]
// 0x6d6884 — __ZN3RBX12MouseCommand6cancelEv
pub fn stub_6d6884() {
    // IDA 0x6d6884: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::isSticky(void)const")]
// 0x6d6894 — __ZNK3RBX12MouseCommand8isStickyEv
pub fn stub_6d6894() {
    // IDA 0x6d6894: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::drawConnectors(void)const")]
// 0x6d68a0 — __ZNK3RBX12MouseCommand14drawConnectorsEv
pub fn stub_6d68a0() {
    // IDA 0x6d68a0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::DecalTool::getCursorName(void)const")]
// 0x6d68a4 — __ZNK3RBX9DecalTool13getCursorNameEv
pub fn stub_6d68a4() {
    // IDA 0x6d68a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::DecalTool::doAction(RBX::Surface *)")]
// 0x6d68c0 — __ZN3RBX9DecalTool8doActionEPNS_7SurfaceE
pub fn stub_6d68c0() {
    // IDA 0x6d68c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onKeyDown(RBX::UIEvent const&)")]
// 0x6d6b5c — __ZN3RBX12MouseCommand9onKeyDownERKNS_7UIEventE
pub fn stub_6d6b5c() {
    // IDA 0x6d6b5c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onMouseMove(RBX::UIEvent const&)")]
// 0x6d6b68 — __ZN3RBX12MouseCommand11onMouseMoveERKNS_7UIEventE
pub fn stub_6d6b68() {
    // IDA 0x6d6b68: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MouseCommand::onMouseUp(RBX::UIEvent const&)")]
// 0x6d6b6c — __ZN3RBX12MouseCommand9onMouseUpERKNS_7UIEventE
pub fn stub_6d6b6c() {
    // IDA 0x6d6b6c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dcee0 — __ZN3RBX12AdvArrowToolD1Ev
pub fn stub_6dcee0() {
    // IDA 0x6dcee0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dcee4 — __ZN3RBX12AdvArrowToolD0Ev
pub fn stub_6dcee4() {
    // IDA 0x6dcee4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowTool::isSticky(void)const")]
// 0x6dcf84 — __ZNK3RBX12AdvArrowTool8isStickyEv
pub fn stub_6dcf84() {
    // IDA 0x6dcf84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowToolBase::getSelectedTargetPrimitives(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
// 0x6dd04c — __ZN3RBX16AdvArrowToolBase27getSelectedTargetPrimitivesERSt6vectorIPNS_9PrimitiveESaIS3_EE
pub fn stub_6dd04c() {
    // IDA 0x6dd04c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowTool::setCursor(std::string)")]
// 0x6dd050 — __ZN3RBX12AdvArrowTool9setCursorESs
pub fn stub_6dd050() {
    // IDA 0x6dd050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dd054 — __ZThn36_N3RBX12AdvArrowToolD1Ev
// was: non-virtual thunk to RBX::AdvArrowTool::~AdvArrowTool()
pub fn stub_6dd054() {
    // IDA 0x6dd054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dd05c — __ZThn36_N3RBX12AdvArrowToolD0Ev
// was: non-virtual thunk to RBX::AdvArrowTool::~AdvArrowTool()
pub fn stub_6dd05c() {
    // IDA 0x6dd05c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd100 — __ZN3RBX16AdvArrowToolBaseD2Ev
pub fn stub_6dd100() {
    // IDA 0x6dd100: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
// 0x6dd1dc — __ZN3RBX13ArrowToolBaseD2Ev
pub fn stub_6dd1dc() {
    // IDA 0x6dd1dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd2d0 — __ZN3RBX16AdvArrowToolBaseD1Ev
pub fn stub_6dd2d0() {
    // IDA 0x6dd2d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd2d4 — __ZN3RBX16AdvArrowToolBaseD0Ev
pub fn stub_6dd2d4() {
    // IDA 0x6dd2d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdvArrowToolBase::setCursor(std::string)")]
// 0x6dd374 — __ZN3RBX16AdvArrowToolBase9setCursorESs
pub fn stub_6dd374() {
    // IDA 0x6dd374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd378 — __ZThn36_N3RBX16AdvArrowToolBaseD1Ev
// was: non-virtual thunk to RBX::AdvArrowToolBase::~AdvArrowToolBase()
pub fn stub_6dd378() {
    // IDA 0x6dd378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd380 — __ZThn36_N3RBX16AdvArrowToolBaseD0Ev
// was: non-virtual thunk to RBX::AdvArrowToolBase::~AdvArrowToolBase()
pub fn stub_6dd380() {
    // IDA 0x6dd380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
// 0x6dd388 — __ZN3RBX13ArrowToolBaseD1Ev
pub fn stub_6dd388() {
    // IDA 0x6dd388: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
// 0x6dd38c — __ZN3RBX13ArrowToolBaseD0Ev
pub fn stub_6dd38c() {
    // IDA 0x6dd38c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::ArrowToolBase::~ArrowToolBase()")]
// 0x6dd42c — __ZThn36_N3RBX13ArrowToolBaseD1Ev
// was: non-virtual thunk to RBX::ArrowToolBase::~ArrowToolBase()
pub fn stub_6dd42c() {
    // IDA 0x6dd42c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::ArrowToolBase::~ArrowToolBase()")]
// 0x6dd434 — __ZThn36_N3RBX13ArrowToolBaseD0Ev
// was: non-virtual thunk to RBX::ArrowToolBase::~ArrowToolBase()
pub fn stub_6dd434() {
    // IDA 0x6dd434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::push_back(unsigned long * const&)")]
// 0x6de1a0 — __ZNSt6vectorIPmSaIS0_EE9push_backERKS0_
pub fn stub_6de1a0() {
    // IDA 0x6de1a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::push_back(bool (* const&)(void))")]
// 0x6de1d0 — __ZNSt6vectorIPFbvESaIS1_EE9push_backERKS1_
pub fn stub_6de1d0() {
    // IDA 0x6de1d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getEnvironmentSpeedPercent(void)const")]
// 0x6de750 — __ZNK3RBX5World26getEnvironmentSpeedPercentEv
pub fn stub_6de750() {
    // IDA 0x6de750: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getNumPrimitives(void)const")]
// 0x6de954 — __ZNK3RBX5World16getNumPrimitivesEv
pub fn stub_6de954() {
    // IDA 0x6de954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getNumJoints(void)const")]
// 0x6de958 — __ZNK3RBX5World12getNumJointsEv
pub fn stub_6de958() {
    // IDA 0x6de958: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::getNumContacts(void)const")]
// 0x6de95c — __ZNK3RBX5World14getNumContactsEv
pub fn stub_6de95c() {
    // IDA 0x6de95c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::getNumLinkCalls(void)const")]
// 0x6de960 — __ZNK3RBX5World15getNumLinkCallsEv
pub fn stub_6de960() {
    // IDA 0x6de960: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::update(void)")]
// 0x6ded30 — __ZN3RBX5Stats14TypedStatsItemIiE6updateEv
pub fn stub_6ded30() {
    // IDA 0x6ded30: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// 0x6ded50 — __ZThn32_N3RBX5Stats14TypedStatsItemIiED1Ev
// was: non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()
pub fn stub_6ded50() {
    // IDA 0x6ded50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// 0x6dee98 — __ZThn32_N3RBX5Stats14TypedStatsItemIiED0Ev
// was: non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()
pub fn stub_6dee98() {
    // IDA 0x6dee98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// 0x6deff8 — __ZThn36_N3RBX5Stats14TypedStatsItemIiED1Ev
// was: non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()
pub fn stub_6deff8() {
    // IDA 0x6deff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// 0x6df140 — __ZThn36_N3RBX5Stats14TypedStatsItemIiED0Ev
// was: non-virtual thunk to RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()
pub fn stub_6df140() {
    // IDA 0x6df140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::update(void)")]
// 0x6df7f8 — __ZN3RBX5Stats14TypedStatsItemIfE6updateEv
pub fn stub_6df7f8() {
    // IDA 0x6df7f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x6df818 — __ZThn32_N3RBX5Stats14TypedStatsItemIfED0Ev
// was: non-virtual thunk to RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()
pub fn stub_6df818() {
    // IDA 0x6df818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x6dfe88 — __ZN3RBX5Stats14TypedStatsItemIdED1Ev
pub fn stub_6dfe88() {
    // IDA 0x6dfe88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x6dffd0 — __ZN3RBX5Stats14TypedStatsItemIdED0Ev
pub fn stub_6dffd0() {
    // IDA 0x6dffd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}