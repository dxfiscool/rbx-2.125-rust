//! watchdog coreB — 120 core stubs EA-sorted, core namespace (rbx::/RBX::/Vector3/SystemAddress/Memory)
//! Source: ida/export.json (85545 funcs) filtered for core namespace (rbx::, RBX::, Vector3, SystemAddress, Memory), SKIP EAs in global set (/tmp/global_eas.txt 70358 unique), EA-sorted asc next 120 uncovered.
//! Range: 0x8865c8..0x892aa0 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + pub fn stub_0xADDR todo!("0xADDR")

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "RBX::PluginManager::StateDataEntry::getToolbar(std::string,RBX::IStudioPluginHost *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE")]
// 0x8865c8 — __ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE — RBX::PluginManager::StateDataEntry::getToolbar(std::string,RBX::IStudioPluginHost *)
pub fn stub_0x8865c8() {
    // IDA 0x8865c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PluginManager::StateDataEntry::hideStudioUI(bool,RBX::IStudioPluginHost *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE")]
// 0x886808 — __ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE — RBX::PluginManager::StateDataEntry::hideStudioUI(bool,RBX::IStudioPluginHost *)
pub fn stub_0x886808() {
    // IDA 0x886808: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PluginManager::StateDataEntry::fireButtonClick(void *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv")]
// 0x886950 — __ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv — RBX::PluginManager::StateDataEntry::fireButtonClick(void *)
pub fn stub_0x886950() {
    // IDA 0x886950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
#[doc(alias = "__ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs")]
// 0x886984 — __ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs — RBX::PluginManager::createToolbar(RBX::Plugin *,std::string)
pub fn stub_0x886984() {
    // IDA 0x886984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs")]
// 0x886b40 — __ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs — non-virtual thunk toRBX::PluginManager::createToolbar(RBX::Plugin *,std::string)
pub fn stub_0x886b40() {
    // IDA 0x886b40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PluginManager>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev")]
// 0x886e58 — __ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev — rbx_core::SharedPtr<RBX::PluginManager>::~shared_ptr()
pub fn stub_0x886e58() {
    // IDA 0x886e58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<void *,rbx_core::SharedPtr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::operator[](void * const&)")]
#[doc(alias = "__ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_")]
// 0x886f1c — __ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_ — std::map<void *,rbx_core::SharedPtr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::operator[](void * const&)
pub fn stub_0x886f1c() {
    // IDA 0x886f1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Button>::operator=(rbx_core::SharedPtr<RBX::Button> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_")]
// 0x887064 — __ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_ — rbx_core::SharedPtr<RBX::Button>::operator=(rbx_core::SharedPtr<RBX::Button> const&)
pub fn stub_0x887064() {
    // IDA 0x887064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")]
// 0x8872e0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_ — std::map<std::string,rbx_core::SharedPtr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::operator[](std::string const&)
pub fn stub_0x8872e0() {
    // IDA 0x8872e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Toolbar>::operator=(rbx_core::SharedPtr<RBX::Toolbar> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_")]
// 0x8874fc — __ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_ — rbx_core::SharedPtr<RBX::Toolbar>::operator=(rbx_core::SharedPtr<RBX::Toolbar> const&)
pub fn stub_0x8874fc() {
    // IDA 0x8874fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD1Ev")]
// 0x887534 — __ZN3RBX13PluginManagerD1Ev — RBX::PluginManager::~PluginManager()
pub fn stub_0x887534() {
    // IDA 0x887534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD0Ev")]
// 0x887538 — __ZN3RBX13PluginManagerD0Ev — RBX::PluginManager::~PluginManager()
pub fn stub_0x887538() {
    // IDA 0x887538: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn32_N3RBX13PluginManagerD1Ev")]
// 0x887600 — __ZThn32_N3RBX13PluginManagerD1Ev — non-virtual thunk toRBX::PluginManager::~PluginManager()
pub fn stub_0x887600() {
    // IDA 0x887600: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn32_N3RBX13PluginManagerD0Ev")]
// 0x887608 — __ZThn32_N3RBX13PluginManagerD0Ev — non-virtual thunk toRBX::PluginManager::~PluginManager()
pub fn stub_0x887608() {
    // IDA 0x887608: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn36_N3RBX13PluginManagerD1Ev")]
// 0x8876d4 — __ZThn36_N3RBX13PluginManagerD1Ev — non-virtual thunk toRBX::PluginManager::~PluginManager()
pub fn stub_0x8876d4() {
    // IDA 0x8876d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn36_N3RBX13PluginManagerD0Ev")]
// 0x8876dc — __ZThn36_N3RBX13PluginManagerD0Ev — non-virtual thunk toRBX::PluginManager::~PluginManager()
pub fn stub_0x8876dc() {
    // IDA 0x8876dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Button::~Button()")]
#[doc(alias = "__ZN3RBX6ButtonD1Ev")]
// 0x8877ac — __ZN3RBX6ButtonD1Ev — RBX::Button::~Button()
pub fn stub_0x8877ac() {
    // IDA 0x8877ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Button::~Button()")]
#[doc(alias = "__ZN3RBX6ButtonD0Ev")]
// 0x8878c0 — __ZN3RBX6ButtonD0Ev — RBX::Button::~Button()
pub fn stub_0x8878c0() {
    // IDA 0x8878c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn32_N3RBX6ButtonD1Ev")]
// 0x8879f8 — __ZThn32_N3RBX6ButtonD1Ev — non-virtual thunk toRBX::Button::~Button()
pub fn stub_0x8879f8() {
    // IDA 0x8879f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn32_N3RBX6ButtonD0Ev")]
// 0x887b08 — __ZThn32_N3RBX6ButtonD0Ev — non-virtual thunk toRBX::Button::~Button()
pub fn stub_0x887b08() {
    // IDA 0x887b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn36_N3RBX6ButtonD1Ev")]
// 0x887c40 — __ZThn36_N3RBX6ButtonD1Ev — non-virtual thunk toRBX::Button::~Button()
pub fn stub_0x887c40() {
    // IDA 0x887c40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn36_N3RBX6ButtonD0Ev")]
// 0x887d50 — __ZThn36_N3RBX6ButtonD0Ev — non-virtual thunk toRBX::Button::~Button()
pub fn stub_0x887d50() {
    // IDA 0x887d50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZN3RBX7ToolbarD1Ev")]
// 0x887e78 — __ZN3RBX7ToolbarD1Ev — RBX::Toolbar::~Toolbar()
pub fn stub_0x887e78() {
    // IDA 0x887e78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZN3RBX7ToolbarD0Ev")]
// 0x887f64 — __ZN3RBX7ToolbarD0Ev — RBX::Toolbar::~Toolbar()
pub fn stub_0x887f64() {
    // IDA 0x887f64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn32_N3RBX7ToolbarD1Ev")]
// 0x888070 — __ZThn32_N3RBX7ToolbarD1Ev — non-virtual thunk toRBX::Toolbar::~Toolbar()
pub fn stub_0x888070() {
    // IDA 0x888070: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn32_N3RBX7ToolbarD0Ev")]
// 0x888158 — __ZThn32_N3RBX7ToolbarD0Ev — non-virtual thunk toRBX::Toolbar::~Toolbar()
pub fn stub_0x888158() {
    // IDA 0x888158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn36_N3RBX7ToolbarD1Ev")]
// 0x888268 — __ZThn36_N3RBX7ToolbarD1Ev — non-virtual thunk toRBX::Toolbar::~Toolbar()
pub fn stub_0x888268() {
    // IDA 0x888268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn36_N3RBX7ToolbarD0Ev")]
// 0x888350 — __ZThn36_N3RBX7ToolbarD0Ev — non-virtual thunk toRBX::Toolbar::~Toolbar()
pub fn stub_0x888350() {
    // IDA 0x888350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x888450 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)
pub fn stub_0x888450() {
    // IDA 0x888450: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E")]
// 0x888478 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)
pub fn stub_0x888478() {
    // IDA 0x888478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEE7destroyEPS8_")]
// 0x889784 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEE7destroyEPS8_ — __gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>*)
pub fn stub_0x889784() {
    // IDA 0x889784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0x889828 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> *)
pub fn stub_0x889828() {
    // IDA 0x889828: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Toolbar> const&)")]
#[doc(alias = "__ZNSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEC2ERS0_RKS5_")]
// 0x889858 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEC2ERS0_RKS5_ — std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Toolbar> const&)
pub fn stub_0x889858() {
    // IDA 0x889858: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// 0x889914 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)
pub fn stub_0x889914() {
    // IDA 0x889914: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
// 0x889a00 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)
pub fn stub_0x889a00() {
    // IDA 0x889a00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_")]
// 0x889a50 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)
pub fn stub_0x889a50() {
    // IDA 0x889a50: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_")]
// 0x889ad4 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)
pub fn stub_0x889ad4() {
    // IDA 0x889ad4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::lower_bound(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_")]
// 0x889bdc — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::lower_bound(std::string const&)
pub fn stub_0x889bdc() {
    // IDA 0x889bdc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_")]
// 0x889f08 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::find(std::string const&)
pub fn stub_0x889f08() {
    // IDA 0x889f08: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::list(std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EEC2ERKS6_")]
// 0x889f58 — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EEC2ERKS6_ — std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::list(std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>> const&)
pub fn stub_0x889f58() {
    // IDA 0x889f58: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>>(std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::__false_type)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type")]
// 0x88a020 — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type — void std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>>(std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::__false_type)
pub fn stub_0x88a020() {
    // IDA 0x88a020: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE8_M_clearEv")]
// 0x88a044 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE8_M_clearEv — std::_List_base<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_clear(void)
pub fn stub_0x88a044() {
    // IDA 0x88a044: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_create_node(rbx_core::SharedPtr<RBX::Plugin> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE14_M_create_nodeERKS4_")]
// 0x88a06c — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE14_M_create_nodeERKS4_ — std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_create_node(rbx_core::SharedPtr<RBX::Plugin> const&)
pub fn stub_0x88a06c() {
    // IDA 0x88a06c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EEC2ERKSD_")]
// 0x88a250 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EEC2ERKSD_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>> const&)
pub fn stub_0x88a250() {
    // IDA 0x88a250: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>*)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE7_M_copyEPKSt13_Rb_tree_nodeIS7_EPSF_")]
// 0x88a294 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE7_M_copyEPKSt13_Rb_tree_nodeIS7_EPSF_ — std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>*)
pub fn stub_0x88a294() {
    // IDA 0x88a294: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x88afb0 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)
pub fn stub_0x88afb0() {
    // IDA 0x88afb0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x88b064 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)
pub fn stub_0x88b064() {
    // IDA 0x88b064: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x88b0b0 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)
pub fn stub_0x88b0b0() {
    // IDA 0x88b0b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_create_node(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE14_M_create_nodeERKS8_")]
// 0x88b118 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE14_M_create_nodeERKS8_ — std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_create_node(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)
pub fn stub_0x88b118() {
    // IDA 0x88b118: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD2Ev")]
// 0x88d7f4 — __ZN3RBX13PluginManagerD2Ev — RBX::PluginManager::~PluginManager()
pub fn stub_0x88d7f4() {
    // IDA 0x88d7f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
#[doc(alias = "__ZN3RBX11PluginMouseC1Ev")]
// 0x88e2c8 — __ZN3RBX11PluginMouseC1Ev — RBX::PluginMouse::PluginMouse(void)
pub fn stub_0x88e2c8() {
    // IDA 0x88e2c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
#[doc(alias = "__ZN3RBX11PluginMouseC2Ev")]
// 0x88e2cc — __ZN3RBX11PluginMouseC2Ev — RBX::PluginMouse::PluginMouse(void)
pub fn stub_0x88e2cc() {
    // IDA 0x88e2cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZN3RBX11PluginMouseD0Ev")]
// 0x88e468 — __ZN3RBX11PluginMouseD0Ev — RBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e468() {
    // IDA 0x88e468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZN3RBX11PluginMouseD1Ev")]
// 0x88e508 — __ZN3RBX11PluginMouseD1Ev — RBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e508() {
    // IDA 0x88e508: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn32_N3RBX11PluginMouseD0Ev")]
// 0x88e50c — __ZThn32_N3RBX11PluginMouseD0Ev — non-virtual thunk toRBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e50c() {
    // IDA 0x88e50c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn36_N3RBX11PluginMouseD0Ev")]
// 0x88e514 — __ZThn36_N3RBX11PluginMouseD0Ev — non-virtual thunk toRBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e514() {
    // IDA 0x88e514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZN3RBX11PluginMouseD2Ev")]
// 0x88e51c — __ZN3RBX11PluginMouseD2Ev — RBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e51c() {
    // IDA 0x88e51c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn32_N3RBX11PluginMouseD1Ev")]
// 0x88e614 — __ZThn32_N3RBX11PluginMouseD1Ev — non-virtual thunk toRBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e614() {
    // IDA 0x88e614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn36_N3RBX11PluginMouseD1Ev")]
// 0x88e61c — __ZThn36_N3RBX11PluginMouseD1Ev — non-virtual thunk toRBX::PluginMouse::~PluginMouse()
pub fn stub_0x88e61c() {
    // IDA 0x88e61c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::getHit(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse6getHitEv")]
// 0x88e624 — __ZNK3RBX11PluginMouse6getHitEv — RBX::PluginMouse::getHit(void)const
pub fn stub_0x88e624() {
    // IDA 0x88e624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::getOrigin(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse9getOriginEv")]
// 0x88e8ec — __ZNK3RBX11PluginMouse9getOriginEv — RBX::PluginMouse::getOrigin(void)const
pub fn stub_0x88e8ec() {
    // IDA 0x88e8ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::getUnitRay(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse10getUnitRayEv")]
// 0x88e9f4 — __ZNK3RBX11PluginMouse10getUnitRayEv — RBX::PluginMouse::getUnitRay(void)const
pub fn stub_0x88e9f4() {
    // IDA 0x88e9f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::getTarget(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse9getTargetEv")]
// 0x88eae0 — __ZNK3RBX11PluginMouse9getTargetEv — RBX::PluginMouse::getTarget(void)const
pub fn stub_0x88eae0() {
    // IDA 0x88eae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::getTargetSurface(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse16getTargetSurfaceEv")]
// 0x88ee2c — __ZNK3RBX11PluginMouse16getTargetSurfaceEv — RBX::PluginMouse::getTargetSurface(void)const
pub fn stub_0x88ee2c() {
    // IDA 0x88ee2c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PluginMouse::update(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX11PluginMouse6updateERKNS_7UIEventE")]
// 0x88ef90 — __ZN3RBX11PluginMouse6updateERKNS_7UIEventE — RBX::PluginMouse::update(RBX::UIEvent const&)
pub fn stub_0x88ef90() {
    // IDA 0x88ef90: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PluginMouse::getX(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse4getXEv")]
// 0x88f2a8 — __ZNK3RBX11PluginMouse4getXEv — RBX::PluginMouse::getX(void)const
pub fn stub_0x88f2a8() {
    // IDA 0x88f2a8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PluginMouse::getY(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse4getYEv")]
// 0x88f2b0 — __ZNK3RBX11PluginMouse4getYEv — RBX::PluginMouse::getY(void)const
pub fn stub_0x88f2b0() {
    // IDA 0x88f2b0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PluginMouse::getViewSizeX(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse12getViewSizeXEv")]
// 0x88f2b8 — __ZNK3RBX11PluginMouse12getViewSizeXEv — RBX::PluginMouse::getViewSizeX(void)const
pub fn stub_0x88f2b8() {
    // IDA 0x88f2b8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PluginMouse::getViewSizeY(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse12getViewSizeYEv")]
// 0x88f2c0 — __ZNK3RBX11PluginMouse12getViewSizeYEv — RBX::PluginMouse::getViewSizeY(void)const
pub fn stub_0x88f2c0() {
    // IDA 0x88f2c0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PluginMouse::checkActive(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse11checkActiveEv")]
// 0x88f4e0 — __ZNK3RBX11PluginMouse11checkActiveEv — RBX::PluginMouse::checkActive(void)const
pub fn stub_0x88f4e0() {
    // IDA 0x88f4e0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
#[doc(alias = "__ZN3RBX15BallCellContactD0Ev")]
// 0x88fdc0 — __ZN3RBX15BallCellContactD0Ev — RBX::BallCellContact::~BallCellContact()
pub fn stub_0x88fdc0() {
    // IDA 0x88fdc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
#[doc(alias = "__ZN3RBX15BallCellContactD1Ev")]
// 0x88fe74 — __ZN3RBX15BallCellContactD1Ev — RBX::BallCellContact::~BallCellContact()
pub fn stub_0x88fe74() {
    // IDA 0x88fe74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
#[doc(alias = "__ZN3RBX15BallCellContactD2Ev")]
// 0x88fe78 — __ZN3RBX15BallCellContactD2Ev — RBX::BallCellContact::~BallCellContact()
pub fn stub_0x88fe78() {
    // IDA 0x88fe78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX15BallCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// 0x88ff94 — __ZN3RBX15BallCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE — RBX::BallCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)
pub fn stub_0x88ff94() {
    // IDA 0x88ff94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::newBallPlaneConnector(RBX::POLY::Face const*)")]
#[doc(alias = "__ZN3RBX15BallCellContact21newBallPlaneConnectorEPKNS_4POLY4FaceE")]
// 0x890268 — __ZN3RBX15BallCellContact21newBallPlaneConnectorEPKNS_4POLY4FaceE — RBX::BallCellContact::newBallPlaneConnector(RBX::POLY::Face const*)
pub fn stub_0x890268() {
    // IDA 0x890268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::newBallEdgeConnector(RBX::POLY::Edge const*)")]
#[doc(alias = "__ZN3RBX15BallCellContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE")]
// 0x890514 — __ZN3RBX15BallCellContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE — RBX::BallCellContact::newBallEdgeConnector(RBX::POLY::Edge const*)
pub fn stub_0x890514() {
    // IDA 0x890514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::newBallVertexConnector(RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX15BallCellContact22newBallVertexConnectorEPKNS_4POLY6VertexE")]
// 0x8908e8 — __ZN3RBX15BallCellContact22newBallVertexConnectorEPKNS_4POLY6VertexE — RBX::BallCellContact::newBallVertexConnector(RBX::POLY::Vertex const*)
pub fn stub_0x8908e8() {
    // IDA 0x8908e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallCellContact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX15BallCellContact34generateDataForMovingAssemblyStageEv")]
// 0x890ad4 — __ZN3RBX15BallCellContact34generateDataForMovingAssemblyStageEv — RBX::BallCellContact::generateDataForMovingAssemblyStage(void)
pub fn stub_0x890ad4() {
    // IDA 0x890ad4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallCellContactEEC2Ev")]
// 0x890ad8 — __ZN3RBX9AllocatorINS_15BallCellContactEEC2Ev — RBX::Allocator<RBX::BallCellContact>::Allocator(void)
pub fn stub_0x890ad8() {
    // IDA 0x890ad8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)")]
#[doc(alias = "__ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE9push_backERKS2_")]
// 0x890b3c — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE9push_backERKS2_ — RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)
pub fn stub_0x890b3c() {
    // IDA 0x890b3c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::plane(void)const")]
#[doc(alias = "__ZNK3RBX4POLY4Face5planeEv")]
// 0x890c24 — __ZNK3RBX4POLY4Face5planeEv — RBX::POLY::Face::plane(void)const
pub fn stub_0x890c24() {
    // IDA 0x890c24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEEnwEm")]
// 0x890ce4 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEnwEm — RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)
pub fn stub_0x890ce4() {
    // IDA 0x890ce4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEEnwEm")]
// 0x890d54 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEnwEm — RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)
pub fn stub_0x890d54() {
    // IDA 0x890d54: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const")]
#[doc(alias = "__ZNK3RBX4POLY4Edge13computeNormalEPKNS0_4FaceE")]
// 0x890dc4 — __ZNK3RBX4POLY4Edge13computeNormalEPKNS0_4FaceE — RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const
pub fn stub_0x890dc4() {
    // IDA 0x890dc4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEEnwEm")]
// 0x890e50 — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEnwEm — RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)
pub fn stub_0x890e50() {
    // IDA 0x890e50: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::CellContact::numConnectors(void)const")]
#[doc(alias = "__ZNK3RBX11CellContact13numConnectorsEv")]
// 0x890ec0 — __ZNK3RBX11CellContact13numConnectorsEv — RBX::CellContact::numConnectors(void)const
pub fn stub_0x890ec0() {
    // IDA 0x890ec0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x890ec8 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x890ec8() {
    // IDA 0x890ec8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// 0x890f18 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x890f18() {
    // IDA 0x890f18: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x890f50 — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x890f50() {
    // IDA 0x890f50: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// 0x890fa0 — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x890fa0() {
    // IDA 0x890fa0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x890fd8 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x890fd8() {
    // IDA 0x890fd8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// 0x891028 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x891028() {
    // IDA 0x891028: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallCellContactEE13releaseMemoryEv")]
// 0x891060 — __ZN3RBX9AllocatorINS_15BallCellContactEE13releaseMemoryEv — RBX::Allocator<RBX::BallCellContact>::releaseMemory(void)
pub fn stub_0x891060() {
    // IDA 0x891060: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// 0x89107c — __ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x89107c() {
    // IDA 0x89107c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x8910ac — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x8910ac() {
    // IDA 0x8910ac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x8910fc — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x8910fc() {
    // IDA 0x8910fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x89114c — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x89114c() {
    // IDA 0x89114c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEEC2Ev")]
// 0x89119c — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEC2Ev — RBX::Allocator<RBX::BallVertexConnector>::Allocator(void)
pub fn stub_0x89119c() {
    // IDA 0x89119c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallVertexConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX19BallVertexConnector16getConnectorTypeEv")]
// 0x891200 — __ZNK3RBX19BallVertexConnector16getConnectorTypeEv — RBX::BallVertexConnector::getConnectorType(void)const
pub fn stub_0x891200() {
    // IDA 0x891200: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEE13releaseMemoryEv")]
// 0x891204 — __ZN3RBX9AllocatorINS_19BallVertexConnectorEE13releaseMemoryEv — RBX::Allocator<RBX::BallVertexConnector>::releaseMemory(void)
pub fn stub_0x891204() {
    // IDA 0x891204: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// 0x891220 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x891220() {
    // IDA 0x891220: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PolyConnector::~PolyConnector()")]
#[doc(alias = "__ZN3RBX13PolyConnectorD1Ev")]
// 0x891250 — __ZN3RBX13PolyConnectorD1Ev — RBX::PolyConnector::~PolyConnector()
pub fn stub_0x891250() {
    // IDA 0x891250: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PolyConnector::~PolyConnector()")]
#[doc(alias = "__ZN3RBX13PolyConnectorD0Ev")]
// 0x891254 — __ZN3RBX13PolyConnectorD0Ev — RBX::PolyConnector::~PolyConnector()
pub fn stub_0x891254() {
    // IDA 0x891254: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEEC2Ev")]
// 0x891258 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEC2Ev — RBX::Allocator<RBX::BallEdgeConnector>::Allocator(void)
pub fn stub_0x891258() {
    // IDA 0x891258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallEdgeConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX17BallEdgeConnector16getConnectorTypeEv")]
// 0x8912bc — __ZNK3RBX17BallEdgeConnector16getConnectorTypeEv — RBX::BallEdgeConnector::getConnectorType(void)const
pub fn stub_0x8912bc() {
    // IDA 0x8912bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEE13releaseMemoryEv")]
// 0x8912c0 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEE13releaseMemoryEv — RBX::Allocator<RBX::BallEdgeConnector>::releaseMemory(void)
pub fn stub_0x8912c0() {
    // IDA 0x8912c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// 0x8912dc — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x8912dc() {
    // IDA 0x8912dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEEC2Ev")]
// 0x89130c — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEC2Ev — RBX::Allocator<RBX::BallPlaneConnector>::Allocator(void)
pub fn stub_0x89130c() {
    // IDA 0x89130c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallPlaneConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX18BallPlaneConnector16getConnectorTypeEv")]
// 0x891370 — __ZNK3RBX18BallPlaneConnector16getConnectorTypeEv — RBX::BallPlaneConnector::getConnectorType(void)const
pub fn stub_0x891370() {
    // IDA 0x891370: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEE13releaseMemoryEv")]
// 0x891374 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEE13releaseMemoryEv — RBX::Allocator<RBX::BallPlaneConnector>::releaseMemory(void)
pub fn stub_0x891374() {
    // IDA 0x891374: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// 0x891390 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x891390() {
    // IDA 0x891390: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PersonalServerService::setPersonalServerGetRankUrl(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService27setPersonalServerGetRankUrlESs")]
// 0x891ae8 — __ZN3RBX21PersonalServerService27setPersonalServerGetRankUrlESs — RBX::PersonalServerService::setPersonalServerGetRankUrl(std::string)
pub fn stub_0x891ae8() {
    // IDA 0x891ae8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::setPersonalServerSetRankUrl(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService27setPersonalServerSetRankUrlESs")]
// 0x891af0 — __ZN3RBX21PersonalServerService27setPersonalServerSetRankUrlESs — RBX::PersonalServerService::setPersonalServerSetRankUrl(std::string)
pub fn stub_0x891af0() {
    // IDA 0x891af0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::setPersonalServerRoleSetsUrl(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService28setPersonalServerRoleSetsUrlESs")]
// 0x891af8 — __ZN3RBX21PersonalServerService28setPersonalServerRoleSetsUrlESs — RBX::PersonalServerService::setPersonalServerRoleSetsUrl(std::string)
pub fn stub_0x891af8() {
    // IDA 0x891af8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::getWebRoleSets(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX21PersonalServerService14getWebRoleSetsEiN5boost8functionIFvSsEEES4_")]
// 0x891bc8 — __ZN3RBX21PersonalServerService14getWebRoleSetsEiN5boost8functionIFvSsEEES4_ — RBX::PersonalServerService::getWebRoleSets(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_0x891bc8() {
    // IDA 0x891bc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::PersonalServerService(void)")]
#[doc(alias = "__ZN3RBX21PersonalServerServiceC1Ev")]
// 0x8920c4 — __ZN3RBX21PersonalServerServiceC1Ev — RBX::PersonalServerService::PersonalServerService(void)
pub fn stub_0x8920c4() {
    // IDA 0x8920c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::PersonalServerService(void)")]
#[doc(alias = "__ZN3RBX21PersonalServerServiceC2Ev")]
// 0x8920c8 — __ZN3RBX21PersonalServerServiceC2Ev — RBX::PersonalServerService::PersonalServerService(void)
pub fn stub_0x8920c8() {
    // IDA 0x8920c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::getCurrentPrivilege(int)")]
#[doc(alias = "__ZN3RBX21PersonalServerService19getCurrentPrivilegeEi")]
// 0x892788 — __ZN3RBX21PersonalServerService19getCurrentPrivilegeEi — RBX::PersonalServerService::getCurrentPrivilege(int)
pub fn stub_0x892788() {
    // IDA 0x892788: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::getRoleSets(void)const")]
#[doc(alias = "__ZNK3RBX21PersonalServerService11getRoleSetsEv")]
// 0x892a94 — __ZNK3RBX21PersonalServerService11getRoleSetsEv — RBX::PersonalServerService::getRoleSets(void)const
pub fn stub_0x892a94() {
    // IDA 0x892a94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::setRoleSets(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService11setRoleSetsESs")]
// 0x892aa0 — __ZN3RBX21PersonalServerService11setRoleSetsESs — RBX::PersonalServerService::setRoleSets(std::string)
pub fn stub_0x892aa0() {
    // IDA 0x892aa0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
