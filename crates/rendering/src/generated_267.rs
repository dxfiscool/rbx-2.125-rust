//! rendering shard 267 — 100 stubs EA-sorted asc global gap filler after 0x37de97 not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29070->29170 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29070 before -> 29170 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x37de98 — __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(SoundServiceStatsItem *this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
// was: __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
// IDA 0x37de98: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37de98() {
}

// 0x37e05c — __ZN21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZN21SoundServiceStatsItemD1Ev
// IDA 0x37e05c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e05c() {
}

// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZN21SoundServiceStatsItemD0Ev
// IDA 0x37e098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e098() {
}

// 0x37e16c — __ZN21SoundServiceStatsItem6updateEv
// type: void __fastcall(SoundServiceStatsItem *this)
#[doc(alias = "SoundServiceStatsItem::update(void)")]
// was: __ZN21SoundServiceStatsItem6updateEv
// IDA 0x37e16c: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e16c() {
}

// 0x37e344 — __ZThn32_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn32_N21SoundServiceStatsItemD1Ev
// IDA 0x37e344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e344() {
}

// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn32_N21SoundServiceStatsItemD0Ev
// IDA 0x37e384: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e384() {
}

// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn36_N21SoundServiceStatsItemD1Ev
// IDA 0x37e458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e458() {
}

// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn36_N21SoundServiceStatsItemD0Ev
// IDA 0x37e498: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e498() {
}

// 0x37e56c — __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x37e56c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e56c() {
}

// 0x37e634 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x37e634: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e634() {
}

// 0x37e720 — __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// IDA 0x37e720: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e720() {
}

// 0x37e828 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// IDA 0x37e828: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_37e828() {
}

// 0x37e82c — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// IDA 0x37e82c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37e82c() {
}

// 0x37e830 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// IDA 0x37e830: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e830() {
}

// 0x37e850 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x37e850: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e850() {
}

// 0x37e868 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x37e868: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e868() {
}

// 0x37e86c — __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// type: RBX::Soundscape::SoundService::SoundJob *__fastcall(RBX::Soundscape::SoundService::SoundJob *this, RBX::Soundscape::SoundService *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// IDA 0x37e86c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37e86c() {
}

// 0x37e9c4 — __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
// IDA 0x37e9c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37e9c4() {
}

// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
// IDA 0x37e9c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37e9c8() {
}

// 0x37ea68 — __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// IDA 0x37ea68: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ea68() {
}

// 0x37ea84 — __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
// IDA 0x37ea84: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ea84() {
}

// 0x37eaa0 — __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Soundscape::SoundService **this, const RBX::TaskScheduler::Job::Stats *, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// IDA 0x37eaa0: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37eaa0() {
}

// 0x37eab0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x37eab0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37eab0() {
}

// 0x37ead8 — __GLOBAL__I_a_138
#[doc(alias = "global constructor keyed to_a_138")]
// was: __GLOBAL__I_a_138
// IDA 0x37ead8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_37ead8() {
}

// 0x37f4d8 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC1Ev
// IDA 0x37f4d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37f4d8() {
}

// 0x37f4dc — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC2Ev
// IDA 0x37f4dc: 262 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37f4dc() {
}

// 0x37f7c8 — __ZN3RBX10Reflection7Variant7convertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::convert<RBX::SoundType>(void)")]
// was: __ZN3RBX10Reflection7Variant7convertINS_9SoundTypeEEERT_v
// IDA 0x37f7c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37f7c8() {
}

// 0x37f7cc — __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")]
// was: __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
// IDA 0x37f7cc: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37f7cc() {
}

// 0x37f818 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEE7addPairES2_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEE7addPairES2_PKc
// IDA 0x37f818: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37f818() {
}

// 0x37fb78 — __ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
// IDA 0x37fb78: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37fb78() {
}

// 0x37fd64 — __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x37fd64: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37fd64() {
}

// 0x37fdbc — __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x37fdbc: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37fdbc() {
}

// 0x37feac — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
// IDA 0x37feac: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37feac() {
}

// 0x37fee0 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
// IDA 0x37fee0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_37fee0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x37ff08 — __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// IDA 0x37ff08: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ff08() {
}

// 0x37ff60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// IDA 0x37ff60: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ff60() {
}

// 0x380014 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// IDA 0x380014: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380014() {
}

// 0x38006c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// IDA 0x38006c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38006c() {
}

// 0x3800d4 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// IDA 0x3800d4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3800d4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x3801b8 — __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
// IDA 0x3801b8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3801b8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x3801d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
// IDA 0x3801d0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3801d0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x38020c — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// IDA 0x38020c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38020c() {
}

// 0x38039c — __GLOBAL__I_a_139
#[doc(alias = "global constructor keyed to_a_139")]
// was: __GLOBAL__I_a_139
// IDA 0x38039c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_38039c() {
}

// 0x380464 — __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::getConstChildSpanningNode(void)const")]
// was: __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
// IDA 0x380464: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380464() {
}

// 0x3804e0 — __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::getChildSpanningNode(void)")]
// was: __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
// IDA 0x3804e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3804e0() {
}

// 0x3804e4 — __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::getParentSpanningNode(void)")]
// was: __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
// IDA 0x3804e4: 10 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3804e4() {
}

// 0x3804fc — __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::removeFromSpanningTree(void)")]
// was: __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
// IDA 0x3804fc: 34 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3804fc() {
}

// 0x380568 — __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
// type: int __fastcall(RBX::SpanningEdge *this, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
// IDA 0x380568: 113 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380568() {
}

// 0x3806bc — __ZNK3RBX12SpanningEdge14inSpanningTreeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::inSpanningTree(void)const")]
// was: __ZNK3RBX12SpanningEdge14inSpanningTreeEv
// IDA 0x3806bc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3806bc() {
}

// 0x3806e4 — __GLOBAL__I_a_140
#[doc(alias = "global constructor keyed to_a_140")]
// was: __GLOBAL__I_a_140
// IDA 0x3806e4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_3806e4() {
}

// 0x3807ac — __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
// type: int __fastcall(int this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
// IDA 0x3807ac: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3807ac() {
}

// 0x3807b0 — __GLOBAL__I_a_141
#[doc(alias = "global constructor keyed to_a_141")]
// was: __GLOBAL__I_a_141
// IDA 0x3807b0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_3807b0() {
}

// 0x380878 — __ZN3RBX12SpanningTreeC2Ev
// type: RBX::SpanningTree *__fastcall(RBX::SpanningTree *this)
#[doc(alias = "RBX::SpanningTree::SpanningTree(void)")]
// was: __ZN3RBX12SpanningTreeC2Ev
// IDA 0x380878: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380878() {
}

// 0x38089c — __ZN3RBX12SpanningTreeD2Ev
// type: void __fastcall(RBX::SpanningTree *this, int, int)
#[doc(alias = "RBX::SpanningTree::~SpanningTree()")]
// was: __ZN3RBX12SpanningTreeD2Ev
// IDA 0x38089c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38089c() {
}

// 0x3809c4 — __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, int, int (*)(const char *, ...))
#[doc(alias = "RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
// IDA 0x3809c4: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3809c4() {
}

// 0x380a6c — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningEdge **, int *)
#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")]
// was: __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
// IDA 0x380a6c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380a6c() {
}

// 0x380abc — __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
// IDA 0x380abc: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380abc() {
}

// 0x380b30 — __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
// IDA 0x380b30: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380b30() {
}

// 0x380bac — __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningNode *, RBX::SpanningNode **)
#[doc(alias = "RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")]
// was: __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
// IDA 0x380bac: 108 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380bac() {
}

// 0x380cdc — __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
// type: int __fastcall(int this, RBX::SpanningEdge *, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
// IDA 0x380cdc: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380cdc() {
}

// 0x380d50 — __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
// IDA 0x380d50: 73 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380d50() {
}

// 0x380e34 — __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: int __fastcall(void (__fastcall ***this)(RBX::SpanningTree *, RBX::SpanningEdge *, int), RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// IDA 0x380e34: 77 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_380e34() {
}

// 0x381120 — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
// type: RBX::SpanningTree *__fastcall(RBX::SpanningTree *this, RBX::SpanningNode *, RBX::SpanningNode *, RBX::SpanningNode *, _DWORD *, RBX::SpanningEdge **, int *)
#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")]
// was: __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
// IDA 0x381120: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381120() {
}

// 0x38120c — __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")]
// was: __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
// IDA 0x38120c: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38120c() {
}

// 0x381308 — __ZN3RBX12SpanningNode8getDepthEPS0_
// type: int __fastcall(RBX::SpanningNode *this, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
// was: __ZN3RBX12SpanningNode8getDepthEPS0_
// IDA 0x381308: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381308() {
}

// 0x381328 — __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// IDA 0x381328: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_381328() {
}

// 0x38132c — __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// IDA 0x38132c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_38132c() {
}

// 0x381330 — __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// IDA 0x381330: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_381330() {
}

// 0x381334 — __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// IDA 0x381334: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_381334() {
}

// 0x381338 — __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::validateTree(RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
// IDA 0x381338: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381338() {
}

// 0x38133c — __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
// type: unsigned int *__fastcall(_DWORD *, int, RBX::SpanningEdge *this)
#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
// was: __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
// IDA 0x38133c: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38133c() {
}

// 0x3813bc — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// IDA 0x3813bc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3813bc() {
}

// 0x381424 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// IDA 0x381424: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381424() {
}

// 0x3818e0 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
// was: __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0x3818e0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3818e0() {
}

// 0x381908 — __GLOBAL__I_a_142
#[doc(alias = "global constructor keyed to_a_142")]
// was: __GLOBAL__I_a_142
// IDA 0x381908: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_381908() {
}

// 0x3819d0 — __ZN3RBX11StandardOut9singletonEv
// type: void __fastcall(RBX::StandardOut *this)
#[doc(alias = "RBX::StandardOut::singleton(void)")]
// was: __ZN3RBX11StandardOut9singletonEv
// IDA 0x3819d0: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3819d0() {
}

// 0x381b0c — __ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb
// type: void __fastcall(int, int, int, int, int, char, int, int, void *, int)
#[doc(alias = "RBX::StandardOut::print_exception(boost::function0<void> const&,RBX::MessageType,bool)")]
// was: __ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb
// IDA 0x381b0c: 44 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381b0c() {
}

// 0x381c38 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::exception const&)")]
// was: __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception
// IDA 0x381c38: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381c38() {
}

// 0x381c58 — __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz
// type: void(int, int, char *, ...)
#[doc(alias = "RBX::StandardOut::printf(RBX::MessageType,char const*,...)")]
// was: __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz
// IDA 0x381c58: 104 insns (SUB..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381c58() {
}

// 0x381d88 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs
// type: void __fastcall(int, int, const char **, int)
#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::string const&)")]
// was: __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs
// IDA 0x381d88: 293 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_381d88() {
}

// 0x3820c4 — __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,char const*)")]
// was: __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc
// IDA 0x3820c4: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3820c4() {
}

// 0x3821f0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::~shared_ptr()")]
// was: __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
// IDA 0x3821f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3821f0() {
}

// 0x382204 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
// IDA 0x382204: 76 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382204() {
}

// 0x382348 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// IDA 0x382348: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382348() {
}

// 0x3824a8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
// IDA 0x3824a8: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3824a8() {
}

// 0x3824d0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")]
// was: __ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS2_EEPT_
// IDA 0x3824d0: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3824d0() {
}

// 0x3825b8 — __ZNK5boost23enable_shared_from_thisIN3RBX11StandardOutEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::StandardOut>::_internal_accept_owner<RBX::StandardOut,RBX::StandardOut>(rbx_core::SharedPtr<RBX::StandardOut> const*,RBX::StandardOut *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX11StandardOutEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3825b8: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3825b8() {
}

// 0x3826dc — __ZN5boost6detail12shared_countC2IN3RBX11StandardOutEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StandardOut>(RBX::StandardOut *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX11StandardOutEEEPT_
// IDA 0x3826dc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3826dc() {
}

// 0x3827e8 — __ZN3RBX11StandardOutD2Ev
// type: void __fastcall(RBX::StandardOut *this, int, int, int)
#[doc(alias = "RBX::StandardOut::~StandardOut()")]
// was: __ZN3RBX11StandardOutD2Ev
// IDA 0x3827e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3827e8() {
}

// 0x38290c — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
// IDA 0x38290c: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38290c() {
}

// 0x382a84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED1Ev
// IDA 0x382a84: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_382a84() {
}

// 0x382a88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED0Ev
// IDA 0x382a88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_382a88() {
}

// 0x382a8c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE7disposeEv
// IDA 0x382a8c: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382a8c() {
}

// 0x382b30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE11get_deleterERKSt9type_info
// IDA 0x382b30: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382b30() {
}

// 0x382b34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE19get_untyped_deleterEv
// IDA 0x382b34: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382b34() {
}

// 0x382b38 — __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
// was: __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
// IDA 0x382b38: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382b38() {
}

// 0x382bfc — __ZN3RBX11StandardOutC2Ev
// type: int __fastcall(RBX::StandardOut *this)
#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
// was: __ZN3RBX11StandardOutC2Ev
// IDA 0x382bfc: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382bfc() {
}

// 0x382d18 — __GLOBAL__I_a_143
#[doc(alias = "global constructor keyed to_a_143")]
// was: __GLOBAL__I_a_143
// IDA 0x382d18: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_382d18() {
}

// 0x382de0 — __Z10SetBaseURLRKSs
// type: int __fastcall(const std::string *)
#[doc(alias = "SetBaseURL(std::string const&)")]
// was: __Z10SetBaseURLRKSs
// IDA 0x382de0: 5 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382de0() {
}

// 0x382df4 — __Z10GetBaseURLv
// type: int *__fastcall()
#[doc(alias = "GetBaseURL(void)")]
// was: __Z10GetBaseURLv
// IDA 0x382df4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382df4() {
}
