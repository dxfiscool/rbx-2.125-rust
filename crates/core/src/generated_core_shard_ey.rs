//! core shard EY — 100 core stubs EA-sorted, lowest uncovered 0xc284e0..0xc3a2bc (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EX 0xc27c18).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xc27c18.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<false,true>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xc284e0 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_c284e0() -> ! {
    todo!("0xc284e0 __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<false,false>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xc28de8 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_c28de8() -> ! {
    todo!("0xc28de8 __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<0>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
// 0xc2b32c — __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi0EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_c2b32c() -> ! {
    todo!("0xc2b32c __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi0EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_")
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<1>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
// 0xc2b484 — __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi1EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_c2b484() -> ! {
    todo!("0xc2b484 __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi1EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_")
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<2>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
// 0xc2b5e4 — __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi2EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_c2b5e4() -> ! {
    todo!("0xc2b5e4 __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi2EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_")
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<0>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
// 0xc2b734 — __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_c2b734() -> ! {
    todo!("0xc2b734 __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_")
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<1>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
// 0xc2b9a8 — __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_c2b9a8() -> ! {
    todo!("0xc2b9a8 __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_")
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<2>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
// 0xc2bc2c — __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi2EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_c2bc2c() -> ! {
    todo!("0xc2bc2c __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi2EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_")
}

#[doc(alias = "boost::system::system_error::~system_error()")]
// 0xc2bfd0 — __ZN5boost6system12system_errorD1Ev
pub fn stub_c2bfd0() -> ! {
    todo!("0xc2bfd0 __ZN5boost6system12system_errorD1Ev")
}

#[doc(alias = "boost::system::system_error::~system_error()")]
// 0xc2c030 — __ZN5boost6system12system_errorD0Ev
pub fn stub_c2c030() -> ! {
    todo!("0xc2c030 __ZN5boost6system12system_errorD0Ev")
}

#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
// 0xc2c220 — __ZN3RBX12CylinderMeshD1Ev
pub fn stub_c2c220() -> ! {
    todo!("0xc2c220 __ZN3RBX12CylinderMeshD1Ev")
}

#[doc(alias = "void boost::throw_exception<boost::bad_function_call>(boost::bad_function_call const&)")]
// 0xc2c2b0 — __ZN5boost15throw_exceptionINS_17bad_function_callEEEvRKT_
pub fn stub_c2c2b0() -> ! {
    todo!("0xc2c2b0 __ZN5boost15throw_exceptionINS_17bad_function_callEEEvRKT_")
}

#[doc(alias = "boost::bad_function_call::~bad_function_call()")]
// 0xc2c400 — __ZN5boost17bad_function_callD1Ev
pub fn stub_c2c400() -> ! {
    todo!("0xc2c400 __ZN5boost17bad_function_callD1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::rethrow(void)const")]
// 0xc2c410 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv
pub fn stub_c2c410() -> ! {
    todo!("0xc2c410 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// 0xc2c4c0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()
pub fn stub_c2c4c0() -> ! {
    todo!("0xc2c4c0 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::rethrow(void)const")]
// 0xc2c580 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::rethrow(void)const
pub fn stub_c2c580() -> ! {
    todo!("0xc2c580 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// 0xc2c590 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()
pub fn stub_c2c590() -> ! {
    todo!("0xc2c590 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// 0xc2c670 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_17bad_function_callEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()
pub fn stub_c2c670() -> ! {
    todo!("0xc2c670 __ZThn8_N5boost16exception_detail19error_info_injectorINS_17bad_function_callEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_base::~clone_base()")]
// 0xc2c730 — __ZN5boost16exception_detail10clone_baseD0Ev
pub fn stub_c2c730() -> ! {
    todo!("0xc2c730 __ZN5boost16exception_detail10clone_baseD0Ev")
}

#[doc(alias = "boost::function1<void,std::exception &>::dummy::nonnull(void)")]
// 0xc2c740 — __ZN5boost9function1IvRSt9exceptionE5dummy7nonnullEv
pub fn stub_c2c740() -> ! {
    todo!("0xc2c740 __ZN5boost9function1IvRSt9exceptionE5dummy7nonnullEv")
}

#[doc(alias = "boost::system::system_error::system_error(boost::system::error_code,char const*)")]
// 0xc2c750 — __ZN5boost6system12system_errorC2ENS0_10error_codeEPKc
pub fn stub_c2c750() -> ! {
    todo!("0xc2c750 __ZN5boost6system12system_errorC2ENS0_10error_codeEPKc")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0xc2c8b0 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev
pub fn stub_c2c8b0() -> ! {
    todo!("0xc2c8b0 __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone(void)const")]
// 0xc2c9c0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone(void)const
pub fn stub_c2c9c0() -> ! {
    todo!("0xc2c9c0 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv")
}

#[doc(alias = "boost::mutex::mutex(void)")]
// 0xc2c9d0 — __ZN5boost5mutexC2Ev
pub fn stub_c2c9d0() -> ! {
    todo!("0xc2c9d0 __ZN5boost5mutexC2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0xc2cb00 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
pub fn stub_c2cb00() -> ! {
    todo!("0xc2cb00 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")
}

#[doc(alias = "void boost::call_once<void (*)(void)>(boost::once_flag &,void (*)(void))")]
// 0xc2cc10 — __ZN5boost9call_onceIPFvvEEEvRNS_9once_flagET_
pub fn stub_c2cc10() -> ! {
    todo!("0xc2cc10 __ZN5boost9call_onceIPFvvEEEvRNS_9once_flagET_")
}

#[doc(alias = "boost::mutex::lock(void)")]
// 0xc2cde0 — __ZN5boost5mutex4lockEv
pub fn stub_c2cde0() -> ! {
    todo!("0xc2cde0 __ZN5boost5mutex4lockEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ICreator const*>> *)")]
// 0xc2d550 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
pub fn stub_c2d550() -> ! {
    todo!("0xc2d550 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
// 0xc2d580 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
pub fn stub_c2d580() -> ! {
    todo!("0xc2d580 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")
}

#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
// 0xc2d7f0 — __ZThn32_N3RBX9BlockMeshD1Ev
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
pub fn stub_c2d7f0() -> ! {
    todo!("0xc2d7f0 __ZThn32_N3RBX9BlockMeshD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
// 0xc2d800 — __ZThn36_N3RBX9BlockMeshD1Ev
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
pub fn stub_c2d800() -> ! {
    todo!("0xc2d800 __ZThn36_N3RBX9BlockMeshD1Ev")
}

#[doc(alias = "std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LightGridChunk **,std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>>,RBX::LightGridChunk * const&)")]
// 0xc2e058 — __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_c2e058() -> ! {
    todo!("0xc2e058 __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::LightObject *,std::allocator<RBX::LightObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LightObject **,std::vector<RBX::LightObject *,std::allocator<RBX::LightObject *>>>,RBX::LightObject * const&)")]
// 0xc2f8c4 — __ZNSt6vectorIPN3RBX11LightObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_c2f8c4() -> ! {
    todo!("0xc2f8c4 __ZNSt6vectorIPN3RBX11LightObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::xline_iterator::xline_iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// 0xc30224 — __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE14xline_iteratorC2ERKS4_
pub fn stub_c30224() -> ! {
    todo!("0xc30224 __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE14xline_iteratorC2ERKS4_")
}

#[doc(alias = "std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::LightGridChunk **,std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>>,unsigned long,RBX::LightGridChunk * const&)")]
// 0xc30328 — __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_c30328() -> ! {
    todo!("0xc30328 __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_impl(boost::exception_detail::bad_exception_ const&)")]
// 0xc30650 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS2_
pub fn stub_c30650() -> ! {
    todo!("0xc30650 __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS2_")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const")]
// 0xc307a0 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const
pub fn stub_c307a0() -> ! {
    todo!("0xc307a0 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
// 0xc307b0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()
pub fn stub_c307b0() -> ! {
    todo!("0xc307b0 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev")
}

#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
// 0xc30870 — __ZN5boost16exception_detail14bad_exception_D0Ev
pub fn stub_c30870() -> ! {
    todo!("0xc30870 __ZN5boost16exception_detail14bad_exception_D0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::dispose(void)")]
// 0xc30930 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE7disposeEv
pub fn stub_c30930() -> ! {
    todo!("0xc30930 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::get_deleter(std::type_info const&)")]
// 0xc30940 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE11get_deleterERKSt9type_info
pub fn stub_c30940() -> ! {
    todo!("0xc30940 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
// 0xc30950 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev
pub fn stub_c30950() -> ! {
    todo!("0xc30950 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const")]
// 0xc30a10 — __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv
pub fn stub_c30a10() -> ! {
    todo!("0xc30a10 __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
// 0xc30b20 — __ZThn20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()
pub fn stub_c30b20() -> ! {
    todo!("0xc30b20 __ZThn20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::~sp_counted_impl_p()")]
// 0xc30be0 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEED1Ev
pub fn stub_c30be0() -> ! {
    todo!("0xc30be0 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEED1Ev")
}

#[doc(alias = "RBX::LightObject::~LightObject()")]
// 0xc31b6c — __ZN3RBX11LightObjectD0Ev
pub fn stub_c31b6c() -> ! {
    todo!("0xc31b6c __ZN3RBX11LightObjectD0Ev")
}

#[doc(alias = "RBX::LightObject::~LightObject()")]
// 0xc31c20 — __ZN3RBX11LightObjectD1Ev
pub fn stub_c31c20() -> ! {
    todo!("0xc31c20 __ZN3RBX11LightObjectD1Ev")
}

#[doc(alias = "RBX::LightObject::~LightObject()")]
// 0xc31c24 — __ZN3RBX11LightObjectD2Ev
pub fn stub_c31c24() -> ! {
    todo!("0xc31c24 __ZN3RBX11LightObjectD2Ev")
}

#[doc(alias = "RBX::LightObject::onSleepingChangedEx(bool)")]
// 0xc31dbc — __ZN3RBX11LightObject19onSleepingChangedExEb
pub fn stub_c31dbc() -> ! {
    todo!("0xc31dbc __ZN3RBX11LightObject19onSleepingChangedExEb")
}

#[doc(alias = "RBX::LightObject::updateCoordinateFrame(bool)")]
// 0xc31ee0 — __ZN3RBX11LightObject21updateCoordinateFrameEb
pub fn stub_c31ee0() -> ! {
    todo!("0xc31ee0 __ZN3RBX11LightObject21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::LightObject::getLightPosition(void)const")]
// 0xc3281c — __ZNK3RBX11LightObject16getLightPositionEv
pub fn stub_c3281c() -> ! {
    todo!("0xc3281c __ZNK3RBX11LightObject16getLightPositionEv")
}

#[doc(alias = "RBX::LightObject::getLightDirection(void)const")]
// 0xc3282c — __ZNK3RBX11LightObject17getLightDirectionEv
pub fn stub_c3282c() -> ! {
    todo!("0xc3282c __ZNK3RBX11LightObject17getLightDirectionEv")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::updateCoordinateFrame(bool)")]
// 0xc3283c — __ZThn392_N3RBX11LightObject21updateCoordinateFrameEb
// was: `non-virtual thunk to'RBX::LightObject::updateCoordinateFrame(bool)
pub fn stub_c3283c() -> ! {
    todo!("0xc3283c __ZThn392_N3RBX11LightObject21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::LightObject::onAncestorChangedEx(void)")]
// 0xc32860 — __ZN3RBX11LightObject19onAncestorChangedExEv
pub fn stub_c32860() -> ! {
    todo!("0xc32860 __ZN3RBX11LightObject19onAncestorChangedExEv")
}

#[doc(alias = "RBX::LightObject::unbind(void)")]
// 0xc331b4 — __ZN3RBX11LightObject6unbindEv
pub fn stub_c331b4() -> ! {
    todo!("0xc331b4 __ZN3RBX11LightObject6unbindEv")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::unbind(void)")]
// 0xc331f4 — __ZThn392_N3RBX11LightObject6unbindEv
// was: `non-virtual thunk to'RBX::LightObject::unbind(void)
pub fn stub_c331f4() -> ! {
    todo!("0xc331f4 __ZThn392_N3RBX11LightObject6unbindEv")
}

#[doc(alias = "RBX::LightObject::invalidateEntity(void)")]
// 0xc3322c — __ZN3RBX11LightObject16invalidateEntityEv
pub fn stub_c3322c() -> ! {
    todo!("0xc3322c __ZN3RBX11LightObject16invalidateEntityEv")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::invalidateEntity(void)")]
// 0xc3325c — __ZThn392_N3RBX11LightObject16invalidateEntityEv
// was: `non-virtual thunk to'RBX::LightObject::invalidateEntity(void)
pub fn stub_c3325c() -> ! {
    todo!("0xc3325c __ZThn392_N3RBX11LightObject16invalidateEntityEv")
}

#[doc(alias = "RBX::LightObject::updateEntity(bool)")]
// 0xc33290 — __ZN3RBX11LightObject12updateEntityEb
pub fn stub_c33290() -> ! {
    todo!("0xc33290 __ZN3RBX11LightObject12updateEntityEb")
}

#[doc(alias = "RBX::resizeShadowProjection(boost::scoped_array<unsigned char> &,unsigned int,unsigned int)")]
// 0xc334e4 — __ZN3RBXL22resizeShadowProjectionERN5boost12scoped_arrayIhEEjj
pub fn stub_c334e4() -> ! {
    todo!("0xc334e4 __ZN3RBXL22resizeShadowProjectionERN5boost12scoped_arrayIhEEjj")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::updateEntity(bool)")]
// 0xc33680 — __ZThn392_N3RBX11LightObject12updateEntityEb
// was: `non-virtual thunk to'RBX::LightObject::updateEntity(bool)
pub fn stub_c33680() -> ! {
    todo!("0xc33680 __ZThn392_N3RBX11LightObject12updateEntityEb")
}

#[doc(alias = "RBX::LightObject::_updateBounds(void)")]
// 0xc3368c — __ZN3RBX11LightObject13_updateBoundsEv
pub fn stub_c3368c() -> ! {
    todo!("0xc3368c __ZN3RBX11LightObject13_updateBoundsEv")
}

#[doc(alias = "RBX::LightObject::getLightExtents(void)const")]
// 0xc33690 — __ZNK3RBX11LightObject15getLightExtentsEv
pub fn stub_c33690() -> ! {
    todo!("0xc33690 __ZNK3RBX11LightObject15getLightExtentsEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>>::~callable_slot()")]
// 0xc336a0 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
pub fn stub_c336a0() -> ! {
    todo!("0xc336a0 __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>>::~callable_slot()")]
// 0xc336fc — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
pub fn stub_c336fc() -> ! {
    todo!("0xc336fc __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0xc33804 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
pub fn stub_c33804() -> ! {
    todo!("0xc33804 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0xc3381c — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
pub fn stub_c3381c() -> ! {
    todo!("0xc3381c __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>>::~callable_slot()")]
// 0xc339d0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_c339d0() -> ! {
    todo!("0xc339d0 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>>::~callable_slot()")]
// 0xc33a2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_c33a2c() -> ! {
    todo!("0xc33a2c __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>,0,void ()(void)>::call(void)")]
// 0xc33b34 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_c33b34() -> ! {
    todo!("0xc33b34 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>,0,void ()(void)>::call(void)")]
// 0xc33b4c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>,0,void ()(void)>::call(void)
pub fn stub_c33b4c() -> ! {
    todo!("0xc33b4c __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "boost::scoped_ptr<RBX::LightShadowMap>::~scoped_ptr()")]
// 0xc3491c — __ZN5boost10scoped_ptrIN3RBX14LightShadowMapEED2Ev
pub fn stub_c3491c() -> ! {
    todo!("0xc3491c __ZN5boost10scoped_ptrIN3RBX14LightShadowMapEED2Ev")
}

#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// 0xc35418 — __ZN3RBX12RenderEntityD0Ev
pub fn stub_c35418() -> ! {
    todo!("0xc35418 __ZN3RBX12RenderEntityD0Ev")
}

#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// 0xc354b8 — __ZN3RBX12RenderEntityD1Ev
pub fn stub_c354b8() -> ! {
    todo!("0xc354b8 __ZN3RBX12RenderEntityD1Ev")
}

#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// 0xc354bc — __ZN3RBX12RenderEntityD2Ev
pub fn stub_c354bc() -> ! {
    todo!("0xc354bc __ZN3RBX12RenderEntityD2Ev")
}

#[doc(alias = "RBX::RenderEntity::getActualMaterial(void)const")]
// 0xc35838 — __ZNK3RBX12RenderEntity17getActualMaterialEv
pub fn stub_c35838() -> ! {
    todo!("0xc35838 __ZNK3RBX12RenderEntity17getActualMaterialEv")
}

#[doc(alias = "RBX::RenderEntity::getMaterial(void)const")]
// 0xc35980 — __ZNK3RBX12RenderEntity11getMaterialEv
pub fn stub_c35980() -> ! {
    todo!("0xc35980 __ZNK3RBX12RenderEntity11getMaterialEv")
}

#[doc(alias = "RBX::RenderEntity::getTechnique(void)const")]
// 0xc359e8 — __ZNK3RBX12RenderEntity12getTechniqueEv
pub fn stub_c359e8() -> ! {
    todo!("0xc359e8 __ZNK3RBX12RenderEntity12getTechniqueEv")
}

#[doc(alias = "RBX::RenderEntity::getDebugMaterial(void)const")]
// 0xc359ec — __ZNK3RBX12RenderEntity16getDebugMaterialEv
pub fn stub_c359ec() -> ! {
    todo!("0xc359ec __ZNK3RBX12RenderEntity16getDebugMaterialEv")
}

#[doc(alias = "RBX::RenderEntity::getLights(void)const")]
// 0xc35aa8 — __ZNK3RBX12RenderEntity9getLightsEv
pub fn stub_c35aa8() -> ! {
    todo!("0xc35aa8 __ZNK3RBX12RenderEntity9getLightsEv")
}

#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// 0xc35b20 — __ZN3RBX10RenderNodeD0Ev
pub fn stub_c35b20() -> ! {
    todo!("0xc35b20 __ZN3RBX10RenderNodeD0Ev")
}

#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// 0xc35bd4 — __ZN3RBX10RenderNodeD1Ev
pub fn stub_c35bd4() -> ! {
    todo!("0xc35bd4 __ZN3RBX10RenderNodeD1Ev")
}

#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// 0xc35bd8 — __ZN3RBX10RenderNodeD2Ev
pub fn stub_c35bd8() -> ! {
    todo!("0xc35bd8 __ZN3RBX10RenderNodeD2Ev")
}

#[doc(alias = "RBX::RenderNode::addEntity(RBX::RenderEntity *)")]
// 0xc35d9c — __ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE
pub fn stub_c35d9c() -> ! {
    todo!("0xc35d9c __ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE")
}

#[doc(alias = "RBX::RenderNode::removeEntity(RBX::RenderEntity *)")]
// 0xc35e2c — __ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE
pub fn stub_c35e2c() -> ! {
    todo!("0xc35e2c __ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE")
}

#[doc(alias = "RBX::RenderNode::getFastFuzzyExtents(void)")]
// 0xc35f64 — __ZN3RBX10RenderNode19getFastFuzzyExtentsEv
pub fn stub_c35f64() -> ! {
    todo!("0xc35f64 __ZN3RBX10RenderNode19getFastFuzzyExtentsEv")
}

#[doc(alias = "non-virtual thunk toRBX::RenderNode::getFastFuzzyExtents(void)")]
// 0xc35f74 — __ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv
// was: `non-virtual thunk to'RBX::RenderNode::getFastFuzzyExtents(void)
pub fn stub_c35f74() -> ! {
    todo!("0xc35f74 __ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv")
}

#[doc(alias = "RBX::RenderNode::_updateBounds(void)")]
// 0xc35f80 — __ZN3RBX10RenderNode13_updateBoundsEv
pub fn stub_c35f80() -> ! {
    todo!("0xc35f80 __ZN3RBX10RenderNode13_updateBoundsEv")
}

#[doc(alias = "std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)")]
// 0xc3602c — __ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_c3602c() -> ! {
    todo!("0xc3602c __ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
// 0xc36a48 — __ZN3RBX9WaterImplD1Ev
pub fn stub_c36a48() -> ! {
    todo!("0xc36a48 __ZN3RBX9WaterImplD1Ev")
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
// 0xc36a4c — __ZN3RBX9WaterImplD0Ev
pub fn stub_c36a4c() -> ! {
    todo!("0xc36a4c __ZN3RBX9WaterImplD0Ev")
}

#[doc(alias = "RBX::WaterImpl::activate(void)")]
// 0xc36af4 — __ZN3RBX9WaterImpl8activateEv
pub fn stub_c36af4() -> ! {
    todo!("0xc36af4 __ZN3RBX9WaterImpl8activateEv")
}

#[doc(alias = "RBX::WaterImpl::update(void)")]
// 0xc36b10 — __ZN3RBX9WaterImpl6updateEv
pub fn stub_c36b10() -> ! {
    todo!("0xc36b10 __ZN3RBX9WaterImpl6updateEv")
}

#[doc(alias = "RBX::WaterImpl::underwater(void)")]
// 0xc37310 — __ZN3RBX9WaterImpl10underwaterEv
pub fn stub_c37310() -> ! {
    todo!("0xc37310 __ZN3RBX9WaterImpl10underwaterEv")
}

#[doc(alias = "RBX::WaterImpl::load(void)")]
// 0xc37cb0 — __ZN3RBX9WaterImpl4loadEv
pub fn stub_c37cb0() -> ! {
    todo!("0xc37cb0 __ZN3RBX9WaterImpl4loadEv")
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
// 0xc38384 — __ZN3RBX9WaterImplD2Ev
pub fn stub_c38384() -> ! {
    todo!("0xc38384 __ZN3RBX9WaterImplD2Ev")
}

#[doc(alias = "std::vector<double,std::allocator<double>>::_M_fill_insert(__gnu_cxx::__normal_iterator<double *,std::vector<double,std::allocator<double>>>,unsigned long,double const&)")]
// 0xc39e78 — __ZNSt6vectorIdSaIdEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPdS1_EEmRKd
pub fn stub_c39e78() -> ! {
    todo!("0xc39e78 __ZNSt6vectorIdSaIdEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPdS1_EEmRKd")
}

#[doc(alias = "std::vector<float,std::allocator<float>>::_M_fill_insert(__gnu_cxx::__normal_iterator<float *,std::vector<float,std::allocator<float>>>,unsigned long,float const&)")]
// 0xc39fec — __ZNSt6vectorIfSaIfEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS1_EEmRKf
pub fn stub_c39fec() -> ! {
    todo!("0xc39fec __ZNSt6vectorIfSaIfEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS1_EEmRKf")
}

#[doc(alias = "std::vector<short,std::allocator<short>>::_M_fill_insert(__gnu_cxx::__normal_iterator<short *,std::vector<short,std::allocator<short>>>,unsigned long,short const&)")]
// 0xc3a164 — __ZNSt6vectorIsSaIsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPsS1_EEmRKs
pub fn stub_c3a164() -> ! {
    todo!("0xc3a164 __ZNSt6vectorIsSaIsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPsS1_EEmRKs")
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned char *,std::vector<unsigned char,std::allocator<unsigned char>>>,unsigned long,unsigned char const&)")]
// 0xc3a2bc — __ZNSt6vectorIhSaIhEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS1_EEmRKh
pub fn stub_c3a2bc() -> ! {
    todo!("0xc3a2bc __ZNSt6vectorIhSaIhEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS1_EEmRKh")
}