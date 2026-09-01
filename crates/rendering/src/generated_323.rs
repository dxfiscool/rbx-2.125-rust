//! rendering shard 323 — 100 stubs 0x48d1b8..0x492098 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 35100->35200 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35100 before -> 35200 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x48d1b8 (lowest remaining 0x48d1b8..0x492098, next lowest 0x492180)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x48d1b8 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
// was: __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_48d1b8() -> ! {
    todo!("0x48d1b8 std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")
}

// 0x48d29c — __ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm
pub fn stub_48d29c() -> ! {
    todo!("0x48d29c std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")
}

// 0x48d2b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_
pub fn stub_48d2b4() -> ! {
    todo!("0x48d2b4 RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")
}

// 0x48d2f0 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
// was: __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_48d2f0() -> ! {
    todo!("0x48d2f0 std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")
}

// 0x48d480 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
// was: __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_
pub fn stub_48d480() -> ! {
    todo!("0x48d480 std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")
}

// 0x48d4b4 — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_48d4b4() -> ! {
    todo!("0x48d4b4 std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")
}

// 0x48d50c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_48d50c() -> ! {
    todo!("0x48d50c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")
}

// 0x48d5c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_48d5c0() -> ! {
    todo!("0x48d5c0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")
}

// 0x48d618 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_48d618() -> ! {
    todo!("0x48d618 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")
}

// 0x48d680 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// was: __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_48d680() -> ! {
    todo!("0x48d680 std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")
}

// 0x48d810 — __ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm
pub fn stub_48d810() -> ! {
    todo!("0x48d810 std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")
}

// 0x48d828 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_
pub fn stub_48d828() -> ! {
    todo!("0x48d828 RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")
}

// 0x48d864 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")]
// was: __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_
pub fn stub_48d864() -> ! {
    todo!("0x48d864 std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")
}

// 0x48d88c — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// was: __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_48d88c() -> ! {
    todo!("0x48d88c std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")
}

// 0x48d970 — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// was: __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_48d970() -> ! {
    todo!("0x48d970 boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x48d9bc — __ZN8DummyJobC2Ebd
// type: DummyJob *__fastcall(DummyJob *__hidden this, bool, double)
#[doc(alias = "DummyJob::DummyJob(bool,double)")]
// was: __ZN8DummyJobC2Ebd
pub fn stub_48d9bc() -> ! {
    todo!("0x48d9bc DummyJob::DummyJob(bool,double)")
}

// 0x48db88 — __ZN8DummyJobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
#[doc(alias = "DummyJob::~DummyJob()")]
// was: __ZN8DummyJobD1Ev
pub fn stub_48db88() -> ! {
    todo!("0x48db88 DummyJob::~DummyJob()")
}

// 0x48db8c — __ZN8DummyJobD0Ev
// type: void __fastcall(DummyJob *__hidden this)
#[doc(alias = "DummyJob::~DummyJob()")]
// was: __ZN8DummyJobD0Ev
pub fn stub_48db8c() -> ! {
    todo!("0x48db8c DummyJob::~DummyJob()")
}

// 0x48dc2c — __ZN8DummyJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN8DummyJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
pub fn stub_48dc2c() -> ! {
    todo!("0x48dc2c DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x48dc34 — __ZN8DummyJob5errorERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "DummyJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN8DummyJob5errorERKN3RBX13TaskScheduler3Job5StatsE
pub fn stub_48dc34() -> ! {
    todo!("0x48dc34 DummyJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x48dc4c — __ZN8DummyJob17getPriorityFactorEv
// type: _DWORD __fastcall(DummyJob *__hidden this)
#[doc(alias = "DummyJob::getPriorityFactor(void)")]
// was: __ZN8DummyJob17getPriorityFactorEv
pub fn stub_48dc4c() -> ! {
    todo!("0x48dc4c DummyJob::getPriorityFactor(void)")
}

// 0x48dc58 — __ZN8DummyJob4stepERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "DummyJob::step(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN8DummyJob4stepERKN3RBX13TaskScheduler3Job5StatsE
pub fn stub_48dc58() -> ! {
    todo!("0x48dc58 DummyJob::step(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x48dc60 — __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv
// type: int(void)
#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")]
// was: __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv
pub fn stub_48dc60() -> ! {
    todo!("0x48dc60 RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")
}

// 0x48dcc0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_48dcc0() -> ! {
    todo!("0x48dcc0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)")
}

// 0x48dce8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_48dce8() -> ! {
    todo!("0x48dce8 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)")
}

// 0x48dd10 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_48dd10() -> ! {
    todo!("0x48dd10 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)")
}

// 0x48dd38 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
pub fn stub_48dd38() -> ! {
    todo!("0x48dd38 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)")
}

// 0x48dd60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_48dd60() -> ! {
    todo!("0x48dd60 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)")
}

// 0x48dd88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_48dd88() -> ! {
    todo!("0x48dd88 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)")
}

// 0x48ddb0 — __GLOBAL__I_a_182
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int)
#[doc(alias = "global constructor keyed to_a_182")]
// was: __GLOBAL__I_a_182
pub fn stub_48ddb0() -> ! {
    todo!("0x48ddb0 `global constructor keyed to'_a_182")
}

// 0x48f7f4 — __ZN3RBX5Decal10setTextureENS_9TextureIdE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Decal::setTexture(RBX::TextureId)")]
// was: __ZN3RBX5Decal10setTextureENS_9TextureIdE
pub fn stub_48f7f4() -> ! {
    todo!("0x48f7f4 RBX::Decal::setTexture(RBX::TextureId)")
}

// 0x48f82c — __ZN3RBX5Decal11setSpecularEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "RBX::Decal::setSpecular(float)")]
// was: __ZN3RBX5Decal11setSpecularEf
pub fn stub_48f82c() -> ! {
    todo!("0x48f82c RBX::Decal::setSpecular(float)")
}

// 0x48f860 — __ZN3RBX5Decal8setShinyEf
// type: _DWORD __fastcall(RBX::Decal *__hidden this, float)
#[doc(alias = "RBX::Decal::setShiny(float)")]
// was: __ZN3RBX5Decal8setShinyEf
pub fn stub_48f860() -> ! {
    todo!("0x48f860 RBX::Decal::setShiny(float)")
}

// 0x48f894 — __ZN3RBX5Decal15setTransparencyEf
// type: _DWORD __fastcall(RBX::Decal *__hidden this, float)
#[doc(alias = "RBX::Decal::setTransparency(float)")]
// was: __ZN3RBX5Decal15setTransparencyEf
pub fn stub_48f894() -> ! {
    todo!("0x48f894 RBX::Decal::setTransparency(float)")
}

// 0x48f8bc — __ZN3RBX5DecalC2Ev
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::Decal(void)")]
// was: __ZN3RBX5DecalC2Ev
pub fn stub_48f8bc() -> ! {
    todo!("0x48f8bc RBX::Decal::Decal(void)")
}

// 0x48fb04 — __ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *)
#[doc(alias = "RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)")]
// was: __ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_
pub fn stub_48fb04() -> ! {
    todo!("0x48fb04 RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)")
}

// 0x48fc28 — __ZN3RBX10Reflection4Type12getSingletonINS_9TextureIdEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextureId>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_9TextureIdEEERKS1_v
pub fn stub_48fc28() -> ! {
    todo!("0x48fc28 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextureId>(void)")
}

// 0x48fc2c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_48fc2c() -> ! {
    todo!("0x48fc2c RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x48fe14 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_48fe14() -> ! {
    todo!("0x48fe14 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x48ffbc — __ZN3RBX10Reflection7Variant7convertINS_9TextureIdEEERT_v
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::convert<RBX::TextureId>(void)")]
// was: __ZN3RBX10Reflection7Variant7convertINS_9TextureIdEEERT_v
pub fn stub_48ffbc() -> ! {
    todo!("0x48ffbc RBX::TextureId & RBX::Reflection::Variant::convert<RBX::TextureId>(void)")
}

// 0x4901a8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11getDataSizeEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11getDataSizeEPKNS0_13DescribedBaseE
pub fn stub_4901a8() -> ! {
    todo!("0x4901a8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getDataSize(RBX::Reflection::DescribedBase const*)const")
}

// 0x490204 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14hasStringValueEv
pub fn stub_490204() -> ! {
    todo!("0x490204 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::hasStringValue(void)const")
}

// 0x490208 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_490208() -> ! {
    todo!("0x490208 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x490324 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_490324() -> ! {
    todo!("0x490324 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x49047c — __ZN3RBX7Texture16setStudsPerTileUEf
// type: _DWORD __fastcall(RBX::Texture *__hidden this, float)
#[doc(alias = "RBX::Texture::setStudsPerTileU(float)")]
// was: __ZN3RBX7Texture16setStudsPerTileUEf
pub fn stub_49047c() -> ! {
    todo!("0x49047c RBX::Texture::setStudsPerTileU(float)")
}

// 0x4904b0 — __ZN3RBX7Texture16setStudsPerTileVEf
// type: _DWORD __fastcall(RBX::Texture *__hidden this, float)
#[doc(alias = "RBX::Texture::setStudsPerTileV(float)")]
// was: __ZN3RBX7Texture16setStudsPerTileVEf
pub fn stub_4904b0() -> ! {
    todo!("0x4904b0 RBX::Texture::setStudsPerTileV(float)")
}

// 0x4904e4 — __ZN3RBX7TextureC2Ev
// type: RBX::Decal *__fastcall(RBX::Texture *this)
#[doc(alias = "RBX::Texture::Texture(void)")]
// was: __ZN3RBX7TextureC2Ev
pub fn stub_4904e4() -> ! {
    todo!("0x4904e4 RBX::Texture::Texture(void)")
}

// 0x49076c — __ZNK3RBX5Decal10getTextureEv
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::getTexture(void)const")]
// was: __ZNK3RBX5Decal10getTextureEv
pub fn stub_49076c() -> ! {
    todo!("0x49076c RBX::Decal::getTexture(void)const")
}

// 0x490770 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED1Ev
pub fn stub_490770() -> ! {
    todo!("0x490770 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")
}

// 0x490794 — __ZNK3RBX5Decal11getSpecularEv
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::getSpecular(void)const")]
// was: __ZNK3RBX5Decal11getSpecularEv
pub fn stub_490794() -> ! {
    todo!("0x490794 RBX::Decal::getSpecular(void)const")
}

// 0x49079c — __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfED1Ev
pub fn stub_49079c() -> ! {
    todo!("0x49079c RBX::Reflection::PropDescriptor<RBX::Decal,float>::~PropDescriptor()")
}

// 0x4907c0 — __ZNK3RBX5Decal8getShinyEv
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::getShiny(void)const")]
// was: __ZNK3RBX5Decal8getShinyEv
pub fn stub_4907c0() -> ! {
    todo!("0x4907c0 RBX::Decal::getShiny(void)const")
}

// 0x4907c8 — __ZNK3RBX5Decal15getTransparencyEv
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::getTransparency(void)const")]
// was: __ZNK3RBX5Decal15getTransparencyEv
pub fn stub_4907c8() -> ! {
    todo!("0x4907c8 RBX::Decal::getTransparency(void)const")
}

// 0x4907d0 — __ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v
// type: int(void)
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v
pub fn stub_4907d0() -> ! {
    todo!("0x4907d0 RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")
}

// 0x490a7c — __ZNK3RBX7Texture16getStudsPerTileUEv
// type: _DWORD __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::getStudsPerTileU(void)const")]
// was: __ZNK3RBX7Texture16getStudsPerTileUEv
pub fn stub_490a7c() -> ! {
    todo!("0x490a7c RBX::Texture::getStudsPerTileU(void)const")
}

// 0x490a84 — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED1Ev
pub fn stub_490a84() -> ! {
    todo!("0x490a84 RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")
}

// 0x490aa8 — __ZNK3RBX7Texture16getStudsPerTileVEv
// type: _DWORD __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::getStudsPerTileV(void)const")]
// was: __ZNK3RBX7Texture16getStudsPerTileVEv
pub fn stub_490aa8() -> ! {
    todo!("0x490aa8 RBX::Texture::getStudsPerTileV(void)const")
}

// 0x490ab0 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev
pub fn stub_490ab0() -> ! {
    todo!("0x490ab0 __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev")
}

// 0x490ab4 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev
pub fn stub_490ab4() -> ! {
    todo!("0x490ab4 __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")
}

// 0x490ab8 — __ZN3RBX5DecalD1Ev
// type: void __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::~Decal()")]
// was: __ZN3RBX5DecalD1Ev
pub fn stub_490ab8() -> ! {
    todo!("0x490ab8 RBX::Decal::~Decal()")
}

// 0x490af8 — __ZN3RBX5DecalD0Ev
// type: void __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::~Decal()")]
// was: __ZN3RBX5DecalD0Ev
pub fn stub_490af8() -> ! {
    todo!("0x490af8 RBX::Decal::~Decal()")
}

// 0x490bd4 — __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv
pub fn stub_490bd4() -> ! {
    todo!("0x490bd4 __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")
}

// 0x490be4 — __ZThn32_N3RBX5DecalD1Ev
// type: void __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// was: __ZThn32_N3RBX5DecalD1Ev
pub fn stub_490be4() -> ! {
    todo!("0x490be4 `non-virtual thunk to'RBX::Decal::~Decal()")
}

// 0x490c28 — __ZThn32_N3RBX5DecalD0Ev
// type: void __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// was: __ZThn32_N3RBX5DecalD0Ev
pub fn stub_490c28() -> ! {
    todo!("0x490c28 `non-virtual thunk to'RBX::Decal::~Decal()")
}

// 0x490d04 — __ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv
pub fn stub_490d04() -> ! {
    todo!("0x490d04 __ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")
}

// 0x490d14 — __ZThn36_N3RBX5DecalD1Ev
// type: void __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// was: __ZThn36_N3RBX5DecalD1Ev
pub fn stub_490d14() -> ! {
    todo!("0x490d14 `non-virtual thunk to'RBX::Decal::~Decal()")
}

// 0x490d58 — __ZThn36_N3RBX5DecalD0Ev
// type: void __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// was: __ZThn36_N3RBX5DecalD0Ev
pub fn stub_490d58() -> ! {
    todo!("0x490d58 `non-virtual thunk to'RBX::Decal::~Decal()")
}

// 0x490e34 — __ZN3RBX7TextureD1Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::~Texture()")]
// was: __ZN3RBX7TextureD1Ev
pub fn stub_490e34() -> ! {
    todo!("0x490e34 RBX::Texture::~Texture()")
}

// 0x490e74 — __ZN3RBX7TextureD0Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::~Texture()")]
// was: __ZN3RBX7TextureD0Ev
pub fn stub_490e74() -> ! {
    todo!("0x490e74 RBX::Texture::~Texture()")
}

// 0x490f50 — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
pub fn stub_490f50() -> ! {
    todo!("0x490f50 __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")
}

// 0x490f60 — __ZThn32_N3RBX7TextureD1Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// was: __ZThn32_N3RBX7TextureD1Ev
pub fn stub_490f60() -> ! {
    todo!("0x490f60 `non-virtual thunk to'RBX::Texture::~Texture()")
}

// 0x490fa4 — __ZThn32_N3RBX7TextureD0Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// was: __ZThn32_N3RBX7TextureD0Ev
pub fn stub_490fa4() -> ! {
    todo!("0x490fa4 `non-virtual thunk to'RBX::Texture::~Texture()")
}

// 0x491080 — __ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
pub fn stub_491080() -> ! {
    todo!("0x491080 __ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")
}

// 0x491090 — __ZThn36_N3RBX7TextureD1Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// was: __ZThn36_N3RBX7TextureD1Ev
pub fn stub_491090() -> ! {
    todo!("0x491090 `non-virtual thunk to'RBX::Texture::~Texture()")
}

// 0x4910d4 — __ZThn36_N3RBX7TextureD0Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// was: __ZThn36_N3RBX7TextureD0Ev
pub fn stub_4910d4() -> ! {
    todo!("0x4910d4 `non-virtual thunk to'RBX::Texture::~Texture()")
}

// 0x4911b0 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv
pub fn stub_4911b0() -> ! {
    todo!("0x4911b0 __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")
}

// 0x491224 — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_491224() -> ! {
    todo!("0x491224 __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x4912ac — __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv
pub fn stub_4912ac() -> ! {
    todo!("0x4912ac __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")
}

// 0x4912b0 — __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v
pub fn stub_4912b0() -> ! {
    todo!("0x4912b0 __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")
}

// 0x491390 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv
pub fn stub_491390() -> ! {
    todo!("0x491390 __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv")
}

// 0x491404 — __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_491404() -> ! {
    todo!("0x491404 __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x49148c — __ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv
pub fn stub_49148c() -> ! {
    todo!("0x49148c __ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv")
}

// 0x491490 — __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v
pub fn stub_491490() -> ! {
    todo!("0x491490 __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")
}

// 0x491570 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev
pub fn stub_491570() -> ! {
    todo!("0x491570 __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")
}

// 0x49160c — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv
pub fn stub_49160c() -> ! {
    todo!("0x49160c __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")
}

// 0x491750 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextureEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextureEEEN5boost10shared_ptrIT_EEv
pub fn stub_491750() -> ! {
    todo!("0x491750 boost::shared_ptr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)")
}

// 0x491800 — __ZN5boost10shared_ptrIN3RBX7TextureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::Texture>::shared_ptr<RBX::Texture,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7TextureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_491800() -> ! {
    todo!("0x491800 boost::shared_ptr<RBX::Texture>::shared_ptr<RBX::Texture,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4918c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(boost::shared_ptr<RBX::Texture> const*,RBX::Texture *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4918c8() -> ! {
    todo!("0x4918c8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(boost::shared_ptr<RBX::Texture> const*,RBX::Texture *)const")
}

// 0x4919b0 — __ZN5boost6detail12shared_countC2IPN3RBX7TextureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7TextureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4919b0() -> ! {
    todo!("0x4919b0 boost::detail::shared_count::shared_count<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x491ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_491ab8() -> ! {
    todo!("0x491ab8 boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x491abc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_491abc() -> ! {
    todo!("0x491abc boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x491ac0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_491ac0() -> ! {
    todo!("0x491ac0 boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x491ae0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_491ae0() -> ! {
    todo!("0x491ae0 boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x491af8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_491af8() -> ! {
    todo!("0x491af8 boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x491afc — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev
pub fn stub_491afc() -> ! {
    todo!("0x491afc __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")
}

// 0x491d40 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev
pub fn stub_491d40() -> ! {
    todo!("0x491d40 __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev")
}

// 0x491ddc — __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv
pub fn stub_491ddc() -> ! {
    todo!("0x491ddc __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv")
}

// 0x491f20 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5DecalEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::Decal> RBX::Creatable<RBX::Instance>::create<RBX::Decal>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5DecalEEEN5boost10shared_ptrIT_EEv
pub fn stub_491f20() -> ! {
    todo!("0x491f20 boost::shared_ptr<RBX::Decal> RBX::Creatable<RBX::Instance>::create<RBX::Decal>(void)")
}

// 0x491fd0 — __ZN5boost10shared_ptrIN3RBX5DecalEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::Decal>::shared_ptr<RBX::Decal,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5DecalEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_491fd0() -> ! {
    todo!("0x491fd0 boost::shared_ptr<RBX::Decal>::shared_ptr<RBX::Decal,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x492098 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(boost::shared_ptr<RBX::Decal> const*,RBX::Decal *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_492098() -> ! {
    todo!("0x492098 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(boost::shared_ptr<RBX::Decal> const*,RBX::Decal *)const")
}
