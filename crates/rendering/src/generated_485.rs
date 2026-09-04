//! rendering shard 485 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xbef0f8..0xc1cdec, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter, global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xbef0f8 — __ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: __ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// IDA 0xbef0f8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef0f8() {
}


// 0xbef138 — __ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this)
#[doc(alias = "non-virtual thunk toRBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: __ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// IDA 0xbef138: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef138() {
}


// 0xbef328 — __ZN4Ogre9SharedPtrINS_8ResourceEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_8ResourceEED1Ev
// IDA 0xbef328: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bef328() {
}


// 0xbf29a8 — __ZN4Ogre9SharedPtrINS_8ResourceEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_8ResourceEED0Ev
// IDA 0xbf29a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf29a8() {
}


// 0xbf2a48 — __ZN4Ogre9SharedPtrINS_8ResourceEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_8ResourceEE7destroyEv
// IDA 0xbf2a48: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2a48() {
}


// 0xbf2a80 — __ZN4Ogre9SharedPtrINS_8ResourceEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::swap(Ogre::SharedPtr<Ogre::Resource>&)")]
// was: __ZN4Ogre9SharedPtrINS_8ResourceEE4swapERS2_
// IDA 0xbf2a80: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2a80() {
}


// 0xbf2c80 — __ZN4Ogre9SharedPtrINS_8MaterialEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_8MaterialEED1Ev
// IDA 0xbf2c80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf2c80() {
}


// 0xbf2cb0 — __ZN4Ogre11MaterialPtrD0Ev
// type: void __fastcall(Ogre::MaterialPtr *__hidden this)
#[doc(alias = "Ogre::MaterialPtr::~MaterialPtr()")]
// was: __ZN4Ogre11MaterialPtrD0Ev
// IDA 0xbf2cb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf2cb0() {
}


// 0xbf61f4 — __ZN4Ogre13RbxTypesetterC1ENS_7FontPtrES1_fff
// type: int __fastcall(int, int, int, int, float, float)
#[doc(alias = "Ogre::RbxTypesetter::RbxTypesetter(Ogre::FontPtr,Ogre::FontPtr,float,float,float)")]
// was: __ZN4Ogre13RbxTypesetterC1ENS_7FontPtrES1_fff
// IDA 0xbf61f4: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf61f4() {
}


// 0xbf62a8 — __ZNK4Ogre13RbxTypesetter12computeArrayERKSsfffN3RBX4Text6XAlignEPN3G3D7Vector2EmiN9__gnu_cxx17__normal_iteratorIPKSt4pairIiNS0_7SpacingEESt6vectorISD_SaISD_EEEESJ_RKNS_7FontPtrEb
// type: int __fastcall(int, int, int, int, float, float, int, int, int, float, int, int, int, int)
#[doc(alias = "Ogre::RbxTypesetter::computeArray(std::string const&,float,float,float,RBX::Text::XAlign,G3D::Vector2 *,unsigned long,int,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,Ogre::FontPtr const&,bool)const")]
// was: __ZNK4Ogre13RbxTypesetter12computeArrayERKSsfffN3RBX4Text6XAlignEPN3G3D7Vector2EmiN9__gnu_cxx17__normal_iteratorIPKSt4pairIiNS0_7SpacingEESt6vectorISD_SaISD_EEEESJ_RKNS_7FontPtrEb
// IDA 0xbf62a8: 365 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf62a8() {
}


// 0xbf66dc — __ZNK4Ogre13RbxTypesetter13getTexturePtrEf
// type: _DWORD __fastcall(Ogre::RbxTypesetter *__hidden this, float)
#[doc(alias = "Ogre::RbxTypesetter::getTexturePtr(float)const")]
// was: __ZNK4Ogre13RbxTypesetter13getTexturePtrEf
// IDA 0xbf66dc: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf66dc() {
}


// 0xbf6790 — __ZNK4Ogre13RbxTypesetter4drawEPN3RBX5AdornERKSsRKN3G3D7Vector2EfRKNS6_6Color4ESC_NS1_4Text6XAlignENSD_6YAlignES9_RKNS6_6Rect2DE
// type: int __fastcall(int, int, int, int, int, float, int, int, int, int, int, int)
#[doc(alias = "Ogre::RbxTypesetter::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const")]
// was: __ZNK4Ogre13RbxTypesetter4drawEPN3RBX5AdornERKSsRKN3G3D7Vector2EfRKNS6_6Color4ESC_NS1_4Text6XAlignENSD_6YAlignES9_RKNS6_6Rect2DE
// IDA 0xbf6790: 844 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf6790() {
}


// 0xbf7268 — __ZNK4Ogre13RbxTypesetter15measureInternalERKSsfRKN3G3D7Vector2EPSt6vectorISt4pairIiNS0_7SpacingEESaISA_EEPb
#[doc(alias = "Ogre::RbxTypesetter::measureInternal(std::string const&,float,G3D::Vector2 const&,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>> *,bool *)const")]
// was: __ZNK4Ogre13RbxTypesetter15measureInternalERKSsfRKN3G3D7Vector2EPSt6vectorISt4pairIiNS0_7SpacingEESaISA_EEPb
// IDA 0xbf7268: 460 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7268() {
}


// 0xbf7794 — __ZNK4Ogre13RbxTypesetter23getCursorPositionInTextERKSsRKN3G3D7Vector2EfN3RBX4Text6XAlignENS8_6YAlignES6_S4_
// type: int __fastcall(int, int, int, int, int, int, int, G3D::Vector2 *)
#[doc(alias = "Ogre::RbxTypesetter::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const")]
// was: __ZNK4Ogre13RbxTypesetter23getCursorPositionInTextERKSsRKN3G3D7Vector2EfN3RBX4Text6XAlignENS8_6YAlignES6_S4_
// IDA 0xbf7794: 494 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7794() {
}


// 0xbf7dcc — __ZNK4Ogre13RbxTypesetter7measureERKSsfRKN3G3D7Vector2EPb
// type: _DWORD __fastcall(Ogre::RbxTypesetter *__hidden this, const std::string *, float, const G3D::Vector2 *, bool *)
#[doc(alias = "Ogre::RbxTypesetter::measure(std::string const&,float,G3D::Vector2 const&,bool *)const")]
// was: __ZNK4Ogre13RbxTypesetter7measureERKSsfRKN3G3D7Vector2EPb
// IDA 0xbf7dcc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7dcc() {
}


// 0xbf8504 — __ZNK3RBX16TypesetterBitmap4drawEPNS_5AdornERKSsRKN3G3D7Vector2EfRKNS5_6Color4ESB_NS_4Text6XAlignENSC_6YAlignES8_RKNS5_6Rect2DE
// type: int __fastcall(int, int, int, int, int, float, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "RBX::TypesetterBitmap::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const")]
// was: __ZNK3RBX16TypesetterBitmap4drawEPNS_5AdornERKSsRKN3G3D7Vector2EfRKNS5_6Color4ESB_NS_4Text6XAlignENSC_6YAlignES8_RKNS5_6Rect2DE
// IDA 0xbf8504: 445 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8504() {
}


// 0xbf8a1c — __ZNK3RBX16TypesetterBitmap6layoutERKSsPSt6vectorINS0_9GlyphLineESaIS4_EEiRKN3G3D12Vector2int16EbPb
#[doc(alias = "RBX::TypesetterBitmap::layout(std::string const&,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>> *,int,G3D::Vector2int16 const&,bool,bool *)const")]
// was: __ZNK3RBX16TypesetterBitmap6layoutERKSsPSt6vectorINS0_9GlyphLineESaIS4_EEiRKN3G3D12Vector2int16EbPb
// IDA 0xbf8a1c: 312 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8a1c() {
}


// 0xbf8d24 — __ZN3RBXL8drawRectEPNS_5AdornEbRKN3G3D6Rect2DES5_RKNS2_7Vector2ES8_RKNS2_6Color4E
#[doc(alias = "RBX::drawRect(RBX::Adorn *,bool,G3D::Rect2D const&,G3D::Rect2D const&,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Color4 const&)")]
// was: __ZN3RBXL8drawRectEPNS_5AdornEbRKN3G3D6Rect2DES5_RKNS2_7Vector2ES8_RKNS2_6Color4E
// IDA 0xbf8d24: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8d24() {
}


// 0xbf8ecc — __ZNK3RBX16TypesetterBitmap23getCursorPositionInTextERKSsRKN3G3D7Vector2EfNS_4Text6XAlignENS7_6YAlignES6_S4_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::TypesetterBitmap::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const")]
// was: __ZNK3RBX16TypesetterBitmap23getCursorPositionInTextERKSsRKN3G3D7Vector2EfNS_4Text6XAlignENS7_6YAlignES6_S4_
// IDA 0xbf8ecc: 244 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8ecc() {
}


// 0xbf9194 — __ZNK3RBX16TypesetterBitmap7measureERKSsfRKN3G3D7Vector2EPb
// type: _DWORD __fastcall(RBX::TypesetterBitmap *__hidden this, const std::string *, float, const G3D::Vector2 *, bool *)
#[doc(alias = "RBX::TypesetterBitmap::measure(std::string const&,float,G3D::Vector2 const&,bool *)const")]
// was: __ZNK3RBX16TypesetterBitmap7measureERKSsfRKN3G3D7Vector2EPb
// IDA 0xbf9194: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf9194() {
}


// 0xbf9278 — __ZN4Ogre13RbxTypesetter12getCharWidthERKNS_7FontPtrEfcf
#[doc(alias = "Ogre::RbxTypesetter::getCharWidth(Ogre::FontPtr const&,float,char,float)")]
// was: __ZN4Ogre13RbxTypesetter12getCharWidthERKNS_7FontPtrEfcf
// IDA 0xbf9278: 121 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf9278() {
}


// 0xbf93d0 — __ZN4Ogre13RbxTypesetterD1Ev
// type: void __fastcall(Ogre::RbxTypesetter *__hidden this)
#[doc(alias = "Ogre::RbxTypesetter::~RbxTypesetter()")]
// was: __ZN4Ogre13RbxTypesetterD1Ev
// IDA 0xbf93d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf93d0() {
}


// 0xbf94a8 — __ZN4Ogre13RbxTypesetterD0Ev
// type: void __fastcall(Ogre::RbxTypesetter *__hidden this)
#[doc(alias = "Ogre::RbxTypesetter::~RbxTypesetter()")]
// was: __ZN4Ogre13RbxTypesetterD0Ev
// IDA 0xbf94a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf94a8() {
}


// 0xbf9bd4 — __ZNSt6vectorISt4pairIiN4Ogre13RbxTypesetter7SpacingEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing>*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,std::pair<int,Ogre::RbxTypesetter::Spacing> const&)")]
// was: __ZNSt6vectorISt4pairIiN4Ogre13RbxTypesetter7SpacingEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// IDA 0xbf9bd4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bf9bd4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xbfa404 — __ZN4Ogre8RbxImage8allocateERSiRKSsib
// type: _DWORD __fastcall(Ogre::RbxImage *__hidden this, std::istream *, const std::string *, int, bool)
#[doc(alias = "Ogre::RbxImage::allocate(std::istream &,std::string const&,int,bool)")]
// was: __ZN4Ogre8RbxImage8allocateERSiRKSsib
// IDA 0xbfa404: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfa404() {
}


// 0xbfa5ac — __ZN4Ogre8RbxImageD1Ev
// type: void __fastcall(Ogre::RbxImage *__hidden this)
#[doc(alias = "Ogre::RbxImage::~RbxImage()")]
// was: __ZN4Ogre8RbxImageD1Ev
// IDA 0xbfa5ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfa5ac() {
}


// 0xbfa5d0 — __ZN4Ogre8RbxImageD0Ev
// type: void __fastcall(Ogre::RbxImage *__hidden this)
#[doc(alias = "Ogre::RbxImage::~RbxImage()")]
// was: __ZN4Ogre8RbxImageD0Ev
// IDA 0xbfa5d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfa5d0() {
}


// 0xbfa684 — __ZNK4Ogre8RbxImage7getSizeEv
// type: _DWORD __fastcall(Ogre::RbxImage *__hidden this)
#[doc(alias = "Ogre::RbxImage::getSize(void)const")]
// was: __ZNK4Ogre8RbxImage7getSizeEv
// IDA 0xbfa684: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfa684() {
}


// 0xbfa68c — __ZNK4Ogre8RbxImage16getOriginalWidthEv
// type: _DWORD __fastcall(Ogre::RbxImage *__hidden this)
#[doc(alias = "Ogre::RbxImage::getOriginalWidth(void)const")]
// was: __ZNK4Ogre8RbxImage16getOriginalWidthEv
// IDA 0xbfa68c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfa68c() {
}


// 0xbfa690 — __ZNK4Ogre8RbxImage17getOriginalHeightEv
// type: _DWORD __fastcall(Ogre::RbxImage *__hidden this)
#[doc(alias = "Ogre::RbxImage::getOriginalHeight(void)const")]
// was: __ZNK4Ogre8RbxImage17getOriginalHeightEv
// IDA 0xbfa690: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfa690() {
}


// 0xbfad1c — __ZN4Ogre25RbxSpatialHashedSceneNodeC1EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::RbxSpatialHashedSceneNode(Ogre::SceneManager *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNodeC1EPNS_12SceneManagerE
// IDA 0xbfad1c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bfad1c() {
}


// 0xbfad20 — __ZN4Ogre25RbxSpatialHashedSceneNodeC2EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::RbxSpatialHashedSceneNode(Ogre::SceneManager *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNodeC2EPNS_12SceneManagerE
// IDA 0xbfad20: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfad20() {
}


// 0xbfae34 — __ZN4Ogre25RbxSpatialHashedSceneNodeD0Ev
// type: void __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::~RbxSpatialHashedSceneNode()")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNodeD0Ev
// IDA 0xbfae34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfae34() {
}


// 0xbfaee8 — __ZN4Ogre25RbxSpatialHashedSceneNodeD1Ev
// type: void __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::~RbxSpatialHashedSceneNode()")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNodeD1Ev
// IDA 0xbfaee8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bfaee8() {
}


// 0xbfaeec — __ZN4Ogre25RbxSpatialHashedSceneNodeD2Ev
// type: void __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::~RbxSpatialHashedSceneNode()")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNodeD2Ev
// IDA 0xbfaeec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfaeec() {
}


// 0xbfaff0 — __ZN4Ogre25RbxSpatialHashedSceneNode8addChildEPNS_4NodeE
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::Node *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::addChild(Ogre::Node *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode8addChildEPNS_4NodeE
// IDA 0xbfaff0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfaff0() {
}


// 0xbfb0c0 — __ZN4Ogre25RbxSpatialHashedSceneNode12isAdmissibleEPNS_20RbxCullableSceneNodeE
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::isAdmissible(Ogre::RbxCullableSceneNode *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode12isAdmissibleEPNS_20RbxCullableSceneNodeE
// IDA 0xbfb0c0: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb0c0() {
}


// 0xbfb14c — __ZN4Ogre25RbxSpatialHashedSceneNode25RemoveFromSpatialInternalEPNS_20RbxCullableSceneNodeE
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::RbxCullableSceneNode *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::RemoveFromSpatialInternal(Ogre::RbxCullableSceneNode *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode25RemoveFromSpatialInternalEPNS_20RbxCullableSceneNodeE
// IDA 0xbfb14c: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb14c() {
}


// 0xbfb1fc — __ZN4Ogre25RbxSpatialHashedSceneNode11removeChildEt
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::removeChild(unsigned short)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode11removeChildEt
// IDA 0xbfb1fc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb1fc() {
}


// 0xbfb214 — __ZN4Ogre25RbxSpatialHashedSceneNode11removeChildEPNS_4NodeE
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::Node *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::removeChild(Ogre::Node *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode11removeChildEPNS_4NodeE
// IDA 0xbfb214: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb214() {
}


// 0xbfb22c — __ZN4Ogre25RbxSpatialHashedSceneNode11removeChildERKSs
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::removeChild(std::string const&)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode11removeChildERKSs
// IDA 0xbfb22c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb22c() {
}


// 0xbfb244 — __ZN4Ogre25RbxSpatialHashedSceneNode17removeAllChildrenEv
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::removeAllChildren(void)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode17removeAllChildrenEv
// IDA 0xbfb244: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb244() {
}


// 0xbfb278 — __ZN4Ogre25RbxSpatialHashedSceneNode13_updateBoundsEv
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_updateBounds(void)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode13_updateBoundsEv
// IDA 0xbfb278: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb278() {
}


// 0xbfb280 — __ZN4Ogre25RbxSpatialHashedSceneNode11updateChildEPNS_20RbxCullableSceneNodeE
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::RbxCullableSceneNode *)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::updateChild(Ogre::RbxCullableSceneNode *)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode11updateChildEPNS_20RbxCullableSceneNodeE
// IDA 0xbfb280: 183 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb280() {
}


// 0xbfb48c — __ZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, bool, bool, bool)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// IDA 0xbfb48c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb48c() {
}


// 0xbfb554 — __ZN4Ogre25RbxSpatialHashedSceneNode12getHashedNumEv
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::getHashedNum(void)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode12getHashedNumEv
// IDA 0xbfb554: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb554() {
}


// 0xbfb55c — __ZN4Ogre25RbxSpatialHashedSceneNode14getUnhashedNumEv
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::getUnhashedNum(void)")]
// was: __ZN4Ogre25RbxSpatialHashedSceneNode14getUnhashedNumEv
// IDA 0xbfb55c: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb55c() {
}


// 0xbfb568 — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter10IntersectsERKN3RBX7ExtentsE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Intersects(RBX::Extents const&)")]
// was: __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter10IntersectsERKN3RBX7ExtentsE
// IDA 0xbfb568: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb568() {
}


// 0xbfb69c — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter8DistanceERKN3RBX7ExtentsE
// type: int __fastcall(int, RBX::Extents *this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Distance(RBX::Extents const&)")]
// was: __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter8DistanceERKN3RBX7ExtentsE
// IDA 0xbfb69c: 4 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb69c() {
}


// 0xbfb6a8 — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter11onPrimitiveEPNS_20RbxCullableSceneNodeEN3RBX15IntersectResultEf
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::onPrimitive(Ogre::RbxCullableSceneNode *,RBX::IntersectResult,float)")]
// was: __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter11onPrimitiveEPNS_20RbxCullableSceneNodeEN3RBX15IntersectResultEf
// IDA 0xbfb6a8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb6a8() {
}


// 0xbfb710 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS4_i
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHash(RBX::World *,RBX::ContactManager*,int)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS4_i
// IDA 0xbfb710: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb710() {
}


// 0xbfb900 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE5setupEv
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::setup(void)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE5setupEv
// IDA 0xbfb900: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb900() {
}


// 0xbfb9f4 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::~SpatialHash()")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EED2Ev
// IDA 0xbfb9f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfb9f4() {
}


// 0xbfbbe4 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE22visitPrimitivesInSpaceEPNS5_11SpaceFilterE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::visitPrimitivesInSpace(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpaceFilter *)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE22visitPrimitivesInSpaceEPNS5_11SpaceFilterE
// IDA 0xbfbbe4: 647 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfbbe4() {
}


// 0xbfc380 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
// IDA 0xbfc380: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc380() {
}


// 0xbfc480 — __ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv
// IDA 0xbfc480: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc480() {
}


// 0xbfc4f0 — __ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv
// IDA 0xbfc4f0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc4f0() {
}


// 0xbfc568 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS2_b
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(Ogre::RbxCullableSceneNode*,bool)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS2_b
// IDA 0xbfc568: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc568() {
}


// 0xbfc644 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS2_b
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(Ogre::RbxCullableSceneNode*,bool)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS2_b
// IDA 0xbfc644: 183 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc644() {
}


// 0xbfc890 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS2_
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(Ogre::RbxCullableSceneNode*)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS2_
// IDA 0xbfc890: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc890() {
}


// 0xbfc9b4 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS2_
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(Ogre::RbxCullableSceneNode*)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS2_
// IDA 0xbfc9b4: 220 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfc9b4() {
}


// 0xbfcc2c — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS2_RKNS_7ExtentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(Ogre::RbxCullableSceneNode*,RBX::Extents const&)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS2_RKNS_7ExtentsE
// IDA 0xbfcc2c: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfcc2c() {
}


// 0xbfce78 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// IDA 0xbfce78: 218 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfce78() {
}


// 0xbfd0c8 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS5_11SpatialNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS5_11SpatialNodeE
// IDA 0xbfd0c8: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfd0c8() {
}


// 0xbfd1a0 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS5_8TreeNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS5_8TreeNodeE
// IDA 0xbfd1a0: 263 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfd1a0() {
}


// 0xbfd478 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
// type: int __fastcall(int, RBX::SpatialHashStatic *this, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
// IDA 0xbfd478: 130 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfd478() {
}


// 0xbfd5f0 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS2_RKNS_12Vector3int32Eb
// type: int __fastcall(int, int, unsigned int *)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::addNode(Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&,bool)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS2_RKNS_12Vector3int32Eb
// IDA 0xbfd5f0: 316 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfd5f0() {
}


// 0xbfd978 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS5_11SpatialNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// was: __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS5_11SpatialNodeE
// IDA 0xbfd978: 124 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfd978() {
}


// 0xbfdae4 — __ZNSt6vectorIPN4Ogre20RbxCullableSceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::RbxCullableSceneNode *,std::allocator<Ogre::RbxCullableSceneNode *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RbxCullableSceneNode **,std::vector<Ogre::RbxCullableSceneNode *,std::allocator<Ogre::RbxCullableSceneNode *>>>,Ogre::RbxCullableSceneNode * const&)")]
// was: __ZNSt6vectorIPN4Ogre20RbxCullableSceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0xbfdae4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bfdae4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xbfdbdc — __ZNSt6vectorIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_
#[doc(alias = "std::vector<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry*,std::vector<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>>,unsigned long,RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry const&)")]
// was: __ZNSt6vectorIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_
// IDA 0xbfdbdc: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfdbdc() {
}


// 0xbfde44 — __ZN3G3D5ArrayIPN4Ogre20RbxCullableSceneNodeELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::RbxCullableSceneNode *,10,32ul>::~Array()")]
// was: __ZN3G3D5ArrayIPN4Ogre20RbxCullableSceneNodeELi10ELm32EED2Ev
// IDA 0xbfde44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfde44() {
}


// 0xbfdf64 — __ZN3G3D5ArrayIPN4Ogre20RbxCullableSceneNodeELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::RbxCullableSceneNode *,10,32ul>::Array(void)")]
// was: __ZN3G3D5ArrayIPN4Ogre20RbxCullableSceneNodeELi10ELm32EEC2Ev
// IDA 0xbfdf64: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfdf64() {
}


// 0xbfee1c — __ZN4Ogre7RBXSSAO18createDummyGBufferEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::createDummyGBuffer(void)")]
// was: __ZN4Ogre7RBXSSAO18createDummyGBufferEv
// IDA 0xbfee1c: 174 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfee1c() {
}


// 0xbfefcc — __ZN4Ogre7RBXSSAOD1Ev
// type: void __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::~RBXSSAO()")]
// was: __ZN4Ogre7RBXSSAOD1Ev
// IDA 0xbfefcc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bfefcc() {
}


// 0xbfefd0 — __ZN4Ogre7RBXSSAOD2Ev
// type: void __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::~RBXSSAO()")]
// was: __ZN4Ogre7RBXSSAOD2Ev
// IDA 0xbfefd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bfefd0() {
}


// 0xbff164 — __ZN4Ogre7RBXSSAO12setSSAOLevelEN3RBX9SSAOLevelE
#[doc(alias = "Ogre::RBXSSAO::setSSAOLevel(RBX::SSAOLevel)")]
// was: __ZN4Ogre7RBXSSAO12setSSAOLevelEN3RBX9SSAOLevelE
// IDA 0xbff164: 334 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bff164() {
}


// 0xbff518 — __ZN4Ogre7RBXSSAO20destroyLostResourcesEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::destroyLostResources(void)")]
// was: __ZN4Ogre7RBXSSAO20destroyLostResourcesEv
// IDA 0xbff518: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bff518() {
}


// 0xbff598 — __ZN4Ogre7RBXSSAO19createLostResourcesEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::createLostResources(void)")]
// was: __ZN4Ogre7RBXSSAO19createLostResourcesEv
// IDA 0xbff598: 1404 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bff598() {
}


// 0xc00380 — __ZN4Ogre7RBXSSAO15createSSAONoiseEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::createSSAONoise(void)")]
// was: __ZN4Ogre7RBXSSAO15createSSAONoiseEv
// IDA 0xc00380: 547 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c00380() {
}


// 0xc0094c — __ZN4Ogre7RBXSSAO16onDeviceRestoredEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::onDeviceRestored(void)")]
// was: __ZN4Ogre7RBXSSAO16onDeviceRestoredEv
// IDA 0xc0094c: 15 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0094c() {
}


// 0xc00974 — __ZN4Ogre7RBXSSAO13renderComputeEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::renderCompute(void)")]
// was: __ZN4Ogre7RBXSSAO13renderComputeEv
// IDA 0xc00974: 2121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c00974() {
}


// 0xc01f6c — __ZN4Ogre7RBXSSAO20renderFullScreenQuadERKNS_11MaterialPtrE
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this, const Ogre::MaterialPtr *)
#[doc(alias = "Ogre::RBXSSAO::renderFullScreenQuad(Ogre::MaterialPtr const&)")]
// was: __ZN4Ogre7RBXSSAO20renderFullScreenQuadERKNS_11MaterialPtrE
// IDA 0xc01f6c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c01f6c() {
}


// 0xc01fc8 — __ZN4Ogre7RBXSSAO14renderCompositEv
// type: _DWORD __fastcall(Ogre::RBXSSAO *__hidden this)
#[doc(alias = "Ogre::RBXSSAO::renderComposit(void)")]
// was: __ZN4Ogre7RBXSSAO14renderCompositEv
// IDA 0xc01fc8: 569 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c01fc8() {
}


// 0xc02d80 — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitor5visitEPN4Ogre10RenderableEtbPNS1_3AnyE
// type: _DWORD __fastcall(ResourceGroupHelper::UpdateMaterialRenderableVisitor *__hidden this, Ogre::Renderable *, unsigned __int16, bool, Ogre::Any *)
#[doc(alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::visit(Ogre::Renderable *,unsigned short,bool,Ogre::Any *)")]
// was: __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitor5visitEPN4Ogre10RenderableEtbPNS1_3AnyE
// IDA 0xc02d80: 439 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c02d80() {
}


// 0xc036ac — __ZN19ResourceGroupHelper30ResourceGroupHelperLogListener13messageLoggedERKSsN4Ogre15LogMessageLevelEbS2_Rb
#[doc(alias = "ResourceGroupHelper::ResourceGroupHelperLogListener::messageLogged(std::string const&,Ogre::LogMessageLevel,bool,std::string const&,bool &)")]
// was: __ZN19ResourceGroupHelper30ResourceGroupHelperLogListener13messageLoggedERKSsN4Ogre15LogMessageLevelEbS2_Rb
// IDA 0xc036ac: 217 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c036ac() {
}


// 0xc03db8 — __ZL28updateMaterialsOnRenderNodesPKN4Ogre9SceneNodeE
// type: _DWORD __fastcall(const Ogre::SceneNode *)
#[doc(alias = "updateMaterialsOnRenderNodes(Ogre::SceneNode const*)")]
// was: __ZL28updateMaterialsOnRenderNodesPKN4Ogre9SceneNodeE
// IDA 0xc03db8: 852 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c03db8() {
}


// 0xc04658 — __ZN19ResourceGroupHelper31visitRecursivelyRenderablesFromEPN4Ogre16OverlayContainerERNS0_10Renderable7VisitorEb
#[doc(alias = "ResourceGroupHelper::visitRecursivelyRenderablesFrom(Ogre::OverlayContainer *,Ogre::Renderable::Visitor &,bool)")]
// was: __ZN19ResourceGroupHelper31visitRecursivelyRenderablesFromEPN4Ogre16OverlayContainerERNS0_10Renderable7VisitorEb
// IDA 0xc04658: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c04658() {
}


// 0xc058d4 — __ZN3RBX17MegaClusterLegacyC1EPN4Ogre15RbxSceneManagerE
// type: _DWORD __fastcall(RBX::MegaClusterLegacy *__hidden this, Ogre::RbxSceneManager *)
#[doc(alias = "RBX::MegaClusterLegacy::MegaClusterLegacy(Ogre::RbxSceneManager *)")]
// was: __ZN3RBX17MegaClusterLegacyC1EPN4Ogre15RbxSceneManagerE
// IDA 0xc058d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c058d4() {
}


// 0xc058d8 — __ZN3RBX17MegaClusterLegacyC2EPN4Ogre15RbxSceneManagerE
// type: _DWORD __fastcall(RBX::MegaClusterLegacy *__hidden this, Ogre::RbxSceneManager *)
#[doc(alias = "RBX::MegaClusterLegacy::MegaClusterLegacy(Ogre::RbxSceneManager *)")]
// was: __ZN3RBX17MegaClusterLegacyC2EPN4Ogre15RbxSceneManagerE
// IDA 0xc058d8: 354 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c058d8() {
}


// 0xc06a98 — __ZN3RBX10outputFaceEPNS_10MEGAVERTEXERKN3G3D7Vector3ERKNS_12OFFSETINFOV2ERKNS_5Voxel13BlockAxisFaceEPKNS2_7Vector2ESF_jjh
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, _DWORD *, int *, int, int, int, char, int)
#[doc(alias = "RBX::outputFace(RBX::MEGAVERTEX *,G3D::Vector3 const&,RBX::OFFSETINFOV2 const&,RBX::Voxel::BlockAxisFace const&,G3D::Vector2 const*,G3D::Vector2 const*,unsigned int,unsigned int,unsigned char)")]
// was: __ZN3RBX10outputFaceEPNS_10MEGAVERTEXERKN3G3D7Vector3ERKNS_12OFFSETINFOV2ERKNS_5Voxel13BlockAxisFaceEPKNS2_7Vector2ESF_jjh
// IDA 0xc06a98: 236 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c06a98() {
}


// 0xc06ea8 — __ZN3RBX17MegaClusterLegacy26updateChunkCoordinateFrameEPN4Ogre20RbxCullableSceneNodeERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::MegaClusterLegacy::updateChunkCoordinateFrame(Ogre::RbxCullableSceneNode *,RBX::SpatialRegion::Id const&)")]
// was: __ZN3RBX17MegaClusterLegacy26updateChunkCoordinateFrameEPN4Ogre20RbxCullableSceneNodeERKNS_13SpatialRegion2IdE
// IDA 0xc06ea8: 172 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c06ea8() {
}


// 0xc0923c — __ZN3RBX11MegaCluster14createGeometryEPNS_10RenderNodeERKN4Ogre29HardwareVertexBufferSharedPtrEPKch
// type: _DWORD __fastcall(RBX::MegaCluster *__hidden this, RBX::RenderNode *, const Ogre::HardwareVertexBufferSharedPtr *, const char *, unsigned __int8)
#[doc(alias = "RBX::MegaCluster::createGeometry(RBX::RenderNode *,Ogre::HardwareVertexBufferSharedPtr const&,char const*,unsigned char)")]
// was: __ZN3RBX11MegaCluster14createGeometryEPNS_10RenderNodeERKN4Ogre29HardwareVertexBufferSharedPtrEPKch
// IDA 0xc0923c: 535 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0923c() {
}


// 0xc0ba10 — __ZNK3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS0_5Water17RelevantNeighborsEPNS8_13LocalAreaInfoE
#[doc(alias = "RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const")]
// was: __ZNK3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS0_5Water17RelevantNeighborsEPNS8_13LocalAreaInfoE
// IDA 0xc0ba10: 455 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0ba10() {
}


// 0xc0d190 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
// was: __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh
// IDA 0xc0d190: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0d190() {
}


// 0xc162c8 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
// was: __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh
// IDA 0xc162c8: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c162c8() {
}


// 0xc184dc — __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk8loadDataINS0_4GridEEEvPKT_RKN3G3D12Vector3int16E
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "void RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::loadData<RBX::Voxel::Grid>(RBX::Voxel::Grid const*,G3D::Vector3int16 const&)")]
// was: __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk8loadDataINS0_4GridEEEvPKT_RKN3G3D12Vector3int16E
// IDA 0xc184dc: 376 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c184dc() {
}


// 0xc188c4 — __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk9fillEmptyERKN3G3D12Vector3int16ES7_
#[doc(alias = "RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillEmpty(G3D::Vector3int16 const&,G3D::Vector3int16 const&)")]
// was: __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk9fillEmptyERKN3G3D12Vector3int16ES7_
// IDA 0xc188c4: 107 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c188c4() {
}


// 0xc18d18 — __ZN3RBX26MaterialTextureCoordinatesC2ERKN3G3D12Vector2int16ES4_S4_fb
// type: _DWORD __fastcall(RBX::MaterialTextureCoordinates *__hidden this, const G3D::Vector2int16 *, const G3D::Vector2int16 *, const G3D::Vector2int16 *, float, bool)
#[doc(alias = "RBX::MaterialTextureCoordinates::MaterialTextureCoordinates(G3D::Vector2int16 const&,G3D::Vector2int16 const&,G3D::Vector2int16 const&,float,bool)")]
// was: __ZN3RBX26MaterialTextureCoordinatesC2ERKN3G3D12Vector2int16ES4_S4_fb
// IDA 0xc18d18: 129 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c18d18() {
}


// 0xc1a250 — __ZN3RBX20getLightContributionERKN3G3D7Vector3Ei
// type: _DWORD __fastcall(RBX *__hidden this, const Vector3 *, int)
#[doc(alias = "RBX::getLightContribution(G3D::Vector3 const&,int)")]
// was: __ZN3RBX20getLightContributionERKN3G3D7Vector3Ei
// IDA 0xc1a250: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1a250() {
}


// 0xc1cdec — __ZN3RBX9LightGrid22occupancyFillBlockDFAAERNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS6_15CoordinateFrameEf
// type: int __fastcall(int, int, int, int, int, float)
#[doc(alias = "RBX::LightGrid::occupancyFillBlockDFAA(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float)")]
// was: __ZN3RBX9LightGrid22occupancyFillBlockDFAAERNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS6_15CoordinateFrameEf
// IDA 0xc1cdec: 436 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1cdec() {
}

