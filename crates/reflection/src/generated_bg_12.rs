//! reflection — generated_bg_12 — 150 stubs EA-sorted asc global gap filler 0x1c48c4..0x1cb6e0 not yet in crates/reflection (global 85545 funcs, 59031 gaps reflection before; 26515->26665 distinct)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 150 uncovered for reflection-bg sorted asc after 0x1c48c4
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x1c48c4 — __ZN6TagLib8instanceEv
// type: _DWORD __fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::instance(void)")]
#[doc(alias = "__ZN6TagLib8instanceEv")]
pub fn stub_0x1c48c4() -> ! {
    todo!("0x1c48c4 __ZN6TagLib8instanceEv")
}

// 0x1c49e4 — __ZN6TagLibD2Ev
// type: void __fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::~TagLib()")]
#[doc(alias = "__ZN6TagLibD2Ev")]
pub fn stub_0x1c49e4() {
    // IDA 0x1c49e4: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x1c4b38 — ___tcf_0_0
#[doc(alias = "___tcf_0_0")]
pub fn stub_0x1c4b38() -> ! {
    todo!("0x1c4b38 ___tcf_0_0")
}

// 0x1c4b48 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>> const&,std::less<int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_")]
pub fn stub_0x1c4b48() -> ! {
    todo!("0x1c4b48 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_")
}

// 0x1c4b88 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::lower_bound(int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_")]
pub fn stub_0x1c4b88() -> ! {
    todo!("0x1c4b88 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_")
}

// 0x1c4bbc — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_Rb_tree_impl<std::less<unsigned short>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>> const&,std::less<unsigned short> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")]
pub fn stub_0x1c4bbc() -> ! {
    todo!("0x1c4bbc __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")
}

// 0x1c4bfc — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE11lower_boundERS1_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::lower_bound(unsigned short const&)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE11lower_boundERS1_")]
pub fn stub_0x1c4bfc() -> ! {
    todo!("0x1c4bfc __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE11lower_boundERS1_")
}

// 0x1c4c30 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKtP10tagTagInfoEEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKtP10tagTagInfoEEE8allocateEmPKv")]
pub fn stub_0x1c4c30() -> ! {
    todo!("0x1c4c30 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKtP10tagTagInfoEEE8allocateEmPKv")
}

// 0x1c4c60 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_create_node(std::pair<unsigned short const,tagTagInfo *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE14_M_create_nodeERKS4_")]
pub fn stub_0x1c4c60() -> ! {
    todo!("0x1c4c60 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE14_M_create_nodeERKS4_")
}

// 0x1c4c90 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,tagTagInfo *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
pub fn stub_0x1c4c90() -> ! {
    todo!("0x1c4c90 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")
}

// 0x1c4d14 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS2_IKtS6_EEEEEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS2_IKtS6_EEEEEE8allocateEmPKv")]
pub fn stub_0x1c4d14() -> ! {
    todo!("0x1c4d14 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS2_IKtS6_EEEEEE8allocateEmPKv")
}

// 0x1c4d44 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_create_node(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_")]
pub fn stub_0x1c4d44() -> ! {
    todo!("0x1c4d44 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_")
}

// 0x1c4d74 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_")]
pub fn stub_0x1c4d74() -> ! {
    todo!("0x1c4d74 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_")
}

// 0x1c4df8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_")]
pub fn stub_0x1c4df8() -> ! {
    todo!("0x1c4df8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_")
}

// 0x1c4eb8 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_0x1c4eb8() -> ! {
    todo!("0x1c4eb8 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

// 0x1c4ef4 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
pub fn stub_0x1c4ef4() -> ! {
    todo!("0x1c4ef4 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")
}

// 0x1c4f30 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")]
pub fn stub_0x1c4f30() -> ! {
    todo!("0x1c4f30 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")
}

// 0x1c5054 — __ZNSt3mapIiPS_ItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_
#[doc(alias = "std::map<int,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>,tagTagInfo *<int>,std::allocator<std::less<unsigned short><int const,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>>>>::operator[](int const&)")]
#[doc(alias = "__ZNSt3mapIiPS_ItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_")]
pub fn stub_0x1c5054() -> ! {
    todo!("0x1c5054 __ZNSt3mapIiPS_ItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_")
}

// 0x1c50c0 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::pair<unsigned short const,tagTagInfo *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueERKS4_")]
pub fn stub_0x1c50c0() -> ! {
    todo!("0x1c50c0 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueERKS4_")
}

// 0x1c5180 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,tagTagInfo *>>,std::pair<unsigned short const,tagTagInfo *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
pub fn stub_0x1c5180() -> ! {
    todo!("0x1c5180 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")
}

// 0x1c52a4 — __ZNSt3mapItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEEixERS5_
// type: int __fastcall(int, unsigned __int16 *)
#[doc(alias = "std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::operator[](unsigned short const&)")]
#[doc(alias = "__ZNSt3mapItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEEixERS5_")]
pub fn stub_0x1c52a4() -> ! {
    todo!("0x1c52a4 __ZNSt3mapItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEEixERS5_")
}

// 0x1c5310 — __Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj
#[doc(alias = "tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")]
#[doc(alias = "__Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj")]
pub fn stub_0x1c5310() -> ! {
    todo!("0x1c5310 __Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj")
}

// 0x1c59bc — __Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP
#[doc(alias = "tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")]
#[doc(alias = "__Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP")]
pub fn stub_0x1c59bc() -> ! {
    todo!("0x1c59bc __Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP")
}

// 0x1c5bf8 — __Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP
#[doc(alias = "tiff_write_geotiff_profile(tiff *,FIBITMAP *)")]
#[doc(alias = "__Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP")]
pub fn stub_0x1c5bf8() -> ! {
    todo!("0x1c5bf8 __Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP")
}

// 0x1c610c — __Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP
#[doc(alias = "tiff_read_geotiff_profile(tiff *,FIBITMAP *)")]
#[doc(alias = "__Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP")]
pub fn stub_0x1c610c() -> ! {
    todo!("0x1c610c __Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP")
}

// 0x1c630c — __Z15XTIFFInitializev
// type: _DWORD __fastcall()
#[doc(alias = "XTIFFInitialize(void)")]
#[doc(alias = "__Z15XTIFFInitializev")]
pub fn stub_0x1c630c() -> ! {
    todo!("0x1c630c __Z15XTIFFInitializev")
}

// 0x1c6354 — __ZL22_XTIFFDefaultDirectoryP4tiff
#[doc(alias = "_XTIFFDefaultDirectory(tiff *)")]
#[doc(alias = "__ZL22_XTIFFDefaultDirectoryP4tiff")]
pub fn stub_0x1c6354() -> ! {
    todo!("0x1c6354 __ZL22_XTIFFDefaultDirectoryP4tiff")
}

// 0x1c6394 — __ZL15append_iptc_tagPhPjtjPKv
// type: _DWORD __fastcall(unsigned __int8 *, unsigned int *, unsigned __int16, unsigned int, const void *__src)
#[doc(alias = "append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")]
#[doc(alias = "__ZL15append_iptc_tagPhPjtjPKv")]
pub fn stub_0x1c6394() -> ! {
    todo!("0x1c6394 __ZL15append_iptc_tagPhPjtjPKv")
}

// 0x1c6448 — _write_iptc_profile
#[doc(alias = "_write_iptc_profile")]
pub fn stub_0x1c6448() -> ! {
    todo!("0x1c6448 _write_iptc_profile")
}

// 0x1c6910 — _read_iptc_profile
#[doc(alias = "_read_iptc_profile")]
pub fn stub_0x1c6910() -> ! {
    todo!("0x1c6910 _read_iptc_profile")
}

// 0x1c7340 — __ZNKSt6vectorISsSaISsEE4sizeEv
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::size(void)const")]
#[doc(alias = "__ZNKSt6vectorISsSaISsEE4sizeEv")]
pub fn stub_0x1c7340() -> ! {
    todo!("0x1c7340 __ZNKSt6vectorISsSaISsEE4sizeEv")
}

// 0x1c7350 — _FreeImage_GetTagKey
#[doc(alias = "_FreeImage_GetTagKey")]
pub fn stub_0x1c7350() -> ! {
    todo!("0x1c7350 _FreeImage_GetTagKey")
}

// 0x1c7360 — _FreeImage_GetTagID
// type: int __fastcall(int result)
#[doc(alias = "_FreeImage_GetTagID")]
pub fn stub_0x1c7360() -> ! {
    todo!("0x1c7360 _FreeImage_GetTagID")
}

// 0x1c7370 — _FreeImage_GetTagType
#[doc(alias = "_FreeImage_GetTagType")]
pub fn stub_0x1c7370() -> ! {
    todo!("0x1c7370 _FreeImage_GetTagType")
}

// 0x1c7380 — _FreeImage_GetTagCount
#[doc(alias = "_FreeImage_GetTagCount")]
pub fn stub_0x1c7380() -> ! {
    todo!("0x1c7380 _FreeImage_GetTagCount")
}

// 0x1c7390 — _FreeImage_GetTagLength
#[doc(alias = "_FreeImage_GetTagLength")]
pub fn stub_0x1c7390() -> ! {
    todo!("0x1c7390 _FreeImage_GetTagLength")
}

// 0x1c73a0 — _FreeImage_GetTagValue
#[doc(alias = "_FreeImage_GetTagValue")]
pub fn stub_0x1c73a0() -> ! {
    todo!("0x1c73a0 _FreeImage_GetTagValue")
}

// 0x1c73b0 — _FreeImage_SetTagID
#[doc(alias = "_FreeImage_SetTagID")]
pub fn stub_0x1c73b0() -> ! {
    todo!("0x1c73b0 _FreeImage_SetTagID")
}

// 0x1c73c8 — _FreeImage_SetTagType
#[doc(alias = "_FreeImage_SetTagType")]
pub fn stub_0x1c73c8() -> ! {
    todo!("0x1c73c8 _FreeImage_SetTagType")
}

// 0x1c73dc — _FreeImage_SetTagCount
#[doc(alias = "_FreeImage_SetTagCount")]
pub fn stub_0x1c73dc() -> ! {
    todo!("0x1c73dc _FreeImage_SetTagCount")
}

// 0x1c73f0 — _FreeImage_SetTagLength
#[doc(alias = "_FreeImage_SetTagLength")]
pub fn stub_0x1c73f0() -> ! {
    todo!("0x1c73f0 _FreeImage_SetTagLength")
}

// 0x1c7404 — __Z22FreeImage_TagDataWidtht
// type: _DWORD __fastcall(unsigned __int16)
#[doc(alias = "FreeImage_TagDataWidth(unsigned short)")]
#[doc(alias = "__Z22FreeImage_TagDataWidtht")]
pub fn stub_0x1c7404() -> ! {
    todo!("0x1c7404 __Z22FreeImage_TagDataWidtht")
}

// 0x1c7428 — _FreeImage_DeleteTag
#[doc(alias = "_FreeImage_DeleteTag")]
pub fn stub_0x1c7428() -> ! {
    todo!("0x1c7428 _FreeImage_DeleteTag")
}

// 0x1c7470 — _FreeImage_SetTagDescription
#[doc(alias = "_FreeImage_SetTagDescription")]
pub fn stub_0x1c7470() -> ! {
    todo!("0x1c7470 _FreeImage_SetTagDescription")
}

// 0x1c74cc — _FreeImage_SetTagKey
#[doc(alias = "_FreeImage_SetTagKey")]
pub fn stub_0x1c74cc() -> ! {
    todo!("0x1c74cc _FreeImage_SetTagKey")
}

// 0x1c7528 — _FreeImage_CreateTag
#[doc(alias = "_FreeImage_CreateTag")]
pub fn stub_0x1c7528() -> ! {
    todo!("0x1c7528 _FreeImage_CreateTag")
}

// 0x1c7580 — _FreeImage_CloneTag
#[doc(alias = "_FreeImage_CloneTag")]
pub fn stub_0x1c7580() -> ! {
    todo!("0x1c7580 _FreeImage_CloneTag")
}

// 0x1c7658 — _FreeImage_SetTagValue
#[doc(alias = "_FreeImage_SetTagValue")]
pub fn stub_0x1c7658() -> ! {
    todo!("0x1c7658 _FreeImage_SetTagValue")
}

// 0x1c7724 — __ZN10FIRationalD1Ev
// type: void __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::~FIRational()")]
#[doc(alias = "__ZN10FIRationalD1Ev")]
pub fn stub_0x1c7724() {
    // IDA 0x1c7724: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x1c7728 — __ZN10FIRational12getNumeratorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::getNumerator(void)")]
#[doc(alias = "__ZN10FIRational12getNumeratorEv")]
pub fn stub_0x1c7728() -> ! {
    todo!("0x1c7728 __ZN10FIRational12getNumeratorEv")
}

// 0x1c7730 — __ZN10FIRational14getDenominatorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::getDenominator(void)")]
#[doc(alias = "__ZN10FIRational14getDenominatorEv")]
pub fn stub_0x1c7730() -> ! {
    todo!("0x1c7730 __ZN10FIRational14getDenominatorEv")
}

// 0x1c7738 — __ZN10FIRationalC2Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
#[doc(alias = "FIRational::FIRational(float)")]
#[doc(alias = "__ZN10FIRationalC2Ef")]
pub fn stub_0x1c7738() -> ! {
    todo!("0x1c7738 __ZN10FIRationalC2Ef")
}

// 0x1c7988 — __ZN10FIRationalC1Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
#[doc(alias = "FIRational::FIRational(float)")]
#[doc(alias = "__ZN10FIRationalC1Ef")]
pub fn stub_0x1c7988() -> ! {
    todo!("0x1c7988 __ZN10FIRationalC1Ef")
}

// 0x1c798c — __ZL9ReadInt32iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadInt32(int,void *)")]
#[doc(alias = "__ZL9ReadInt32iPv")]
pub fn stub_0x1c798c() -> ! {
    todo!("0x1c798c __ZL9ReadInt32iPv")
}

// 0x1c79d8 — __ZL10ReadUint16iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadUint16(int,void *)")]
#[doc(alias = "__ZL10ReadUint16iPv")]
pub fn stub_0x1c79d8() -> ! {
    todo!("0x1c79d8 __ZL10ReadUint16iPv")
}

// 0x1c79f8 — __ZL10ReadUint32iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadUint32(int,void *)")]
#[doc(alias = "__ZL10ReadUint32iPv")]
pub fn stub_0x1c79f8() -> ! {
    todo!("0x1c79f8 __ZL10ReadUint32iPv")
}

// 0x1c79fc — __ZL18FreeImage_strnicmpPKcS0_m
// type: _DWORD __fastcall(const char *, const char *, unsigned int)
#[doc(alias = "FreeImage_strnicmp(char const*,char const*,unsigned long)")]
#[doc(alias = "__ZL18FreeImage_strnicmpPKcS0_m")]
pub fn stub_0x1c79fc() -> ! {
    todo!("0x1c79fc __ZL18FreeImage_strnicmpPKcS0_m")
}

// 0x1c7d28 — __ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE
#[doc(alias = "processExifTag(FIBITMAP *,FITAG *,char *,int,TagLib::MDMODEL)")]
#[doc(alias = "__ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE")]
pub fn stub_0x1c7d28() -> ! {
    todo!("0x1c7d28 __ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE")
}

// 0x1c81a4 — _jpeg_read_exif_profile
#[doc(alias = "_jpeg_read_exif_profile")]
pub fn stub_0x1c81a4() -> ! {
    todo!("0x1c81a4 _jpeg_read_exif_profile")
}

// 0x1c8d60 — __ZSt16__deque_buf_sizem
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "std::__deque_buf_size(unsigned long)")]
#[doc(alias = "__ZSt16__deque_buf_sizem")]
pub fn stub_0x1c8d60() -> ! {
    todo!("0x1c8d60 __ZSt16__deque_buf_sizem")
}

// 0x1c8d84 — __ZNSt5dequeItSaItEE15_M_destroy_dataESt15_Deque_iteratorItRtPtES5_RKS0_
// type: void()
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short> const&)")]
#[doc(alias = "__ZNSt5dequeItSaItEE15_M_destroy_dataESt15_Deque_iteratorItRtPtES5_RKS0_")]
pub fn stub_0x1c8d84() -> ! {
    todo!("0x1c8d84 __ZNSt5dequeItSaItEE15_M_destroy_dataESt15_Deque_iteratorItRtPtES5_RKS0_")
}

// 0x1c8d88 — __ZNSt5dequeIPhSaIS0_EE15_M_destroy_dataESt15_Deque_iteratorIS0_RS0_PS0_ES6_RKS1_
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_data(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *> const&)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE15_M_destroy_dataESt15_Deque_iteratorIS0_RS0_PS0_ES6_RKS1_")]
pub fn stub_0x1c8d88() -> ! {
    todo!("0x1c8d88 __ZNSt5dequeIPhSaIS0_EE15_M_destroy_dataESt15_Deque_iteratorIS0_RS0_PS0_ES6_RKS1_")
}

// 0x1c8d8c — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE4findERS1_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::find(unsigned int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE4findERS1_")]
pub fn stub_0x1c8d8c() -> ! {
    todo!("0x1c8d8c __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE4findERS1_")
}

// 0x1c8de8 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE13_Rb_tree_implIS6_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS2_EERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_Rb_tree_impl<std::less<unsigned int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>> const&,std::less<unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE13_Rb_tree_implIS6_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS2_EERKS6_")]
pub fn stub_0x1c8de8() -> ! {
    todo!("0x1c8de8 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE13_Rb_tree_implIS6_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS2_EERKS6_")
}

// 0x1c8e28 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE11lower_boundERS1_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::lower_bound(unsigned int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE11lower_boundERS1_")]
pub fn stub_0x1c8e28() -> ! {
    todo!("0x1c8e28 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE11lower_boundERS1_")
}

// 0x1c8e5c — __ZNSt15_Deque_iteratorItRtPtE11_M_set_nodeEPS1_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::_M_set_node(unsigned short **)")]
#[doc(alias = "__ZNSt15_Deque_iteratorItRtPtE11_M_set_nodeEPS1_")]
pub fn stub_0x1c8e5c() -> ! {
    todo!("0x1c8e5c __ZNSt15_Deque_iteratorItRtPtE11_M_set_nodeEPS1_")
}

// 0x1c8e8c — __ZNSt15_Deque_iteratorItRtPtEmmEv
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator--(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorItRtPtEmmEv")]
pub fn stub_0x1c8e8c() -> ! {
    todo!("0x1c8e8c __ZNSt15_Deque_iteratorItRtPtEmmEv")
}

// 0x1c8ecc — __ZStmiIPhRS0_PS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::difference_type std::operator-<unsigned char *,unsigned char *&,unsigned char **>(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&)")]
#[doc(alias = "__ZStmiIPhRS0_PS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_")]
pub fn stub_0x1c8ecc() -> ! {
    todo!("0x1c8ecc __ZStmiIPhRS0_PS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_")
}

// 0x1c8f1c — __ZStmiIN6TagLib7MDMODELERS1_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::difference_type std::operator-<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&)")]
#[doc(alias = "__ZStmiIN6TagLib7MDMODELERS1_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_")]
pub fn stub_0x1c8f1c() -> ! {
    todo!("0x1c8f1c __ZStmiIN6TagLib7MDMODELERS1_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_")
}

// 0x1c8f6c — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERKS1_PS2_EppEv
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>::operator++(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIN6TagLib7MDMODELERKS1_PS2_EppEv")]
pub fn stub_0x1c8f6c() -> ! {
    todo!("0x1c8f6c __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERKS1_PS2_EppEv")
}

// 0x1c8fc4 — __ZNSt15_Deque_iteratorItRtPtEppEv
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator++(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorItRtPtEppEv")]
pub fn stub_0x1c8fc4() -> ! {
    todo!("0x1c8fc4 __ZNSt15_Deque_iteratorItRtPtEppEv")
}

// 0x1c9004 — __ZStmiItRKtPS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::difference_type std::operator-<unsigned short,unsigned short const&,unsigned short const*>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&)")]
#[doc(alias = "__ZStmiItRKtPS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_")]
pub fn stub_0x1c9004() -> ! {
    todo!("0x1c9004 __ZStmiItRKtPS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_")
}

// 0x1c9054 — __ZStmiIPhRKS0_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::difference_type std::operator-<unsigned char *,unsigned char * const&,unsigned char * const*>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&)")]
#[doc(alias = "__ZStmiIPhRKS0_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_")]
pub fn stub_0x1c9054() -> ! {
    todo!("0x1c9054 __ZStmiIPhRKS0_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_")
}

// 0x1c90a4 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKjiEEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKjiEEE8allocateEmPKv")]
pub fn stub_0x1c90a4() -> ! {
    todo!("0x1c90a4 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKjiEEE8allocateEmPKv")
}

// 0x1c90d4 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE14_M_create_nodeERKS2_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_create_node(std::pair<unsigned int const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE14_M_create_nodeERKS2_")]
pub fn stub_0x1c90d4() -> ! {
    todo!("0x1c90d4 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE14_M_create_nodeERKS2_")
}

// 0x1c9104 — __ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv")]
pub fn stub_0x1c9104() -> ! {
    todo!("0x1c9104 __ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv")
}

// 0x1c9124 — __ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv")]
pub fn stub_0x1c9124() -> ! {
    todo!("0x1c9124 __ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv")
}

// 0x1c9144 — __ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char *>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv")]
pub fn stub_0x1c9144() -> ! {
    todo!("0x1c9144 __ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv")
}

// 0x1c9164 — __ZN9__gnu_cxx13new_allocatorIPN6TagLib7MDMODELEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL *>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIPN6TagLib7MDMODELEE8allocateEmPKv")]
pub fn stub_0x1c9164() -> ! {
    todo!("0x1c9164 __ZN9__gnu_cxx13new_allocatorIPN6TagLib7MDMODELEE8allocateEmPKv")
}

// 0x1c9184 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_allocate_mapEm")]
pub fn stub_0x1c9184() -> ! {
    todo!("0x1c9184 __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_allocate_mapEm")
}

// 0x1c922c — __ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv
// type: int __fastcall(int, unsigned int)
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char **>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv")]
pub fn stub_0x1c922c() -> ! {
    todo!("0x1c922c __ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv")
}

// 0x1c924c — __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm")]
pub fn stub_0x1c924c() -> ! {
    todo!("0x1c924c __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm")
}

// 0x1c92f4 — __ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short *>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv")]
pub fn stub_0x1c92f4() -> ! {
    todo!("0x1c92f4 __ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv")
}

// 0x1c9314 — __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm")]
pub fn stub_0x1c9314() -> ! {
    todo!("0x1c9314 __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm")
}

// 0x1c93bc — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_")]
pub fn stub_0x1c93bc() -> ! {
    todo!("0x1c93bc __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_")
}

// 0x1c94ac — __ZNSt5dequeItSaItEE15_M_pop_back_auxEv
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_pop_back_aux(void)")]
#[doc(alias = "__ZNSt5dequeItSaItEE15_M_pop_back_auxEv")]
pub fn stub_0x1c94ac() -> ! {
    todo!("0x1c94ac __ZNSt5dequeItSaItEE15_M_pop_back_auxEv")
}

// 0x1c94e0 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
pub fn stub_0x1c94e0() -> ! {
    todo!("0x1c94e0 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

// 0x1c951c — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev")]
pub fn stub_0x1c951c() {
    // IDA 0x1c951c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x1c9550 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
pub fn stub_0x1c9550() -> ! {
    todo!("0x1c9550 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

// 0x1c95d4 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// type: int __fastcall(void *__src)
#[doc(alias = "TagLib::MDMODEL * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_")]
pub fn stub_0x1c95d4() -> ! {
    todo!("0x1c95d4 __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_")
}

// 0x1c9604 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "TagLib::MDMODEL * * std::__copy<true,std::random_access_iterator_tag>::copy<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
#[doc(alias = "__ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_")]
pub fn stub_0x1c9604() -> ! {
    todo!("0x1c9604 __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_")
}

// 0x1c9630 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_
// type: int __fastcall(void *__src)
#[doc(alias = "unsigned short * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_")]
pub fn stub_0x1c9630() -> ! {
    todo!("0x1c9630 __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_")
}

// 0x1c9660 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "unsigned short * * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
#[doc(alias = "__ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_")]
pub fn stub_0x1c9660() -> ! {
    todo!("0x1c9660 __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_")
}

// 0x1c968c — __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeItSaItEE17_M_reallocate_mapEmb")]
pub fn stub_0x1c968c() -> ! {
    todo!("0x1c968c __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb")
}

// 0x1c97b4 — __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm")]
pub fn stub_0x1c97b4() -> ! {
    todo!("0x1c97b4 __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm")
}

// 0x1c97e8 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_
// type: int __fastcall(void *__src)
#[doc(alias = "unsigned char ** * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_")]
pub fn stub_0x1c97e8() -> ! {
    todo!("0x1c97e8 __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_")
}

// 0x1c9818 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "unsigned char ** * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
#[doc(alias = "__ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_")]
pub fn stub_0x1c9818() -> ! {
    todo!("0x1c9818 __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_")
}

// 0x1c9844 — __ZNSt5dequeItSaItEE4backEv
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::back(void)")]
#[doc(alias = "__ZNSt5dequeItSaItEE4backEv")]
pub fn stub_0x1c9844() -> ! {
    todo!("0x1c9844 __ZNSt5dequeItSaItEE4backEv")
}

// 0x1c9884 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::pair<unsigned int const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_")]
pub fn stub_0x1c9884() -> ! {
    todo!("0x1c9884 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_")
}

// 0x1c9944 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,int>>,std::pair<unsigned int const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
pub fn stub_0x1c9944() -> ! {
    todo!("0x1c9944 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

// 0x1c9a68 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_
// type: int __fastcall(_DWORD *, int *)
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_")]
pub fn stub_0x1c9a68() -> ! {
    todo!("0x1c9a68 __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_")
}

// 0x1c9a98 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb")]
pub fn stub_0x1c9a98() -> ! {
    todo!("0x1c9a98 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb")
}

// 0x1c9bc0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm")]
pub fn stub_0x1c9bc0() -> ! {
    todo!("0x1c9bc0 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm")
}

// 0x1c9bf4 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator++(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv")]
pub fn stub_0x1c9bf4() -> ! {
    todo!("0x1c9bf4 __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv")
}

// 0x1c9c34 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_aux<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type")]
pub fn stub_0x1c9c34() -> ! {
    todo!("0x1c9c34 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type")
}

// 0x1c9ca4 — __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
#[doc(alias = "__ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_")]
pub fn stub_0x1c9ca4() -> ! {
    todo!("0x1c9ca4 __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_")
}

// 0x1c9d24 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_a<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,TagLib::MDMODEL>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL>)")]
#[doc(alias = "__ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E")]
pub fn stub_0x1c9d24() -> ! {
    todo!("0x1c9d24 __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E")
}

// 0x1c9da0 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator--(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv")]
pub fn stub_0x1c9da0() -> ! {
    todo!("0x1c9da0 __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv")
}

// 0x1c9de0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::back(void)")]
#[doc(alias = "__ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv")]
pub fn stub_0x1c9de0() -> ! {
    todo!("0x1c9de0 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv")
}

// 0x1c9e20 — __ZNSt15_Deque_iteratorItRKtPS0_EppEv
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::operator++(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorItRKtPS0_EppEv")]
pub fn stub_0x1c9e20() -> ! {
    todo!("0x1c9e20 __ZNSt15_Deque_iteratorItRKtPS0_EppEv")
}

// 0x1c9e78 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_")]
pub fn stub_0x1c9e78() -> ! {
    todo!("0x1c9e78 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_")
}

// 0x1ca124 — __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
#[doc(alias = "__ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")]
pub fn stub_0x1ca124() -> ! {
    todo!("0x1ca124 __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")
}

// 0x1ca1a0 — __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
#[doc(alias = "__ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_")]
pub fn stub_0x1ca1a0() -> ! {
    todo!("0x1ca1a0 __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_")
}

// 0x1ca21c — __ZSt4copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
#[doc(alias = "__ZSt4copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")]
pub fn stub_0x1ca21c() -> ! {
    todo!("0x1ca21c __ZSt4copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")
}

// 0x1ca298 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_St11__true_type
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::__true_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_St11__true_type")]
pub fn stub_0x1ca298() -> ! {
    todo!("0x1ca298 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_St11__true_type")
}

// 0x1ca314 — __ZSt18uninitialized_copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::uninitialized_copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
#[doc(alias = "__ZSt18uninitialized_copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")]
pub fn stub_0x1ca314() -> ! {
    todo!("0x1ca314 __ZSt18uninitialized_copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")
}

// 0x1ca394 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEtET0_T_S9_S8_SaIT1_E
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,unsigned short>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short>)")]
#[doc(alias = "__ZSt22__uninitialized_copy_aISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEtET0_T_S9_S8_SaIT1_E")]
pub fn stub_0x1ca394() -> ! {
    todo!("0x1ca394 __ZSt22__uninitialized_copy_aISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEtET0_T_S9_S8_SaIT1_E")
}

// 0x1ca410 — __ZNSt15_Deque_iteratorIPhRKS0_PS1_EppEv
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::operator++(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIPhRKS0_PS1_EppEv")]
pub fn stub_0x1ca410() -> ! {
    todo!("0x1ca410 __ZNSt15_Deque_iteratorIPhRKS0_PS1_EppEv")
}

// 0x1ca468 — __ZStmiItRtPtENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS6_S9_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::difference_type std::operator-<unsigned short,unsigned short &,unsigned short *>(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&)")]
#[doc(alias = "__ZStmiItRtPtENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS6_S9_")]
pub fn stub_0x1ca468() -> ! {
    todo!("0x1ca468 __ZStmiItRtPtENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS6_S9_")
}

// 0x1ca4b8 — __ZNSt15_Deque_iteratorIPhRS0_PS0_E11_M_set_nodeEPS2_
// type: int(void)
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::_M_set_node(unsigned char ***)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIPhRS0_PS0_E11_M_set_nodeEPS2_")]
pub fn stub_0x1ca4b8() -> ! {
    todo!("0x1ca4b8 __ZNSt15_Deque_iteratorIPhRS0_PS0_E11_M_set_nodeEPS2_")
}

// 0x1ca4e8 — __ZNSt15_Deque_iteratorIPhRS0_PS0_EppEv
// type: int *__fastcall(int *)
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator++(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIPhRS0_PS0_EppEv")]
pub fn stub_0x1ca4e8() -> ! {
    todo!("0x1ca4e8 __ZNSt15_Deque_iteratorIPhRS0_PS0_EppEv")
}

// 0x1ca528 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPhRKS4_PS5_ES3_IS4_RS4_PS4_EEET0_T_SD_SC_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPhRKS4_PS5_ES3_IS4_RS4_PS4_EEET0_T_SD_SC_")]
pub fn stub_0x1ca528() -> ! {
    todo!("0x1ca528 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPhRKS4_PS5_ES3_IS4_RS4_PS4_EEET0_T_SD_SC_")
}

// 0x1ca7d4 — __ZSt10__copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
#[doc(alias = "__ZSt10__copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")]
pub fn stub_0x1ca7d4() -> ! {
    todo!("0x1ca7d4 __ZSt10__copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")
}

// 0x1ca850 — __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorIPhRKS3_PS4_ES2_IS3_RS3_PS3_EEET0_T_SC_SB_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
#[doc(alias = "__ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorIPhRKS3_PS4_ES2_IS3_RS3_PS3_EEET0_T_SC_SB_")]
pub fn stub_0x1ca850() -> ! {
    todo!("0x1ca850 __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorIPhRKS3_PS4_ES2_IS3_RS3_PS3_EEET0_T_SC_SB_")
}

// 0x1ca8cc — __ZSt4copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
#[doc(alias = "__ZSt4copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")]
pub fn stub_0x1ca8cc() -> ! {
    todo!("0x1ca8cc __ZSt4copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")
}

// 0x1ca948 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_St11__true_type
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::__true_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_St11__true_type")]
pub fn stub_0x1ca948() -> ! {
    todo!("0x1ca948 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_St11__true_type")
}

// 0x1ca9c4 — __ZSt18uninitialized_copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::uninitialized_copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
#[doc(alias = "__ZSt18uninitialized_copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")]
pub fn stub_0x1ca9c4() -> ! {
    todo!("0x1ca9c4 __ZSt18uninitialized_copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")
}

// 0x1caa44 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_ES1_ET0_T_SA_S9_SaIT1_E
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,unsigned char *>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *>)")]
#[doc(alias = "__ZSt22__uninitialized_copy_aISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_ES1_ET0_T_SA_S9_SaIT1_E")]
pub fn stub_0x1caa44() -> ! {
    todo!("0x1caa44 __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_ES1_ET0_T_SA_S9_SaIT1_E")
}

// 0x1caac0 — __ZNSt5dequeIPhSaIS0_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE17_M_reallocate_mapEmb")]
pub fn stub_0x1caac0() -> ! {
    todo!("0x1caac0 __ZNSt5dequeIPhSaIS0_EE17_M_reallocate_mapEmb")
}

// 0x1cabe8 — __ZNSt5dequeIPhSaIS0_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE22_M_reserve_map_at_backEm")]
pub fn stub_0x1cabe8() -> ! {
    todo!("0x1cabe8 __ZNSt5dequeIPhSaIS0_EE22_M_reserve_map_at_backEm")
}

// 0x1cac1c — __ZNSt5dequeIPhSaIS0_EE16_M_push_back_auxERKS0_
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_push_back_aux(unsigned char * const&)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE16_M_push_back_auxERKS0_")]
pub fn stub_0x1cac1c() -> ! {
    todo!("0x1cac1c __ZNSt5dequeIPhSaIS0_EE16_M_push_back_auxERKS0_")
}

// 0x1cac80 — __ZNSt5dequeIPhSaIS0_EE9push_backERKS0_
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::push_back(unsigned char * const&)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE9push_backERKS0_")]
pub fn stub_0x1cac80() -> ! {
    todo!("0x1cac80 __ZNSt5dequeIPhSaIS0_EE9push_backERKS0_")
}

// 0x1cacc4 — __ZNSt15_Deque_iteratorIPhRS0_PS0_EmmEv
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator--(void)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIPhRS0_PS0_EmmEv")]
pub fn stub_0x1cacc4() -> ! {
    todo!("0x1cacc4 __ZNSt15_Deque_iteratorIPhRS0_PS0_EmmEv")
}

// 0x1cad04 — __ZNSt5dequeIPhSaIS0_EE4backEv
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::back(void)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE4backEv")]
pub fn stub_0x1cad04() -> ! {
    todo!("0x1cad04 __ZNSt5dequeIPhSaIS0_EE4backEv")
}

// 0x1cad44 — __ZNSt5dequeItSaItEE16_M_push_back_auxERKt
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_push_back_aux(unsigned short const&)")]
#[doc(alias = "__ZNSt5dequeItSaItEE16_M_push_back_auxERKt")]
pub fn stub_0x1cad44() -> ! {
    todo!("0x1cad44 __ZNSt5dequeItSaItEE16_M_push_back_auxERKt")
}

// 0x1cada8 — __ZNSt5dequeItSaItEE9push_backERKt
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::push_back(unsigned short const&)")]
#[doc(alias = "__ZNSt5dequeItSaItEE9push_backERKt")]
pub fn stub_0x1cada8() -> ! {
    todo!("0x1cada8 __ZNSt5dequeItSaItEE9push_backERKt")
}

// 0x1cadec — __ZNSt11_Deque_baseItSaItEE16_M_destroy_nodesEPPtS3_
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_destroy_nodes(unsigned short **,unsigned short **)")]
#[doc(alias = "__ZNSt11_Deque_baseItSaItEE16_M_destroy_nodesEPPtS3_")]
pub fn stub_0x1cadec() -> ! {
    todo!("0x1cadec __ZNSt11_Deque_baseItSaItEE16_M_destroy_nodesEPPtS3_")
}

// 0x1caedc — __ZNSt11_Deque_baseItSaItEED2Ev
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseItSaItEED2Ev")]
pub fn stub_0x1caedc() {
    // IDA 0x1caedc: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x1caf10 — __ZNSt5dequeItSaItEED2Ev
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::~deque()")]
#[doc(alias = "__ZNSt5dequeItSaItEED2Ev")]
pub fn stub_0x1caf10() {
    // IDA 0x1caf10: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x1caf80 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE16_M_push_back_auxERKS1_
// type: int(void)
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_push_back_aux(TagLib::MDMODEL const&)")]
#[doc(alias = "__ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE16_M_push_back_auxERKS1_")]
pub fn stub_0x1caf80() -> ! {
    todo!("0x1caf80 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE16_M_push_back_auxERKS1_")
}

// 0x1cafe4 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE9push_backERKS1_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::push_back(TagLib::MDMODEL const&)")]
#[doc(alias = "__ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE9push_backERKS1_")]
pub fn stub_0x1cafe4() -> ! {
    todo!("0x1cafe4 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE9push_backERKS1_")
}

// 0x1cb028 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_pop_back_auxEv
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_pop_back_aux(void)")]
#[doc(alias = "__ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_pop_back_auxEv")]
pub fn stub_0x1cb028() -> ! {
    todo!("0x1cb028 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_pop_back_auxEv")
}

// 0x1cb05c — __ZNSt5dequeIPhSaIS0_EE15_M_pop_back_auxEv
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_pop_back_aux(void)")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EE15_M_pop_back_auxEv")]
pub fn stub_0x1cb05c() -> ! {
    todo!("0x1cb05c __ZNSt5dequeIPhSaIS0_EE15_M_pop_back_auxEv")
}

// 0x1cb090 — __ZNSt11_Deque_baseIPhSaIS0_EE16_M_destroy_nodesEPPS0_S4_
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_nodes(unsigned char ***,unsigned char ***)")]
#[doc(alias = "__ZNSt11_Deque_baseIPhSaIS0_EE16_M_destroy_nodesEPPS0_S4_")]
pub fn stub_0x1cb090() -> ! {
    todo!("0x1cb090 __ZNSt11_Deque_baseIPhSaIS0_EE16_M_destroy_nodesEPPS0_S4_")
}

// 0x1cb180 — __ZNSt11_Deque_baseIPhSaIS0_EED2Ev
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIPhSaIS0_EED2Ev")]
pub fn stub_0x1cb180() {
    // IDA 0x1cb180: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x1cb1b4 — __ZNSt5dequeIPhSaIS0_EED2Ev
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIPhSaIS0_EED2Ev")]
pub fn stub_0x1cb1b4() {
    // IDA 0x1cb1b4: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x1cb224 — __ZNSt3mapIjiSt4lessIjESaISt4pairIKjiEEEixERS3_
#[doc(alias = "std::map<unsigned int,int,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::operator[](unsigned int const&)")]
#[doc(alias = "__ZNSt3mapIjiSt4lessIjESaISt4pairIKjiEEEixERS3_")]
pub fn stub_0x1cb224() -> ! {
    todo!("0x1cb224 __ZNSt3mapIjiSt4lessIjESaISt4pairIKjiEEEixERS3_")
}

// 0x1cb290 — __ZNSt11_Deque_baseItSaItEE15_M_create_nodesEPPtS3_
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_create_nodes(unsigned short **,unsigned short **)")]
#[doc(alias = "__ZNSt11_Deque_baseItSaItEE15_M_create_nodesEPPtS3_")]
pub fn stub_0x1cb290() -> ! {
    todo!("0x1cb290 __ZNSt11_Deque_baseItSaItEE15_M_create_nodesEPPtS3_")
}

// 0x1cb510 — __ZNSt11_Deque_baseItSaItEE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseItSaItEE17_M_initialize_mapEm")]
pub fn stub_0x1cb510() -> ! {
    todo!("0x1cb510 __ZNSt11_Deque_baseItSaItEE17_M_initialize_mapEm")
}

// 0x1cb6e0 — __ZNSt11_Deque_baseItSaItEEC2ERKS0_m
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&,unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseItSaItEEC2ERKS0_m")]
pub fn stub_0x1cb6e0() -> ! {
    todo!("0x1cb6e0 __ZNSt11_Deque_baseItSaItEEC2ERKS0_m")
}
