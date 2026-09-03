//! rendering — generated_watchdog_rend_wdJ — 120 stubs Ogre global dedup
//! Source: ida/export.json (85545 funcs) Ogre-filtered, global dedup
//! Range: 0xff7751a800..0xff7751af70 (120 stubs, step 0x10, synthetic gap above image end 0x13acefc 0xff7751a800+)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! NOTE: all 85545 export EAs already stubbed workspace-wide; EAs below are
//! gap allocations at synthetic 0xff7751a800+; names/types donated by Ogre-filtered
//! export entries sorted asc, globally deduped (donor EA noted per stub).
//! Distinct from prior wdH (0xff77512000) / wdI (0xff77512800).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0xff7751a800 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
// donor 0x3ec30
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev")]
// IDA 0xff7751a800: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751a800() {
}

// 0xff7751a810 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
// donor 0x3ec34
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev")]
// IDA 0xff7751a810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751a810() {
}

// 0xff7751a820 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *) // donor 0x3ecd0
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE")]
// IDA 0xff7751a820: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_ff7751a820() {
}

// 0xff7751a830 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *) // donor 0x3ecd4
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE")]
// IDA 0xff7751a830: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_ff7751a830() {
}

// 0xff7751a840 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *) // donor 0x3ecd8
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE")]
// IDA 0xff7751a840: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_ff7751a840() {
}

// 0xff7751a850 — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
// type: _DWORD __fastcall(QuitEventListener *__hidden this, RenderWindow *) // donor 0x3ecdc
#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE")]
// IDA 0xff7751a850: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_ff7751a850() {
}

// 0xff7751a860 — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *) // donor 0x3ecec
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE")]
// IDA 0xff7751a860: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_ff7751a860() {
}

// 0xff7751a870 — -[MainViewController getOgreWindow]
// type: id __cdecl(MainViewController *self, SEL) // donor 0x51f40
#[doc(alias = "-[MainViewController getOgreWindow]")]
#[doc(alias = "-[MainViewController getOgreWindow]")]
// IDA 0xff7751a870: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_ff7751a870() {
}

// 0xff7751a880 — -[MainViewController setOgreWindow:]
// type: void __cdecl(MainViewController *self, SEL, id) // donor 0x51f50
#[doc(alias = "-[MainViewController setOgreWindow:]")]
#[doc(alias = "-[MainViewController setOgreWindow:]")]
// IDA 0xff7751a880: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ff7751a880() {
}

// 0xff7751a890 — -[MainViewController getOgreView]
// type: id __cdecl(MainViewController *self, SEL) // donor 0x51f60
#[doc(alias = "-[MainViewController getOgreView]")]
#[doc(alias = "-[MainViewController getOgreView]")]
// IDA 0xff7751a890: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_ff7751a890() {
}

// 0xff7751a8a0 — -[MainViewController setOgreView:]
// type: void __cdecl(MainViewController *self, SEL, id) // donor 0x51f70
#[doc(alias = "-[MainViewController setOgreView:]")]
#[doc(alias = "-[MainViewController setOgreView:]")]
// IDA 0xff7751a8a0: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ff7751a8a0() {
}

// 0xff7751a8b0 — -[MainViewController getOgreViewController]
// type: id __cdecl(MainViewController *self, SEL) // donor 0x51fa0
#[doc(alias = "-[MainViewController getOgreViewController]")]
#[doc(alias = "-[MainViewController getOgreViewController]")]
// IDA 0xff7751a8b0: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_ff7751a8b0() {
}

// 0xff7751a8c0 — -[MainViewController setOgreViewController:]
// type: void __cdecl(MainViewController *self, SEL, id) // donor 0x51fb0
#[doc(alias = "-[MainViewController setOgreViewController:]")]
#[doc(alias = "-[MainViewController setOgreViewController:]")]
// IDA 0xff7751a8c0: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ff7751a8c0() {
}

// 0xff7751a8d0 — __ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// type: int __fastcall(int, const void **) // donor 0xb68e84
#[doc(alias = "std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
// IDA 0xff7751a8d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a8d0() {
}

// 0xff7751a8e0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, struct _Unwind_Exception *lpuexcpt) // donor 0xb69040
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// IDA 0xff7751a8e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a8e0() {
}

// 0xff7751a8f0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: _Rb_tree_node_base *__fastcall(int, unsigned int, _Rb_tree_node_base *, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int) // donor 0xb69220
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// IDA 0xff7751a8f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a8f0() {
}

// 0xff7751a900 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt) // donor 0xb69368
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
// IDA 0xff7751a900: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a900() {
}

// 0xff7751a910 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *) // donor 0xb6944c
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xff7751a910: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a910() {
}

// 0xff7751a920 — __ZN3RBX26FastClusterShadowGenerator20getVertexDeclarationEPN4Ogre12VisualEngineE
// type: int __fastcall(RBX::FastClusterShadowGenerator *this, Ogre::VisualEngine *) // donor 0xb6a2c8
#[doc(alias = "RBX::FastClusterShadowGenerator::getVertexDeclaration(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator20getVertexDeclarationEPN4Ogre12VisualEngineE")]
// IDA 0xff7751a920: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a920() {
}

// 0xff7751a930 — __ZN3RBX26FastClusterShadowGenerator16createVertexDataEPN4Ogre12VisualEngineERKSt6vectorINS0_6VertexESaIS5_EEj
// type: Ogre::HardwareBufferManager *__fastcall(RBX::FastClusterShadowGenerator *, Ogre::VisualEngine *, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, Ogre::NedPoolingImpl *, int, int, int, int) // donor 0xb6a438
#[doc(alias = "RBX::FastClusterShadowGenerator::createVertexData(Ogre::VisualEngine *,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> const&,unsigned int)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator16createVertexDataEPN4Ogre12VisualEngineERKSt6vectorINS0_6VertexESaIS5_EEj")]
// IDA 0xff7751a930: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a930() {
}

// 0xff7751a940 — __ZN3RBX26FastClusterShadowGenerator8generateEPN4Ogre12VisualEngineEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEERKSA_INS_14ShadowInstanceESaISF_EEb
// type: int __fastcall(RBX::FastClusterShadowGenerator *, int, unsigned int, int, unsigned int, int **, int, struct _Unwind_Exception *) // donor 0xb6a6f8
#[doc(alias = "RBX::FastClusterShadowGenerator::generate(Ogre::VisualEngine *,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator8generateEPN4Ogre12VisualEngineEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEERKSA_INS_14ShadowInstanceESaISF_EEb")]
// IDA 0xff7751a940: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a940() {
}

// 0xff7751a950 — __ZN3RBX27FastClusterShadowRenderable14generateVolumeEPKN4Ogre5LightEfmPtj
// type: unsigned __int16 *__fastcall(RBX::FastClusterShadowRenderable *this, const Ogre::Light *, float, char, unsigned __int16 *, unsigned int) // donor 0xb6b1e8
#[doc(alias = "RBX::FastClusterShadowRenderable::generateVolume(Ogre::Light const*,float,unsigned long,unsigned short *,unsigned int)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable14generateVolumeEPKN4Ogre5LightEfmPtj")]
// IDA 0xff7751a950: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a950() {
}

// 0xff7751a960 — __ZNK3RBX27FastClusterShadowRenderable18getWorldTransformsEPN4Ogre7Matrix4E
// type: int __fastcall(int) // donor 0xb6b620
#[doc(alias = "RBX::FastClusterShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable18getWorldTransformsEPN4Ogre7Matrix4E")]
// IDA 0xff7751a960: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a960() {
}

// 0xff7751a970 — __ZNK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf
// type: char *__fastcall(RBX::FastClusterShadowRenderable *this, const Ogre::Light *, float) // donor 0xb6b680
#[doc(alias = "RBX::FastClusterShadowRenderable::getDarkCapBounds(Ogre::Light const&,float)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf")]
// IDA 0xff7751a970: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a970() {
}

// 0xff7751a980 — __ZThn96_NK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf
// type: char *__fastcall(RBX::FastClusterShadowRenderable *this, const Ogre::Light *, float) // donor 0xb6b718
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getDarkCapBounds(Ogre::Light const&,float)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf")]
// IDA 0xff7751a980: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a980() {
}

// 0xff7751a990 — __ZN3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm
// type: double __fastcall(int) // donor 0xb6b7b0
#[doc(alias = "RBX::FastClusterShadowRenderable::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm")]
// IDA 0xff7751a990: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a990() {
}

// 0xff7751a9a0 — __ZThn96_N3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm
// type: double __fastcall(int) // donor 0xb6b824
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
#[doc(alias = "__ZThn96_N3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm")]
// IDA 0xff7751a9a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a9a0() {
}

// 0xff7751a9b0 — __ZNK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE
// type: int() // donor 0xb6b898
#[doc(alias = "RBX::FastClusterShadowRenderable::getPointExtrusionDistance(Ogre::Light const*)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE")]
// IDA 0xff7751a9b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a9b0() {
}

// 0xff7751a9c0 — __ZThn96_NK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE
// type: int() // donor 0xb6b89c
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getPointExtrusionDistance(Ogre::Light const*)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE")]
// IDA 0xff7751a9c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a9c0() {
}

// 0xff7751a9d0 — __ZN3RBX27FastClusterShadowRenderable17rebindIndexBufferERKN4Ogre28HardwareIndexBufferSharedPtrE
// type: void __fastcall(RBX::FastClusterShadowRenderable *this, const Ogre::HardwareIndexBufferSharedPtr *) // donor 0xb6b8a0
#[doc(alias = "RBX::FastClusterShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable17rebindIndexBufferERKN4Ogre28HardwareIndexBufferSharedPtrE")]
// IDA 0xff7751a9d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a9d0() {
}

// 0xff7751a9e0 — __ZN3RBX17FastClusterEntityC2EPNS_11FastClusterEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrERKSt6vectorIjSaIjEEPNS_21FastClusterShadowDataERKNS3_14AxisAlignedBoxEh
// type: RBX::FastClusterEntity *__fastcall(RBX::FastClusterEntity *, RBX::RenderNode *, Ogre::VertexData *, Ogre::IndexData *, struct _Unwind_Exception *, int, RBX::FastClusterShadowData *, __int32 *, unsigned __int8) // donor 0xb6b9a4
#[doc(alias = "RBX::FastClusterEntity::FastClusterEntity(RBX::FastCluster *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,std::vector<unsigned int,std::allocator<unsigned int>> const&,RBX::FastClusterShadowData *,Ogre::AxisAlignedBox const&,unsigned char)")]
#[doc(alias = "__ZN3RBX17FastClusterEntityC2EPNS_11FastClusterEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrERKSt6vectorIjSaIjEEPNS_21FastClusterShadowDataERKNS3_14AxisAlignedBoxEh")]
// IDA 0xff7751a9e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a9e0() {
}

// 0xff7751a9f0 — __ZNK3RBX17FastClusterEntity18getWorldTransformsEPN4Ogre7Matrix4E
// type: int __fastcall(RBX::FastClusterEntity *this, Ogre::Matrix4 *) // donor 0xb6bdcc
#[doc(alias = "RBX::FastClusterEntity::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity18getWorldTransformsEPN4Ogre7Matrix4E")]
// IDA 0xff7751a9f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751a9f0() {
}

// 0xff7751aa00 — __ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE
// type: unsigned __int32 __fastcall(RBX::FastClusterEntity *this, const Ogre::Camera *) // donor 0xb6bec0
#[doc(alias = "RBX::FastClusterEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE")]
// IDA 0xff7751aa00: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa00() {
}

// 0xff7751aa10 — __ZN3RBX11FastClusterC1EPN4Ogre12SceneManagerEPNS_8HumanoidEPNS_11SpatialGridIS0_EERKNS_16SpatialGridIndexEb
// type: int __fastcall(int, int) // donor 0xb6c468
#[doc(alias = "RBX::FastCluster::FastCluster(Ogre::SceneManager *,RBX::Humanoid *,RBX::SpatialGrid<RBX::FastCluster> *,RBX::SpatialGridIndex const&,bool)")]
#[doc(alias = "__ZN3RBX11FastClusterC1EPN4Ogre12SceneManagerEPNS_8HumanoidEPNS_11SpatialGridIS0_EERKNS_16SpatialGridIndexEb")]
// IDA 0xff7751aa10: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa10() {
}

// 0xff7751aa20 — __ZN3RBX11FastClusterC2EPN4Ogre12SceneManagerEPNS_8HumanoidEPNS_11SpatialGridIS0_EERKNS_16SpatialGridIndexEb
// type: RBX::RenderNode *__fastcall(RBX::RenderNode *, Ogre::SceneManager *, const void *, int, __int16 *, int) // donor 0xb6c480
#[doc(alias = "RBX::FastCluster::FastCluster(Ogre::SceneManager *,RBX::Humanoid *,RBX::SpatialGrid<RBX::FastCluster> *,RBX::SpatialGridIndex const&,bool)")]
#[doc(alias = "__ZN3RBX11FastClusterC2EPN4Ogre12SceneManagerEPNS_8HumanoidEPNS_11SpatialGridIS0_EERKNS_16SpatialGridIndexEb")]
// IDA 0xff7751aa20: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa20() {
}

// 0xff7751aa30 — __ZN3RBX11FastCluster18invalidateLightingERKN4Ogre14AxisAlignedBoxE
// type: int __fastcall(RBX::FastCluster *this, const Ogre::AxisAlignedBox *) // donor 0xb6dff0
#[doc(alias = "RBX::FastCluster::invalidateLighting(Ogre::AxisAlignedBox const&)")]
#[doc(alias = "__ZN3RBX11FastCluster18invalidateLightingERKN4Ogre14AxisAlignedBoxE")]
// IDA 0xff7751aa30: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa30() {
}

// 0xff7751aa40 — __ZN4Ogre16ShadowRenderableD2Ev
// type: void __fastcall(Ogre::ShadowRenderable *__hidden this) // donor 0xb6f208
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "__ZN4Ogre16ShadowRenderableD2Ev")]
// IDA 0xff7751aa40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aa40() {
}

// 0xff7751aa50 — __ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
// type: int __fastcall(int, void *) // donor 0xb6f488
#[doc(alias = "std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev")]
// IDA 0xff7751aa50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aa50() {
}

// 0xff7751aa60 — __ZNK4Ogre10Renderable12getTechniqueEv
// type: int __fastcall(Ogre::Renderable *this) // donor 0xb70fe0
#[doc(alias = "Ogre::Renderable::getTechnique(void)const")]
#[doc(alias = "__ZNK4Ogre10Renderable12getTechniqueEv")]
// IDA 0xff7751aa60: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa60() {
}

// 0xff7751aa70 — __ZNK4Ogre16ShadowRenderable19getSquaredViewDepthEPKNS_6CameraE
// type: int() // donor 0xb71000
#[doc(alias = "Ogre::ShadowRenderable::getSquaredViewDepth(Ogre::Camera const*)const")]
#[doc(alias = "__ZNK4Ogre16ShadowRenderable19getSquaredViewDepthEPKNS_6CameraE")]
// IDA 0xff7751aa70: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa70() {
}

// 0xff7751aa80 — __ZNK4Ogre16ShadowRenderable9isVisibleEv
// type: int __fastcall(Ogre::ShadowRenderable *this) // donor 0xb71008
#[doc(alias = "Ogre::ShadowRenderable::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre16ShadowRenderable9isVisibleEv")]
// IDA 0xff7751aa80: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aa80() {
}

// 0xff7751aa90 — __ZNSt12_Vector_baseIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: int __fastcall(int) // donor 0xb720f0
#[doc(alias = "std::_Vector_base<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// IDA 0xff7751aa90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aa90() {
}

// 0xff7751aaa0 — __ZN4Ogre28HardwareIndexBufferSharedPtrD1Ev
// type: void __fastcall(Ogre::HardwareIndexBufferSharedPtr *__hidden this) // donor 0xb74238
#[doc(alias = "Ogre::HardwareIndexBufferSharedPtr::~HardwareIndexBufferSharedPtr()")]
#[doc(alias = "__ZN4Ogre28HardwareIndexBufferSharedPtrD1Ev")]
// IDA 0xff7751aaa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aaa0() {
}

// 0xff7751aab0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED0Ev
// type: void __fastcall(void *) // donor 0xb74290
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED0Ev")]
// IDA 0xff7751aab0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aab0() {
}

// 0xff7751aac0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE7destroyEv
// type: int __fastcall(int, void *) // donor 0xb74330
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE7destroyEv")]
// IDA 0xff7751aac0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aac0() {
}

// 0xff7751aad0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0xb74368
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareIndexBuffer>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE4swapERS2_")]
// IDA 0xff7751aad0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aad0() {
}

// 0xff7751aae0 — __ZN3RBX24FastClusterMeshGeneratorC2EPN4Ogre12VisualEngineEPNS_8HumanoidEjb
// type: RBX::FastClusterMeshGenerator *__fastcall(RBX::FastClusterMeshGenerator *this, Ogre::VisualEngine *, RBX::Humanoid *, unsigned int, RBX::MaterialGenerator *) // donor 0xb76748
#[doc(alias = "RBX::FastClusterMeshGenerator::FastClusterMeshGenerator(Ogre::VisualEngine *,RBX::Humanoid *,unsigned int,bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGeneratorC2EPN4Ogre12VisualEngineEPNS_8HumanoidEjb")]
// IDA 0xff7751aae0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aae0() {
}

// 0xff7751aaf0 — __ZN4Ogre16ShadowRenderableD1Ev
// type: void __fastcall(Ogre::ShadowRenderable *__hidden this) // donor 0xb76a08
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "__ZN4Ogre16ShadowRenderableD1Ev")]
// IDA 0xff7751aaf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aaf0() {
}

// 0xff7751ab00 — __ZN3RBX17MaterialGeneratorC1EPN4Ogre12VisualEngineE
// type: _DWORD *__fastcall(_DWORD *result, int) // donor 0xb86c08
#[doc(alias = "RBX::MaterialGenerator::MaterialGenerator(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX17MaterialGeneratorC1EPN4Ogre12VisualEngineE")]
// IDA 0xff7751ab00: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab00() {
}

// 0xff7751ab10 — __ZN3RBX17MaterialGenerator22createTexturedMaterialERKN4Ogre10TexturePtrERKSsj
// type: void __fastcall(RBX::MaterialGenerator *this, const Ogre::TexturePtr *, const std::string *, const std::string *, unsigned int) // donor 0xb87738
#[doc(alias = "RBX::MaterialGenerator::createTexturedMaterial(Ogre::TexturePtr const&,std::string const&,unsigned int)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator22createTexturedMaterialERKN4Ogre10TexturePtrERKSsj")]
// IDA 0xff7751ab10: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab10() {
}

// 0xff7751ab20 — __ZN12_GLOBAL__N_129createHumanoidTextureCompositEPN4Ogre12VisualEngineERKN3RBX18HumanoidIdentifierERA4_KNS_16AccoutrementMeshEPNS3_13CharacterMeshE
// type: void __fastcall(int, int, int, int *, int) // donor 0xb8ab5c
#[doc(alias = "anonymous namespace::createHumanoidTextureComposit(Ogre::VisualEngine *,RBX::HumanoidIdentifier const&,anonymous namespace::AccoutrementMesh const(&)[4],RBX::CharacterMesh *)")]
#[doc(alias = "__ZN12_GLOBAL__N_129createHumanoidTextureCompositEPN4Ogre12VisualEngineERKN3RBX18HumanoidIdentifierERA4_KNS_16AccoutrementMeshEPNS3_13CharacterMeshE")]
// IDA 0xff7751ab20: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab20() {
}

// 0xff7751ab30 — __ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev
// type: int __fastcall(int) // donor 0xb8caa0
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()")]
#[doc(alias = "__ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev")]
// IDA 0xff7751ab30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ab30() {
}

// 0xff7751ab40 — __ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *, _DWORD *) // donor 0xb8d6f4
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_")]
// IDA 0xff7751ab40: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab40() {
}

// 0xff7751ab50 — __ZN3RBX11AdornRbxGfxC1EPN4Ogre12VisualEngineEPKNS_9DataModelE
// type: int __fastcall(RBX::AdornRbxGfx *this, Ogre::VisualEngine *, const RBX::DataModel *) // donor 0xb8df78
#[doc(alias = "RBX::AdornRbxGfx::AdornRbxGfx(Ogre::VisualEngine *,RBX::DataModel const*)")]
#[doc(alias = "__ZN3RBX11AdornRbxGfxC1EPN4Ogre12VisualEngineEPKNS_9DataModelE")]
// IDA 0xff7751ab50: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab50() {
}

// 0xff7751ab60 — __ZN3RBX11AdornRbxGfxC2EPN4Ogre12VisualEngineEPKNS_9DataModelE
// type: RBX::Adorn *__fastcall(RBX::AdornRbxGfx *this, Ogre::VisualEngine *, const RBX::DataModel *, int) // donor 0xb8df7c
#[doc(alias = "RBX::AdornRbxGfx::AdornRbxGfx(Ogre::VisualEngine *,RBX::DataModel const*)")]
#[doc(alias = "__ZN3RBX11AdornRbxGfxC2EPN4Ogre12VisualEngineEPKNS_9DataModelE")]
// IDA 0xff7751ab60: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab60() {
}

// 0xff7751ab70 — __ZN3RBX11AdornRbxGfx12registerMeshERSsRN4Ogre7MeshPtrE
// type: void __fastcall(RBX::AdornRbxGfx *this, std::string *, Ogre::MeshPtr *) // donor 0xb8f1b8
#[doc(alias = "RBX::AdornRbxGfx::registerMesh(std::string &,Ogre::MeshPtr &)")]
#[doc(alias = "__ZN3RBX11AdornRbxGfx12registerMeshERSsRN4Ogre7MeshPtrE")]
// IDA 0xff7751ab70: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab70() {
}

// 0xff7751ab80 — __ZN3RBX11AdornRbxGfx17RESTORE_EXPLOSIONEPN4Ogre14ParticleSystemEPNS1_15ParticleEmitterEif
// type: void __fastcall(RBX::AdornRbxGfx *this, Ogre::ParticleSystem *, Ogre::ParticleEmitter *, int, float32_t) // donor 0xb962e8
#[doc(alias = "RBX::AdornRbxGfx::RESTORE_EXPLOSION(Ogre::ParticleSystem *,Ogre::ParticleEmitter *,int,float)")]
#[doc(alias = "__ZN3RBX11AdornRbxGfx17RESTORE_EXPLOSIONEPN4Ogre14ParticleSystemEPNS1_15ParticleEmitterEif")]
// IDA 0xff7751ab80: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ab80() {
}

// 0xff7751ab90 — __ZN4Ogre7MeshPtrD1Ev
// type: void __fastcall(Ogre::MeshPtr *__hidden this) // donor 0xb98608
#[doc(alias = "Ogre::MeshPtr::~MeshPtr()")]
#[doc(alias = "__ZN4Ogre7MeshPtrD1Ev")]
// IDA 0xff7751ab90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ab90() {
}

// 0xff7751aba0 — __ZNSt3mapISsN4Ogre7MeshPtrESt4lessISsESaISt4pairIKSsS1_EEEixERS5_
// type: uint32_t *__fastcall(int, const void **) // donor 0xb98874
#[doc(alias = "std::map<std::string,Ogre::MeshPtr,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN4Ogre7MeshPtrESt4lessISsESaISt4pairIKSsS1_EEEixERS5_")]
// IDA 0xff7751aba0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aba0() {
}

// 0xff7751abb0 — __ZN4Ogre9SharedPtrINS_10DataStreamEED1Ev
// type: int __fastcall(int) // donor 0xb98bb0
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10DataStreamEED1Ev")]
// IDA 0xff7751abb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751abb0() {
}

// 0xff7751abc0 — __ZNSt3mapIN3RBX12Vector3int32EPN4Ogre9SceneNodeESt4lessIS1_ESaISt4pairIKS1_S4_EEEixERS8_
// type: _Rb_tree_node_base **__fastcall(int, int *) // donor 0xb98c04
#[doc(alias = "std::map<RBX::Vector3int32,Ogre::SceneNode *,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::operator[](RBX::Vector3int32 const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX12Vector3int32EPN4Ogre9SceneNodeESt4lessIS1_ESaISt4pairIKS1_S4_EEEixERS8_")]
// IDA 0xff7751abc0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751abc0() {
}

// 0xff7751abd0 — __ZNSt6vectorIPN4Ogre9SceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void *__fastcall(int, char *__src, _DWORD *) // donor 0xb98c94
#[doc(alias = "std::vector<Ogre::SceneNode *,std::allocator<Ogre::SceneNode *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SceneNode **,std::vector<Ogre::SceneNode *,std::allocator<Ogre::SceneNode *>>>,Ogre::SceneNode * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre9SceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0xff7751abd0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ff7751abd0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xff7751abe0 — __ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *) // donor 0xb98d8c
#[doc(alias = "std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// IDA 0xff7751abe0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751abe0() {
}

// 0xff7751abf0 — __ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, _DWORD *, int *) // donor 0xb990d4
#[doc(alias = "std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// IDA 0xff7751abf0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751abf0() {
}

// 0xff7751ac00 — __ZNSt6vectorISt17_Rb_tree_iteratorISt4pairIKN3RBX12Vector3int32EPN4Ogre9SceneNodeEEESaIS9_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS9_SB_EERKS9_
// type: _DWORD *__fastcall(int, char *, _DWORD *) // donor 0xb991d4
#[doc(alias = "std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>*,std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> const&)")]
#[doc(alias = "__ZNSt6vectorISt17_Rb_tree_iteratorISt4pairIKN3RBX12Vector3int32EPN4Ogre9SceneNodeEEESaIS9_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS9_SB_EERKS9_")]
// IDA 0xff7751ac00: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ff7751ac00() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xff7751ac10 — __ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *) // donor 0xb992d8
#[doc(alias = "std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// IDA 0xff7751ac10: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac10() {
}

// 0xff7751ac20 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: void __fastcall(int, _DWORD *) // donor 0xb99300
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MeshPtr>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0xff7751ac20: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac20() {
}

// 0xff7751ac30 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN4Ogre7MeshPtrEEE7destroyEPS5_
// type: void __fastcall(int, int) // donor 0xb99330
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,Ogre::MeshPtr>>::destroy(std::pair<std::string const,Ogre::MeshPtr>*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIKSsN4Ogre7MeshPtrEEE7destroyEPS5_")]
// IDA 0xff7751ac30: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac30() {
}

// 0xff7751ac40 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, const void **) // donor 0xb99630
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MeshPtr>>,std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// IDA 0xff7751ac40: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac40() {
}

// 0xff7751ac50 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, const void **) // donor 0xb99978
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
// IDA 0xff7751ac50: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac50() {
}

// 0xff7751ac60 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, const void **) // donor 0xb999ec
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert_unique(std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_")]
// IDA 0xff7751ac60: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac60() {
}

// 0xff7751ac70 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_
// type: _DWORD *__fastcall(int, const std::string *, int, int, void *, int) // donor 0xb99ad0
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_create_node(std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_")]
// IDA 0xff7751ac70: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac70() {
}

// 0xff7751ac80 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_
// type: _DWORD *__fastcall(int, const void **) // donor 0xb99be8
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_")]
// IDA 0xff7751ac80: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ac80() {
}

// 0xff7751ac90 — __ZNSt6vectorIPN4Ogre12RbxSubEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void *__fastcall(int, char *__src, _DWORD *) // donor 0xb99c8c
#[doc(alias = "std::vector<Ogre::RbxSubEntity *,std::allocator<Ogre::RbxSubEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RbxSubEntity **,std::vector<Ogre::RbxSubEntity *,std::allocator<Ogre::RbxSubEntity *>>>,Ogre::RbxSubEntity * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre12RbxSubEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0xff7751ac90: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ff7751ac90() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xff7751aca0 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEED0Ev
// type: void __fastcall(_DWORD *) // donor 0xb99d88
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEED0Ev")]
// IDA 0xff7751aca0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751aca0() {
}

// 0xff7751acb0 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE7destroyEv
// type: void __fastcall(int) // donor 0xb99e48
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE7destroyEv")]
// IDA 0xff7751acb0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751acb0() {
}

// 0xff7751acc0 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0xb99f40
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::swap(Ogre::SharedPtr<Ogre::GpuProgramParameters>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE4swapERS2_")]
// IDA 0xff7751acc0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751acc0() {
}

// 0xff7751acd0 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEED1Ev
// type: _DWORD *__fastcall(_DWORD *) // donor 0xb99f60
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEED1Ev")]
// IDA 0xff7751acd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751acd0() {
}

// 0xff7751ace0 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEED0Ev
// type: void __fastcall(_DWORD *) // donor 0xb99f90
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEED0Ev")]
// IDA 0xff7751ace0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ace0() {
}

// 0xff7751acf0 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv
// type: void __fastcall(int) // donor 0xb9a050
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv")]
// IDA 0xff7751acf0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751acf0() {
}

// 0xff7751ad00 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0xb9a150
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::swap(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE4swapERS2_")]
// IDA 0xff7751ad00: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad00() {
}

// 0xff7751ad10 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0xb9a170
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::swap(Ogre::SharedPtr<Ogre::GpuNamedConstants>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE4swapERS2_")]
// IDA 0xff7751ad10: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad10() {
}

// 0xff7751ad20 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev
// type: void __fastcall(void *) // donor 0xb9a190
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev")]
// IDA 0xff7751ad20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ad20() {
}

// 0xff7751ad30 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv
// type: int __fastcall(int, void *) // donor 0xb9a230
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv")]
// IDA 0xff7751ad30: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad30() {
}

// 0xff7751ad40 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0xb9a268
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::swap(Ogre::SharedPtr<Ogre::GpuSharedParameters>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_")]
// IDA 0xff7751ad40: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad40() {
}

// 0xff7751ad50 — __ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE
// type: RBX::ManualObjectMeshGenAdapter *__fastcall(RBX::ManualObjectMeshGenAdapter *this, Ogre::ManualObject *) // donor 0xb9a284
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE")]
// IDA 0xff7751ad50: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad50() {
}

// 0xff7751ad60 — __ZN4Ogre9SharedPtrINS_4MeshEED0Ev
// type: void __fastcall(void *) // donor 0xb9a920
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEED0Ev")]
// IDA 0xff7751ad60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ad60() {
}

// 0xff7751ad70 — __ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv
// type: int __fastcall(int, void *) // donor 0xb9a9c0
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv")]
// IDA 0xff7751ad70: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad70() {
}

// 0xff7751ad80 — __ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0xb9a9f8
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::swap(Ogre::SharedPtr<Ogre::Mesh>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_")]
// IDA 0xff7751ad80: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad80() {
}

// 0xff7751ad90 — __ZN4Ogre4Node11setListenerEPNS0_8ListenerE
// type: int __fastcall(int result, int) // donor 0xb9aa2c
#[doc(alias = "Ogre::Node::setListener(Ogre::Node::Listener *)")]
#[doc(alias = "__ZN4Ogre4Node11setListenerEPNS0_8ListenerE")]
// IDA 0xff7751ad90: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ad90() {
}

// 0xff7751ada0 — __ZNK4Ogre4Node11getListenerEv
// type: int __fastcall(Ogre::Node *this) // donor 0xb9aa34
#[doc(alias = "Ogre::Node::getListener(void)const")]
#[doc(alias = "__ZNK4Ogre4Node11getListenerEv")]
// IDA 0xff7751ada0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ada0() {
}

// 0xff7751adb0 — __ZN4Ogre4Node10setUserAnyERKNS_3AnyE
// type: void __fastcall(Ogre::Node *this, const Ogre::Any *) // donor 0xb9aa3c
#[doc(alias = "Ogre::Node::setUserAny(Ogre::Any const&)")]
#[doc(alias = "__ZN4Ogre4Node10setUserAnyERKNS_3AnyE")]
// IDA 0xff7751adb0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751adb0() {
}

// 0xff7751adc0 — __ZNK4Ogre4Node10getUserAnyEv
// type: _DWORD *__fastcall(Ogre::Node *this) // donor 0xb9aa44
#[doc(alias = "Ogre::Node::getUserAny(void)const")]
#[doc(alias = "__ZNK4Ogre4Node10getUserAnyEv")]
// IDA 0xff7751adc0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751adc0() {
}

// 0xff7751add0 — __ZNK4Ogre9SceneNode14isInSceneGraphEv
// type: int __fastcall(Ogre::SceneNode *this) // donor 0xb9aa4c
#[doc(alias = "Ogre::SceneNode::isInSceneGraph(void)const")]
#[doc(alias = "__ZNK4Ogre9SceneNode14isInSceneGraphEv")]
// IDA 0xff7751add0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751add0() {
}

// 0xff7751ade0 — __ZN4Ogre9SceneNode15_notifyRootNodeEv
// type: int __fastcall(int this) // donor 0xb9aa54
#[doc(alias = "Ogre::SceneNode::_notifyRootNode(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode15_notifyRootNodeEv")]
// IDA 0xff7751ade0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ade0() {
}

// 0xff7751adf0 — __ZN4Ogre9SceneNode18getAutoTrackTargetEv
// type: int __fastcall(Ogre::SceneNode *this) // donor 0xb9aa5c
#[doc(alias = "Ogre::SceneNode::getAutoTrackTarget(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode18getAutoTrackTargetEv")]
// IDA 0xff7751adf0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751adf0() {
}

// 0xff7751ae00 — __ZN4Ogre9SceneNode18getAutoTrackOffsetEv
// type: char *__fastcall(Ogre::SceneNode *this) // donor 0xb9aa64
#[doc(alias = "Ogre::SceneNode::getAutoTrackOffset(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode18getAutoTrackOffsetEv")]
// IDA 0xff7751ae00: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae00() {
}

// 0xff7751ae10 — __ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv
// type: char *__fastcall(Ogre::SceneNode *this) // donor 0xb9aa6c
#[doc(alias = "Ogre::SceneNode::getAutoTrackLocalDirection(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv")]
// IDA 0xff7751ae10: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae10() {
}

// 0xff7751ae20 — __ZN4Ogre17istreamDataStreamC1EPSib
// type: Ogre::istreamDataStream *__fastcall(Ogre::istreamDataStream *this, std::istream *, bool) // donor 0xb9b3bc
#[doc(alias = "Ogre::istreamDataStream::istreamDataStream(std::istream *,bool)")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamC1EPSib")]
// IDA 0xff7751ae20: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae20() {
}

// 0xff7751ae30 — __ZN4Ogre17istreamDataStreamD0Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this) // donor 0xb9b52c
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD0Ev")]
// IDA 0xff7751ae30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ae30() {
}

// 0xff7751ae40 — __ZN4Ogre17istreamDataStreamD1Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this) // donor 0xb9b5e0
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD1Ev")]
// IDA 0xff7751ae40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ae40() {
}

// 0xff7751ae50 — __ZN4Ogre17istreamDataStreamD2Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this) // donor 0xb9b5e4
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD2Ev")]
// IDA 0xff7751ae50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ff7751ae50() {
}

// 0xff7751ae60 — __ZN4Ogre17istreamDataStream4readEPvm
// type: int __fastcall(std::istream **this, char *, int) // donor 0xb9b744
#[doc(alias = "Ogre::istreamDataStream::read(void *,unsigned long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4readEPvm")]
// IDA 0xff7751ae60: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae60() {
}

// 0xff7751ae70 — __ZN4Ogre17istreamDataStream8readLineEPcmRKSs
// type: unsigned int __fastcall(std::istream **this, char *, unsigned int, char **) // donor 0xb9b758
#[doc(alias = "Ogre::istreamDataStream::readLine(char *,unsigned long,std::string const&)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream8readLineEPcmRKSs")]
// IDA 0xff7751ae70: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae70() {
}

// 0xff7751ae80 — __ZN4Ogre17istreamDataStream4skipEl
// type: int __fastcall(Ogre::istreamDataStream *this, int) // donor 0xb9bbd0
#[doc(alias = "Ogre::istreamDataStream::skip(long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4skipEl")]
// IDA 0xff7751ae80: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae80() {
}

// 0xff7751ae90 — __ZN4Ogre17istreamDataStream4seekEm
// type: int __fastcall(Ogre::istreamDataStream *this, int) // donor 0xb9bbf8
#[doc(alias = "Ogre::istreamDataStream::seek(unsigned long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4seekEm")]
// IDA 0xff7751ae90: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751ae90() {
}

// 0xff7751aea0 — __ZNK4Ogre17istreamDataStream4tellEv
// type: int __fastcall(Ogre::istreamDataStream *this) // donor 0xb9bc20
#[doc(alias = "Ogre::istreamDataStream::tell(void)const")]
#[doc(alias = "__ZNK4Ogre17istreamDataStream4tellEv")]
// IDA 0xff7751aea0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aea0() {
}

// 0xff7751aeb0 — __ZNK4Ogre17istreamDataStream3eofEv
// type: int __fastcall(Ogre::istreamDataStream *this) // donor 0xb9bc64
#[doc(alias = "Ogre::istreamDataStream::eof(void)const")]
#[doc(alias = "__ZNK4Ogre17istreamDataStream3eofEv")]
// IDA 0xff7751aeb0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aeb0() {
}

// 0xff7751aec0 — __ZN4Ogre17istreamDataStream5closeEv
// type: int __fastcall(Ogre::istreamDataStream *this) // donor 0xb9bc78
#[doc(alias = "Ogre::istreamDataStream::close(void)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream5closeEv")]
// IDA 0xff7751aec0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aec0() {
}

// 0xff7751aed0 — __ZN4Ogre7QuadricC1Ev
// type: int __fastcall(int this) // donor 0xb9c320
#[doc(alias = "Ogre::Quadric::Quadric(void)")]
#[doc(alias = "__ZN4Ogre7QuadricC1Ev")]
// IDA 0xff7751aed0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aed0() {
}

// 0xff7751aee0 — __ZN4Ogre7Quadric9setOriginERKNS_7Vector3E
// type: int __fastcall(int this, const Vector3 *) // donor 0xb9c344
#[doc(alias = "Ogre::Quadric::setOrigin(Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre7Quadric9setOriginERKNS_7Vector3E")]
// IDA 0xff7751aee0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aee0() {
}

// 0xff7751aef0 — __ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, float32_t, float32_t, int, int) // donor 0xb9c358
#[doc(alias = "Ogre::Quadric::createCylinder(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii")]
// IDA 0xff7751aef0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751aef0() {
}

// 0xff7751af00 — __ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii
// type: int __fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float, float, int, int) // donor 0xb9e7e8
#[doc(alias = "Ogre::Quadric::createDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii")]
// IDA 0xff7751af00: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af00() {
}

// 0xff7751af10 — __ZN4Ogre7Quadric17createPartialDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffiiff
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, float32_t, int, int, float32_t, float32_t) // donor 0xb9e830
#[doc(alias = "Ogre::Quadric::createPartialDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int,float,float)")]
#[doc(alias = "__ZN4Ogre7Quadric17createPartialDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffiiff")]
// IDA 0xff7751af10: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af10() {
}

// 0xff7751af20 — __ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, int, int) // donor 0xba0b70
#[doc(alias = "Ogre::Quadric::createSphere(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii")]
// IDA 0xff7751af20: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af20() {
}

// 0xff7751af30 — __ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: _DWORD *__fastcall(int, char *, __int64 *) // donor 0xba3c7c
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0xff7751af30: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ff7751af30() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xff7751af40 — __ZNK4Ogre10RbxArchive15isCaseSensitiveEv
// type: int __fastcall(Ogre::RbxArchive *this) // donor 0xba4494
#[doc(alias = "Ogre::RbxArchive::isCaseSensitive(void)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive15isCaseSensitiveEv")]
// IDA 0xff7751af40: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af40() {
}

// 0xff7751af50 — __ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
// type: void __fastcall(int, const char **, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, char, char, char, char, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0xba4498
#[doc(alias = "Ogre::RbxArchive::doStaticFindFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE")]
// IDA 0xff7751af50: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af50() {
}

// 0xff7751af60 — __ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
// type: void __fastcall(struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int, int, int) // donor 0xba4a18
#[doc(alias = "Ogre::RbxArchive::findFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE")]
// IDA 0xff7751af60: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af60() {
}

// 0xff7751af70 — __ZN4OgreL16concatenate_pathERKSsS1_
// type: void __fastcall(Ogre *this, const std::string *, const std::string *) // donor 0xba5874
#[doc(alias = "Ogre::concatenate_path(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4OgreL16concatenate_pathERKSsS1_")]
// IDA 0xff7751af70: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ff7751af70() {
}

