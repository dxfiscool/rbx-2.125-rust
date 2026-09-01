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
pub fn stub_37de98() -> ! {
    todo!("0x37de98 SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")
}

// 0x37e05c — __ZN21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZN21SoundServiceStatsItemD1Ev
pub fn stub_37e05c() -> ! {
    todo!("0x37e05c SoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZN21SoundServiceStatsItemD0Ev
pub fn stub_37e098() -> ! {
    todo!("0x37e098 SoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e16c — __ZN21SoundServiceStatsItem6updateEv
// type: void __fastcall(SoundServiceStatsItem *this)
#[doc(alias = "SoundServiceStatsItem::update(void)")]
// was: __ZN21SoundServiceStatsItem6updateEv
pub fn stub_37e16c() -> ! {
    todo!("0x37e16c SoundServiceStatsItem::update(void)")
}

// 0x37e344 — __ZThn32_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn32_N21SoundServiceStatsItemD1Ev
pub fn stub_37e344() -> ! {
    todo!("0x37e344 non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn32_N21SoundServiceStatsItemD0Ev
pub fn stub_37e384() -> ! {
    todo!("0x37e384 non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn36_N21SoundServiceStatsItemD1Ev
pub fn stub_37e458() -> ! {
    todo!("0x37e458 non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// was: __ZThn36_N21SoundServiceStatsItemD0Ev
pub fn stub_37e498() -> ! {
    todo!("0x37e498 non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e56c — __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_37e56c() -> ! {
    todo!("0x37e56c boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e634 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_37e634() -> ! {
    todo!("0x37e634 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")
}

// 0x37e720 — __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
pub fn stub_37e720() -> ! {
    todo!("0x37e720 boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e828 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
pub fn stub_37e828() -> ! {
    todo!("0x37e828 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37e82c — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
pub fn stub_37e82c() -> ! {
    todo!("0x37e82c boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37e830 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
pub fn stub_37e830() -> ! {
    todo!("0x37e830 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37e850 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_37e850() -> ! {
    todo!("0x37e850 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x37e868 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_37e868() -> ! {
    todo!("0x37e868 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x37e86c — __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// type: RBX::Soundscape::SoundService::SoundJob *__fastcall(RBX::Soundscape::SoundService::SoundJob *this, RBX::Soundscape::SoundService *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
pub fn stub_37e86c() -> ! {
    todo!("0x37e86c RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")
}

// 0x37e9c4 — __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
pub fn stub_37e9c4() -> ! {
    todo!("0x37e9c4 RBX::Soundscape::SoundService::SoundJob::~SoundJob()")
}

// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
pub fn stub_37e9c8() -> ! {
    todo!("0x37e9c8 RBX::Soundscape::SoundService::SoundJob::~SoundJob()")
}

// 0x37ea68 — __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_37ea68() -> ! {
    todo!("0x37ea68 RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37ea84 — __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_37ea84() -> ! {
    todo!("0x37ea84 RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37eaa0 — __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Soundscape::SoundService **this, const RBX::TaskScheduler::Job::Stats *, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
pub fn stub_37eaa0() -> ! {
    todo!("0x37eaa0 RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37eab0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_37eab0() -> ! {
    todo!("0x37eab0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")
}

// 0x37ead8 — __GLOBAL__I_a_138
#[doc(alias = "global constructor keyed to_a_138")]
// was: __GLOBAL__I_a_138
pub fn stub_37ead8() -> ! {
    todo!("0x37ead8 global constructor keyed to_a_138")
}

// 0x37f4d8 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC1Ev
pub fn stub_37f4d8() -> ! {
    todo!("0x37f4d8 RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")
}

// 0x37f4dc — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC2Ev
pub fn stub_37f4dc() -> ! {
    todo!("0x37f4dc RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")
}

// 0x37f7c8 — __ZN3RBX10Reflection7Variant7convertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::convert<RBX::SoundType>(void)")]
// was: __ZN3RBX10Reflection7Variant7convertINS_9SoundTypeEEERT_v
pub fn stub_37f7c8() -> ! {
    todo!("0x37f7c8 RBX::SoundType & RBX::Reflection::Variant::convert<RBX::SoundType>(void)")
}

// 0x37f7cc — __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")]
// was: __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
pub fn stub_37f7cc() -> ! {
    todo!("0x37f7cc RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")
}

// 0x37f818 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEE7addPairES2_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEE7addPairES2_PKc
pub fn stub_37f818() -> ! {
    todo!("0x37f818 RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")
}

// 0x37fb78 — __ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
pub fn stub_37fb78() -> ! {
    todo!("0x37fb78 RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")
}

// 0x37fd64 — __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_37fd64() -> ! {
    todo!("0x37fd64 RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x37fdbc — __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_37fdbc() -> ! {
    todo!("0x37fdbc RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x37feac — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
pub fn stub_37feac() -> ! {
    todo!("0x37feac std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")
}

// 0x37fee0 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
pub fn stub_37fee0() -> ! {
    todo!("0x37fee0 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")
}

// 0x37ff08 — __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_37ff08() -> ! {
    todo!("0x37ff08 std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")
}

// 0x37ff60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_37ff60() -> ! {
    todo!("0x37ff60 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0x380014 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_380014() -> ! {
    todo!("0x380014 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0x38006c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_38006c() -> ! {
    todo!("0x38006c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0x3800d4 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_3800d4() -> ! {
    todo!("0x3800d4 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")
}

// 0x3801b8 — __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
pub fn stub_3801b8() -> ! {
    todo!("0x3801b8 std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")
}

// 0x3801d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
pub fn stub_3801d0() -> ! {
    todo!("0x3801d0 RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")
}

// 0x38020c — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
// was: __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_38020c() -> ! {
    todo!("0x38020c std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")
}

// 0x38039c — __GLOBAL__I_a_139
#[doc(alias = "global constructor keyed to_a_139")]
// was: __GLOBAL__I_a_139
pub fn stub_38039c() -> ! {
    todo!("0x38039c global constructor keyed to_a_139")
}

// 0x380464 — __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::getConstChildSpanningNode(void)const")]
// was: __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
pub fn stub_380464() -> ! {
    todo!("0x380464 RBX::SpanningEdge::getConstChildSpanningNode(void)const")
}

// 0x3804e0 — __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::getChildSpanningNode(void)")]
// was: __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
pub fn stub_3804e0() -> ! {
    todo!("0x3804e0 RBX::SpanningEdge::getChildSpanningNode(void)")
}

// 0x3804e4 — __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::getParentSpanningNode(void)")]
// was: __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
pub fn stub_3804e4() -> ! {
    todo!("0x3804e4 RBX::SpanningEdge::getParentSpanningNode(void)")
}

// 0x3804fc — __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::removeFromSpanningTree(void)")]
// was: __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
pub fn stub_3804fc() -> ! {
    todo!("0x3804fc RBX::SpanningEdge::removeFromSpanningTree(void)")
}

// 0x380568 — __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
// type: int __fastcall(RBX::SpanningEdge *this, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
pub fn stub_380568() -> ! {
    todo!("0x380568 RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")
}

// 0x3806bc — __ZNK3RBX12SpanningEdge14inSpanningTreeEv
// type: int __fastcall(RBX::SpanningEdge *this)
#[doc(alias = "RBX::SpanningEdge::inSpanningTree(void)const")]
// was: __ZNK3RBX12SpanningEdge14inSpanningTreeEv
pub fn stub_3806bc() -> ! {
    todo!("0x3806bc RBX::SpanningEdge::inSpanningTree(void)const")
}

// 0x3806e4 — __GLOBAL__I_a_140
#[doc(alias = "global constructor keyed to_a_140")]
// was: __GLOBAL__I_a_140
pub fn stub_3806e4() -> ! {
    todo!("0x3806e4 global constructor keyed to_a_140")
}

// 0x3807ac — __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
// type: int __fastcall(int this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
pub fn stub_3807ac() -> ! {
    todo!("0x3807ac RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")
}

// 0x3807b0 — __GLOBAL__I_a_141
#[doc(alias = "global constructor keyed to_a_141")]
// was: __GLOBAL__I_a_141
pub fn stub_3807b0() -> ! {
    todo!("0x3807b0 global constructor keyed to_a_141")
}

// 0x380878 — __ZN3RBX12SpanningTreeC2Ev
// type: RBX::SpanningTree *__fastcall(RBX::SpanningTree *this)
#[doc(alias = "RBX::SpanningTree::SpanningTree(void)")]
// was: __ZN3RBX12SpanningTreeC2Ev
pub fn stub_380878() -> ! {
    todo!("0x380878 RBX::SpanningTree::SpanningTree(void)")
}

// 0x38089c — __ZN3RBX12SpanningTreeD2Ev
// type: void __fastcall(RBX::SpanningTree *this, int, int)
#[doc(alias = "RBX::SpanningTree::~SpanningTree()")]
// was: __ZN3RBX12SpanningTreeD2Ev
pub fn stub_38089c() -> ! {
    todo!("0x38089c RBX::SpanningTree::~SpanningTree()")
}

// 0x3809c4 — __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, int, int (*)(const char *, ...))
#[doc(alias = "RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
pub fn stub_3809c4() -> ! {
    todo!("0x3809c4 RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")
}

// 0x380a6c — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningEdge **, int *)
#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")]
// was: __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
pub fn stub_380a6c() -> ! {
    todo!("0x380a6c RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")
}

// 0x380abc — __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
pub fn stub_380abc() -> ! {
    todo!("0x380abc RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x380b30 — __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
pub fn stub_380b30() -> ! {
    todo!("0x380b30 RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")
}

// 0x380bac — __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningNode *, RBX::SpanningNode **)
#[doc(alias = "RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")]
// was: __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
pub fn stub_380bac() -> ! {
    todo!("0x380bac RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")
}

// 0x380cdc — __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
// type: int __fastcall(int this, RBX::SpanningEdge *, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
pub fn stub_380cdc() -> ! {
    todo!("0x380cdc RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x380d50 — __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
pub fn stub_380d50() -> ! {
    todo!("0x380d50 RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")
}

// 0x380e34 — __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: int __fastcall(void (__fastcall ***this)(RBX::SpanningTree *, RBX::SpanningEdge *, int), RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_380e34() -> ! {
    todo!("0x380e34 RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x381120 — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
// type: RBX::SpanningTree *__fastcall(RBX::SpanningTree *this, RBX::SpanningNode *, RBX::SpanningNode *, RBX::SpanningNode *, _DWORD *, RBX::SpanningEdge **, int *)
#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")]
// was: __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
pub fn stub_381120() -> ! {
    todo!("0x381120 RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")
}

// 0x38120c — __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")]
// was: __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
pub fn stub_38120c() -> ! {
    todo!("0x38120c RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")
}

// 0x381308 — __ZN3RBX12SpanningNode8getDepthEPS0_
// type: int __fastcall(RBX::SpanningNode *this, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
// was: __ZN3RBX12SpanningNode8getDepthEPS0_
pub fn stub_381308() -> ! {
    todo!("0x381308 RBX::SpanningNode::getDepth(RBX::SpanningNode*)")
}

// 0x381328 — __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_381328() -> ! {
    todo!("0x381328 RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x38132c — __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
pub fn stub_38132c() -> ! {
    todo!("0x38132c RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")
}

// 0x381330 — __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
// was: __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
pub fn stub_381330() -> ! {
    todo!("0x381330 RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")
}

// 0x381334 — __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: void __fastcall(RBX::SpanningTree *this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_381334() -> ! {
    todo!("0x381334 RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x381338 — __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
// type: int __fastcall(RBX::SpanningTree *this, RBX::SpanningNode *)
#[doc(alias = "RBX::SpanningTree::validateTree(RBX::SpanningNode *)")]
// was: __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
pub fn stub_381338() -> ! {
    todo!("0x381338 RBX::SpanningTree::validateTree(RBX::SpanningNode *)")
}

// 0x38133c — __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
// type: unsigned int *__fastcall(_DWORD *, int, RBX::SpanningEdge *this)
#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
// was: __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
pub fn stub_38133c() -> ! {
    todo!("0x38133c RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")
}

// 0x3813bc — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_3813bc() -> ! {
    todo!("0x3813bc std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")
}

// 0x381424 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_381424() -> ! {
    todo!("0x381424 std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")
}

// 0x3818e0 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
// was: __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_3818e0() -> ! {
    todo!("0x3818e0 std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")
}

// 0x381908 — __GLOBAL__I_a_142
#[doc(alias = "global constructor keyed to_a_142")]
// was: __GLOBAL__I_a_142
pub fn stub_381908() -> ! {
    todo!("0x381908 global constructor keyed to_a_142")
}

// 0x3819d0 — __ZN3RBX11StandardOut9singletonEv
// type: void __fastcall(RBX::StandardOut *this)
#[doc(alias = "RBX::StandardOut::singleton(void)")]
// was: __ZN3RBX11StandardOut9singletonEv
pub fn stub_3819d0() -> ! {
    todo!("0x3819d0 RBX::StandardOut::singleton(void)")
}

// 0x381b0c — __ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb
// type: void __fastcall(int, int, int, int, int, char, int, int, void *, int)
#[doc(alias = "RBX::StandardOut::print_exception(boost::function0<void> const&,RBX::MessageType,bool)")]
// was: __ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb
pub fn stub_381b0c() -> ! {
    todo!("0x381b0c RBX::StandardOut::print_exception(boost::function0<void> const&,RBX::MessageType,bool)")
}

// 0x381c38 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::exception const&)")]
// was: __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception
pub fn stub_381c38() -> ! {
    todo!("0x381c38 RBX::StandardOut::print(RBX::MessageType,std::exception const&)")
}

// 0x381c58 — __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz
// type: void(int, int, char *, ...)
#[doc(alias = "RBX::StandardOut::printf(RBX::MessageType,char const*,...)")]
// was: __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz
pub fn stub_381c58() -> ! {
    todo!("0x381c58 RBX::StandardOut::printf(RBX::MessageType,char const*,...)")
}

// 0x381d88 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs
// type: void __fastcall(int, int, const char **, int)
#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::string const&)")]
// was: __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs
pub fn stub_381d88() -> ! {
    todo!("0x381d88 RBX::StandardOut::print(RBX::MessageType,std::string const&)")
}

// 0x3820c4 — __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,char const*)")]
// was: __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc
pub fn stub_3820c4() -> ! {
    todo!("0x3820c4 RBX::StandardOut::print(RBX::MessageType,char const*)")
}

// 0x3821f0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::StandardOut>::~shared_ptr()")]
// was: __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
pub fn stub_3821f0() -> ! {
    todo!("0x3821f0 boost::shared_ptr<RBX::StandardOut>::~shared_ptr()")
}

// 0x382204 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
pub fn stub_382204() -> ! {
    todo!("0x382204 rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")
}

// 0x382348 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
pub fn stub_382348() -> ! {
    todo!("0x382348 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")
}

// 0x3824a8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
pub fn stub_3824a8() -> ! {
    todo!("0x3824a8 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")
}

// 0x3824d0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")]
// was: __ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS2_EEPT_
pub fn stub_3824d0() -> ! {
    todo!("0x3824d0 boost::shared_ptr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")
}

// 0x3825b8 — __ZNK5boost23enable_shared_from_thisIN3RBX11StandardOutEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::StandardOut>::_internal_accept_owner<RBX::StandardOut,RBX::StandardOut>(boost::shared_ptr<RBX::StandardOut> const*,RBX::StandardOut *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX11StandardOutEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3825b8() -> ! {
    todo!("0x3825b8 void boost::enable_shared_from_this<RBX::StandardOut>::_internal_accept_owner<RBX::StandardOut,RBX::StandardOut>(boost::shared_ptr<RBX::StandardOut> const*,RBX::StandardOut *)const")
}

// 0x3826dc — __ZN5boost6detail12shared_countC2IN3RBX11StandardOutEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StandardOut>(RBX::StandardOut *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX11StandardOutEEEPT_
pub fn stub_3826dc() -> ! {
    todo!("0x3826dc boost::detail::shared_count::shared_count<RBX::StandardOut>(RBX::StandardOut *)")
}

// 0x3827e8 — __ZN3RBX11StandardOutD2Ev
// type: void __fastcall(RBX::StandardOut *this, int, int, int)
#[doc(alias = "RBX::StandardOut::~StandardOut()")]
// was: __ZN3RBX11StandardOutD2Ev
pub fn stub_3827e8() -> ! {
    todo!("0x3827e8 RBX::StandardOut::~StandardOut()")
}

// 0x38290c — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
pub fn stub_38290c() -> ! {
    todo!("0x38290c rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")
}

// 0x382a84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED1Ev
pub fn stub_382a84() -> ! {
    todo!("0x382a84 boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")
}

// 0x382a88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEED0Ev
pub fn stub_382a88() -> ! {
    todo!("0x382a88 boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")
}

// 0x382a8c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE7disposeEv
pub fn stub_382a8c() -> ! {
    todo!("0x382a8c boost::detail::sp_counted_impl_p<RBX::StandardOut>::dispose(void)")
}

// 0x382b30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE11get_deleterERKSt9type_info
pub fn stub_382b30() -> ! {
    todo!("0x382b30 boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_deleter(std::type_info const&)")
}

// 0x382b34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11StandardOutEE19get_untyped_deleterEv
pub fn stub_382b34() -> ! {
    todo!("0x382b34 boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_untyped_deleter(void)")
}

// 0x382b38 — __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
// was: __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
pub fn stub_382b38() -> ! {
    todo!("0x382b38 RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")
}

// 0x382bfc — __ZN3RBX11StandardOutC2Ev
// type: int __fastcall(RBX::StandardOut *this)
#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
// was: __ZN3RBX11StandardOutC2Ev
pub fn stub_382bfc() -> ! {
    todo!("0x382bfc RBX::StandardOut::StandardOut(void)")
}

// 0x382d18 — __GLOBAL__I_a_143
#[doc(alias = "global constructor keyed to_a_143")]
// was: __GLOBAL__I_a_143
pub fn stub_382d18() -> ! {
    todo!("0x382d18 global constructor keyed to_a_143")
}

// 0x382de0 — __Z10SetBaseURLRKSs
// type: int __fastcall(const std::string *)
#[doc(alias = "SetBaseURL(std::string const&)")]
// was: __Z10SetBaseURLRKSs
pub fn stub_382de0() -> ! {
    todo!("0x382de0 SetBaseURL(std::string const&)")
}

// 0x382df4 — __Z10GetBaseURLv
// type: int *__fastcall()
#[doc(alias = "GetBaseURL(void)")]
// was: __Z10GetBaseURLv
pub fn stub_382df4() -> ! {
    todo!("0x382df4 GetBaseURL(void)")
}
