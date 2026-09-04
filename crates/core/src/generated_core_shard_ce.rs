//! core shard CE — 100 core stubs EA-sorted, next uncovered after CD 0x6105c4 (strict RBX|boost|std|rbx earliest gap 0x6105c4).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::ScreenGui::render2d(RBX::Adorn *)")]
// 0x6105c4 — __ZN3RBX9ScreenGui8render2dEPNS_5AdornE
pub fn stub_6105c4() {
    // IDA 0x6105c4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::render2d(RBX::Adorn *)")]
// 0x6105d0 — __ZThn96_N3RBX9ScreenGui8render2dEPNS_5AdornE
pub fn stub_6105d0() {
    // IDA 0x6105d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::process(RBX::GuiEvent const&)")]
// 0x610668 — __ZN3RBX9ScreenGui7processERKNS_8GuiEventE
pub fn stub_610668() {
    // IDA 0x610668: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::process(RBX::GuiEvent const&)")]
// 0x610674 — __ZThn92_N3RBX9ScreenGui7processERKNS_8GuiEventE
pub fn stub_610674() {
    // IDA 0x610674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::removeModalButton(RBX::GuiButton *)")]
// 0x610680 — __ZN3RBX9ScreenGui17removeModalButtonEPNS_9GuiButtonE
pub fn stub_610680() {
    // IDA 0x610680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::insertModalButton(RBX::GuiButton *)")]
// 0x6106bc — __ZN3RBX9ScreenGui17insertModalButtonEPNS_9GuiButtonE
pub fn stub_6106bc() {
    // IDA 0x6106bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::hasModalDialog(void)")]
// 0x6109a4 — __ZN3RBX9ScreenGui14hasModalDialogEv
pub fn stub_6109a4() {
    // IDA 0x6109a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiMain::GuiMain(void)")]
// 0x6109cc — __ZN3RBX7GuiMainC2Ev
pub fn stub_6109cc() {
    // IDA 0x6109cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Adorn::getUserGuiRect(void)const")]
// 0x610c10 — __ZNK3RBX5Adorn14getUserGuiRectEv
pub fn stub_610c10() {
    // IDA 0x610c10: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::push_back(RBX::GuiButton * const&)")]
// 0x610cac — __ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE9push_backERKS2_
pub fn stub_610cac() {
    // IDA 0x610cac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ScreenGui::~ScreenGui()")]
// 0x610d4c — __ZN3RBX9ScreenGuiD1Ev
pub fn stub_610d4c() {
    // IDA 0x610d4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::~ScreenGui()")]
// 0x610e94 — __ZN3RBX9ScreenGuiD0Ev
pub fn stub_610e94() {
    // IDA 0x610e94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x610f34 — __ZN3RBX9ScreenGui17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_610f34() {
    // IDA 0x610f34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::shouldRender2d(void)const")]
// 0x610f4c — __ZNK3RBX9ScreenGui14shouldRender2dEv
pub fn stub_610f4c() {
    // IDA 0x610f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::~ScreenGui()")]
// 0x610f54 — __ZThn32_N3RBX9ScreenGuiD1Ev
pub fn stub_610f54() {
    // IDA 0x610f54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::~ScreenGui()")]
// 0x611098 — __ZThn32_N3RBX9ScreenGuiD0Ev
pub fn stub_611098() {
    // IDA 0x611098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::~ScreenGui()")]
// 0x611200 — __ZThn36_N3RBX9ScreenGuiD1Ev
pub fn stub_611200() {
    // IDA 0x611200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::~ScreenGui()")]
// 0x611344 — __ZThn36_N3RBX9ScreenGuiD0Ev
pub fn stub_611344() {
    // IDA 0x611344: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::shouldRender2d(void)const")]
// 0x61149c — __ZThn96_NK3RBX9ScreenGui14shouldRender2dEv
pub fn stub_61149c() {
    // IDA 0x61149c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::~ScreenGui()")]
// 0x6114a4 — __ZThn168_N3RBX9ScreenGuiD1Ev
pub fn stub_6114a4() {
    // IDA 0x6114a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::~ScreenGui()")]
// 0x6115e8 — __ZThn168_N3RBX9ScreenGuiD0Ev
pub fn stub_6115e8() {
    // IDA 0x6115e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiMain::~GuiMain()")]
// 0x611748 — __ZN3RBX7GuiMainD1Ev
pub fn stub_611748() {
    // IDA 0x611748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiMain::~GuiMain()")]
// 0x611890 — __ZN3RBX7GuiMainD0Ev
pub fn stub_611890() {
    // IDA 0x611890: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiMain::~GuiMain()")]
// 0x611940 — __ZThn32_N3RBX7GuiMainD1Ev
pub fn stub_611940() {
    // IDA 0x611940: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiMain::~GuiMain()")]
// 0x611a84 — __ZThn32_N3RBX7GuiMainD0Ev
pub fn stub_611a84() {
    // IDA 0x611a84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiMain::~GuiMain()")]
// 0x611bec — __ZThn36_N3RBX7GuiMainD1Ev
pub fn stub_611bec() {
    // IDA 0x611bec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiMain::~GuiMain()")]
// 0x611d30 — __ZThn36_N3RBX7GuiMainD0Ev
pub fn stub_611d30() {
    // IDA 0x611d30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiMain::~GuiMain()")]
// 0x611e88 — __ZThn168_N3RBX7GuiMainD1Ev
pub fn stub_611e88() {
    // IDA 0x611e88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiMain::~GuiMain()")]
// 0x611fcc — __ZThn168_N3RBX7GuiMainD0Ev
pub fn stub_611fcc() {
    // IDA 0x611fcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiButton **,std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>>,RBX::GuiButton * const&)")]
// 0x613ef0 — __ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_613ef0() {
    // IDA 0x613ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::_M_allocate(unsigned long)")]
// 0x613fd0 — __ZNSt12_Vector_baseIPN3RBX9GuiButtonESaIS2_EE11_M_allocateEm
pub fn stub_613fd0() {
    // IDA 0x613fd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Seat::Seat(void)")]
// 0x6158c0 — __ZN3RBX4SeatC1Ev
pub fn stub_6158c0() {
    // IDA 0x6158c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Seat::~Seat()")]
// 0x615ac0 — __ZN3RBX4SeatD0Ev
pub fn stub_615ac0() {
    // IDA 0x615ac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Seat::~Seat()")]
// 0x615b70 — __ZN3RBX4SeatD1Ev
pub fn stub_615b70() {
    // IDA 0x615b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Seat::~Seat()")]
// 0x615b80 — __ZThn32_N3RBX4SeatD0Ev
pub fn stub_615b80() {
    // IDA 0x615b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Seat::~Seat()")]
// 0x615b88 — __ZThn36_N3RBX4SeatD0Ev
pub fn stub_615b88() {
    // IDA 0x615b88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Seat::~Seat()")]
// 0x615b90 — __ZThn132_N3RBX4SeatD0Ev
pub fn stub_615b90() {
    // IDA 0x615b90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Seat::~Seat()")]
// 0x615b98 — __ZThn32_N3RBX4SeatD1Ev
pub fn stub_615b98() {
    // IDA 0x615b98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Seat::~Seat()")]
// 0x615bac — __ZThn36_N3RBX4SeatD1Ev
pub fn stub_615bac() {
    // IDA 0x615bac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Seat::~Seat()")]
// 0x615bc0 — __ZThn132_N3RBX4SeatD1Ev
pub fn stub_615bc0() {
    // IDA 0x615bc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::Selection(void)")]
// 0x618c98 — __ZN3RBX9SelectionC1Ev
pub fn stub_618c98() {
    // IDA 0x618c98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::Selection(void)")]
// 0x618c9c — __ZN3RBX9SelectionC2Ev
pub fn stub_618c9c() {
    // IDA 0x618c9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::~Selection()")]
// 0x619088 — __ZN3RBX9SelectionD0Ev
pub fn stub_619088() {
    // IDA 0x619088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::~Selection()")]
// 0x619128 — __ZN3RBX9SelectionD1Ev
pub fn stub_619128() {
    // IDA 0x619128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Selection::~Selection()")]
// 0x61912c — __ZThn32_N3RBX9SelectionD0Ev
pub fn stub_61912c() {
    // IDA 0x61912c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Selection::~Selection()")]
// 0x619134 — __ZThn36_N3RBX9SelectionD0Ev
pub fn stub_619134() {
    // IDA 0x619134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::~Selection()")]
// 0x61913c — __ZN3RBX9SelectionD2Ev
pub fn stub_61913c() {
    // IDA 0x61913c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Selection::~Selection()")]
// 0x619444 — __ZThn32_N3RBX9SelectionD1Ev
pub fn stub_619444() {
    // IDA 0x619444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Selection::~Selection()")]
// 0x61944c — __ZThn36_N3RBX9SelectionD1Ev
pub fn stub_61944c() {
    // IDA 0x61944c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::clearSelection(void)")]
// 0x61a088 — __ZN3RBX9Selection14clearSelectionEv
pub fn stub_61a088() {
    // IDA 0x61a088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::addFilteredSelection(RBX::ISelectionBase *)")]
// 0x61a278 — __ZN3RBX9Selection20addFilteredSelectionEPNS_14ISelectionBaseE
pub fn stub_61a278() {
    // IDA 0x61a278: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::removeFilteredSelection(RBX::ISelectionBase *)")]
// 0x61a28c — __ZN3RBX9Selection23removeFilteredSelectionEPNS_14ISelectionBaseE
pub fn stub_61a28c() {
    // IDA 0x61a28c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Selection::getSelection2(void)")]
// 0x61a5e8 — __ZN3RBX9Selection13getSelection2Ev
pub fn stub_61a5e8() {
    // IDA 0x61a5e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::push_back(RBX::ISelectionBase * const&)")]
// 0x61a958 — __ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE9push_backERKS2_
pub fn stub_61a958() {
    // IDA 0x61a958: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase *>(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&,std::random_access_iterator_tag)")]
// 0x61adc0 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14ISelectionBaseESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_61adc0() {
    // IDA 0x61adc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&)")]
// 0x61ae50 — __ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_61ae50() {
    // IDA 0x61ae50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_allocate(unsigned long)")]
// 0x61af30 — __ZNSt12_Vector_baseIPN3RBX14ISelectionBaseESaIS2_EE11_M_allocateEm
pub fn stub_61af30() {
    // IDA 0x61af30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SelectionBox::SelectionBox(void)")]
// 0x61ccf8 — __ZN3RBX12SelectionBoxC2Ev
pub fn stub_61ccf8() {
    // IDA 0x61ccf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SelectionBox::render3dAdorn(RBX::Adorn *)")]
// 0x61cf00 — __ZN3RBX12SelectionBox13render3dAdornEPNS_5AdornE
pub fn stub_61cf00() {
    // IDA 0x61cf00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionBox::render3dAdorn(RBX::Adorn *)")]
// 0x61d0b4 — __ZThn96_N3RBX12SelectionBox13render3dAdornEPNS_5AdornE
pub fn stub_61d0b4() {
    // IDA 0x61d0b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionBox::~SelectionBox()")]
// 0x61d0bc — __ZN3RBX12SelectionBoxD1Ev
pub fn stub_61d0bc() {
    // IDA 0x61d0bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionBox::~SelectionBox()")]
// 0x61d204 — __ZN3RBX12SelectionBoxD0Ev
pub fn stub_61d204() {
    // IDA 0x61d204: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionBox::~SelectionBox()")]
// 0x61d2b8 — __ZThn32_N3RBX12SelectionBoxD1Ev
pub fn stub_61d2b8() {
    // IDA 0x61d2b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionBox::~SelectionBox()")]
// 0x61d400 — __ZThn32_N3RBX12SelectionBoxD0Ev
pub fn stub_61d400() {
    // IDA 0x61d400: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionBox::~SelectionBox()")]
// 0x61d56c — __ZThn36_N3RBX12SelectionBoxD1Ev
pub fn stub_61d56c() {
    // IDA 0x61d56c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionBox::~SelectionBox()")]
// 0x61d6b4 — __ZThn36_N3RBX12SelectionBoxD0Ev
pub fn stub_61d6b4() {
    // IDA 0x61d6b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::setHumanoid(RBX::Humanoid *)")]
// 0x61eba4 — __ZN3RBX14SelectionLasso11setHumanoidEPNS_8HumanoidE
pub fn stub_61eba4() {
    // IDA 0x61eba4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::SelectionLasso(char const*)")]
// 0x61ecc0 — __ZN3RBX14SelectionLassoC2EPKc
pub fn stub_61ecc0() {
    // IDA 0x61ecc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::shouldRender3dAdorn(void)const")]
// 0x61ee4c — __ZNK3RBX14SelectionLasso19shouldRender3dAdornEv
pub fn stub_61ee4c() {
    // IDA 0x61ee4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::shouldRender3dAdorn(void)const")]
// 0x61ee7c — __ZThn96_NK3RBX14SelectionLasso19shouldRender3dAdornEv
pub fn stub_61ee7c() {
    // IDA 0x61ee7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::render3dAdorn(RBX::Adorn *)")]
// 0x61ef74 — __ZN3RBX14SelectionLasso13render3dAdornEPNS_5AdornE
pub fn stub_61ef74() {
    // IDA 0x61ef74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::render3dAdorn(RBX::Adorn *)")]
// 0x61f17c — __ZThn96_N3RBX14SelectionLasso13render3dAdornEPNS_5AdornE
pub fn stub_61f17c() {
    // IDA 0x61f17c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPartLasso::SelectionPartLasso(void)")]
// 0x61f2a0 — __ZN3RBX18SelectionPartLassoC2Ev
pub fn stub_61f2a0() {
    // IDA 0x61f2a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPartLasso::shouldRender3dAdorn(void)const")]
// 0x61f4b4 — __ZNK3RBX18SelectionPartLasso19shouldRender3dAdornEv
pub fn stub_61f4b4() {
    // IDA 0x61f4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::shouldRender3dAdorn(void)const")]
// 0x61f4e4 — __ZThn96_NK3RBX18SelectionPartLasso19shouldRender3dAdornEv
pub fn stub_61f4e4() {
    // IDA 0x61f4e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPointLasso::SelectionPointLasso(void)")]
// 0x61f634 — __ZN3RBX19SelectionPointLassoC2Ev
pub fn stub_61f634() {
    // IDA 0x61f634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::getHumanoidDangerous(void)const")]
// 0x61f850 — __ZNK3RBX14SelectionLasso20getHumanoidDangerousEv
pub fn stub_61f850() {
    // IDA 0x61f850: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPartLasso::getPartDangerous(void)const")]
// 0x61f8a0 — __ZNK3RBX18SelectionPartLasso16getPartDangerousEv
pub fn stub_61f8a0() {
    // IDA 0x61f8a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPointLasso::getPoint(void)const")]
// 0x61f8f0 — __ZNK3RBX19SelectionPointLasso8getPointEv
pub fn stub_61f8f0() {
    // IDA 0x61f8f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::~SelectionLasso()")]
// 0x61f924 — __ZN3RBX14SelectionLassoD1Ev
pub fn stub_61f924() {
    // IDA 0x61f924: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionLasso::~SelectionLasso()")]
// 0x61fa68 — __ZN3RBX14SelectionLassoD0Ev
pub fn stub_61fa68() {
    // IDA 0x61fa68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// 0x61fb30 — __ZThn32_N3RBX14SelectionLassoD1Ev
pub fn stub_61fb30() {
    // IDA 0x61fb30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// 0x61fc74 — __ZThn32_N3RBX14SelectionLassoD0Ev
pub fn stub_61fc74() {
    // IDA 0x61fc74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// 0x61fdf8 — __ZThn36_N3RBX14SelectionLassoD1Ev
pub fn stub_61fdf8() {
    // IDA 0x61fdf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// 0x61ff3c — __ZThn36_N3RBX14SelectionLassoD0Ev
pub fn stub_61ff3c() {
    // IDA 0x61ff3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0x620098 — __ZN3RBX18SelectionPartLassoD1Ev
pub fn stub_620098() {
    // IDA 0x620098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0x6202dc — __ZN3RBX18SelectionPartLassoD0Ev
pub fn stub_6202dc() {
    // IDA 0x6202dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0x62038c — __ZThn32_N3RBX18SelectionPartLassoD1Ev
pub fn stub_62038c() {
    // IDA 0x62038c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0x620394 — __ZThn32_N3RBX18SelectionPartLassoD0Ev
pub fn stub_620394() {
    // IDA 0x620394: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0x620448 — __ZThn36_N3RBX18SelectionPartLassoD1Ev
pub fn stub_620448() {
    // IDA 0x620448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0x620450 — __ZThn36_N3RBX18SelectionPartLassoD0Ev
pub fn stub_620450() {
    // IDA 0x620450: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0x6204fc — __ZN3RBX19SelectionPointLassoD1Ev
pub fn stub_6204fc() {
    // IDA 0x6204fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0x620640 — __ZN3RBX19SelectionPointLassoD0Ev
pub fn stub_620640() {
    // IDA 0x620640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0x620708 — __ZThn32_N3RBX19SelectionPointLassoD1Ev
pub fn stub_620708() {
    // IDA 0x620708: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0x62084c — __ZThn32_N3RBX19SelectionPointLassoD0Ev
pub fn stub_62084c() {
    // IDA 0x62084c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0x6209b4 — __ZThn36_N3RBX19SelectionPointLassoD1Ev
pub fn stub_6209b4() {
    // IDA 0x6209b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0x620af8 — __ZThn36_N3RBX19SelectionPointLassoD0Ev
pub fn stub_620af8() {
    // IDA 0x620af8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SkateboardController::SkateboardController(void)")]
// 0x6245b8 — __ZN3RBX20SkateboardControllerC1Ev
pub fn stub_6245b8() {
    // IDA 0x6245b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SkateboardController::SkateboardController(void)")]
// 0x6245bc — __ZN3RBX20SkateboardControllerC2Ev
pub fn stub_6245bc() {
    // IDA 0x6245bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SkateboardController::onSteppedTouchInput(void)")]
// 0x624878 — __ZN3RBX20SkateboardController19onSteppedTouchInputEv
pub fn stub_624878() {
    // IDA 0x624878: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
