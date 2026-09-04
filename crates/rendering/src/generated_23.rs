//! rendering shard D — next 100 stubs EA-sorted after 0xd14558 strict Ogre|G3D (filtered 13663 total, 7751 prior, 100 this batch — 0xd14558..0xd3c464)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd14558 — __ZN4Ogre15ResourceManager11parseScriptERNS_9SharedPtrINS_10DataStreamEEERKSs
#[doc(alias = "Ogre::ResourceManager::parseScript(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)")]
// was: Ogre::ResourceManager::parseScript(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)
// IDA 0xd14558: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d14558() {
}

// 0xd262c8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4Ogre4NodeEEEEEE20construct_with_valueIJS9_EEEvDpOT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>>>>::construct_with_value<std::pair<std::string const,Ogre::Node *>>(std::pair<std::string const,Ogre::Node *> &&)")]
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>>>>::construct_with_value<std::pair<std::string const,Ogre::Node *>>(std::pair<std::string const,Ogre::Node *> &&)
// IDA 0xd262c8: 59 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d262c8() {
}

// 0xd26368 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
// IDA 0xd26368: 148 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26368() {
}

// 0xd26510 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
// IDA 0xd26510: 56 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26510() {
}

// 0xd265c0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::~table()
// IDA 0xd265c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d265c0() {
}

// 0xd26674 — __ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node *>,false>::~_Rb_tree_impl()
// IDA 0xd26674: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d26674() {
}

// 0xd26678 — __ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node *>,false>::~_Rb_tree_impl()
// IDA 0xd26678: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d26678() {
}

// 0xd26688 — __ZNSt12_Vector_baseIPN4Ogre4NodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Node *,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Node *,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd26688: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d26688() {
}

// 0xd26698 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::Vector4>> *)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::Vector4>> *)
// IDA 0xd26698: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26698() {
}

// 0xd266c0 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
// IDA 0xd266c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d266c0() {
}

// 0xd27d58 — __ZNSt10_List_baseIPN4Ogre16OverlayContainerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::OverlayContainer *,Ogre::STLAllocator<Ogre::OverlayContainer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::OverlayContainer *,Ogre::STLAllocator<Ogre::OverlayContainer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xd27d58: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d27d58() {
}

// 0xd27d5c — __ZNSt10_List_baseIPN4Ogre16OverlayContainerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::OverlayContainer *,Ogre::STLAllocator<Ogre::OverlayContainer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::OverlayContainer *,Ogre::STLAllocator<Ogre::OverlayContainer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xd27d5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27d5c() {
}

// 0xd296f8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xd296f8: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d296f8() {
}

// 0xd2979c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::OverlayContainer *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::OverlayContainer *> const&)
// IDA 0xd2979c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2979c() {
}

// 0xd29880 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::OverlayContainer *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::OverlayContainer *> const&)
// IDA 0xd29880: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29880() {
}

// 0xd299d4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd299d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d299d4() {
}

// 0xd299d8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd299d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d299d8() {
}

// 0xd299e4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::OverlayContainer *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::OverlayContainer *>> *)
// IDA 0xd299e4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d299e4() {
}

// 0xd36d54 — __ZN4Ogre19PanelOverlayElement14CmdTransparentD0Ev
#[doc(alias = "Ogre::PanelOverlayElement::CmdTransparent::~CmdTransparent()")]
// was: Ogre::PanelOverlayElement::CmdTransparent::~CmdTransparent()
// IDA 0xd36d54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d36d54() {
}

// 0xd36d60 — __ZN4Ogre19PanelOverlayElement11CmdUVCoordsD0Ev
#[doc(alias = "Ogre::PanelOverlayElement::CmdUVCoords::~CmdUVCoords()")]
// was: Ogre::PanelOverlayElement::CmdUVCoords::~CmdUVCoords()
// IDA 0xd36d60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d36d60() {
}

// 0xd36d6c — __ZNK4Ogre16OverlayContainer11isContainerEv
#[doc(alias = "Ogre::OverlayContainer::isContainer(void)const")]
// was: Ogre::OverlayContainer::isContainer(void)const
// IDA 0xd36d6c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36d6c() {
}

// 0xd36d70 — __ZNK4Ogre14OverlayElement12isKeyEnabledEv
#[doc(alias = "Ogre::OverlayElement::isKeyEnabled(void)const")]
// was: Ogre::OverlayElement::isKeyEnabled(void)const
// IDA 0xd36d70: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36d70() {
}

// 0xd36d74 — __ZNK4Ogre14OverlayElement11isCloneableEv
#[doc(alias = "Ogre::OverlayElement::isCloneable(void)const")]
// was: Ogre::OverlayElement::isCloneable(void)const
// IDA 0xd36d74: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36d74() {
}

// 0xd36d7c — __ZN4Ogre14OverlayElement12setCloneableEb
#[doc(alias = "Ogre::OverlayElement::setCloneable(bool)")]
// was: Ogre::OverlayElement::setCloneable(bool)
// IDA 0xd36d7c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36d7c() {
}

// 0xd36d84 — __ZNK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xd36d84: 7 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36d84() {
}

// 0xd36da4 — __ZNK4Ogre14OverlayElement9getLightsEv
#[doc(alias = "Ogre::OverlayElement::getLights(void)const")]
// was: Ogre::OverlayElement::getLights(void)const
// IDA 0xd36da4: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36da4() {
}

// 0xd36e20 — __ZNK4Ogre16OverlayContainer23isChildrenProcessEventsEv
#[doc(alias = "Ogre::OverlayContainer::isChildrenProcessEvents(void)const")]
// was: Ogre::OverlayContainer::isChildrenProcessEvents(void)const
// IDA 0xd36e20: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36e20() {
}

// 0xd36e24 — __ZN4Ogre16OverlayContainer24setChildrenProcessEventsEb
#[doc(alias = "Ogre::OverlayContainer::setChildrenProcessEvents(bool)")]
// was: Ogre::OverlayContainer::setChildrenProcessEvents(bool)
// IDA 0xd36e24: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36e24() {
}

// 0xd36e2c — __ZN4Ogre10Renderable10postRenderEPNS_12SceneManagerEPNS_12RenderSystemE
#[doc(alias = "Ogre::Renderable::postRender(Ogre::SceneManager *,Ogre::RenderSystem *)")]
// was: Ogre::Renderable::postRender(Ogre::SceneManager *,Ogre::RenderSystem *)
// IDA 0xd36e2c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d36e2c() {
}

// 0xd36e30 — __ZThn12_NK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "non-virtual thunk toOgre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: non-virtual thunk to Ogre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xd36e30: 7 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36e30() {
}

// 0xd36e50 — __ZThn12_NK4Ogre14OverlayElement9getLightsEv
#[doc(alias = "non-virtual thunk toOgre::OverlayElement::getLights(void)const")]
// was: non-virtual thunk to Ogre::OverlayElement::getLights(void)const
// IDA 0xd36e50: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36e50() {
}

// 0xd36ecc — __ZNK4Ogre10Renderable15getCastsShadowsEv
#[doc(alias = "Ogre::Renderable::getCastsShadows(void)const")]
// was: Ogre::Renderable::getCastsShadows(void)const
// IDA 0xd36ecc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36ecc() {
}

// 0xd36ed0 — __ZN4Ogre10Renderable26setPolygonModeOverrideableEb
#[doc(alias = "Ogre::Renderable::setPolygonModeOverrideable(bool)")]
// was: Ogre::Renderable::setPolygonModeOverrideable(bool)
// IDA 0xd36ed0: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36ed0() {
}

// 0xd36ed8 — __ZNK4Ogre10Renderable26getPolygonModeOverrideableEv
#[doc(alias = "Ogre::Renderable::getPolygonModeOverrideable(void)const")]
// was: Ogre::Renderable::getPolygonModeOverrideable(void)const
// IDA 0xd36ed8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36ed8() {
}

// 0xd36ee0 — __ZN4Ogre10Renderable10setUserAnyERKNS_3AnyE
#[doc(alias = "Ogre::Renderable::setUserAny(Ogre::Any const&)")]
// was: Ogre::Renderable::setUserAny(Ogre::Any const&)
// IDA 0xd36ee0: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36ee0() {
}

// 0xd36eec — __ZNK4Ogre10Renderable19getRenderSystemDataEv
#[doc(alias = "Ogre::Renderable::getRenderSystemData(void)const")]
// was: Ogre::Renderable::getRenderSystemData(void)const
// IDA 0xd36eec: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36eec() {
}

// 0xd36ef0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::ParamCommand *>>,std::pair<std::string const,Ogre::ParamCommand *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::ParamCommand *>>,std::pair<std::string const,Ogre::ParamCommand *> const&)
// IDA 0xd36ef0: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d36ef0() {
}

// 0xd370d0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ParamCommand *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ParamCommand *> const&)
// IDA 0xd370d0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d370d0() {
}

// 0xd37224 — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParameterDef*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParameterDef const&)")]
// was: std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParameterDef*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParameterDef const&)
// IDA 0xd37224: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_d37224() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd377b0 — __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xd377b0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d377b0() {
}

// 0xd377b4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd377b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d377b4() {
}

// 0xd377b8 — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS7_
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd377b8: 104 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d377b8() {
}

// 0xd378d8 — __ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPKN4Ogre12ParameterDefESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEPS3_SB_ET0_T_SG_SF_T1_
#[doc(alias = "Ogre::ParameterDef* std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::ParameterDef const*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParameterDef*,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::ParameterDef const*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::ParameterDef const*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParameterDef*,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::ParameterDef* std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::ParameterDef const*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParameterDef*,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::ParameterDef const*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::ParameterDef const*,std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParameterDef*,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xd378d8: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d378d8() {
}

// 0xd37ad4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ParamDictionary> const&)
// IDA 0xd37ad4: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37ad4() {
}

// 0xd37b48 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xd37b48: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37b48() {
}

// 0xd37ca0 — __ZN4Ogre8Particle11setRotationERKNS_6RadianE
#[doc(alias = "Ogre::Particle::setRotation(Ogre::Radian const&)")]
// was: Ogre::Particle::setRotation(Ogre::Radian const&)
// IDA 0xd37ca0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37ca0() {
}

// 0xd37cc4 — __ZN4Ogre8Particle13setDimensionsEff
#[doc(alias = "Ogre::Particle::setDimensions(float,float)")]
// was: Ogre::Particle::setDimensions(float,float)
// IDA 0xd37cc4: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37cc4() {
}

// 0xd37ce0 — __ZN4Ogre8Particle12_notifyOwnerEPNS_14ParticleSystemE
#[doc(alias = "Ogre::Particle::_notifyOwner(Ogre::ParticleSystem *)")]
// was: Ogre::Particle::_notifyOwner(Ogre::ParticleSystem *)
// IDA 0xd37ce0: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37ce0() {
}

// 0xd37ce4 — __ZN4Ogre8Particle15resetDimensionsEv
#[doc(alias = "Ogre::Particle::resetDimensions(void)")]
// was: Ogre::Particle::resetDimensions(void)
// IDA 0xd37ce4: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37ce4() {
}

// 0xd37d20 — __ZN4Ogre15ParticleEmitterC2EPNS_14ParticleSystemE
#[doc(alias = "Ogre::ParticleEmitter::ParticleEmitter(Ogre::ParticleSystem *)")]
// was: Ogre::ParticleEmitter::ParticleEmitter(Ogre::ParticleSystem *)
// IDA 0xd37d20: 378 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d37d20() {
}

// 0xd381d8 — __ZN4Ogre15ParticleEmitterD0Ev
#[doc(alias = "Ogre::ParticleEmitter::~ParticleEmitter()")]
// was: Ogre::ParticleEmitter::~ParticleEmitter()
// IDA 0xd381d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d381d8() {
}

// 0xd38268 — __ZN4Ogre15ParticleEmitterD1Ev
#[doc(alias = "Ogre::ParticleEmitter::~ParticleEmitter()")]
// was: Ogre::ParticleEmitter::~ParticleEmitter()
// IDA 0xd38268: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d38268() {
}

// 0xd38274 — __ZN4Ogre15ParticleEmitterD2Ev
#[doc(alias = "Ogre::ParticleEmitter::~ParticleEmitter()")]
// was: Ogre::ParticleEmitter::~ParticleEmitter()
// IDA 0xd38274: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d38274() {
}

// 0xd38384 — __ZN4Ogre15ParticleEmitter11setPositionERKNS_7Vector3E
#[doc(alias = "Ogre::ParticleEmitter::setPosition(Ogre::Vector3 const&)")]
// was: Ogre::ParticleEmitter::setPosition(Ogre::Vector3 const&)
// IDA 0xd38384: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38384() {
}

// 0xd38394 — __ZNK4Ogre15ParticleEmitter11getPositionEv
#[doc(alias = "Ogre::ParticleEmitter::getPosition(void)const")]
// was: Ogre::ParticleEmitter::getPosition(void)const
// IDA 0xd38394: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38394() {
}

// 0xd38398 — __ZN4Ogre15ParticleEmitter12setDirectionERKNS_7Vector3E
#[doc(alias = "Ogre::ParticleEmitter::setDirection(Ogre::Vector3 const&)")]
// was: Ogre::ParticleEmitter::setDirection(Ogre::Vector3 const&)
// IDA 0xd38398: 102 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38398() {
}

// 0xd38524 — __ZNK4Ogre15ParticleEmitter12getDirectionEv
#[doc(alias = "Ogre::ParticleEmitter::getDirection(void)const")]
// was: Ogre::ParticleEmitter::getDirection(void)const
// IDA 0xd38524: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38524() {
}

// 0xd38528 — __ZN4Ogre15ParticleEmitter5setUpERKNS_7Vector3E
#[doc(alias = "Ogre::ParticleEmitter::setUp(Ogre::Vector3 const&)")]
// was: Ogre::ParticleEmitter::setUp(Ogre::Vector3 const&)
// IDA 0xd38528: 26 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38528() {
}

// 0xd3858c — __ZNK4Ogre15ParticleEmitter5getUpEv
#[doc(alias = "Ogre::ParticleEmitter::getUp(void)const")]
// was: Ogre::ParticleEmitter::getUp(void)const
// IDA 0xd3858c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3858c() {
}

// 0xd38590 — __ZN4Ogre15ParticleEmitter8setAngleERKNS_6RadianE
#[doc(alias = "Ogre::ParticleEmitter::setAngle(Ogre::Radian const&)")]
// was: Ogre::ParticleEmitter::setAngle(Ogre::Radian const&)
// IDA 0xd38590: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38590() {
}

// 0xd38598 — __ZNK4Ogre15ParticleEmitter8getAngleEv
#[doc(alias = "Ogre::ParticleEmitter::getAngle(void)const")]
// was: Ogre::ParticleEmitter::getAngle(void)const
// IDA 0xd38598: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38598() {
}

// 0xd3859c — __ZN4Ogre15ParticleEmitter19setParticleVelocityEf
#[doc(alias = "Ogre::ParticleEmitter::setParticleVelocity(float)")]
// was: Ogre::ParticleEmitter::setParticleVelocity(float)
// IDA 0xd3859c: 3 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3859c() {
}

// 0xd385a8 — __ZN4Ogre15ParticleEmitter19setParticleVelocityEff
#[doc(alias = "Ogre::ParticleEmitter::setParticleVelocity(float,float)")]
// was: Ogre::ParticleEmitter::setParticleVelocity(float,float)
// IDA 0xd385a8: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385a8() {
}

// 0xd385b4 — __ZN4Ogre15ParticleEmitter15setEmissionRateEf
#[doc(alias = "Ogre::ParticleEmitter::setEmissionRate(float)")]
// was: Ogre::ParticleEmitter::setEmissionRate(float)
// IDA 0xd385b4: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385b4() {
}

// 0xd385b8 — __ZNK4Ogre15ParticleEmitter15getEmissionRateEv
#[doc(alias = "Ogre::ParticleEmitter::getEmissionRate(void)const")]
// was: Ogre::ParticleEmitter::getEmissionRate(void)const
// IDA 0xd385b8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385b8() {
}

// 0xd385bc — __ZN4Ogre15ParticleEmitter13setTimeToLiveEf
#[doc(alias = "Ogre::ParticleEmitter::setTimeToLive(float)")]
// was: Ogre::ParticleEmitter::setTimeToLive(float)
// IDA 0xd385bc: 3 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385bc() {
}

// 0xd385c8 — __ZN4Ogre15ParticleEmitter13setTimeToLiveEff
#[doc(alias = "Ogre::ParticleEmitter::setTimeToLive(float,float)")]
// was: Ogre::ParticleEmitter::setTimeToLive(float,float)
// IDA 0xd385c8: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385c8() {
}

// 0xd385d4 — __ZN4Ogre15ParticleEmitter9setColourERKNS_11ColourValueE
#[doc(alias = "Ogre::ParticleEmitter::setColour(Ogre::ColourValue const&)")]
// was: Ogre::ParticleEmitter::setColour(Ogre::ColourValue const&)
// IDA 0xd385d4: 7 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385d4() {
}

// 0xd385ec — __ZN4Ogre15ParticleEmitter9setColourERKNS_11ColourValueES3_
#[doc(alias = "Ogre::ParticleEmitter::setColour(Ogre::ColourValue const&,Ogre::ColourValue const&)")]
// was: Ogre::ParticleEmitter::setColour(Ogre::ColourValue const&,Ogre::ColourValue const&)
// IDA 0xd385ec: 7 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d385ec() {
}

// 0xd38604 — __ZNK4Ogre15ParticleEmitter7getNameEv
#[doc(alias = "Ogre::ParticleEmitter::getName(void)const")]
// was: Ogre::ParticleEmitter::getName(void)const
// IDA 0xd38604: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38604() {
}

// 0xd38608 — __ZN4Ogre15ParticleEmitter7setNameERKSs
#[doc(alias = "Ogre::ParticleEmitter::setName(std::string const&)")]
// was: Ogre::ParticleEmitter::setName(std::string const&)
// IDA 0xd38608: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38608() {
}

// 0xd38614 — __ZNK4Ogre15ParticleEmitter17getEmittedEmitterEv
#[doc(alias = "Ogre::ParticleEmitter::getEmittedEmitter(void)const")]
// was: Ogre::ParticleEmitter::getEmittedEmitter(void)const
// IDA 0xd38614: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38614() {
}

// 0xd38618 — __ZN4Ogre15ParticleEmitter17setEmittedEmitterERKSs
#[doc(alias = "Ogre::ParticleEmitter::setEmittedEmitter(std::string const&)")]
// was: Ogre::ParticleEmitter::setEmittedEmitter(std::string const&)
// IDA 0xd38618: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38618() {
}

// 0xd38624 — __ZNK4Ogre15ParticleEmitter9isEmittedEv
#[doc(alias = "Ogre::ParticleEmitter::isEmitted(void)const")]
// was: Ogre::ParticleEmitter::isEmitted(void)const
// IDA 0xd38624: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38624() {
}

// 0xd3862c — __ZN4Ogre15ParticleEmitter10setEmittedEb
#[doc(alias = "Ogre::ParticleEmitter::setEmitted(bool)")]
// was: Ogre::ParticleEmitter::setEmitted(bool)
// IDA 0xd3862c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3862c() {
}

// 0xd38634 — __ZN4Ogre15ParticleEmitter20genEmissionDirectionERNS_7Vector3E
#[doc(alias = "Ogre::ParticleEmitter::genEmissionDirection(Ogre::Vector3 &)")]
// was: Ogre::ParticleEmitter::genEmissionDirection(Ogre::Vector3 &)
// IDA 0xd38634: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38634() {
}

// 0xd38690 — __ZN4Ogre15ParticleEmitter19genEmissionVelocityERNS_7Vector3E
#[doc(alias = "Ogre::ParticleEmitter::genEmissionVelocity(Ogre::Vector3 &)")]
// was: Ogre::ParticleEmitter::genEmissionVelocity(Ogre::Vector3 &)
// IDA 0xd38690: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d38690() {
}

// 0xd386f8 — __ZN4Ogre15ParticleEmitter14genEmissionTTLEv
#[doc(alias = "Ogre::ParticleEmitter::genEmissionTTL(void)")]
// was: Ogre::ParticleEmitter::genEmissionTTL(void)
// IDA 0xd386f8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d386f8() {
}

// 0xd3873c — __ZN4Ogre15ParticleEmitter24genConstantEmissionCountEf
#[doc(alias = "Ogre::ParticleEmitter::genConstantEmissionCount(float)")]
// was: Ogre::ParticleEmitter::genConstantEmissionCount(float)
// IDA 0xd3873c: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3873c() {
}

// 0xd3881c — __ZN4Ogre15ParticleEmitter17genEmissionColourERNS_11ColourValueE
#[doc(alias = "Ogre::ParticleEmitter::genEmissionColour(Ogre::ColourValue &)")]
// was: Ogre::ParticleEmitter::genEmissionColour(Ogre::ColourValue &)
// IDA 0xd3881c: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3881c() {
}

// 0xd388f0 — __ZN4Ogre15ParticleEmitter17addBaseParametersEv
#[doc(alias = "Ogre::ParticleEmitter::addBaseParameters(void)")]
// was: Ogre::ParticleEmitter::addBaseParameters(void)
// IDA 0xd388f0: 4957 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d388f0() {
}

// 0xd3c368 — __ZNK4Ogre15ParticleEmitter19getParticleVelocityEv
#[doc(alias = "Ogre::ParticleEmitter::getParticleVelocity(void)const")]
// was: Ogre::ParticleEmitter::getParticleVelocity(void)const
// IDA 0xd3c368: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c368() {
}

// 0xd3c370 — __ZNK4Ogre15ParticleEmitter22getMinParticleVelocityEv
#[doc(alias = "Ogre::ParticleEmitter::getMinParticleVelocity(void)const")]
// was: Ogre::ParticleEmitter::getMinParticleVelocity(void)const
// IDA 0xd3c370: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c370() {
}

// 0xd3c378 — __ZNK4Ogre15ParticleEmitter22getMaxParticleVelocityEv
#[doc(alias = "Ogre::ParticleEmitter::getMaxParticleVelocity(void)const")]
// was: Ogre::ParticleEmitter::getMaxParticleVelocity(void)const
// IDA 0xd3c378: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c378() {
}

// 0xd3c380 — __ZN4Ogre15ParticleEmitter22setMinParticleVelocityEf
#[doc(alias = "Ogre::ParticleEmitter::setMinParticleVelocity(float)")]
// was: Ogre::ParticleEmitter::setMinParticleVelocity(float)
// IDA 0xd3c380: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c380() {
}

// 0xd3c388 — __ZN4Ogre15ParticleEmitter22setMaxParticleVelocityEf
#[doc(alias = "Ogre::ParticleEmitter::setMaxParticleVelocity(float)")]
// was: Ogre::ParticleEmitter::setMaxParticleVelocity(float)
// IDA 0xd3c388: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c388() {
}

// 0xd3c390 — __ZNK4Ogre15ParticleEmitter13getTimeToLiveEv
#[doc(alias = "Ogre::ParticleEmitter::getTimeToLive(void)const")]
// was: Ogre::ParticleEmitter::getTimeToLive(void)const
// IDA 0xd3c390: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c390() {
}

// 0xd3c398 — __ZNK4Ogre15ParticleEmitter16getMinTimeToLiveEv
#[doc(alias = "Ogre::ParticleEmitter::getMinTimeToLive(void)const")]
// was: Ogre::ParticleEmitter::getMinTimeToLive(void)const
// IDA 0xd3c398: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c398() {
}

// 0xd3c3a0 — __ZNK4Ogre15ParticleEmitter16getMaxTimeToLiveEv
#[doc(alias = "Ogre::ParticleEmitter::getMaxTimeToLive(void)const")]
// was: Ogre::ParticleEmitter::getMaxTimeToLive(void)const
// IDA 0xd3c3a0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3a0() {
}

// 0xd3c3a8 — __ZN4Ogre15ParticleEmitter16setMinTimeToLiveEf
#[doc(alias = "Ogre::ParticleEmitter::setMinTimeToLive(float)")]
// was: Ogre::ParticleEmitter::setMinTimeToLive(float)
// IDA 0xd3c3a8: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3a8() {
}

// 0xd3c3b0 — __ZN4Ogre15ParticleEmitter16setMaxTimeToLiveEf
#[doc(alias = "Ogre::ParticleEmitter::setMaxTimeToLive(float)")]
// was: Ogre::ParticleEmitter::setMaxTimeToLive(float)
// IDA 0xd3c3b0: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3b0() {
}

// 0xd3c3b8 — __ZNK4Ogre15ParticleEmitter9getColourEv
#[doc(alias = "Ogre::ParticleEmitter::getColour(void)const")]
// was: Ogre::ParticleEmitter::getColour(void)const
// IDA 0xd3c3b8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3b8() {
}

// 0xd3c3bc — __ZNK4Ogre15ParticleEmitter19getColourRangeStartEv
#[doc(alias = "Ogre::ParticleEmitter::getColourRangeStart(void)const")]
// was: Ogre::ParticleEmitter::getColourRangeStart(void)const
// IDA 0xd3c3bc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3bc() {
}

// 0xd3c3c0 — __ZNK4Ogre15ParticleEmitter17getColourRangeEndEv
#[doc(alias = "Ogre::ParticleEmitter::getColourRangeEnd(void)const")]
// was: Ogre::ParticleEmitter::getColourRangeEnd(void)const
// IDA 0xd3c3c0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3c0() {
}

// 0xd3c3c4 — __ZN4Ogre15ParticleEmitter19setColourRangeStartERKNS_11ColourValueE
#[doc(alias = "Ogre::ParticleEmitter::setColourRangeStart(Ogre::ColourValue const&)")]
// was: Ogre::ParticleEmitter::setColourRangeStart(Ogre::ColourValue const&)
// IDA 0xd3c3c4: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3c4() {
}

// 0xd3c3d0 — __ZN4Ogre15ParticleEmitter17setColourRangeEndERKNS_11ColourValueE
#[doc(alias = "Ogre::ParticleEmitter::setColourRangeEnd(Ogre::ColourValue const&)")]
// was: Ogre::ParticleEmitter::setColourRangeEnd(Ogre::ColourValue const&)
// IDA 0xd3c3d0: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3d0() {
}

// 0xd3c3dc — __ZN4Ogre15ParticleEmitter10setEnabledEb
#[doc(alias = "Ogre::ParticleEmitter::setEnabled(bool)")]
// was: Ogre::ParticleEmitter::setEnabled(bool)
// IDA 0xd3c3dc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c3dc() {
}

// 0xd3c440 — __ZNK4Ogre15ParticleEmitter10getEnabledEv
#[doc(alias = "Ogre::ParticleEmitter::getEnabled(void)const")]
// was: Ogre::ParticleEmitter::getEnabled(void)const
// IDA 0xd3c440: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c440() {
}

// 0xd3c448 — __ZN4Ogre15ParticleEmitter12setStartTimeEf
#[doc(alias = "Ogre::ParticleEmitter::setStartTime(float)")]
// was: Ogre::ParticleEmitter::setStartTime(float)
// IDA 0xd3c448: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c448() {
}

// 0xd3c464 — __ZNK4Ogre15ParticleEmitter12getStartTimeEv
#[doc(alias = "Ogre::ParticleEmitter::getStartTime(void)const")]
// was: Ogre::ParticleEmitter::getStartTime(void)const
// IDA 0xd3c464: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d3c464() {
}