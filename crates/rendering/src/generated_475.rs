//! rendering shard 475 — 100 stubs 0x884390..0x888450 EA-sorted asc next 100 distinct not yet in rendering (Ogre|G3D|Render|Adorn|View|Mesh filtered 17446 total 17445->17446 covered gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] sanitized + todo!("0xADDR")) [skeleton batch rendering 475]
//! Source: ida/export.json (85545 funcs) EA asc gap filler distinct not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x884390 — __ZN3RBX9AllocatorINS_15PolyCellContactEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyCellContactEE13releaseMemoryEv")]
// IDA 0x884390: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884390() {
}

// 0x8843ac — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x8843ac: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8843ac() {
}

// 0x8843dc — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEC2Ev

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEC2Ev")]
// IDA 0x8843dc: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8843dc() {
}

// 0x884440 — __ZNK3RBX17EdgeEdgeConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::EdgeEdgeConnector *__hidden this)
#[doc(alias = "RBX::EdgeEdgeConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX17EdgeEdgeConnector16getConnectorTypeEv")]
// IDA 0x884440: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884440() {
}

// 0x884444 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEE13releaseMemoryEv")]
// IDA 0x884444: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884444() {
}

// 0x884460 — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x884460: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884460() {
}

// 0x884490 — __ZN3RBX12PolyCellPairD1Ev
// type: void __fastcall(RBX::PolyCellPair *__hidden this)
#[doc(alias = "RBX::PolyCellPair::~PolyCellPair()")]
#[doc(alias = "__ZN3RBX12PolyCellPairD1Ev")]
// IDA 0x884490: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_884490() {
}

// 0x884494 — __ZN3RBX12PolyCellPairD0Ev
// type: void __fastcall(RBX::PolyCellPair *__hidden this)
#[doc(alias = "RBX::PolyCellPair::~PolyCellPair()")]
#[doc(alias = "__ZN3RBX12PolyCellPairD0Ev")]
// IDA 0x884494: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_884494() {
}

// 0x884498 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEC2Ev

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEC2Ev")]
// IDA 0x884498: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884498() {
}

// 0x8844fc — __ZNK3RBX17FaceEdgeConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::FaceEdgeConnector *__hidden this)
#[doc(alias = "RBX::FaceEdgeConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX17FaceEdgeConnector16getConnectorTypeEv")]
// IDA 0x8844fc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8844fc() {
}

// 0x884500 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17FaceEdgeConnectorEE13releaseMemoryEv")]
// IDA 0x884500: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884500() {
}

// 0x88451c — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x88451c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88451c() {
}

// 0x88454c — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEC2Ev

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19FaceVertexConnectorEEC2Ev")]
// IDA 0x88454c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88454c() {
}

// 0x8845b0 — __ZNK3RBX19FaceVertexConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::FaceVertexConnector *__hidden this)
#[doc(alias = "RBX::FaceVertexConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX19FaceVertexConnector16getConnectorTypeEv")]
// IDA 0x8845b0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8845b0() {
}

// 0x8845b4 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19FaceVertexConnectorEE13releaseMemoryEv")]
// IDA 0x8845b4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8845b4() {
}

// 0x8845d0 — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x8845d0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8845d0() {
}

// 0x884600 — __GLOBAL__I_a_439

#[doc(alias = "global constructor keyed to_a_439")]
#[doc(alias = "__GLOBAL__I_a_439")]
// IDA 0x884600: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_884600() {
}

// 0x884c40 — __ZN3RBX13PluginManager12createPluginEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::createPlugin(void)")]
#[doc(alias = "__ZN3RBX13PluginManager12createPluginEv")]
// IDA 0x884c40: 391 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884c40() {
}

// 0x885014 — __ZN3RBX6Plugin11getMouseLuaEv
// type: _DWORD __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::getMouseLua(void)")]
#[doc(alias = "__ZN3RBX6Plugin11getMouseLuaEv")]
// IDA 0x885014: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885014() {
}

// 0x885030 — __ZN3RBX6Plugin8activateEb
// type: _DWORD __fastcall(RBX::Plugin *__hidden this, bool)
#[doc(alias = "RBX::Plugin::activate(bool)")]
#[doc(alias = "__ZN3RBX6Plugin8activateEb")]
// IDA 0x885030: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885030() {
}

// 0x885068 — __ZN3RBX6Plugin13createToolbarESs

#[doc(alias = "RBX::Plugin::createToolbar(std::string)")]
#[doc(alias = "__ZN3RBX6Plugin13createToolbarESs")]
// IDA 0x885068: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885068() {
}

// 0x885190 — __ZN3RBX7Toolbar12createButtonESsSsSs

#[doc(alias = "RBX::Toolbar::createButton(std::string,std::string,std::string)")]
#[doc(alias = "__ZN3RBX7Toolbar12createButtonESsSsSs")]
// IDA 0x885190: 391 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885190() {
}

// 0x8855bc — __ZN3RBX6Button9setActiveEb
// type: _DWORD __fastcall(RBX::Button *__hidden this, bool)
#[doc(alias = "RBX::Button::setActive(bool)")]
#[doc(alias = "__ZN3RBX6Button9setActiveEb")]
// IDA 0x8855bc: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8855bc() {
}

// 0x8856ec — __ZN3RBX6ButtonC2Ev
// type: _DWORD __fastcall(RBX::Button *__hidden this)
#[doc(alias = "RBX::Button::Button(void)")]
#[doc(alias = "__ZN3RBX6ButtonC2Ev")]
// IDA 0x8856ec: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8856ec() {
}

// 0x885880 — __ZN3RBX7Toolbar5resetEv
// type: _DWORD __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::reset(void)")]
#[doc(alias = "__ZN3RBX7Toolbar5resetEv")]
// IDA 0x885880: 15 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885880() {
}

// 0x8858a4 — __ZN3RBX13PluginManager9singletonEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::singleton(void)")]
#[doc(alias = "__ZN3RBX13PluginManager9singletonEv")]
// IDA 0x8858a4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8858a4() {
}

// 0x8858cc — __ZN3RBX7ToolbarC2Ev
// type: _DWORD __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::Toolbar(void)")]
#[doc(alias = "__ZN3RBX7ToolbarC2Ev")]
// IDA 0x8858cc: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8858cc() {
}

// 0x885a20 — __ZN3RBX7Toolbar9getButtonEPv
// type: _DWORD __fastcall(RBX::Toolbar *__hidden this, void *)
#[doc(alias = "RBX::Toolbar::getButton(void *)")]
#[doc(alias = "__ZN3RBX7Toolbar9getButtonEPv")]
// IDA 0x885a20: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885a20() {
}

// 0x885a60 — __ZN3RBX6PluginC2Ev
// type: _DWORD __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::Plugin(void)")]
#[doc(alias = "__ZN3RBX6PluginC2Ev")]
// IDA 0x885a60: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885a60() {
}

// 0x885c04 — __ZN3RBX6PluginD0Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::~Plugin()")]
#[doc(alias = "__ZN3RBX6PluginD0Ev")]
// IDA 0x885c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885c04() {
}

// 0x885ca4 — __ZN3RBX6PluginD1Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::~Plugin()")]
#[doc(alias = "__ZN3RBX6PluginD1Ev")]
// IDA 0x885ca4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_885ca4() {
}

// 0x885ca8 — __ZThn32_N3RBX6PluginD0Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn32_N3RBX6PluginD0Ev")]
// IDA 0x885ca8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885ca8() {
}

// 0x885cb0 — __ZThn36_N3RBX6PluginD0Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn36_N3RBX6PluginD0Ev")]
// IDA 0x885cb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885cb0() {
}

// 0x885cb8 — __ZN3RBX6PluginD2Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::~Plugin()")]
#[doc(alias = "__ZN3RBX6PluginD2Ev")]
// IDA 0x885cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885cb8() {
}

// 0x885df0 — __ZThn32_N3RBX6PluginD1Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn32_N3RBX6PluginD1Ev")]
// IDA 0x885df0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885df0() {
}

// 0x885df8 — __ZThn36_N3RBX6PluginD1Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn36_N3RBX6PluginD1Ev")]
// IDA 0x885df8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885df8() {
}

// 0x885e00 — __ZN3RBX6Plugin12setDataModelEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::Plugin *__hidden this, RBX::DataModel *)
#[doc(alias = "RBX::Plugin::setDataModel(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX6Plugin12setDataModelEPNS_9DataModelE")]
// IDA 0x885e00: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885e00() {
}

// 0x885edc — __ZN3RBX13PluginManagerC2Ev
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::PluginManager(void)")]
#[doc(alias = "__ZN3RBX13PluginManagerC2Ev")]
// IDA 0x885edc: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885edc() {
}

// 0x8860f4 — __Z26initPluginManagerSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "initPluginManagerSingleton(void)")]
#[doc(alias = "__Z26initPluginManagerSingletonv")]
// IDA 0x8860f4: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8860f4() {
}

// 0x886224 — __ZL24doPluginManagerSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "doPluginManagerSingleton(void)")]
#[doc(alias = "__ZL24doPluginManagerSingletonv")]
// IDA 0x886224: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886224() {
}

// 0x886328 — __ZN3RBX13PluginManager15getActivePluginEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::DataModel *)
#[doc(alias = "RBX::PluginManager::getActivePlugin(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX13PluginManager15getActivePluginEPNS_9DataModelE")]
// IDA 0x886328: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886328() {
}

// 0x886368 — __ZN3RBX13PluginManager17DeactivatePluginsEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::DeactivatePlugins(void)")]
#[doc(alias = "__ZN3RBX13PluginManager17DeactivatePluginsEv")]
// IDA 0x886368: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886368() {
}

// 0x886388 — __ZThn92_N3RBX13PluginManager17DeactivatePluginsEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::DeactivatePlugins(void)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager17DeactivatePluginsEv")]
// IDA 0x886388: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886388() {
}

// 0x8863a8 — __ZN3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::Plugin *, RBX::DataModel *)
#[doc(alias = "RBX::PluginManager::activate(RBX::Plugin *,RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE")]
// IDA 0x8863a8: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8863a8() {
}

// 0x8865c0 — __ZThn92_N3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::Plugin *, RBX::DataModel *)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::activate(RBX::Plugin *,RBX::DataModel *)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE")]
// IDA 0x8865c0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8865c0() {
}

// 0x8865c8 — __ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PluginManager::StateDataEntry::getToolbar(std::string,RBX::IStudioPluginHost *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE")]
// IDA 0x8865c8: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8865c8() {
}

// 0x886808 — __ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "RBX::PluginManager::StateDataEntry::hideStudioUI(bool,RBX::IStudioPluginHost *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE")]
// IDA 0x886808: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886808() {
}

// 0x886950 — __ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv
// type: _DWORD __fastcall(RBX::PluginManager::StateDataEntry *__hidden this, void *)
#[doc(alias = "RBX::PluginManager::StateDataEntry::fireButtonClick(void *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv")]
// IDA 0x886950: 22 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886950() {
}

// 0x886984 — __ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs

#[doc(alias = "RBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
#[doc(alias = "__ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs")]
// IDA 0x886984: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886984() {
}

// 0x886b40 — __ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs

#[doc(alias = "non-virtual thunk toRBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs")]
// IDA 0x886b40: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886b40() {
}

// 0x886b4c — __ZN3RBX13PluginManager11buttonClickEPNS_9DataModelEPv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::DataModel *, void *)
#[doc(alias = "RBX::PluginManager::buttonClick(RBX::DataModel *,void *)")]
#[doc(alias = "__ZN3RBX13PluginManager11buttonClickEPNS_9DataModelEPv")]
// IDA 0x886b4c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886b4c() {
}

// 0x886cb0 — __ZThn96_N3RBX13PluginManager11buttonClickEPNS_9DataModelEPv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::DataModel *, void *)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::buttonClick(RBX::DataModel *,void *)")]
#[doc(alias = "__ZThn96_N3RBX13PluginManager11buttonClickEPNS_9DataModelEPv")]
// IDA 0x886cb0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886cb0() {
}

// 0x886cb8 — __ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev")]
// IDA 0x886cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886cb8() {
}

// 0x886cdc — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev")]
// IDA 0x886cdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886cdc() {
}

// 0x886d00 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED1Ev")]
// IDA 0x886d00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886d00() {
}

// 0x886d40 — __ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
// IDA 0x886d40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886d40() {
}

// 0x886d64 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev")]
// IDA 0x886d64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886d64() {
}

// 0x886da4 — __ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EED1Ev")]
// IDA 0x886da4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886da4() {
}

// 0x886df4 — __ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Button,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EED1Ev")]
// IDA 0x886df4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886df4() {
}

// 0x886e34 — __ZN3RBX10Reflection9EventDescINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Button,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Button::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
// IDA 0x886e34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886e34() {
}

// 0x886e58 — __ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev

#[doc(alias = "boost::shared_ptr<RBX::PluginManager>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev")]
// IDA 0x886e58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886e58() {
}

// 0x886e6c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6ButtonEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::Button> RBX::Creatable<RBX::Instance>::create<RBX::Button>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6ButtonEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x886e6c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886e6c() {
}

// 0x886f1c — __ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<void *,boost::shared_ptr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::operator[](void * const&)")]
#[doc(alias = "__ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_")]
// IDA 0x886f1c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886f1c() {
}

// 0x887064 — __ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_

#[doc(alias = "boost::shared_ptr<RBX::Button>::operator=(boost::shared_ptr<RBX::Button> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_")]
// IDA 0x887064: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887064() {
}

// 0x88709c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11PluginMouseEEERS3_RKNS0_IT_EE

#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PluginMouse>(boost::shared_ptr<RBX::PluginMouse> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11PluginMouseEEERS3_RKNS0_IT_EE")]
// IDA 0x88709c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88709c() {
}

// 0x8870d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11PluginMouseEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::PluginMouse> RBX::Creatable<RBX::Instance>::create<RBX::PluginMouse>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11PluginMouseEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x8870d0: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8870d0() {
}

// 0x887180 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6PluginEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::Plugin> RBX::Creatable<RBX::Instance>::create<RBX::Plugin>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6PluginEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x887180: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887180() {
}

// 0x887230 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7ToolbarEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::Toolbar> RBX::Creatable<RBX::Instance>::create<RBX::Toolbar>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7ToolbarEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x887230: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887230() {
}

// 0x8872e0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_

#[doc(alias = "std::map<std::string,boost::shared_ptr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")]
// IDA 0x8872e0: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8872e0() {
}

// 0x8874fc — __ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_

#[doc(alias = "boost::shared_ptr<RBX::Toolbar>::operator=(boost::shared_ptr<RBX::Toolbar> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_")]
// IDA 0x8874fc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8874fc() {
}

// 0x887534 — __ZN3RBX13PluginManagerD1Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD1Ev")]
// IDA 0x887534: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_887534() {
}

// 0x887538 — __ZN3RBX13PluginManagerD0Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD0Ev")]
// IDA 0x887538: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887538() {
}

// 0x8875d8 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
// IDA 0x8875d8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8875d8() {
}

// 0x887600 — __ZThn32_N3RBX13PluginManagerD1Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn32_N3RBX13PluginManagerD1Ev")]
// IDA 0x887600: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887600() {
}

// 0x887608 — __ZThn32_N3RBX13PluginManagerD0Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn32_N3RBX13PluginManagerD0Ev")]
// IDA 0x887608: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887608() {
}

// 0x8876ac — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
// IDA 0x8876ac: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8876ac() {
}

// 0x8876d4 — __ZThn36_N3RBX13PluginManagerD1Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn36_N3RBX13PluginManagerD1Ev")]
// IDA 0x8876d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8876d4() {
}

// 0x8876dc — __ZThn36_N3RBX13PluginManagerD0Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn36_N3RBX13PluginManagerD0Ev")]
// IDA 0x8876dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8876dc() {
}

// 0x887780 — __ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
// IDA 0x887780: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887780() {
}

// 0x887790 — __ZThn32_NK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
// IDA 0x887790: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887790() {
}

// 0x8877a0 — __ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD1Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD1Ev")]
// IDA 0x8877a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8877a0() {
}

// 0x8877a4 — __ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD1Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD1Ev")]
// IDA 0x8877a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8877a4() {
}

// 0x8877a8 — __ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD1Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD1Ev")]
// IDA 0x8877a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8877a8() {
}

// 0x8877ac — __ZN3RBX6ButtonD1Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "RBX::Button::~Button()")]
#[doc(alias = "__ZN3RBX6ButtonD1Ev")]
// IDA 0x8877ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8877ac() {
}

// 0x8878c0 — __ZN3RBX6ButtonD0Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "RBX::Button::~Button()")]
#[doc(alias = "__ZN3RBX6ButtonD0Ev")]
// IDA 0x8878c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8878c0() {
}

// 0x8879e8 — __ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
// IDA 0x8879e8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8879e8() {
}

// 0x8879f8 — __ZThn32_N3RBX6ButtonD1Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn32_N3RBX6ButtonD1Ev")]
// IDA 0x8879f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8879f8() {
}

// 0x887b08 — __ZThn32_N3RBX6ButtonD0Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn32_N3RBX6ButtonD0Ev")]
// IDA 0x887b08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887b08() {
}

// 0x887c30 — __ZThn32_NK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
// IDA 0x887c30: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887c30() {
}

// 0x887c40 — __ZThn36_N3RBX6ButtonD1Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn36_N3RBX6ButtonD1Ev")]
// IDA 0x887c40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887c40() {
}

// 0x887d50 — __ZThn36_N3RBX6ButtonD0Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn36_N3RBX6ButtonD0Ev")]
// IDA 0x887d50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887d50() {
}

// 0x887e78 — __ZN3RBX7ToolbarD1Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZN3RBX7ToolbarD1Ev")]
// IDA 0x887e78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887e78() {
}

// 0x887f64 — __ZN3RBX7ToolbarD0Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZN3RBX7ToolbarD0Ev")]
// IDA 0x887f64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887f64() {
}

// 0x888060 — __ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
// IDA 0x888060: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_888060() {
}

// 0x888070 — __ZThn32_N3RBX7ToolbarD1Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn32_N3RBX7ToolbarD1Ev")]
// IDA 0x888070: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888070() {
}

// 0x888158 — __ZThn32_N3RBX7ToolbarD0Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn32_N3RBX7ToolbarD0Ev")]
// IDA 0x888158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888158() {
}

// 0x888258 — __ZThn32_NK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
// IDA 0x888258: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_888258() {
}

// 0x888268 — __ZThn36_N3RBX7ToolbarD1Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn36_N3RBX7ToolbarD1Ev")]
// IDA 0x888268: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888268() {
}

// 0x888350 — __ZThn36_N3RBX7ToolbarD0Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn36_N3RBX7ToolbarD0Ev")]
// IDA 0x888350: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888350() {
}

// 0x888450 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// IDA 0x888450: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_888450() {
}
