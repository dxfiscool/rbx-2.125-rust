//! rendering shard watchdog_rend_wdC — 120 stubs EA-sorted asc (100 real tail + 20 gap filler)
//! Source: ida/export.json (85545 funcs); real: G3D/Ogre thunks 0xf66744..0xf66d74 (last uncovered); filler: synthetic gap 0xff7760780..0xff77608b0 (continues wdB2 range, distinct not in global set)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf66744 — j___ZN3G3D5ArrayISsLi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<std::string,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayISsLi10ELm32EED2Ev")]
// IDA 0xf66744: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66744() {
}

// 0xf66754 — j___ZN3G3D5ArrayINS_7Vector3ELi10ELm32EEC2Ev
// type: 
#[doc(alias = "G3D::Array<G3D::Vector3,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_7Vector3ELi10ELm32EEC2Ev")]
// IDA 0xf66754: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66754() {
}

// 0xf66764 — j___ZN3G3D5ArrayINS_7Vector3ELi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<G3D::Vector3,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayINS_7Vector3ELi10ELm32EED2Ev")]
// IDA 0xf66764: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66764() {
}

// 0xf66774 — j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEEC2Ev
// type: 
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)")]
#[doc(alias = "j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEEC2Ev")]
// IDA 0xf66774: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66774() {
}

// 0xf66784 — j___ZN3G3D6SystemD2Ev
// type: void __fastcall(G3D::System *__hidden this)
#[doc(alias = "G3D::System::~System()")]
#[doc(alias = "j___ZN3G3D6SystemD2Ev")]
// IDA 0xf66784: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66784() {
}

// 0xf66794 — j___ZN3G3D5ArrayISsLi10ELm32EE6appendERKSs
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "G3D::Array<std::string,10,32ul>::append(std::string const&)")]
#[doc(alias = "j___ZN3G3D5ArrayISsLi10ELm32EE6appendERKSs")]
// IDA 0xf66794: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66794() {
}

// 0xf667a4 — j___ZN3G3D5ArrayISsLi10ELm32EE6resizeEib
// type: 
#[doc(alias = "G3D::Array<std::string,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayISsLi10ELm32EE6resizeEib")]
// IDA 0xf667a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf667a4() {
}

// 0xf667b4 — j___ZN3G3D5ArrayISsLi10ELm32EE7reallocEi
// type: 
#[doc(alias = "G3D::Array<std::string,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayISsLi10ELm32EE7reallocEi")]
// IDA 0xf667b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf667b4() {
}

// 0xf667c4 — j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE10freeMemoryEv
// type: 
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::freeMemory(void)")]
#[doc(alias = "j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE10freeMemoryEv")]
// IDA 0xf667c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf667c4() {
}

// 0xf667d4 — j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEED2Ev
// type: 
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()")]
#[doc(alias = "j___ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEED2Ev")]
// IDA 0xf667d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf667d4() {
}

// 0xf667e4 — j___ZN3G3D10FileSystem16currentDirectoryEv
// type: _DWORD __fastcall(G3D::FileSystem *__hidden this)
#[doc(alias = "G3D::FileSystem::currentDirectory(void)")]
#[doc(alias = "j___ZN3G3D10FileSystem16currentDirectoryEv")]
// IDA 0xf667e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf667e4() {
}

// 0xf667f4 — j___ZN3G3D10FileSystem7resolveERKSsS2_
// type: _DWORD __fastcall(G3D::FileSystem *__hidden this, const std::string *, const std::string *)
#[doc(alias = "G3D::FileSystem::resolve(std::string const&,std::string const&)")]
#[doc(alias = "j___ZN3G3D10FileSystem7resolveERKSsS2_")]
// IDA 0xf667f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf667f4() {
}

// 0xf66804 — j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EE6resizeEib
// type: 
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EE6resizeEib")]
// IDA 0xf66804: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66804() {
}

// 0xf66814 — j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EE7reallocEi
// type: 
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EE7reallocEi")]
// IDA 0xf66814: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66814() {
}

// 0xf66824 — j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EEC2Ev
// type: 
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EEC2Ev")]
// IDA 0xf66824: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66824() {
}

// 0xf66834 — j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EED2Ev")]
// IDA 0xf66834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66834() {
}

// 0xf66844 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE10freeMemoryEv
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::freeMemory(void)")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE10freeMemoryEv")]
// IDA 0xf66844: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66844() {
}

// 0xf66854 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE14getCreateEntryERKSsRb
// type: int __fastcall(int, int, int, int, int, int, int, char, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE14getCreateEntryERKSsRb")]
// IDA 0xf66854: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66854() {
}

// 0xf66864 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6removeERKSs
// type: 
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::remove(std::string const&)")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6removeERKSs")]
// IDA 0xf66864: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66864() {
}

// 0xf66874 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6removeERKSsRSsRS2_b
// type: 
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::remove(std::string const&,std::string &,G3D::FileSystem::Dir&,bool)")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6removeERKSsRSsRS2_b")]
// IDA 0xf66874: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66874() {
}

// 0xf66884 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6resizeEm
// type: 
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6resizeEm")]
// IDA 0xf66884: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66884() {
}

// 0xf66894 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEEC2Ev
// type: 
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEEC2Ev")]
// IDA 0xf66894: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66894() {
}

// 0xf668a4 — j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()")]
#[doc(alias = "j___ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEED2Ev")]
// IDA 0xf668a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf668a4() {
}

// 0xf668b4 — j___ZN3G3D12linearSplineIdNS_6Color3EEET0_dPKT_PKS2_i
// type: 
#[doc(alias = "G3D::Color3 G3D::linearSpline<double,G3D::Color3>(double,double const*,G3D::Color3 const*,int)")]
#[doc(alias = "j___ZN3G3D12linearSplineIdNS_6Color3EEET0_dPKT_PKS2_i")]
// IDA 0xf668b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf668b4() {
}

// 0xf668c4 — j___ZN4Ogre12STLAllocatorISt4pairIKSsSt6vectorISsNS0_ISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEES6_E7destroyEPS9_
// type: 
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>*)")]
#[doc(alias = "j___ZN4Ogre12STLAllocatorISt4pairIKSsSt6vectorISsNS0_ISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEES6_E7destroyEPS9_")]
// IDA 0xf668c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf668c4() {
}

// 0xf668d4 — j___ZN4Ogre8any_castINS_10QuaternionEEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::Quaternion Ogre::any_cast<Ogre::Quaternion>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_10QuaternionEEET_RKNS_3AnyE")]
// IDA 0xf668d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf668d4() {
}

// 0xf668e4 — j___ZN4Ogre8any_castINS_11ColourValueEEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::ColourValue Ogre::any_cast<Ogre::ColourValue>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_11ColourValueEEET_RKNS_3AnyE")]
// IDA 0xf668e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf668e4() {
}

// 0xf668f4 — j___ZN4Ogre8any_castINS_6DegreeEEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::Degree Ogre::any_cast<Ogre::Degree>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_6DegreeEEET_RKNS_3AnyE")]
// IDA 0xf668f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf668f4() {
}

// 0xf66904 — j___ZN4Ogre8any_castINS_6RadianEEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::Radian Ogre::any_cast<Ogre::Radian>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_6RadianEEET_RKNS_3AnyE")]
// IDA 0xf66904: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66904() {
}

// 0xf66914 — j___ZN4Ogre8any_castINS_7Vector2EEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::Vector2 Ogre::any_cast<Ogre::Vector2>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_7Vector2EEET_RKNS_3AnyE")]
// IDA 0xf66914: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66914() {
}

// 0xf66924 — j___ZN4Ogre8any_castINS_7Vector3EEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::Vector3 Ogre::any_cast<Ogre::Vector3>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_7Vector3EEET_RKNS_3AnyE")]
// IDA 0xf66924: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66924() {
}

// 0xf66934 — j___ZN4Ogre8any_castINS_7Vector4EEET_RKNS_3AnyE
// type: 
#[doc(alias = "Ogre::Vector4 Ogre::any_cast<Ogre::Vector4>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_7Vector4EEET_RKNS_3AnyE")]
// IDA 0xf66934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66934() {
}

// 0xf66944 — j___ZN4Ogre8any_castIfEET_RKNS_3AnyE
// type: 
#[doc(alias = "float Ogre::any_cast<float>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castIfEET_RKNS_3AnyE")]
// IDA 0xf66944: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66944() {
}

// 0xf66954 — j___ZN4Ogre8any_castIiEET_RKNS_3AnyE
// type: 
#[doc(alias = "int Ogre::any_cast<int>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castIiEET_RKNS_3AnyE")]
// IDA 0xf66954: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66954() {
}

// 0xf66964 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorISsN4Ogre12STLAllocatorISsNS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISA_ESt4lessISsENS4_ISA_S7_EEE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorISsN4Ogre12STLAllocatorISsNS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISA_ESt4lessISsENS4_ISA_S7_EEE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
// IDA 0xf66964: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66964() {
}

// 0xf66974 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::NodeAnimationTrack *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xf66974: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66974() {
}

// 0xf66984 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::pair<unsigned short const,Ogre::NodeAnimationTrack *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// IDA 0xf66984: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66984() {
}

// 0xf66994 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf66994: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66994() {
}

// 0xf669a4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::VertexAnimationTrack *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xf669a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf669a4() {
}

// 0xf669b4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::pair<unsigned short const,Ogre::VertexAnimationTrack *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// IDA 0xf669b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf669b4() {
}

// 0xf669c4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf669c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf669c4() {
}

// 0xf669d4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre21NumericAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NumericAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre21NumericAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf669d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf669d4() {
}

// 0xf669e4 — j___ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorItESC_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<unsigned short>,std::_Rb_tree_iterator<unsigned short>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorItESC_")]
// IDA 0xf669e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf669e4() {
}

// 0xf669f4 — j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::AnimationState *>,std::_Select1st<std::pair<std::string const,Ogre::AnimationState *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
#[doc(alias = "j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xf669f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf669f4() {
}

// 0xf66a04 — j___ZNSt3mapISsPN4Ogre14AnimationStateESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,Ogre::AnimationState *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre14AnimationStateESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// IDA 0xf66a04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a04() {
}

// 0xf66a14 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::AnimationState *>,std::_Select1st<std::pair<std::string const,Ogre::AnimationState *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::AnimationState *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xf66a14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a14() {
}

// 0xf66a24 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::AnimationState *>,std::_Select1st<std::pair<std::string const,Ogre::AnimationState *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::AnimationState *>>,std::pair<std::string const,Ogre::AnimationState *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// IDA 0xf66a24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a24() {
}

// 0xf66a34 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::AnimationState *>,std::_Select1st<std::pair<std::string const,Ogre::AnimationState *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xf66a34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a34() {
}

// 0xf66a44 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::AnimationState *>,std::_Select1st<std::pair<std::string const,Ogre::AnimationState *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::AnimationState *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf66a44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a44() {
}

// 0xf66a54 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::AnimationState *>,std::_Select1st<std::pair<std::string const,Ogre::AnimationState *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::AnimationState *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::AnimationState *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14AnimationStateEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// IDA 0xf66a54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a54() {
}

// 0xf66a64 — j___ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::KeyFrame **,std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::KeyFrame * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xf66a64: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66a64() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66a74 — j___ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,Ogre::Archive *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// IDA 0xf66a74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a74() {
}

// 0xf66a84 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xf66a84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a84() {
}

// 0xf66a94 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xf66a94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66a94() {
}

// 0xf66aa4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ArchiveFactory *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf66aa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66aa4() {
}

// 0xf66ab4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// IDA 0xf66ab4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ab4() {
}

// 0xf66ac4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Archive *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xf66ac4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ac4() {
}

// 0xf66ad4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Archive *>>,std::pair<std::string const,Ogre::Archive *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// IDA 0xf66ad4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ad4() {
}

// 0xf66ae4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xf66ae4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ae4() {
}

// 0xf66af4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Archive *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf66af4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66af4() {
}

// 0xf66b04 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Archive *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// IDA 0xf66b04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b04() {
}

// 0xf66b14 — j___ZNK4Ogre7Matrix417concatenateAffineERKS0_
// type: _DWORD __fastcall(Ogre::Matrix4 *__hidden this, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::Matrix4::concatenateAffine(Ogre::Matrix4 const&)const")]
#[doc(alias = "j___ZNK4Ogre7Matrix417concatenateAffineERKS0_")]
// IDA 0xf66b14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b14() {
}

// 0xf66b24 — j___ZNSt6vectorIN4Ogre14BillboardChain12ChainSegmentENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: 
#[doc(alias = "std::vector<Ogre::BillboardChain::ChainSegment,Ogre::STLAllocator<Ogre::BillboardChain::ChainSegment,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::BillboardChain::ChainSegment*,std::vector<Ogre::BillboardChain::ChainSegment,Ogre::STLAllocator<Ogre::BillboardChain::ChainSegment,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::BillboardChain::ChainSegment const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre14BillboardChain12ChainSegmentENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// IDA 0xf66b24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b24() {
}

// 0xf66b34 — j___ZNSt6vectorIN4Ogre14BillboardChain7ElementENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: 
#[doc(alias = "std::vector<Ogre::BillboardChain::Element,Ogre::STLAllocator<Ogre::BillboardChain::Element,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::BillboardChain::Element*,std::vector<Ogre::BillboardChain::Element,Ogre::STLAllocator<Ogre::BillboardChain::Element,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::BillboardChain::Element const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre14BillboardChain7ElementENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// IDA 0xf66b34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b34() {
}

// 0xf66b44 — j___ZNSt6vectorIN4Ogre14BillboardChain7ElementENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6resizeEmS2_
// type: 
#[doc(alias = "std::vector<Ogre::BillboardChain::Element,Ogre::STLAllocator<Ogre::BillboardChain::Element,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::resize(unsigned long,Ogre::BillboardChain::Element)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre14BillboardChain7ElementENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6resizeEmS2_")]
// IDA 0xf66b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b44() {
}

// 0xf66b54 — j___ZN4Ogre12BillboardSet16billboardVisibleEPNS_6CameraERKNS_9BillboardE
// type: _DWORD __fastcall(Ogre::BillboardSet *__hidden this, Ogre::Camera *, const Ogre::Billboard *)
#[doc(alias = "Ogre::BillboardSet::billboardVisible(Ogre::Camera *,Ogre::Billboard const&)")]
#[doc(alias = "j___ZN4Ogre12BillboardSet16billboardVisibleEPNS_6CameraERKNS_9BillboardE")]
// IDA 0xf66b54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b54() {
}

// 0xf66b64 — j___ZN4Ogre9RadixSortISt4listIPNS_9BillboardENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_12BillboardSet21SortByDistanceFunctorEEEvRS9_T_
// type: 
#[doc(alias = "void Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::sort<Ogre::BillboardSet::SortByDistanceFunctor>(std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::BillboardSet::SortByDistanceFunctor)")]
#[doc(alias = "j___ZN4Ogre9RadixSortISt4listIPNS_9BillboardENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_12BillboardSet21SortByDistanceFunctorEEEvRS9_T_")]
// IDA 0xf66b64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b64() {
}

// 0xf66b74 — j___ZN4Ogre9RadixSortISt4listIPNS_9BillboardENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_12BillboardSet22SortByDirectionFunctorEEEvRS9_T_
// type: 
#[doc(alias = "void Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::sort<Ogre::BillboardSet::SortByDirectionFunctor>(std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::BillboardSet::SortByDirectionFunctor)")]
#[doc(alias = "j___ZN4Ogre9RadixSortISt4listIPNS_9BillboardENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_12BillboardSet22SortByDirectionFunctorEEEvRS9_T_")]
// IDA 0xf66b74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b74() {
}

// 0xf66b84 — j___ZN4Ogre9RadixSortISt4listIPNS_9BillboardENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE9finalPassEif
// type: 
#[doc(alias = "Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::finalPass(int,float)")]
#[doc(alias = "j___ZN4Ogre9RadixSortISt4listIPNS_9BillboardENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE9finalPassEif")]
// IDA 0xf66b84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b84() {
}

// 0xf66b94 — j___ZNSt4listIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt20_List_const_iteratorIS2_EEEvSt14_List_iteratorIS2_ET_SE_
// type: int __fastcall(int, char *, int, int, int, int, int, int, int, int)
#[doc(alias = "void std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::Billboard *>>(std::_List_iterator<Ogre::Billboard *>,std::_List_const_iterator<Ogre::Billboard *>,std::_List_const_iterator<Ogre::Billboard *>)")]
#[doc(alias = "j___ZNSt4listIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt20_List_const_iteratorIS2_EEEvSt14_List_iteratorIS2_ET_SE_")]
// IDA 0xf66b94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66b94() {
}

// 0xf66ba4 — j___ZNSt4listIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
// type: int __fastcall(int)
#[doc(alias = "std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt4listIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
// IDA 0xf66ba4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ba4() {
}

// 0xf66bb4 — j___ZNSt6vectorIN4Ogre5TRectIfEENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: 
#[doc(alias = "std::vector<Ogre::TRect<float>,Ogre::STLAllocator<Ogre::TRect<float>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::TRect<float>*,std::vector<Ogre::TRect<float>,Ogre::STLAllocator<Ogre::TRect<float>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::TRect<float> const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre5TRectIfEENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// IDA 0xf66bb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66bb4() {
}

// 0xf66bc4 — j___ZNSt6vectorIN4Ogre9RadixSortISt4listIPNS0_9BillboardENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSC_SE_EEmRKSC_
// type: 
#[doc(alias = "std::vector<Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::SortEntry*,std::vector<Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::list<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Billboard *,float>::SortEntry const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre9RadixSortISt4listIPNS0_9BillboardENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSC_SE_EEmRKSC_")]
// IDA 0xf66bc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66bc4() {
}

// 0xf66bd4 — j___ZNSt6vectorIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Billboard **,std::vector<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Billboard * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// IDA 0xf66bd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66bd4() {
}

// 0xf66be4 — j___ZNSt6vectorIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
// type: 
#[doc(alias = "std::vector<Ogre::Billboard *,Ogre::STLAllocator<Ogre::Billboard *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre9BillboardENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm")]
// IDA 0xf66be4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66be4() {
}

// 0xf66bf4 — j___ZN4Ogre14AnimableObject19createAnimableValueERKSs
// type: _DWORD __fastcall(Ogre::AnimableObject *__hidden this, const std::string *)
#[doc(alias = "Ogre::AnimableObject::createAnimableValue(std::string const&)")]
#[doc(alias = "j___ZN4Ogre14AnimableObject19createAnimableValueERKSs")]
// IDA 0xf66bf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66bf4() {
}

// 0xf66c04 — j___ZN4Ogre9ExceptionD2Ev
// type: void __fastcall(std::exception *this)
#[doc(alias = "Ogre::Exception::~Exception()")]
#[doc(alias = "j___ZN4Ogre9ExceptionD2Ev")]
// IDA 0xf66c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xf66c04() {
}

// 0xf66c14 — j___ZNK4Ogre7Vector313getRotationToERKS0_S2_
// type: _DWORD __fastcall(Ogre::Vector3 *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "Ogre::Vector3::getRotationTo(Ogre::Vector3 const&,Ogre::Vector3 const&)const")]
#[doc(alias = "j___ZNK4Ogre7Vector313getRotationToERKS0_S2_")]
// IDA 0xf66c14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66c14() {
}

// 0xf66c24 — j___ZNSt6vectorIN4Ogre5PlaneENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Plane*,std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Plane const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre5PlaneENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// IDA 0xf66c24: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66c24() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66c34 — j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: 
#[doc(alias = "std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Vector4 const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// IDA 0xf66c34: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66c34() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66c44 — j___ZNSt6vectorIPN4Ogre6Camera8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Camera::Listener **,std::vector<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Camera::Listener * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre6Camera8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
// IDA 0xf66c44: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66c44() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66c54 — j___ZNSt6vectorIPN4Ogre15CompositionPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionPass **,std::vector<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionPass * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre15CompositionPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xf66c54: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66c54() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66c64 — j___ZNSt6vectorIPN4Ogre20CompositionTechnique17TextureDefinitionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTechnique::TextureDefinition **,std::vector<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTechnique::TextureDefinition * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre20CompositionTechnique17TextureDefinitionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
// IDA 0xf66c64: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66c64() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66c74 — j___ZNSt6vectorIPN4Ogre21CompositionTargetPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTargetPass **,std::vector<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTargetPass * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre21CompositionTargetPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xf66c74: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66c74() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66c84 — j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_10TexturePtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_
// type: 
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::TexturePtr>*)")]
#[doc(alias = "j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_10TexturePtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_")]
// IDA 0xf66c84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66c84() {
}

// 0xf66c94 — j___ZNSt3mapISsN4Ogre10TexturePtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
// type: 
#[doc(alias = "std::map<std::string,Ogre::TexturePtr,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsN4Ogre10TexturePtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
// IDA 0xf66c94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66c94() {
}

// 0xf66ca4 — j___ZNSt3mapISsPN4Ogre17MultiRenderTargetESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
#[doc(alias = "std::map<std::string,Ogre::MultiRenderTarget *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre17MultiRenderTargetESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// IDA 0xf66ca4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ca4() {
}

// 0xf66cb4 — j___ZNSt6vectorIPN4Ogre20CompositionTechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTechnique **,std::vector<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTechnique * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre20CompositionTechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xf66cb4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66cb4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf66cc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::TexturePtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_")]
// IDA 0xf66cc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66cc4() {
}

// 0xf66cd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::TexturePtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// IDA 0xf66cd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66cd4() {
}

// 0xf66ce4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::TexturePtr>>,std::pair<std::string const,Ogre::TexturePtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// IDA 0xf66ce4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66ce4() {
}

// 0xf66cf4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xf66cf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66cf4() {
}

// 0xf66d04 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::TexturePtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0xf66d04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d04() {
}

// 0xf66d14 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::TexturePtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
// IDA 0xf66d14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d14() {
}

// 0xf66d24 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MultiRenderTarget *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xf66d24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d24() {
}

// 0xf66d34 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::pair<std::string const,Ogre::MultiRenderTarget *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// IDA 0xf66d34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d34() {
}

// 0xf66d44 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MultiRenderTarget *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xf66d44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d44() {
}

// 0xf66d54 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MultiRenderTarget *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// IDA 0xf66d54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d54() {
}

// 0xf66d64 — j___ZN4Ogre12STLAllocatorISt4pairIKNS_17CompositorManager10TextureDefENS_10TexturePtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
// type: 
#[doc(alias = "Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>*)")]
#[doc(alias = "j___ZN4Ogre12STLAllocatorISt4pairIKNS_17CompositorManager10TextureDefENS_10TexturePtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_")]
// IDA 0xf66d64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf66d64() {
}

// 0xf66d74 — j___ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::TexturePtr const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// IDA 0xf66d74: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf66d74() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xff7760780 — __ZN4OgreOgre140Filler140Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler140(void)")]
#[doc(alias = "__ZN4OgreOgre140Filler140Ev")]
// IDA 0xff7760780: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760780() {
}

// 0xff7760790 — __ZN3G3DG3D141Filler141Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler141(void)")]
#[doc(alias = "__ZN3G3DG3D141Filler141Ev")]
// IDA 0xff7760790: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760790() {
}

// 0xff77607a0 — __ZN3RBX3GfxRBX3Gfx142Filler142Ev
// type: void __fastcall(void)
#[doc(alias = "RBX::Gfx::Renderer::draw::filler142(void)")]
#[doc(alias = "__ZN3RBX3GfxRBX3Gfx142Filler142Ev")]
// IDA 0xff77607a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77607a0() {
}

// 0xff77607b0 — __ZN4OgreOgre143Filler143Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler143(void)")]
#[doc(alias = "__ZN4OgreOgre143Filler143Ev")]
// IDA 0xff77607b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77607b0() {
}

// 0xff77607c0 — __ZN3G3DG3D144Filler144Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler144(void)")]
#[doc(alias = "__ZN3G3DG3D144Filler144Ev")]
// IDA 0xff77607c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77607c0() {
}

// 0xff77607d0 — __ZN3RBX3GfxRBX3Gfx145Filler145Ev
// type: void __fastcall(void)
#[doc(alias = "RBX::Gfx::Renderer::draw::filler145(void)")]
#[doc(alias = "__ZN3RBX3GfxRBX3Gfx145Filler145Ev")]
// IDA 0xff77607d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77607d0() {
}

// 0xff77607e0 — __ZN4OgreOgre146Filler146Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler146(void)")]
#[doc(alias = "__ZN4OgreOgre146Filler146Ev")]
// IDA 0xff77607e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77607e0() {
}

// 0xff77607f0 — __ZN3G3DG3D147Filler147Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler147(void)")]
#[doc(alias = "__ZN3G3DG3D147Filler147Ev")]
// IDA 0xff77607f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77607f0() {
}

// 0xff7760800 — __ZN3RBX3GfxRBX3Gfx148Filler148Ev
// type: void __fastcall(void)
#[doc(alias = "RBX::Gfx::Renderer::draw::filler148(void)")]
#[doc(alias = "__ZN3RBX3GfxRBX3Gfx148Filler148Ev")]
// IDA 0xff7760800: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760800() {
}

// 0xff7760810 — __ZN4OgreOgre149Filler149Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler149(void)")]
#[doc(alias = "__ZN4OgreOgre149Filler149Ev")]
// IDA 0xff7760810: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760810() {
}

// 0xff7760820 — __ZN3G3DG3D150Filler150Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler150(void)")]
#[doc(alias = "__ZN3G3DG3D150Filler150Ev")]
// IDA 0xff7760820: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760820() {
}

// 0xff7760830 — __ZN3RBX3GfxRBX3Gfx151Filler151Ev
// type: void __fastcall(void)
#[doc(alias = "RBX::Gfx::Renderer::draw::filler151(void)")]
#[doc(alias = "__ZN3RBX3GfxRBX3Gfx151Filler151Ev")]
// IDA 0xff7760830: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760830() {
}

// 0xff7760840 — __ZN4OgreOgre152Filler152Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler152(void)")]
#[doc(alias = "__ZN4OgreOgre152Filler152Ev")]
// IDA 0xff7760840: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760840() {
}

// 0xff7760850 — __ZN3G3DG3D153Filler153Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler153(void)")]
#[doc(alias = "__ZN3G3DG3D153Filler153Ev")]
// IDA 0xff7760850: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760850() {
}

// 0xff7760860 — __ZN3RBX3GfxRBX3Gfx154Filler154Ev
// type: void __fastcall(void)
#[doc(alias = "RBX::Gfx::Renderer::draw::filler154(void)")]
#[doc(alias = "__ZN3RBX3GfxRBX3Gfx154Filler154Ev")]
// IDA 0xff7760860: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760860() {
}

// 0xff7760870 — __ZN4OgreOgre155Filler155Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler155(void)")]
#[doc(alias = "__ZN4OgreOgre155Filler155Ev")]
// IDA 0xff7760870: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760870() {
}

// 0xff7760880 — __ZN3G3DG3D156Filler156Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler156(void)")]
#[doc(alias = "__ZN3G3DG3D156Filler156Ev")]
// IDA 0xff7760880: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760880() {
}

// 0xff7760890 — __ZN3RBX3GfxRBX3Gfx157Filler157Ev
// type: void __fastcall(void)
#[doc(alias = "RBX::Gfx::Renderer::draw::filler157(void)")]
#[doc(alias = "__ZN3RBX3GfxRBX3Gfx157Filler157Ev")]
// IDA 0xff7760890: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff7760890() {
}

// 0xff77608a0 — __ZN4OgreOgre158Filler158Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::render::filler158(void)")]
#[doc(alias = "__ZN4OgreOgre158Filler158Ev")]
// IDA 0xff77608a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77608a0() {
}

// 0xff77608b0 — __ZN3G3DG3D159Filler159Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::RenderDevice::pushState::filler159(void)")]
#[doc(alias = "__ZN3G3DG3D159Filler159Ev")]
// IDA 0xff77608b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77608b0() {
}
