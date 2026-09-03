//! network generated_watchdog_net_b — watchdog net_b RakNet part B 120 stubs (global dedup fallback)
//! Filter: RakNet remaining (944 total, 0 remaining — pool exhausted, using global EA-sorted gap filler distinct globally)
//! Source: ida/export.json (85545 funcs, base 0x4000) | remaining pool 4437 genuine gaps, batch 120
//! Range 0x3fcb84..0x505188 | EA-sorted asc | SharedPtr = rbx_core::SharedPtr (Arc), not boost | // 0xADDR + alias + todo!
//! muse-spark-1.3 watchdog net_b

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x3fcb84 — __ZN3RBX11shared_fromINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::ModelInstance> RBX::shared_from<RBX::ModelInstance>(RBX::ModelInstance*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0x3fcb84() -> ! { todo!("0x3fcb84 __ZN3RBX11shared_fromINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEPS4_") }

// 0x402f14 — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>>::operator=(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEaSERKS5_")]
pub fn stub_0x402f14() -> ! { todo!("0x402f14 __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEaSERKS5_") }

// 0x402f4c — __ZN3RBX11shared_fromINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEPS6_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> RBX::shared_from<RBX::FilteredSelection<RBX::PVInstance>>(RBX::FilteredSelection<RBX::PVInstance>*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEPS6_")]
pub fn stub_0x402f4c() -> ! { todo!("0x402f4c __ZN3RBX11shared_fromINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEPS6_") }

// 0x411d70 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::PartInstance>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::_M_erase(std::_Rb_tree_node<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_0x411d70() -> ! { todo!("0x411d70 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E") }

// 0x411d98 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::PartInstance>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_0x411d98() -> ! { todo!("0x411d98 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E") }

// 0x49df7c — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::resize(unsigned long,RBX::PyramidInstance::NumSidesEnum)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_")]
pub fn stub_0x49df7c() -> ! { todo!("0x49df7c __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_") }

// 0x49dfb0 — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::push_back(RBX::PyramidInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_")]
pub fn stub_0x49dfb0() -> ! { todo!("0x49dfb0 __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_") }

// 0x49dfd8 — __ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::PyramidInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x49dfd8() -> ! { todo!("0x49dfd8 __ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_") }

// 0x49e030 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x49e030() -> ! { todo!("0x49e030 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_") }

// 0x49e0e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x49e0e4() -> ! { todo!("0x49e0e4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_") }

// 0x49e13c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x49e13c() -> ! { todo!("0x49e13c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_") }

// 0x49e1a4 — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,RBX::PyramidInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x49e1a4() -> ! { todo!("0x49e1a4 __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_") }

// 0x49e288 — __ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm")]
pub fn stub_0x49e288() -> ! { todo!("0x49e288 __ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm") }

// 0x49e2a0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PyramidInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *>(RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_")]
pub fn stub_0x49e2a0() -> ! { todo!("0x49e2a0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_") }

// 0x49e2dc — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,unsigned long,RBX::PyramidInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x49e2dc() -> ! { todo!("0x49e2dc __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_") }

// 0x49e46c — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::resize(unsigned long,RBX::PrismInstance::NumSidesEnum)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_")]
pub fn stub_0x49e46c() -> ! { todo!("0x49e46c __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_") }

// 0x49e4a0 — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::push_back(RBX::PrismInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_")]
pub fn stub_0x49e4a0() -> ! { todo!("0x49e4a0 __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_") }

// 0x49e4c8 — __ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::PrismInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x49e4c8() -> ! { todo!("0x49e4c8 __ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_") }

// 0x49e520 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x49e520() -> ! { todo!("0x49e520 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_") }

// 0x49e5d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x49e5d4() -> ! { todo!("0x49e5d4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_") }

// 0x49e62c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x49e62c() -> ! { todo!("0x49e62c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_") }

// 0x49e694 — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,RBX::PrismInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x49e694() -> ! { todo!("0x49e694 __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_") }

// 0x49e778 — __ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm")]
pub fn stub_0x49e778() -> ! { todo!("0x49e778 __ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm") }

// 0x49e790 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::PrismInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *>(RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_")]
pub fn stub_0x49e790() -> ! { todo!("0x49e790 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_") }

// 0x49e7cc — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,unsigned long,RBX::PrismInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x49e7cc() -> ! { todo!("0x49e7cc __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_") }

// 0x49e95c — __ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::ExtrudedPartInstance::VisualTrussStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x49e95c() -> ! { todo!("0x49e95c __ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_") }

// 0x49e9b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x49e9b4() -> ! { todo!("0x49e9b4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_") }

// 0x49ea68 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x49ea68() -> ! { todo!("0x49ea68 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_") }

// 0x49eac0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x49eac0() -> ! { todo!("0x49eac0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_") }

// 0x49eb28 — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::resize(unsigned long,RBX::ExtrudedPartInstance::VisualTrussStyle)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE6resizeEmS2_")]
pub fn stub_0x49eb28() -> ! { todo!("0x49eb28 __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE6resizeEmS2_") }

// 0x49eb5c — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::push_back(RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE9push_backERKS2_")]
pub fn stub_0x49eb5c() -> ! { todo!("0x49eb5c __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE9push_backERKS2_") }

// 0x49eb84 — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ExtrudedPartInstance::VisualTrussStyle*,std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>>,RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x49eb84() -> ! { todo!("0x49eb84 __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_") }

// 0x49ec68 — __ZNSt12_Vector_baseIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE11_M_allocateEm")]
pub fn stub_0x49ec68() -> ! { todo!("0x49ec68 __ZNSt12_Vector_baseIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE11_M_allocateEm") }

// 0x49ec80 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ExtrudedPartInstance16VisualTrussStyleES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::ExtrudedPartInstance::VisualTrussStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ExtrudedPartInstance::VisualTrussStyle *,RBX::ExtrudedPartInstance::VisualTrussStyle *>(RBX::ExtrudedPartInstance::VisualTrussStyle *,RBX::ExtrudedPartInstance::VisualTrussStyle *,RBX::ExtrudedPartInstance::VisualTrussStyle *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ExtrudedPartInstance16VisualTrussStyleES6_EET0_T_S8_S7_")]
pub fn stub_0x49ec80() -> ! { todo!("0x49ec80 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ExtrudedPartInstance16VisualTrussStyleES6_EET0_T_S8_S7_") }

// 0x49ecbc — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ExtrudedPartInstance::VisualTrussStyle*,std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>>,unsigned long,RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x49ecbc() -> ! { todo!("0x49ecbc __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_") }

// 0x4a52dc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE")]
pub fn stub_0x4a52dc() -> ! { todo!("0x4a52dc __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE") }

// 0x4a53cc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x4a53cc() -> ! { todo!("0x4a53cc __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv") }

// 0x4a53d0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x4a53d0() -> ! { todo!("0x4a53d0 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv") }

// 0x4a54c4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev")]
pub fn stub_0x4a54c4() -> ! { todo!("0x4a54c4 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev") }

// 0x4a55d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev")]
pub fn stub_0x4a55d4() -> ! { todo!("0x4a55d4 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev") }

// 0x4a5704 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev")]
pub fn stub_0x4a5704() -> ! { todo!("0x4a5704 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev") }

// 0x4a5730 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev")]
pub fn stub_0x4a5730() -> ! { todo!("0x4a5730 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev") }

// 0x4a5804 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_
// type: int(void)
#[doc(alias = "boost::function2<void,boost::shared_ptr<RBX::Instance>,float>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,float> const&)")]
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_")]
pub fn stub_0x4a5804() -> ! { todo!("0x4a5804 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_") }

// 0x4a72a8 — __ZN3RBX20ExtrudedPartInstance14setPartSizeXmlERKN3G3D7Vector3E
// type: int __fastcall(RBX::ExtrudedPartInstance *this, const G3D::Vector3 *)
#[doc(alias = "RBX::ExtrudedPartInstance::setPartSizeXml(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX20ExtrudedPartInstance14setPartSizeXmlERKN3G3D7Vector3E")]
pub fn stub_0x4a72a8() -> ! { todo!("0x4a72a8 __ZN3RBX20ExtrudedPartInstance14setPartSizeXmlERKN3G3D7Vector3E") }

// 0x4a7de0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::ExtrudedPartInstance> RBX::Creatable<RBX::Instance>::create<RBX::ExtrudedPartInstance>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x4a7de0() -> ! { todo!("0x4a7de0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv") }

// 0x4a7e94 — __ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::ExtrudedPartInstance>::shared_ptr<RBX::ExtrudedPartInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x4a7e94() -> ! { todo!("0x4a7e94 __ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x4a8044 — __ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x4a8044() -> ! { todo!("0x4a8044 __ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x4a814c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x4a814c() -> ! { todo!("0x4a814c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev") }

// 0x4a8150 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x4a8150() -> ! { todo!("0x4a8150 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev") }

// 0x4a8154 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4a8154() -> ! { todo!("0x4a8154 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv") }

// 0x4a8174 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x4a8174() -> ! { todo!("0x4a8174 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info") }

// 0x4a818c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x4a818c() -> ! { todo!("0x4a818c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv") }

// 0x4c5b28 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15PyramidInstance12NumSidesEnumEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PyramidInstance::NumSidesEnum>(RBX::PyramidInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15PyramidInstance12NumSidesEnumEEERS3_RKT_")]
pub fn stub_0x4c5b28() -> ! { todo!("0x4c5b28 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15PyramidInstance12NumSidesEnumEEERS3_RKT_") }

// 0x4c5b78 — __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE9singletonEv")]
pub fn stub_0x4c5b78() -> ! { todo!("0x4c5b78 __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE9singletonEv") }

// 0x4c5be4 — __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE14construct_funcEPKcPc")]
pub fn stub_0x4c5be4() -> ! { todo!("0x4c5be4 __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE14construct_funcEPKcPc") }

// 0x4c5bf0 — __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE13destruct_funcEPc")]
pub fn stub_0x4c5bf0() -> ! { todo!("0x4c5bf0 __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE13destruct_funcEPc") }

// 0x4c5cc0 — __ZN3rbx8any_castIRKN3RBX15PyramidInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::PyramidInstance::NumSidesEnum const& rbx::any_cast<RBX::PyramidInstance::NumSidesEnum const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX15PyramidInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c5cc0() -> ! { todo!("0x4c5cc0 __ZN3rbx8any_castIRKN3RBX15PyramidInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

// 0x4c5e2c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c5e2c() -> ! { todo!("0x4c5e2c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

// 0x4c6550 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13PrismInstance12NumSidesEnumEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PrismInstance::NumSidesEnum>(RBX::PrismInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13PrismInstance12NumSidesEnumEEERS3_RKT_")]
pub fn stub_0x4c6550() -> ! { todo!("0x4c6550 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13PrismInstance12NumSidesEnumEEERS3_RKT_") }

// 0x4c65a0 — __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE9singletonEv")]
pub fn stub_0x4c65a0() -> ! { todo!("0x4c65a0 __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE9singletonEv") }

// 0x4c660c — __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE14construct_funcEPKcPc")]
pub fn stub_0x4c660c() -> ! { todo!("0x4c660c __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE14construct_funcEPKcPc") }

// 0x4c6618 — __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE13destruct_funcEPc")]
pub fn stub_0x4c6618() -> ! { todo!("0x4c6618 __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE13destruct_funcEPc") }

// 0x4c66e8 — __ZN3rbx8any_castIRKN3RBX13PrismInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::PrismInstance::NumSidesEnum const& rbx::any_cast<RBX::PrismInstance::NumSidesEnum const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13PrismInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c66e8() -> ! { todo!("0x4c66e8 __ZN3rbx8any_castIRKN3RBX13PrismInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

// 0x4c6854 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c6854() -> ! { todo!("0x4c6854 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

// 0x4c6f78 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ExtrudedPartInstance16VisualTrussStyleEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ExtrudedPartInstance::VisualTrussStyle>(RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ExtrudedPartInstance16VisualTrussStyleEEERS3_RKT_")]
pub fn stub_0x4c6f78() -> ! { todo!("0x4c6f78 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ExtrudedPartInstance16VisualTrussStyleEEERS3_RKT_") }

// 0x4c6fc8 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE9singletonEv")]
pub fn stub_0x4c6fc8() -> ! { todo!("0x4c6fc8 __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE9singletonEv") }

// 0x4c7034 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE14construct_funcEPKcPc")]
pub fn stub_0x4c7034() -> ! { todo!("0x4c7034 __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE14construct_funcEPKcPc") }

// 0x4c7040 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE13destruct_funcEPc")]
pub fn stub_0x4c7040() -> ! { todo!("0x4c7040 __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE13destruct_funcEPc") }

// 0x4c7110 — __ZN3rbx8any_castIRKN3RBX20ExtrudedPartInstance16VisualTrussStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::ExtrudedPartInstance::VisualTrussStyle const& rbx::any_cast<RBX::ExtrudedPartInstance::VisualTrussStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX20ExtrudedPartInstance16VisualTrussStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c7110() -> ! { todo!("0x4c7110 __ZN3rbx8any_castIRKN3RBX20ExtrudedPartInstance16VisualTrussStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

// 0x4c727c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c727c() -> ! { todo!("0x4c727c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

// 0x4e5a78 — __ZN3RBX12MotorFeature7canJoinEPNS_8InstanceES2_
// type: _DWORD __fastcall(RBX::MotorFeature *__hidden this, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::MotorFeature::canJoin(RBX::Instance *,RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12MotorFeature7canJoinEPNS_8InstanceES2_")]
pub fn stub_0x4e5a78() -> ! { todo!("0x4e5a78 __ZN3RBX12MotorFeature7canJoinEPNS_8InstanceES2_") }

// 0x4e5b64 — __ZN3RBX12MotorFeature4joinEPNS_8InstanceES2_
// type: _DWORD __fastcall(RBX::MotorFeature *__hidden this, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::MotorFeature::join(RBX::Instance *,RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12MotorFeature4joinEPNS_8InstanceES2_")]
pub fn stub_0x4e5b64() -> ! { todo!("0x4e5b64 __ZN3RBX12MotorFeature4joinEPNS_8InstanceES2_") }

// 0x4efd38 — __ZNK3RBX4Fire11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Fire *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Fire::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX4Fire11askAddChildEPKNS_8InstanceE")]
pub fn stub_0x4efd38() -> ! { todo!("0x4efd38 __ZNK3RBX4Fire11askAddChildEPKNS_8InstanceE") }

// 0x4efd3c — __ZNK3RBX4Fire12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Fire *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Fire::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX4Fire12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x4efd3c() -> ! { todo!("0x4efd3c __ZNK3RBX4Fire12askSetParentEPKNS_8InstanceE") }

// 0x4f0004 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FireEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::Fire> RBX::Creatable<RBX::Instance>::create<RBX::Fire>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4FireEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x4f0004() -> ! { todo!("0x4f0004 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FireEEEN5boost10shared_ptrIT_EEv") }

// 0x4f00b4 — __ZN5boost10shared_ptrIN3RBX4FireEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::Fire>::shared_ptr<RBX::Fire,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4FireEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x4f00b4() -> ! { todo!("0x4f00b4 __ZN5boost10shared_ptrIN3RBX4FireEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x4f0264 — __ZN5boost6detail12shared_countC2IPN3RBX4FireENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4FireENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x4f0264() -> ! { todo!("0x4f0264 __ZN5boost6detail12shared_countC2IPN3RBX4FireENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x4f036c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x4f036c() -> ! { todo!("0x4f036c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev") }

// 0x4f0370 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x4f0370() -> ! { todo!("0x4f0370 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev") }

// 0x4f0374 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4f0374() -> ! { todo!("0x4f0374 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv") }

// 0x4f0394 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x4f0394() -> ! { todo!("0x4f0394 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info") }

// 0x4f03ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x4f03ac() -> ! { todo!("0x4f03ac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv") }

// 0x4f1da0 — __ZN3RBX4Flag19onEvent_flagTouchedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Flag::onEvent_flagTouched(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX4Flag19onEvent_flagTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x4f1da0() -> ! { todo!("0x4f1da0 __ZN3RBX4Flag19onEvent_flagTouchedEN5boost10shared_ptrINS_8InstanceEEE") }

// 0x4f1eac — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4FlagENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4FlagENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")]
pub fn stub_0x4f1eac() -> ! { todo!("0x4f1eac __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4FlagENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_") }

// 0x4f22bc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FlagEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::Flag> RBX::Creatable<RBX::Instance>::create<RBX::Flag>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4FlagEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x4f22bc() -> ! { todo!("0x4f22bc __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FlagEEEN5boost10shared_ptrIT_EEv") }

// 0x4f2370 — __ZN5boost10shared_ptrIN3RBX4FlagEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::Flag>::shared_ptr<RBX::Flag,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4FlagEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x4f2370() -> ! { todo!("0x4f2370 __ZN5boost10shared_ptrIN3RBX4FlagEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x4f2520 — __ZN5boost6detail12shared_countC2IPN3RBX4FlagENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4FlagENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x4f2520() -> ! { todo!("0x4f2520 __ZN5boost6detail12shared_countC2IPN3RBX4FlagENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x4f2628 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x4f2628() -> ! { todo!("0x4f2628 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev") }

// 0x4f262c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x4f262c() -> ! { todo!("0x4f262c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev") }

// 0x4f2630 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4f2630() -> ! { todo!("0x4f2630 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv") }

// 0x4f2650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x4f2650() -> ! { todo!("0x4f2650 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info") }

// 0x4f2668 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x4f2668() -> ! { todo!("0x4f2668 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv") }

// 0x4f2a08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x4f2a08() -> ! { todo!("0x4f2a08 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE") }

// 0x4f2a68 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_0x4f2a68() -> ! { todo!("0x4f2a68 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_") }

// 0x4f2a84 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4FlagEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Flag *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX4FlagEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x4f2a84() -> ! { todo!("0x4f2a84 __ZN5boost3_bi5list2INS0_5valueIPN3RBX4FlagEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i") }

// 0x4f2b5c — __ZNK5boost4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Flag*,boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")]
pub fn stub_0x4f2b5c() -> ! { todo!("0x4f2b5c __ZNK5boost4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_") }

// 0x4f39a4 — __ZN3RBX9FlagStand20onEvent_standTouchedEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::FlagStand::onEvent_standTouched(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX9FlagStand20onEvent_standTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x4f39a4() -> ! { todo!("0x4f39a4 __ZN3RBX9FlagStand20onEvent_standTouchedEN5boost10shared_ptrINS_8InstanceEEE") }

// 0x4f47b4 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_9FlagStandENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_9FlagStandENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")]
pub fn stub_0x4f47b4() -> ! { todo!("0x4f47b4 __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_9FlagStandENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_") }

// 0x4f4adc — __ZN3RBX15ServiceProvider6createINS_16FlagStandServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::FlagStandService * RBX::ServiceProvider::create<RBX::FlagStandService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_16FlagStandServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_0x4f4adc() -> ! { todo!("0x4f4adc __ZN3RBX15ServiceProvider6createINS_16FlagStandServiceEEEPT_PKNS_8InstanceE") }

// 0x4f5524 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FlagStandEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::FlagStand> RBX::Creatable<RBX::Instance>::create<RBX::FlagStand>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9FlagStandEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x4f5524() -> ! { todo!("0x4f5524 __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FlagStandEEEN5boost10shared_ptrIT_EEv") }

// 0x4f55d8 — __ZN5boost10shared_ptrIN3RBX9FlagStandEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::FlagStand>::shared_ptr<RBX::FlagStand,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9FlagStandEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x4f55d8() -> ! { todo!("0x4f55d8 __ZN5boost10shared_ptrIN3RBX9FlagStandEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x4f5788 — __ZN5boost6detail12shared_countC2IPN3RBX9FlagStandENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9FlagStandENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x4f5788() -> ! { todo!("0x4f5788 __ZN5boost6detail12shared_countC2IPN3RBX9FlagStandENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x4f5890 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x4f5890() -> ! { todo!("0x4f5890 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev") }

// 0x4f5894 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x4f5894() -> ! { todo!("0x4f5894 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev") }

// 0x4f5898 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4f5898() -> ! { todo!("0x4f5898 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv") }

// 0x4f58b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x4f58b8() -> ! { todo!("0x4f58b8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info") }

// 0x4f58d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x4f58d0() -> ! { todo!("0x4f58d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv") }

// 0x4fefb8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12GameSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::GameSettings> RBX::Creatable<RBX::Instance>::create<RBX::GameSettings>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12GameSettingsEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x4fefb8() -> ! { todo!("0x4fefb8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12GameSettingsEEEN5boost10shared_ptrIT_EEv") }

// 0x4ff068 — __ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::GameSettings>::shared_ptr<RBX::GameSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x4ff068() -> ! { todo!("0x4ff068 __ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x4ff130 — __ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x4ff130() -> ! { todo!("0x4ff130 __ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x4ff238 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x4ff238() -> ! { todo!("0x4ff238 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev") }

// 0x4ff23c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4ff23c() -> ! { todo!("0x4ff23c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv") }

// 0x4ff864 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13disconnectAllEv")]
pub fn stub_0x4ff864() -> ! { todo!("0x4ff864 __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13disconnectAllEv") }

// 0x4ffaa4 — __ZN3RBX20CameraZoomOutCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraZoomOutCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CameraZoomOutCommand::CameraZoomOutCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX20CameraZoomOutCommandC2EPNS_9WorkspaceE")]
pub fn stub_0x4ffaa4() -> ! { todo!("0x4ffaa4 __ZN3RBX20CameraZoomOutCommandC2EPNS_9WorkspaceE") }

// 0x4ffbec — __ZN3RBX19CameraZoomInCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraZoomInCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CameraZoomInCommand::CameraZoomInCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX19CameraZoomInCommandC2EPNS_9WorkspaceE")]
pub fn stub_0x4ffbec() -> ! { todo!("0x4ffbec __ZN3RBX19CameraZoomInCommandC2EPNS_9WorkspaceE") }

// 0x4ffd34 — __ZN3RBX21CameraTiltDownCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraTiltDownCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CameraTiltDownCommand::CameraTiltDownCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX21CameraTiltDownCommandC2EPNS_9WorkspaceE")]
pub fn stub_0x4ffd34() -> ! { todo!("0x4ffd34 __ZN3RBX21CameraTiltDownCommandC2EPNS_9WorkspaceE") }

// 0x4ffe7c — __ZN3RBX19CameraTiltUpCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraTiltUpCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CameraTiltUpCommand::CameraTiltUpCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX19CameraTiltUpCommandC2EPNS_9WorkspaceE")]
pub fn stub_0x4ffe7c() -> ! { todo!("0x4ffe7c __ZN3RBX19CameraTiltUpCommandC2EPNS_9WorkspaceE") }

// 0x4fffc4 — __ZN3RBX21CameraPanRightCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraPanRightCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CameraPanRightCommand::CameraPanRightCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX21CameraPanRightCommandC2EPNS_9WorkspaceE")]
pub fn stub_0x4fffc4() -> ! { todo!("0x4fffc4 __ZN3RBX21CameraPanRightCommandC2EPNS_9WorkspaceE") }

// 0x50010c — __ZN3RBX20CameraPanLeftCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraPanLeftCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CameraPanLeftCommand::CameraPanLeftCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX20CameraPanLeftCommandC2EPNS_9WorkspaceE")]
pub fn stub_0x50010c() -> ! { todo!("0x50010c __ZN3RBX20CameraPanLeftCommandC2EPNS_9WorkspaceE") }

// 0x505188 — __ZN3RBX15GeometryService33getPartsTouchingExtentsWithIgnoreERKNS_7ExtentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS8_EEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, char, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::GeometryService::getPartsTouchingExtentsWithIgnore(RBX::Extents const&,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)")]
#[doc(alias = "__ZN3RBX15GeometryService33getPartsTouchingExtentsWithIgnoreERKNS_7ExtentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS8_EEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE")]
pub fn stub_0x505188() -> ! { todo!("0x505188 __ZN3RBX15GeometryService33getPartsTouchingExtentsWithIgnoreERKNS_7ExtentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS8_EEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE") }
