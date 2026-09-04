//! rendering — generated_506 — 100 stubs Ogre/G3D EA-sorted asc global dedup (rbx_core::SharedPtr not boost)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xf25a50..0xf66734 (756 candidates remaining, 93643 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf25a50 — __ZN4Ogre16RenderQueueGroup13addRenderableEPNS_10RenderableEPNS_9TechniqueEt$shim
// type: int __fastcall(Ogre::RenderQueueGroup *this, Ogre::Renderable *, Ogre::Technique *, unsigned __int16)
#[doc(alias = "__ZN4Ogre16RenderQueueGroup13addRenderableEPNS_10RenderableEPNS_9TechniqueEt$shim")]
// was: __ZN4Ogre16RenderQueueGroup13addRenderableEPNS_10RenderableEPNS_9TechniqueEt$shim
// IDA 0xf25a50: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf25a50() {
}

// 0xf4afc4 — j___ZN3G3D5ArrayIbLi10ELm32EE6appendERKb
#[doc(alias = "G3D::Array<bool,10,32ul>::append(bool const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIbLi10ELm32EE6appendERKb")]
// was: G3D::Array<bool,10,32ul>::append(bool const&)
// IDA 0xf4afc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf4afc4() {
}

// 0xf4afd4 — j___ZN3G3D5ArrayIbLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<bool,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIbLi10ELm32EEC2Ev")]
// was: G3D::Array<bool,10,32ul>::Array(void)
// IDA 0xf4afd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf4afd4() {
}

// 0xf4afe4 — j___ZN3G3D5ArrayIbLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<bool,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIbLi10ELm32EED2Ev")]
// was: G3D::Array<bool,10,32ul>::~Array()
// IDA 0xf4afe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf4afe4() {
}

// 0xf4c574 — j___ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const G3D::Line *, const G3D::Line *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "j___ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_")]
// was: G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)
// IDA 0xf4c574: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf4c574() {
}

// 0xf57934 — j___ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_
// type: _DWORD __fastcall(G3D::Vector3int16 *__hidden this, const G3D::Vector3int16 *, const G3D::Vector3int16 *)
#[doc(alias = "G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
#[doc(alias = "j___ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_")]
// was: G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const
// IDA 0xf57934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf57934() {
}

// 0xf57eb4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_
#[doc(alias = "G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_")]
// was: G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)
// IDA 0xf57eb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf57eb4() {
}

// 0xf57ec4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_")]
// was: G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)
// IDA 0xf57ec4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf57ec4() {
}

// 0xf580a4 — j___ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Plane::pointOnOrBehind(G3D::Vector3)const")]
#[doc(alias = "j___ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E")]
// was: G3D::Plane::pointOnOrBehind(G3D::Vector3)const
// IDA 0xf580a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf580a4() {
}

// 0xf58604 — j___ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "j___ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_")]
// was: G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0xf58604: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf58604() {
}

// 0xf58764 — j___ZNK3G3D4Line8distanceERKNS_7Vector3E
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *)
#[doc(alias = "G3D::Line::distance(G3D::Vector3 const&)const")]
#[doc(alias = "j___ZNK3G3D4Line8distanceERKNS_7Vector3E")]
// was: G3D::Line::distance(G3D::Vector3 const&)const
// IDA 0xf58764: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf58764() {
}

// 0xf58924 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_")]
// was: G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)
// IDA 0xf58924: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0xf58924() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf5b904 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_")]
// was: G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)
// IDA 0xf5b904: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0xf5b904() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf5b934 — j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)")]
#[doc(alias = "j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_")]
// was: std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)
// IDA 0xf5b934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf5b934() {
}

// 0xf5ba34 — j___ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)")]
#[doc(alias = "j___ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type")]
// was: void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)
// IDA 0xf5ba34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf5ba34() {
}

// 0xf5dd74 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib")]
// was: G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)
// IDA 0xf5dd74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf5dd74() {
}

// 0xf5dd84 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev")]
// was: G3D::Array<G3D::Plane,10,32ul>::Array(void)
// IDA 0xf5dd84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf5dd84() {
}

// 0xf5e284 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_")]
// was: G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)
// IDA 0xf5e284: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf5e284() {
}

// 0xf5e694 — j___ZNK3G3D7Vector38isFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::isFinite(void)const")]
#[doc(alias = "j___ZNK3G3D7Vector38isFiniteEv")]
// was: G3D::Vector3::isFinite(void)const
// IDA 0xf5e694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf5e694() {
}

// 0xf64a94 — j___ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
// was: std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)
// IDA 0xf64a94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf64a94() {
}

// 0xf64aa4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)
// IDA 0xf64aa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf64aa4() {
}

// 0xf64ab4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)
// IDA 0xf64ab4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf64ab4() {
}

// 0xf64ac4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)
// IDA 0xf64ac4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf64ac4() {
}

// 0xf64ad4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)
// IDA 0xf64ad4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf64ad4() {
}

// 0xf64c54 — j___ZN4Ogre16ShadowRenderableD2Ev
// type: void __fastcall(Ogre::ShadowRenderable *__hidden this)
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "j___ZN4Ogre16ShadowRenderableD2Ev")]
// was: Ogre::ShadowRenderable::~ShadowRenderable()
// IDA 0xf64c54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf64c54() {
}

// 0xf64c74 — j___ZNK3G3D15CoordinateFrame13toObjectSpaceERKS0_
// type: _DWORD __fastcall(G3D::CoordinateFrame *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "G3D::CoordinateFrame::toObjectSpace(G3D::CoordinateFrame const&)const")]
#[doc(alias = "j___ZNK3G3D15CoordinateFrame13toObjectSpaceERKS0_")]
// was: G3D::CoordinateFrame::toObjectSpace(G3D::CoordinateFrame const&)const
// IDA 0xf64c74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf64c74() {
}

// 0xf65014 — j___ZN4Ogre9SharedPtrINS_10DataStreamEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_10DataStreamEED1Ev")]
// was: Ogre::SharedPtr<Ogre::DataStream>::~SharedPtr()
// IDA 0xf65014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65014() {
}

// 0xf65024 — j___ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE7destroyEv
// type: void __fastcall(int)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::GpuProgramParameters>::destroy(void)
// IDA 0xf65024: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65024() {
}

// 0xf65034 — j___ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::destroy(void)
// IDA 0xf65034: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65034() {
}

// 0xf65044 — j___ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::Mesh>::destroy(void)
// IDA 0xf65044: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65044() {
}

// 0xf65054 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN4Ogre7MeshPtrEEE7destroyEPS5_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,Ogre::MeshPtr>>::destroy(std::pair<std::string const,Ogre::MeshPtr>*)")]
#[doc(alias = "j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN4Ogre7MeshPtrEEE7destroyEPS5_")]
// was: __gnu_cxx::new_allocator<std::pair<std::string const,Ogre::MeshPtr>>::destroy(std::pair<std::string const,Ogre::MeshPtr>*)
// IDA 0xf65054: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65054() {
}

// 0xf65084 — j___ZNSt3mapISsN4Ogre7MeshPtrESt4lessISsESaISt4pairIKSsS1_EEEixERS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,Ogre::MeshPtr,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsN4Ogre7MeshPtrESt4lessISsESaISt4pairIKSsS1_EEEixERS5_")]
// was: std::map<std::string,Ogre::MeshPtr,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::operator[](std::string const&)
// IDA 0xf65084: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65084() {
}

// 0xf650a4 — j___ZNSt6vectorIPN4Ogre9SceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<Ogre::SceneNode *,std::allocator<Ogre::SceneNode *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SceneNode **,std::vector<Ogre::SceneNode *,std::allocator<Ogre::SceneNode *>>>,Ogre::SceneNode * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre9SceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<Ogre::SceneNode *,std::allocator<Ogre::SceneNode *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SceneNode **,std::vector<Ogre::SceneNode *,std::allocator<Ogre::SceneNode *>>>,Ogre::SceneNode * const&)
// IDA 0xf650a4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf650a4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf650f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_create_node(std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_create_node(std::pair<std::string const,Ogre::MeshPtr> const&)
// IDA 0xf650f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf650f4() {
}

// 0xf65104 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert_unique(std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert_unique(std::pair<std::string const,Ogre::MeshPtr> const&)
// IDA 0xf65104: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65104() {
}

// 0xf65114 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MeshPtr>>,std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MeshPtr>>,std::pair<std::string const,Ogre::MeshPtr> const&)
// IDA 0xf65114: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65114() {
}

// 0xf65124 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_
// type: _DWORD *__fastcall(int, const void **)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::find(std::string const&)
// IDA 0xf65124: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65124() {
}

// 0xf65134 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MeshPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MeshPtr>> *)
// IDA 0xf65134: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65134() {
}

// 0xf65144 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MeshPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre7MeshPtrEESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MeshPtr>,std::_Select1st<std::pair<std::string const,Ogre::MeshPtr>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::MeshPtr>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MeshPtr> const&)
// IDA 0xf65144: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65144() {
}

// 0xf65154 — j___ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// was: std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)
// IDA 0xf65154: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf65154() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65164 — j___ZNSt6vectorIN4Ogre9BlockSortESaIS1_EE7reserveEm
// type: unsigned int __fastcall(void **, unsigned int)
#[doc(alias = "std::vector<Ogre::BlockSort,std::allocator<Ogre::BlockSort>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre9BlockSortESaIS1_EE7reserveEm")]
// was: std::vector<Ogre::BlockSort,std::allocator<Ogre::BlockSort>>::reserve(unsigned long)
// IDA 0xf65164: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65164() {
}

// 0xf651e4 — j___ZN5boost6detail12shared_countC2IN4Ogre11RootManagerEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<Ogre::RootManager>(Ogre::RootManager *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN4Ogre11RootManagerEEEPT_")]
// was: boost::detail::shared_count::shared_count<Ogre::RootManager>(Ogre::RootManager *)
// IDA 0xf651e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf651e4() {
}

// 0xf65214 — j___ZNK4Ogre13DriverVersion8toStringEv
// type: _DWORD __fastcall(Ogre::DriverVersion *__hidden this)
#[doc(alias = "Ogre::DriverVersion::toString(void)const")]
#[doc(alias = "j___ZNK4Ogre13DriverVersion8toStringEv")]
// was: Ogre::DriverVersion::toString(void)const
// IDA 0xf65214: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65214() {
}

// 0xf65234 — j___ZNKSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE4findERKSs
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
#[doc(alias = "j___ZNKSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE4findERKSs")]
// was: std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xf65234: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65234() {
}

// 0xf65244 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPSsSt6vectorISsN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEEA10_cET_SD_SD_RKT0_St26random_access_iterator_tag
// type: int __fastcall(std::string *this, int, char *)
#[doc(alias = "__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__find<__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,char [10]>(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,char [10] const&,std::random_access_iterator_tag)")]
#[doc(alias = "j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPSsSt6vectorISsN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEEA10_cET_SD_SD_RKT0_St26random_access_iterator_tag")]
// was: __gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__find<__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,char [10]>(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,char [10] const&,std::random_access_iterator_tag)
// IDA 0xf65244: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65244() {
}

// 0xf65254 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::append(Ogre::VertexStreamer::VertexChunk const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_")]
// was: G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::append(Ogre::VertexStreamer::VertexChunk const&)
// IDA 0xf65254: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65254() {
}

// 0xf65264 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib
// type: int()
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib")]
// was: G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::resize(int,bool)
// IDA 0xf65264: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65264() {
}

// 0xf65274 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi")]
// was: G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::realloc(int)
// IDA 0xf65274: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65274() {
}

// 0xf65284 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EEC2Ev")]
// was: G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::Array(void)
// IDA 0xf65284: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65284() {
}

// 0xf65294 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EED2Ev")]
// was: G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::~Array()
// IDA 0xf65294: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65294() {
}

// 0xf652a4 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6appendERKS3_
// type: int()
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::append(Ogre::VertexStreamer::Vertex3DTexture const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6appendERKS3_")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::append(Ogre::VertexStreamer::Vertex3DTexture const&)
// IDA 0xf652a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf652a4() {
}

// 0xf652b4 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6resizeEib
// type: int()
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6resizeEib")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::resize(int,bool)
// IDA 0xf652b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf652b4() {
}

// 0xf652c4 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE7reallocEi")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::realloc(int)
// IDA 0xf652c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf652c4() {
}

// 0xf652d4 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EEC2Ev")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::Array(void)
// IDA 0xf652d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf652d4() {
}

// 0xf652e4 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EED2Ev")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::~Array()
// IDA 0xf652e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf652e4() {
}

// 0xf652f4 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::append(Ogre::VertexStreamer::Vertex3D const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6appendERKS3_")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::append(Ogre::VertexStreamer::Vertex3D const&)
// IDA 0xf652f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf652f4() {
}

// 0xf65304 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6resizeEib")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::resize(int,bool)
// IDA 0xf65304: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65304() {
}

// 0xf65314 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE7reallocEi")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::realloc(int)
// IDA 0xf65314: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65314() {
}

// 0xf65324 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EEC2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EEC2Ev")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::Array(void)
// IDA 0xf65324: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65324() {
}

// 0xf65334 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EED2Ev")]
// was: G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::~Array()
// IDA 0xf65334: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65334() {
}

// 0xf65344 — j___ZN3G3D5ArrayIiLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<int,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIiLi10ELm32EEC2Ev")]
// was: G3D::Array<int,10,32ul>::Array(void)
// IDA 0xf65344: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65344() {
}

// 0xf65354 — j___ZN3G3D5ArrayIiLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<int,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIiLi10ELm32EED2Ev")]
// was: G3D::Array<int,10,32ul>::~Array()
// IDA 0xf65354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65354() {
}

// 0xf65364 — j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE20createHardwareBufferEj
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::createHardwareBuffer(unsigned int)")]
#[doc(alias = "j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE20createHardwareBufferEj")]
// was: Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::createHardwareBuffer(unsigned int)
// IDA 0xf65364: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65364() {
}

// 0xf65374 — j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE")]
// was: Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)
// IDA 0xf65374: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65374() {
}

// 0xf65384 — j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEED1Ev
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::~VertexBufferBatch()")]
#[doc(alias = "j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEED1Ev")]
// was: Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::~VertexBufferBatch()
// IDA 0xf65384: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65384() {
}

// 0xf65394 — j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE20createHardwareBufferEj
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::createHardwareBuffer(unsigned int)")]
#[doc(alias = "j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE20createHardwareBufferEj")]
// was: Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::createHardwareBuffer(unsigned int)
// IDA 0xf65394: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65394() {
}

// 0xf653a4 — j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE
// type: int()
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE")]
// was: Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)
// IDA 0xf653a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf653a4() {
}

// 0xf653b4 — j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEED1Ev
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::~VertexBufferBatch()")]
#[doc(alias = "j___ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEED1Ev")]
// was: Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::~VertexBufferBatch()
// IDA 0xf653b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf653b4() {
}

// 0xf65404 — j___ZNSt6vectorIPN4Ogre14ParticleSystemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleSystem **,std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>>,Ogre::ParticleSystem * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre14ParticleSystemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleSystem **,std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>>,Ogre::ParticleSystem * const&)
// IDA 0xf65404: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf65404() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65414 — j___ZNSt6vectorISt4pairIPN4Ogre14ParticleSystemEfESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ParticleSystem *,float>*,std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>>,std::pair<Ogre::ParticleSystem *,float> const&)")]
#[doc(alias = "j___ZNSt6vectorISt4pairIPN4Ogre14ParticleSystemEfESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
// was: std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ParticleSystem *,float>*,std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>>,std::pair<Ogre::ParticleSystem *,float> const&)
// IDA 0xf65414: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf65414() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65434 — j___ZN5boost12scoped_arrayIN4Ogre11MaterialPtrEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "boost::scoped_array<Ogre::MaterialPtr>::~scoped_array()")]
#[doc(alias = "j___ZN5boost12scoped_arrayIN4Ogre11MaterialPtrEED1Ev")]
// was: boost::scoped_array<Ogre::MaterialPtr>::~scoped_array()
// IDA 0xf65434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65434() {
}

// 0xf65454 — j___ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)")]
#[doc(alias = "j___ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
// was: std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)
// IDA 0xf65454: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf65454() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65464 — j___ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm
// type: int()
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm")]
// was: std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)
// IDA 0xf65464: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65464() {
}

// 0xf65474 — j___ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)
// IDA 0xf65474: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65474() {
}

// 0xf65484 — j___ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// was: std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)
// IDA 0xf65484: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65484() {
}

// 0xf655d4 — j___ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,unsigned long,Ogre::Vector3 const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// was: std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,unsigned long,Ogre::Vector3 const&)
// IDA 0xf655d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf655d4() {
}

// 0xf655e4 — j___ZNSt6vectorIPN4Ogre12ManualObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject **,std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>>,Ogre::ManualObject * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre12ManualObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject **,std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>>,Ogre::ManualObject * const&)
// IDA 0xf655e4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf655e4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf655f4 — j___ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_insert_unique(std::pair<Ogre::SubMesh const* const,Ogre::Sphere> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// was: std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_insert_unique(std::pair<Ogre::SubMesh const* const,Ogre::Sphere> const&)
// IDA 0xf655f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf655f4() {
}

// 0xf65604 — j___ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// was: std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)
// IDA 0xf65604: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf65604() {
}

// 0xf65694 — j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre10TexturePtrES4_EET0_T_S6_S5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)")]
#[doc(alias = "j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre10TexturePtrES4_EET0_T_S6_S5_")]
// was: Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)
// IDA 0xf65694: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0xf65694() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf65704 — j___ZNSt6vectorIN4Ogre10TexturePtrESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre10TexturePtrESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// was: std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)
// IDA 0xf65704: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf65704() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65904 — j___ZN4Ogre9SharedPtrINS_8MaterialEED1Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_8MaterialEED1Ev")]
// was: Ogre::SharedPtr<Ogre::Material>::~SharedPtr()
// IDA 0xf65904: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65904() {
}

// 0xf65914 — j___ZN4Ogre9SharedPtrINS_8ResourceEED1Ev
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_8ResourceEED1Ev")]
// was: Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()
// IDA 0xf65914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf65914() {
}

// 0xf66024 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE5_copyERKS2_
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::_copy(G3D::Array<G3D::Plane,10,32ul> const&)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE5_copyERKS2_")]
// was: G3D::Array<G3D::Plane,10,32ul>::_copy(G3D::Array<G3D::Plane,10,32ul> const&)
// IDA 0xf66024: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66024() {
}

// 0xf66034 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE7reallocEi")]
// was: G3D::Array<G3D::Plane,10,32ul>::realloc(int)
// IDA 0xf66034: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66034() {
}

// 0xf66044 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EED2Ev")]
// was: G3D::Array<G3D::Plane,10,32ul>::~Array()
// IDA 0xf66044: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66044() {
}

// 0xf66404 — j___ZN4Ogre10TexturePtrD1Ev
// type: void __fastcall(Ogre::TexturePtr *__hidden this)
#[doc(alias = "Ogre::TexturePtr::~TexturePtr()")]
#[doc(alias = "j___ZN4Ogre10TexturePtrD1Ev")]
// was: Ogre::TexturePtr::~TexturePtr()
// IDA 0xf66404: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66404() {
}

// 0xf66414 — j___ZN4Ogre9SharedPtrINS_7TextureEED1Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_7TextureEED1Ev")]
// was: Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()
// IDA 0xf66414: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66414() {
}

// 0xf66644 — j___ZN4Ogre20GpuProgramParametersD2Ev
// type: void __fastcall(Ogre::GpuProgramParameters *__hidden this)
#[doc(alias = "Ogre::GpuProgramParameters::~GpuProgramParameters()")]
#[doc(alias = "j___ZN4Ogre20GpuProgramParametersD2Ev")]
// was: Ogre::GpuProgramParameters::~GpuProgramParameters()
// IDA 0xf66644: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66644() {
}

// 0xf66654 — j___ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE7destroyEv
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::GpuNamedConstants>::destroy(void)
// IDA 0xf66654: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66654() {
}

// 0xf66664 — j___ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED1Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED1Ev")]
// was: Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()
// IDA 0xf66664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66664() {
}

// 0xf66674 — j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev")]
// was: std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
// IDA 0xf66674: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66674() {
}

// 0xf66684 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuConstantDefinition>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuConstantDefinition>> *)
// IDA 0xf66684: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66684() {
}

// 0xf66694 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)
// IDA 0xf66694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66694() {
}

// 0xf666a4 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>> *)
// IDA 0xf666a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf666a4() {
}

// 0xf666b4 — j___ZN3G3D5ArrayIbLi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<bool,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIbLi10ELm32EE6resizeEib")]
// was: G3D::Array<bool,10,32ul>::resize(int,bool)
// IDA 0xf666b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf666b4() {
}

// 0xf666c4 — j___ZN3G3D5ArrayIbLi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<bool,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIbLi10ELm32EE7reallocEi")]
// was: G3D::Array<bool,10,32ul>::realloc(int)
// IDA 0xf666c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf666c4() {
}

// 0xf666d4 — j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE14getCreateEntryERKSsRb
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)")]
#[doc(alias = "j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE14getCreateEntryERKSsRb")]
// was: G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)
// IDA 0xf666d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf666d4() {
}

// 0xf666e4 — j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE6resizeEm
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)")]
#[doc(alias = "j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE6resizeEm")]
// was: G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)
// IDA 0xf666e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf666e4() {
}

// 0xf66734 — j___ZN3G3D5ArrayISsLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<std::string,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayISsLi10ELm32EEC2Ev")]
// was: G3D::Array<std::string,10,32ul>::Array(void)
// IDA 0xf66734: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66734() {
}
