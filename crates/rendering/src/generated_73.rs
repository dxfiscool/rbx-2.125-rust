//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xc96110..0xc9bfc4 (100 stubs, 8355 prior -> +100, 4878 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xc96110 — __ZN4Ogre15ResourceManager14resourceExistsERKSs
#[doc(alias = "Ogre::ResourceManager::resourceExists(std::string const&)")]
// was: Ogre::ResourceManager::resourceExists(std::string const&)
// IDA 0xc96110: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96110() {
}

// 0xc96220 — __ZN4Ogre15ResourceManager14resourceExistsEy
#[doc(alias = "Ogre::ResourceManager::resourceExists(unsigned long long)")]
// was: Ogre::ResourceManager::resourceExists(unsigned long long)
// IDA 0xc96220: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96220() {
}

// 0xc96324 — __ZN4Ogre15ResourceManager10setVerboseEb
#[doc(alias = "Ogre::ResourceManager::setVerbose(bool)")]
// was: Ogre::ResourceManager::setVerbose(bool)
// IDA 0xc96324: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96324() {
}

// 0xc9632c — __ZN4Ogre15ResourceManager10getVerboseEv
#[doc(alias = "Ogre::ResourceManager::getVerbose(void)")]
// was: Ogre::ResourceManager::getVerbose(void)
// IDA 0xc9632c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9632c() {
}

// 0xc96338 — __ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
#[doc(alias = "std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xc96338: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96338() {
}

// 0xc96488 — __ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
#[doc(alias = "std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)")]
// was: std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)
// IDA 0xc96488: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_c96488() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xc965a4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>(unsigned int const&,std::pair<unsigned int const,Ogre::Font::GlyphInfo> &&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>(unsigned int const&,std::pair<unsigned int const,Ogre::Font::GlyphInfo> &&)
// IDA 0xc965a4: 213 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c965a4() {
}

// 0xc967d8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)
// IDA 0xc967d8: 148 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c967d8() {
}

// 0xc96980 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)
// IDA 0xc96980: 56 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96980() {
}

// 0xc96a30 — __ZN4Ogre9SharedPtrINS_4FontEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Font>::~SharedPtr()
// IDA 0xc96a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96a30() {
}

// 0xc96ae0 — __ZN4Ogre9SharedPtrINS_4FontEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Font>::~SharedPtr()
// IDA 0xc96ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96ae0() {
}

// 0xc96bd4 — __ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::Font>::destroy(void)
// IDA 0xc96bd4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96bd4() {
}

// 0xc96c0c — __ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::swap(Ogre::SharedPtr<Ogre::Font>&)")]
// was: Ogre::SharedPtr<Ogre::Font>::swap(Ogre::SharedPtr<Ogre::Font>&)
// IDA 0xc96c0c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96c0c() {
}

// 0xc96c28 — __ZN4Ogre7FontPtrD0Ev
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
// was: Ogre::FontPtr::~FontPtr()
// IDA 0xc96c28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96c28() {
}

// 0xc96d50 — __ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc
#[doc(alias = "Ogre::FreeImageLoadErrorHandler(FREE_IMAGE_FORMAT,char const*)")]
// was: Ogre::FreeImageLoadErrorHandler(FREE_IMAGE_FORMAT,char const*)
// IDA 0xc96d50: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96d50() {
}

// 0xc96f14 — __ZN4Ogre14FreeImageCodec7startupEv
#[doc(alias = "Ogre::FreeImageCodec::startup(void)")]
// was: Ogre::FreeImageCodec::startup(void)
// IDA 0xc96f14: 761 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96f14() {
}

// 0xc9778c — __ZN4Ogre14FreeImageCodec8shutdownEv
#[doc(alias = "Ogre::FreeImageCodec::shutdown(void)")]
// was: Ogre::FreeImageCodec::shutdown(void)
// IDA 0xc9778c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9778c() {
}

// 0xc977e0 — __ZNK4Ogre14FreeImageCodec6encodeERNS_9SharedPtrINS_16MemoryDataStreamEEERNS1_INS_5Codec9CodecDataEEE
#[doc(alias = "Ogre::FreeImageCodec::encode(Ogre::SharedPtr<Ogre::MemoryDataStream> &,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const")]
// was: Ogre::FreeImageCodec::encode(Ogre::SharedPtr<Ogre::MemoryDataStream> &,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const
// IDA 0xc977e0: 548 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c977e0() {
}

// 0xc97db4 — __ZNK4Ogre14FreeImageCodec4codeERNS_9SharedPtrINS_16MemoryDataStreamEEERNS1_INS_5Codec9CodecDataEEE
#[doc(alias = "Ogre::FreeImageCodec::code(Ogre::SharedPtr<Ogre::MemoryDataStream> &,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const")]
// was: Ogre::FreeImageCodec::code(Ogre::SharedPtr<Ogre::MemoryDataStream> &,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const
// IDA 0xc97db4: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c97db4() {
}

// 0xc97f68 — __ZNK4Ogre14FreeImageCodec10codeToFileERNS_9SharedPtrINS_16MemoryDataStreamEEERKSsRNS1_INS_5Codec9CodecDataEEE
#[doc(alias = "Ogre::FreeImageCodec::codeToFile(Ogre::SharedPtr<Ogre::MemoryDataStream> &,std::string const&,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const")]
// was: Ogre::FreeImageCodec::codeToFile(Ogre::SharedPtr<Ogre::MemoryDataStream> &,std::string const&,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const
// IDA 0xc97f68: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c97f68() {
}

// 0xc97f8c — __ZNK4Ogre14FreeImageCodec6decodeERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::FreeImageCodec::decode(Ogre::SharedPtr<Ogre::DataStream> &)const")]
// was: Ogre::FreeImageCodec::decode(Ogre::SharedPtr<Ogre::DataStream> &)const
// IDA 0xc97f8c: 894 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c97f8c() {
}

// 0xc988c4 — __ZNK4Ogre14FreeImageCodec7getTypeEv
#[doc(alias = "Ogre::FreeImageCodec::getType(void)const")]
// was: Ogre::FreeImageCodec::getType(void)const
// IDA 0xc988c4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c988c4() {
}

// 0xc988d0 — __ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm
#[doc(alias = "Ogre::FreeImageCodec::magicNumberToFileExt(char const*,unsigned long)const")]
// was: Ogre::FreeImageCodec::magicNumberToFileExt(char const*,unsigned long)const
// IDA 0xc988d0: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c988d0() {
}

// 0xc98a48 — __ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::list<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~list()")]
// was: std::list<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~list()
// IDA 0xc98a48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98a48() {
}

// 0xc98af0 — __ZN4Ogre14FreeImageCodecD1Ev
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
// was: Ogre::FreeImageCodec::~FreeImageCodec()
// IDA 0xc98af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98af0() {
}

// 0xc98b50 — __ZN4Ogre14FreeImageCodecD0Ev
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
// was: Ogre::FreeImageCodec::~FreeImageCodec()
// IDA 0xc98b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98b50() {
}

// 0xc98c30 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xc98c30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c98c30() {
}

// 0xc98c34 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xc98c34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98c34() {
}

// 0xc98cb4 — __ZN4Ogre7FrustumC1ERKSs
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
// was: Ogre::Frustum::Frustum(std::string const&)
// IDA 0xc98cb4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c98cb4() {
}

// 0xc98cc0 — __ZN4Ogre7FrustumC2ERKSs
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
// was: Ogre::Frustum::Frustum(std::string const&)
// IDA 0xc98cc0: 617 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c98cc0() {
}

// 0xc99398 — __ZN4Ogre7FrustumD0Ev
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: Ogre::Frustum::~Frustum()
// IDA 0xc99398: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c99398() {
}

// 0xc99428 — __ZN4Ogre7FrustumD1Ev
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: Ogre::Frustum::~Frustum()
// IDA 0xc99428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c99428() {
}

// 0xc99434 — __ZThn4_N4Ogre7FrustumD0Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: non-virtual thunk toOgre::Frustum::~Frustum()
// IDA 0xc99434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c99434() {
}

// 0xc994c8 — __ZThn188_N4Ogre7FrustumD0Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: non-virtual thunk toOgre::Frustum::~Frustum()
// IDA 0xc994c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c994c8() {
}

// 0xc9955c — __ZN4Ogre7FrustumD2Ev
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: Ogre::Frustum::~Frustum()
// IDA 0xc9955c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c9955c() {
}

// 0xc996e0 — __ZThn4_N4Ogre7FrustumD1Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: non-virtual thunk toOgre::Frustum::~Frustum()
// IDA 0xc996e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c996e0() {
}

// 0xc996ec — __ZThn188_N4Ogre7FrustumD1Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: non-virtual thunk toOgre::Frustum::~Frustum()
// IDA 0xc996ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c996ec() {
}

// 0xc996f8 — __ZN4Ogre7Frustum7setFOVyERKNS_6RadianE
#[doc(alias = "Ogre::Frustum::setFOVy(Ogre::Radian const&)")]
// was: Ogre::Frustum::setFOVy(Ogre::Radian const&)
// IDA 0xc996f8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c996f8() {
}

// 0xc9970c — __ZNK4Ogre7Frustum7getFOVyEv
#[doc(alias = "Ogre::Frustum::getFOVy(void)const")]
// was: Ogre::Frustum::getFOVy(void)const
// IDA 0xc9970c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9970c() {
}

// 0xc99710 — __ZN4Ogre7Frustum18setFarClipDistanceEf
#[doc(alias = "Ogre::Frustum::setFarClipDistance(float)")]
// was: Ogre::Frustum::setFarClipDistance(float)
// IDA 0xc99710: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99710() {
}

// 0xc99724 — __ZNK4Ogre7Frustum18getFarClipDistanceEv
#[doc(alias = "Ogre::Frustum::getFarClipDistance(void)const")]
// was: Ogre::Frustum::getFarClipDistance(void)const
// IDA 0xc99724: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99724() {
}

// 0xc9972c — __ZN4Ogre7Frustum19setNearClipDistanceEf
#[doc(alias = "Ogre::Frustum::setNearClipDistance(float)")]
// was: Ogre::Frustum::setNearClipDistance(float)
// IDA 0xc9972c: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9972c() {
}

// 0xc9991c — __ZNK4Ogre7Frustum19getNearClipDistanceEv
#[doc(alias = "Ogre::Frustum::getNearClipDistance(void)const")]
// was: Ogre::Frustum::getNearClipDistance(void)const
// IDA 0xc9991c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9991c() {
}

// 0xc99924 — __ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E
#[doc(alias = "Ogre::Frustum::setFrustumOffset(Ogre::Vector2 const&)")]
// was: Ogre::Frustum::setFrustumOffset(Ogre::Vector2 const&)
// IDA 0xc99924: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99924() {
}

// 0xc99940 — __ZN4Ogre7Frustum16setFrustumOffsetEff
#[doc(alias = "Ogre::Frustum::setFrustumOffset(float,float)")]
// was: Ogre::Frustum::setFrustumOffset(float,float)
// IDA 0xc99940: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99940() {
}

// 0xc99958 — __ZNK4Ogre7Frustum16getFrustumOffsetEv
#[doc(alias = "Ogre::Frustum::getFrustumOffset(void)const")]
// was: Ogre::Frustum::getFrustumOffset(void)const
// IDA 0xc99958: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99958() {
}

// 0xc99960 — __ZN4Ogre7Frustum14setFocalLengthEf
#[doc(alias = "Ogre::Frustum::setFocalLength(float)")]
// was: Ogre::Frustum::setFocalLength(float)
// IDA 0xc99960: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99960() {
}

// 0xc99b50 — __ZNK4Ogre7Frustum14getFocalLengthEv
#[doc(alias = "Ogre::Frustum::getFocalLength(void)const")]
// was: Ogre::Frustum::getFocalLength(void)const
// IDA 0xc99b50: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b50() {
}

// 0xc99b58 — __ZNK4Ogre7Frustum19getProjectionMatrixEv
#[doc(alias = "Ogre::Frustum::getProjectionMatrix(void)const")]
// was: Ogre::Frustum::getProjectionMatrix(void)const
// IDA 0xc99b58: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b58() {
}

// 0xc99b70 — __ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv
#[doc(alias = "Ogre::Frustum::getProjectionMatrixWithRSDepth(void)const")]
// was: Ogre::Frustum::getProjectionMatrixWithRSDepth(void)const
// IDA 0xc99b70: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b70() {
}

// 0xc99b88 — __ZNK4Ogre7Frustum21getProjectionMatrixRSEv
#[doc(alias = "Ogre::Frustum::getProjectionMatrixRS(void)const")]
// was: Ogre::Frustum::getProjectionMatrixRS(void)const
// IDA 0xc99b88: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b88() {
}

// 0xc99ba0 — __ZNK4Ogre7Frustum13getViewMatrixEv
#[doc(alias = "Ogre::Frustum::getViewMatrix(void)const")]
// was: Ogre::Frustum::getViewMatrix(void)const
// IDA 0xc99ba0: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99ba0() {
}

// 0xc99bb8 — __ZNK4Ogre7Frustum16getFrustumPlanesEv
#[doc(alias = "Ogre::Frustum::getFrustumPlanes(void)const")]
// was: Ogre::Frustum::getFrustumPlanes(void)const
// IDA 0xc99bb8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99bb8() {
}

// 0xc99bd0 — __ZNK4Ogre7Frustum15getFrustumPlaneEt
#[doc(alias = "Ogre::Frustum::getFrustumPlane(unsigned short)const")]
// was: Ogre::Frustum::getFrustumPlane(unsigned short)const
// IDA 0xc99bd0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99bd0() {
}

// 0xc99bec — __ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)const")]
// was: Ogre::Frustum::isVisible(Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)const
// IDA 0xc99bec: 102 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99bec() {
}

// 0xc99d28 — __ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Vector3 const&,Ogre::FrustumPlane *)const")]
// was: Ogre::Frustum::isVisible(Ogre::Vector3 const&,Ogre::FrustumPlane *)const
// IDA 0xc99d28: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99d28() {
}

// 0xc99d90 — __ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Sphere const&,Ogre::FrustumPlane *)const")]
// was: Ogre::Frustum::isVisible(Ogre::Sphere const&,Ogre::FrustumPlane *)const
// IDA 0xc99d90: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99d90() {
}

// 0xc99e0c — __ZNK4Ogre7Frustum12getTypeFlagsEv
#[doc(alias = "Ogre::Frustum::getTypeFlags(void)const")]
// was: Ogre::Frustum::getTypeFlags(void)const
// IDA 0xc99e0c: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99e0c() {
}

// 0xc99e1c — __ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_
#[doc(alias = "Ogre::Frustum::calcProjectionParameters(float &,float &,float &,float &)const")]
// was: Ogre::Frustum::calcProjectionParameters(float &,float &,float &,float &)const
// IDA 0xc99e1c: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99e1c() {
}

// 0xc9a054 — __ZNK4Ogre7Frustum17updateFrustumImplEv
#[doc(alias = "Ogre::Frustum::updateFrustumImpl(void)const")]
// was: Ogre::Frustum::updateFrustumImpl(void)const
// IDA 0xc9a054: 394 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9a054() {
}

// 0xc9a5c0 — __ZNK4Ogre7Frustum13updateFrustumEv
#[doc(alias = "Ogre::Frustum::updateFrustum(void)const")]
// was: Ogre::Frustum::updateFrustum(void)const
// IDA 0xc9a5c0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9a5c0() {
}

// 0xc9a5e4 — __ZNK4Ogre7Frustum16updateVertexDataEv
#[doc(alias = "Ogre::Frustum::updateVertexData(void)const")]
// was: Ogre::Frustum::updateVertexData(void)const
// IDA 0xc9a5e4: 512 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9a5e4() {
}

// 0xc9ab80 — __ZNK4Ogre7Frustum15isViewOutOfDateEv
#[doc(alias = "Ogre::Frustum::isViewOutOfDate(void)const")]
// was: Ogre::Frustum::isViewOutOfDate(void)const
// IDA 0xc9ab80: 146 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9ab80() {
}

// 0xc9ad5c — __ZNK4Ogre7Frustum18isFrustumOutOfDateEv
#[doc(alias = "Ogre::Frustum::isFrustumOutOfDate(void)const")]
// was: Ogre::Frustum::isFrustumOutOfDate(void)const
// IDA 0xc9ad5c: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9ad5c() {
}

// 0xc9ae20 — __ZNK4Ogre7Frustum14updateViewImplEv
#[doc(alias = "Ogre::Frustum::updateViewImpl(void)const")]
// was: Ogre::Frustum::updateViewImpl(void)const
// IDA 0xc9ae20: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9ae20() {
}

// 0xc9aeb8 — __ZNK4Ogre7Frustum22calcViewMatrixRelativeERKNS_7Vector3ERNS_7Matrix4E
#[doc(alias = "Ogre::Frustum::calcViewMatrixRelative(Ogre::Vector3 const&,Ogre::Matrix4 &)const")]
// was: Ogre::Frustum::calcViewMatrixRelative(Ogre::Vector3 const&,Ogre::Matrix4 &)const
// IDA 0xc9aeb8: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9aeb8() {
}

// 0xc9af60 — __ZNK4Ogre7Frustum10updateViewEv
#[doc(alias = "Ogre::Frustum::updateView(void)const")]
// was: Ogre::Frustum::updateView(void)const
// IDA 0xc9af60: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9af60() {
}

// 0xc9af84 — __ZNK4Ogre7Frustum23updateFrustumPlanesImplEv
#[doc(alias = "Ogre::Frustum::updateFrustumPlanesImpl(void)const")]
// was: Ogre::Frustum::updateFrustumPlanesImpl(void)const
// IDA 0xc9af84: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9af84() {
}

// 0xc9b11c — __ZNK4Ogre7Frustum19updateFrustumPlanesEv
#[doc(alias = "Ogre::Frustum::updateFrustumPlanes(void)const")]
// was: Ogre::Frustum::updateFrustumPlanes(void)const
// IDA 0xc9b11c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b11c() {
}

// 0xc9b14c — __ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCornersImpl(void)const")]
// was: Ogre::Frustum::updateWorldSpaceCornersImpl(void)const
// IDA 0xc9b14c: 197 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b14c() {
}

// 0xc9b438 — __ZNK4Ogre7Frustum23updateWorldSpaceCornersEv
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCorners(void)const")]
// was: Ogre::Frustum::updateWorldSpaceCorners(void)const
// IDA 0xc9b438: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b438() {
}

// 0xc9b460 — __ZNK4Ogre7Frustum14getAspectRatioEv
#[doc(alias = "Ogre::Frustum::getAspectRatio(void)const")]
// was: Ogre::Frustum::getAspectRatio(void)const
// IDA 0xc9b460: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b460() {
}

// 0xc9b468 — __ZN4Ogre7Frustum14setAspectRatioEf
#[doc(alias = "Ogre::Frustum::setAspectRatio(float)")]
// was: Ogre::Frustum::setAspectRatio(float)
// IDA 0xc9b468: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b468() {
}

// 0xc9b47c — __ZNK4Ogre7Frustum14getBoundingBoxEv
#[doc(alias = "Ogre::Frustum::getBoundingBox(void)const")]
// was: Ogre::Frustum::getBoundingBox(void)const
// IDA 0xc9b47c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b47c() {
}

// 0xc9b484 — __ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::Frustum::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::Frustum::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xc9b484: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b484() {
}

// 0xc9b4a0 — __ZNK4Ogre7Frustum14getMovableTypeEv
#[doc(alias = "Ogre::Frustum::getMovableType(void)const")]
// was: Ogre::Frustum::getMovableType(void)const
// IDA 0xc9b4a0: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4a0() {
}

// 0xc9b4ac — __ZNK4Ogre7Frustum17getBoundingRadiusEv
#[doc(alias = "Ogre::Frustum::getBoundingRadius(void)const")]
// was: Ogre::Frustum::getBoundingRadius(void)const
// IDA 0xc9b4ac: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4ac() {
}

// 0xc9b4cc — __ZNK4Ogre7Frustum11getMaterialEv
#[doc(alias = "Ogre::Frustum::getMaterial(void)const")]
// was: Ogre::Frustum::getMaterial(void)const
// IDA 0xc9b4cc: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4cc() {
}

// 0xc9b4d4 — __ZThn188_NK4Ogre7Frustum11getMaterialEv
#[doc(alias = "non-virtual thunk toOgre::Frustum::getMaterial(void)const")]
// was: non-virtual thunk toOgre::Frustum::getMaterial(void)const
// IDA 0xc9b4d4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4d4() {
}

// 0xc9b4dc — __ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xc9b4dc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4dc() {
}

// 0xc9b500 — __ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
// was: non-virtual thunk toOgre::Frustum::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xc9b500: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b500() {
}

// 0xc9b524 — __ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xc9b524: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b524() {
}

// 0xc9b57c — __ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: non-virtual thunk toOgre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xc9b57c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b57c() {
}

// 0xc9b5d8 — __ZNK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xc9b5d8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b5d8() {
}

// 0xc9b638 — __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "non-virtual thunk toOgre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: non-virtual thunk toOgre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xc9b638: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b638() {
}

// 0xc9b69c — __ZNK4Ogre7Frustum9getLightsEv
#[doc(alias = "Ogre::Frustum::getLights(void)const")]
// was: Ogre::Frustum::getLights(void)const
// IDA 0xc9b69c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b69c() {
}

// 0xc9b784 — __ZThn188_NK4Ogre7Frustum9getLightsEv
#[doc(alias = "non-virtual thunk toOgre::Frustum::getLights(void)const")]
// was: non-virtual thunk toOgre::Frustum::getLights(void)const
// IDA 0xc9b784: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b784() {
}

// 0xc9b86c — __ZN4Ogre7Frustum20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::Frustum::_notifyCurrentCamera(Ogre::Camera *)")]
// was: Ogre::Frustum::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xc9b86c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b86c() {
}

// 0xc9b888 — __ZNK4Ogre7Frustum17invalidateFrustumEv
#[doc(alias = "Ogre::Frustum::invalidateFrustum(void)const")]
// was: Ogre::Frustum::invalidateFrustum(void)const
// IDA 0xc9b888: 6 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b888() {
}

// 0xc9b89c — __ZNK4Ogre7Frustum14invalidateViewEv
#[doc(alias = "Ogre::Frustum::invalidateView(void)const")]
// was: Ogre::Frustum::invalidateView(void)const
// IDA 0xc9b89c: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b89c() {
}

// 0xc9b8ac — __ZNK4Ogre7Frustum20getWorldSpaceCornersEv
#[doc(alias = "Ogre::Frustum::getWorldSpaceCorners(void)const")]
// was: Ogre::Frustum::getWorldSpaceCorners(void)const
// IDA 0xc9b8ac: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b8ac() {
}

// 0xc9b8c4 — __ZN4Ogre7Frustum17setProjectionTypeENS_14ProjectionTypeE
#[doc(alias = "Ogre::Frustum::setProjectionType(Ogre::ProjectionType)")]
// was: Ogre::Frustum::setProjectionType(Ogre::ProjectionType)
// IDA 0xc9b8c4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b8c4() {
}

// 0xc9b8d8 — __ZNK4Ogre7Frustum17getProjectionTypeEv
#[doc(alias = "Ogre::Frustum::getProjectionType(void)const")]
// was: Ogre::Frustum::getProjectionType(void)const
// IDA 0xc9b8d8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b8d8() {
}

// 0xc9b8e0 — __ZNK4Ogre7Frustum24getPositionForViewUpdateEv
#[doc(alias = "Ogre::Frustum::getPositionForViewUpdate(void)const")]
// was: Ogre::Frustum::getPositionForViewUpdate(void)const
// IDA 0xc9b8e0: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b8e0() {
}

// 0xc9b8e8 — __ZNK4Ogre7Frustum27getOrientationForViewUpdateEv
#[doc(alias = "Ogre::Frustum::getOrientationForViewUpdate(void)const")]
// was: Ogre::Frustum::getOrientationForViewUpdate(void)const
// IDA 0xc9b8e8: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b8e8() {
}

// 0xc9b8f0 — __ZN4Ogre7Frustum16enableReflectionERKNS_5PlaneE
#[doc(alias = "Ogre::Frustum::enableReflection(Ogre::Plane const&)")]
// was: Ogre::Frustum::enableReflection(Ogre::Plane const&)
// IDA 0xc9b8f0: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b8f0() {
}

// 0xc9b970 — __ZN4Ogre7Frustum16enableReflectionEPKNS_12MovablePlaneE
#[doc(alias = "Ogre::Frustum::enableReflection(Ogre::MovablePlane const*)")]
// was: Ogre::Frustum::enableReflection(Ogre::MovablePlane const*)
// IDA 0xc9b970: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b970() {
}

// 0xc9ba18 — __ZN4Ogre7Frustum17disableReflectionEv
#[doc(alias = "Ogre::Frustum::disableReflection(void)")]
// was: Ogre::Frustum::disableReflection(void)
// IDA 0xc9ba18: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9ba18() {
}

// 0xc9ba50 — __ZNK4Ogre7Frustum13projectSphereERKNS_6SphereEPfS4_S4_S4_
#[doc(alias = "Ogre::Frustum::projectSphere(Ogre::Sphere const&,float *,float *,float *,float *)const")]
// was: Ogre::Frustum::projectSphere(Ogre::Sphere const&,float *,float *,float *,float *)const
// IDA 0xc9ba50: 375 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9ba50() {
}

// 0xc9bfc4 — __ZN4Ogre7Frustum25enableCustomNearClipPlaneEPKNS_12MovablePlaneE
#[doc(alias = "Ogre::Frustum::enableCustomNearClipPlane(Ogre::MovablePlane const*)")]
// was: Ogre::Frustum::enableCustomNearClipPlane(Ogre::MovablePlane const*)
// IDA 0xc9bfc4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9bfc4() {
}
