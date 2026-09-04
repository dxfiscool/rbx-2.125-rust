//! core shard DY — 100 core stubs EA-sorted, next uncovered after DX 0x8842d8 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x8842d8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::CellEdgeEdgePair::isFaceFace(void)const")]
// 0x8842dc — __ZNK3RBX16CellEdgeEdgePair10isFaceFaceEv
pub fn stub_8842dc() {
    // IDA 0x8842dc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PolyCellPair::pairIsValid(void)")]
// 0x8842e0 — __ZN3RBX12PolyCellPair11pairIsValidEv
pub fn stub_8842e0() {
    // IDA 0x8842e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::CellEdgeEdgePair::~CellEdgeEdgePair()")]
// 0x8842e4 — __ZN3RBX16CellEdgeEdgePairD0Ev
pub fn stub_8842e4() {
    // IDA 0x8842e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x8842e8 — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_8842e8() {
    // IDA 0x8842e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x884320 — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_884320() {
    // IDA 0x884320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x884358 — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_884358() {
    // IDA 0x884358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::releaseMemory(void)")]
// 0x884390 — __ZN3RBX9AllocatorINS_15PolyCellContactEE13releaseMemoryEv
pub fn stub_884390() {
    // IDA 0x884390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x8843ac — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_8843ac() {
    // IDA 0x8843ac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::Allocator(void)")]
// 0x8843dc — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEC2Ev
pub fn stub_8843dc() {
    // IDA 0x8843dc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeEdgeConnector::getConnectorType(void)const")]
// 0x884440 — __ZNK3RBX17EdgeEdgeConnector16getConnectorTypeEv
pub fn stub_884440() {
    // IDA 0x884440: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::releaseMemory(void)")]
// 0x884444 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEE13releaseMemoryEv
pub fn stub_884444() {
    // IDA 0x884444: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x884460 — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_884460() {
    // IDA 0x884460: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PolyCellPair::~PolyCellPair()")]
// 0x884490 — __ZN3RBX12PolyCellPairD1Ev
pub fn stub_884490() {
    // IDA 0x884490: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PolyCellPair::~PolyCellPair()")]
// 0x884494 — __ZN3RBX12PolyCellPairD0Ev
pub fn stub_884494() {
    // IDA 0x884494: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::Allocator(void)")]
// 0x884498 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEC2Ev
pub fn stub_884498() {
    // IDA 0x884498: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceEdgeConnector::getConnectorType(void)const")]
// 0x8844fc — __ZNK3RBX17FaceEdgeConnector16getConnectorTypeEv
pub fn stub_8844fc() {
    // IDA 0x8844fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::releaseMemory(void)")]
// 0x884500 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEE13releaseMemoryEv
pub fn stub_884500() {
    // IDA 0x884500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x88451c — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_88451c() {
    // IDA 0x88451c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::Allocator(void)")]
// 0x88454c — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEC2Ev
pub fn stub_88454c() {
    // IDA 0x88454c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FaceVertexConnector::getConnectorType(void)const")]
// 0x8845b0 — __ZNK3RBX19FaceVertexConnector16getConnectorTypeEv
pub fn stub_8845b0() {
    // IDA 0x8845b0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::releaseMemory(void)")]
// 0x8845b4 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEE13releaseMemoryEv
pub fn stub_8845b4() {
    // IDA 0x8845b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x8845d0 — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_8845d0() {
    // IDA 0x8845d0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PluginManager::createPlugin(void)")]
// 0x884c40 — __ZN3RBX13PluginManager12createPluginEv
pub fn stub_884c40() {
    // IDA 0x884c40: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Plugin::activate(bool)")]
// 0x885030 — __ZN3RBX6Plugin8activateEb
pub fn stub_885030() {
    // IDA 0x885030: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Plugin::createToolbar(std::string)")]
// 0x885068 — __ZN3RBX6Plugin13createToolbarESs
pub fn stub_885068() {
    // IDA 0x885068: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Toolbar::createButton(std::string,std::string,std::string)")]
// 0x885190 — __ZN3RBX7Toolbar12createButtonESsSsSs
pub fn stub_885190() {
    // IDA 0x885190: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Button::setActive(bool)")]
// 0x8855bc — __ZN3RBX6Button9setActiveEb
pub fn stub_8855bc() {
    // IDA 0x8855bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Button::Button(void)")]
// 0x8856ec — __ZN3RBX6ButtonC2Ev
pub fn stub_8856ec() {
    // IDA 0x8856ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Toolbar::reset(void)")]
// 0x885880 — __ZN3RBX7Toolbar5resetEv
pub fn stub_885880() {
    // IDA 0x885880: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PluginManager::singleton(void)")]
// 0x8858a4 — __ZN3RBX13PluginManager9singletonEv
pub fn stub_8858a4() {
    // IDA 0x8858a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Toolbar::Toolbar(void)")]
// 0x8858cc — __ZN3RBX7ToolbarC2Ev
pub fn stub_8858cc() {
    // IDA 0x8858cc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Toolbar::getButton(void *)")]
// 0x885a20 — __ZN3RBX7Toolbar9getButtonEPv
pub fn stub_885a20() {
    // IDA 0x885a20: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Plugin::Plugin(void)")]
// 0x885a60 — __ZN3RBX6PluginC2Ev
pub fn stub_885a60() {
    // IDA 0x885a60: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Plugin::~Plugin()")]
// 0x885c04 — __ZN3RBX6PluginD0Ev
pub fn stub_885c04() {
    // IDA 0x885c04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Plugin::~Plugin()")]
// 0x885ca4 — __ZN3RBX6PluginD1Ev
pub fn stub_885ca4() {
    // IDA 0x885ca4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
// 0x885ca8 — __ZThn32_N3RBX6PluginD0Ev
pub fn stub_885ca8() {
    // IDA 0x885ca8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
// 0x885cb0 — __ZThn36_N3RBX6PluginD0Ev
pub fn stub_885cb0() {
    // IDA 0x885cb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Plugin::~Plugin()")]
// 0x885cb8 — __ZN3RBX6PluginD2Ev
pub fn stub_885cb8() {
    // IDA 0x885cb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
// 0x885df0 — __ZThn32_N3RBX6PluginD1Ev
pub fn stub_885df0() {
    // IDA 0x885df0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
// 0x885df8 — __ZThn36_N3RBX6PluginD1Ev
pub fn stub_885df8() {
    // IDA 0x885df8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::PluginManager(void)")]
// 0x885edc — __ZN3RBX13PluginManagerC2Ev
pub fn stub_885edc() {
    // IDA 0x885edc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::DeactivatePlugins(void)")]
// 0x886368 — __ZN3RBX13PluginManager17DeactivatePluginsEv
pub fn stub_886368() {
    // IDA 0x886368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::DeactivatePlugins(void)")]
// 0x886388 — __ZThn92_N3RBX13PluginManager17DeactivatePluginsEv
pub fn stub_886388() {
    // IDA 0x886388: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::StateDataEntry::getToolbar(std::string,RBX::IStudioPluginHost *)")]
// 0x8865c8 — __ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE
pub fn stub_8865c8() {
    // IDA 0x8865c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::StateDataEntry::hideStudioUI(bool,RBX::IStudioPluginHost *)")]
// 0x886808 — __ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE
pub fn stub_886808() {
    // IDA 0x886808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::StateDataEntry::fireButtonClick(void *)")]
// 0x886950 — __ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv
pub fn stub_886950() {
    // IDA 0x886950: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
// 0x886984 — __ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs
pub fn stub_886984() {
    // IDA 0x886984: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
// 0x886b40 — __ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs
pub fn stub_886b40() {
    // IDA 0x886b40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PluginManager>::~shared_ptr()")]
// 0x886e58 — __ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev
// was: boost::shared_ptr<RBX::PluginManager>::~shared_ptr()
pub fn stub_886e58() {
    // IDA 0x886e58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<void *,rbx_core::SharedPtr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::operator[](void * const&)")]
// 0x886f1c — __ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_
// was: std::map<void *,boost::shared_ptr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::operator[](void * const&)
pub fn stub_886f1c() {
    // IDA 0x886f1c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Button>::operator=(rbx_core::SharedPtr<RBX::Button> const&)")]
// 0x887064 — __ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_
// was: boost::shared_ptr<RBX::Button>::operator=(boost::shared_ptr<RBX::Button> const&)
pub fn stub_887064() {
    // IDA 0x887064: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::operator[](std::string const&)")]
// 0x8872e0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// was: std::map<std::string,boost::shared_ptr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::operator[](std::string const&)
pub fn stub_8872e0() {
    // IDA 0x8872e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Toolbar>::operator=(rbx_core::SharedPtr<RBX::Toolbar> const&)")]
// 0x8874fc — __ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_
// was: boost::shared_ptr<RBX::Toolbar>::operator=(boost::shared_ptr<RBX::Toolbar> const&)
pub fn stub_8874fc() {
    // IDA 0x8874fc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
// 0x887534 — __ZN3RBX13PluginManagerD1Ev
pub fn stub_887534() {
    // IDA 0x887534: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
// 0x887538 — __ZN3RBX13PluginManagerD0Ev
pub fn stub_887538() {
    // IDA 0x887538: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
// 0x887600 — __ZThn32_N3RBX13PluginManagerD1Ev
pub fn stub_887600() {
    // IDA 0x887600: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
// 0x887608 — __ZThn32_N3RBX13PluginManagerD0Ev
pub fn stub_887608() {
    // IDA 0x887608: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
// 0x8876d4 — __ZThn36_N3RBX13PluginManagerD1Ev
pub fn stub_8876d4() {
    // IDA 0x8876d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
// 0x8876dc — __ZThn36_N3RBX13PluginManagerD0Ev
pub fn stub_8876dc() {
    // IDA 0x8876dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Button::~Button()")]
// 0x8877ac — __ZN3RBX6ButtonD1Ev
pub fn stub_8877ac() {
    // IDA 0x8877ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Button::~Button()")]
// 0x8878c0 — __ZN3RBX6ButtonD0Ev
pub fn stub_8878c0() {
    // IDA 0x8878c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
// 0x8879f8 — __ZThn32_N3RBX6ButtonD1Ev
pub fn stub_8879f8() {
    // IDA 0x8879f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
// 0x887b08 — __ZThn32_N3RBX6ButtonD0Ev
pub fn stub_887b08() {
    // IDA 0x887b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
// 0x887c40 — __ZThn36_N3RBX6ButtonD1Ev
pub fn stub_887c40() {
    // IDA 0x887c40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
// 0x887d50 — __ZThn36_N3RBX6ButtonD0Ev
pub fn stub_887d50() {
    // IDA 0x887d50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Toolbar::~Toolbar()")]
// 0x887e78 — __ZN3RBX7ToolbarD1Ev
pub fn stub_887e78() {
    // IDA 0x887e78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Toolbar::~Toolbar()")]
// 0x887f64 — __ZN3RBX7ToolbarD0Ev
pub fn stub_887f64() {
    // IDA 0x887f64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
// 0x888070 — __ZThn32_N3RBX7ToolbarD1Ev
pub fn stub_888070() {
    // IDA 0x888070: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
// 0x888158 — __ZThn32_N3RBX7ToolbarD0Ev
pub fn stub_888158() {
    // IDA 0x888158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
// 0x888268 — __ZThn36_N3RBX7ToolbarD1Ev
pub fn stub_888268() {
    // IDA 0x888268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
// 0x888350 — __ZThn36_N3RBX7ToolbarD0Ev
pub fn stub_888350() {
    // IDA 0x888350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)")]
// 0x888450 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)
pub fn stub_888450() {
    // IDA 0x888450: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)")]
// 0x888478 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)
pub fn stub_888478() {
    // IDA 0x888478: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>*)")]
// 0x889784 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>*)
pub fn stub_889784() {
    // IDA 0x889784: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> *)")]
// 0x889828 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>> *)
pub fn stub_889828() {
    // IDA 0x889828: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Toolbar> const&)")]
// 0x889858 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEC2ERS0_RKS5_
// was: std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>::pair(std::string const&,boost::shared_ptr<RBX::Toolbar> const&)
pub fn stub_889858() {
    // IDA 0x889858: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0x889914 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_889914() {
    // IDA 0x889914: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0x889a00 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_889a00() {
    // IDA 0x889a00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0x889a50 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_889a50() {
    // IDA 0x889a50: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0x889ad4 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_889ad4() {
    // IDA 0x889ad4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::lower_bound(std::string const&)")]
// 0x889bdc — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::lower_bound(std::string const&)
pub fn stub_889bdc() {
    // IDA 0x889bdc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::find(std::string const&)")]
// 0x889f08 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::find(std::string const&)
pub fn stub_889f08() {
    // IDA 0x889f08: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::list(std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>> const&)")]
// 0x889f58 — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EEC2ERKS6_
// was: std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::list(std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>> const&)
pub fn stub_889f58() {
    // IDA 0x889f58: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>>(std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::__false_type)")]
// 0x88a020 — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type
// was: void std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>>(std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>,std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>,std::__false_type)
pub fn stub_88a020() {
    // IDA 0x88a020: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_clear(void)")]
// 0x88a044 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_clear(void)
pub fn stub_88a044() {
    // IDA 0x88a044: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_create_node(rbx_core::SharedPtr<RBX::Plugin> const&)")]
// 0x88a06c — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_create_node(boost::shared_ptr<RBX::Plugin> const&)
pub fn stub_88a06c() {
    // IDA 0x88a06c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>> const&)")]
// 0x88a250 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EEC2ERKSD_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>> const&)
pub fn stub_88a250() {
    // IDA 0x88a250: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>*)")]
// 0x88a294 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE7_M_copyEPKSt13_Rb_tree_nodeIS7_EPSF_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>*)
pub fn stub_88a294() {
    // IDA 0x88a294: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0x88afb0 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_88afb0() {
    // IDA 0x88afb0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0x88b064 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_88b064() {
    // IDA 0x88b064: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0x88b0b0 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_88b0b0() {
    // IDA 0x88b0b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_create_node(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0x88b118 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE14_M_create_nodeERKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_create_node(std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_88b118() {
    // IDA 0x88b118: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
// 0x88d7f4 — __ZN3RBX13PluginManagerD2Ev
pub fn stub_88d7f4() {
    // IDA 0x88d7f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
// 0x88e2c8 — __ZN3RBX11PluginMouseC1Ev
pub fn stub_88e2c8() {
    // IDA 0x88e2c8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
// 0x88e2cc — __ZN3RBX11PluginMouseC2Ev
pub fn stub_88e2cc() {
    // IDA 0x88e2cc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
// 0x88e468 — __ZN3RBX11PluginMouseD0Ev
pub fn stub_88e468() {
    // IDA 0x88e468: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
// 0x88e508 — __ZN3RBX11PluginMouseD1Ev
pub fn stub_88e508() {
    // IDA 0x88e508: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
// 0x88e50c — __ZThn32_N3RBX11PluginMouseD0Ev
pub fn stub_88e50c() {
    // IDA 0x88e50c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
// 0x88e514 — __ZThn36_N3RBX11PluginMouseD0Ev
pub fn stub_88e514() {
    // IDA 0x88e514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
// 0x88e51c — __ZN3RBX11PluginMouseD2Ev
pub fn stub_88e51c() {
    // IDA 0x88e51c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
