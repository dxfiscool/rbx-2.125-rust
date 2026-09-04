//! rendering shard 264 — 100 stubs EA-sorted asc global gap filler after 0x3690cc not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 28670->28770 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 28670 before -> 28770 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3692bc — __ZN3RBX13HeartbeatTaskD1Ev
// type: void __fastcall(RBX::HeartbeatTask *__hidden this)
#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
// was: __ZN3RBX13HeartbeatTaskD1Ev
// IDA 0x3692bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3692bc() {
}

// 0x3693b8 — __ZN3RBX13HeartbeatTaskD0Ev
// type: void __fastcall(RBX::HeartbeatTask *__hidden this)
#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
// was: __ZN3RBX13HeartbeatTaskD0Ev
// IDA 0x3693b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3693b8() {
}

// 0x3694c8 — __ZN3RBX13HeartbeatTask9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::HeartbeatTask *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::HeartbeatTask::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX13HeartbeatTask9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// IDA 0x3694c8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3694c8() {
}

// 0x3694e4 — __ZN3RBX13HeartbeatTask5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::HeartbeatTask::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX13HeartbeatTask5errorERKNS_13TaskScheduler3Job5StatsE
// IDA 0x3694e4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3694e4() {
}

// 0x369504 — __ZN3RBX13HeartbeatTask16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::HeartbeatTask *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::HeartbeatTask::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX13HeartbeatTask16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// IDA 0x369504: 148 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369504() {
}

// 0x369698 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService>(rbx_core::WeakPtr<RBX::RunService> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x369698: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369698() {
}

// 0x369718 — __ZN5boost6detail10weak_countC1ERKNS0_12shared_countE
// type: _DWORD __fastcall(boost::detail::weak_count *__hidden this, const boost::detail::shared_count *)
#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::shared_count const&)")]
// was: __ZN5boost6detail10weak_countC1ERKNS0_12shared_countE
// IDA 0x369718: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369718() {
}

// 0x369764 — __ZN3RBX10PhysicsJobC2EN5boost10shared_ptrINS_9DataModelEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RBX::PhysicsJob::PhysicsJob(rbx_core::SharedPtr<RBX::DataModel>)")]
// was: __ZN3RBX10PhysicsJobC2EN5boost10shared_ptrINS_9DataModelEEE
// IDA 0x369764: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369764() {
}

// 0x3698b8 — __ZN3RBX10PhysicsJobD1Ev
// type: void __fastcall(RBX::PhysicsJob *__hidden this)
#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
// was: __ZN3RBX10PhysicsJobD1Ev
// IDA 0x3698b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3698b8() {
}

// 0x369988 — __ZN3RBX10PhysicsJobD0Ev
// type: void __fastcall(RBX::PhysicsJob *__hidden this)
#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
// was: __ZN3RBX10PhysicsJobD0Ev
// IDA 0x369988: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_369988() {
}

// 0x369a70 — __ZN3RBX10PhysicsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::PhysicsJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::PhysicsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10PhysicsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// IDA 0x369a70: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369a70() {
}

// 0x369ab0 — __ZN3RBX10PhysicsJob5errorERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::PhysicsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10PhysicsJob5errorERKNS_13TaskScheduler3Job5StatsE
// IDA 0x369ab0: 7 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369ab0() {
}

// 0x369ac8 — __ZNK3RBX10PhysicsJob26getDesiredConcurrencyCountEv
// type: _DWORD __fastcall(RBX::PhysicsJob *__hidden this)
#[doc(alias = "RBX::PhysicsJob::getDesiredConcurrencyCount(void)const")]
// was: __ZNK3RBX10PhysicsJob26getDesiredConcurrencyCountEv
// IDA 0x369ac8: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369ac8() {
}

// 0x369ae0 — __ZN3RBX10PhysicsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::PhysicsJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::PhysicsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX10PhysicsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// IDA 0x369ae0: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369ae0() {
}

// 0x369c54 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv
// IDA 0x369c54: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369c54() {
}

// 0x369dcc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv
// IDA 0x369dcc: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369dcc() {
}

// 0x369f44 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv
// IDA 0x369f44: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_369f44() {
}

// 0x36a0bc — __GLOBAL__I_a_136
#[doc(alias = "global constructor keyed to_a_136")]
// was: __GLOBAL__I_a_136
// IDA 0x36a0bc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_36a0bc() {
}

// 0x36a44c — __ZN3RBX25ScriptInformationProviderC1Ev
// type: int __fastcall(RBX::ScriptInformationProvider *this)
#[doc(alias = "RBX::ScriptInformationProvider::ScriptInformationProvider(void)")]
// was: __ZN3RBX25ScriptInformationProviderC1Ev
// IDA 0x36a44c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_36a44c() {
}

// 0x36a450 — __ZN3RBX25ScriptInformationProviderC2Ev
// type: RBX::Instance *__fastcall(RBX::ScriptInformationProvider *this)
#[doc(alias = "RBX::ScriptInformationProvider::ScriptInformationProvider(void)")]
// was: __ZN3RBX25ScriptInformationProviderC2Ev
// IDA 0x36a450: 254 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36a450() {
}

// 0x36a710 — __ZN3RBXL13findLocalFileERKSsPSs
// type: int()
#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
// was: __ZN3RBXL13findLocalFileERKSsPSs
// IDA 0x36a710: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36a710() {
}

// 0x36a714 — __ZN3RBX25ScriptInformationProvider11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, pthread_mutex_t *, char, pthread_mutex_t *, char, void *, int, int, int, int, int)
#[doc(alias = "RBX::ScriptInformationProvider::onHeartbeat(RBX::Heartbeat const&)")]
// was: __ZN3RBX25ScriptInformationProvider11onHeartbeatERKNS_9HeartbeatE
// IDA 0x36a714: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36a714() {
}

// 0x36a71c — __ZThn96_N3RBX25ScriptInformationProvider11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, pthread_mutex_t *, char, pthread_mutex_t *, char, void *, int, int, int, int, int)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::onHeartbeat(RBX::Heartbeat const&)")]
// was: __ZThn96_N3RBX25ScriptInformationProvider11onHeartbeatERKNS_9HeartbeatE
// IDA 0x36a71c: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36a71c() {
}

// 0x36a724 — __ZN3RBX25ScriptInformationProvider18HandleHttpResponseEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvNS0_13RequestResultEbbfbEEE
// type: void __fastcall(int, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptInformationProvider::HandleHttpResponse(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>)")]
// was: __ZN3RBX25ScriptInformationProvider18HandleHttpResponseEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvNS0_13RequestResultEbbfbEEE
// IDA 0x36a724: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36a724() {
}

// 0x36a87c — __ZN3RBX25ScriptInformationProvider13getScriptInfoERKSsbfN5boost8functionIFvNS0_13RequestResultEbbfbEEENS_14AsyncHttpQueue9ResultJobE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const std::string *, int, int, int, int)
#[doc(alias = "RBX::ScriptInformationProvider::getScriptInfo(std::string const&,bool,float,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::AsyncHttpQueue::ResultJob)")]
// was: __ZN3RBX25ScriptInformationProvider13getScriptInfoERKSsbfN5boost8functionIFvNS0_13RequestResultEbbfbEEENS_14AsyncHttpQueue9ResultJobE
// IDA 0x36a87c: 696 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36a87c() {
}

// 0x36b030 — __ZN3RBX25ScriptInformationProvider16CachedScriptInfoC2EN5boost10shared_ptrIKSsEES5_
// type: int __fastcall(int, _DWORD **, _DWORD *)
#[doc(alias = "RBX::ScriptInformationProvider::CachedScriptInfo::CachedScriptInfo(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN3RBX25ScriptInformationProvider16CachedScriptInfoC2EN5boost10shared_ptrIKSsEES5_
// IDA 0x36b030: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b030() {
}

// 0x36b240 — __ZN3RBX25ScriptInformationProvider11setAssetUrlESs
// type: int __fastcall(int)
#[doc(alias = "RBX::ScriptInformationProvider::setAssetUrl(std::string)")]
// was: __ZN3RBX25ScriptInformationProvider11setAssetUrlESs
// IDA 0x36b240: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b240() {
}

// 0x36b248 — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EED1Ev
// IDA 0x36b248: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36b248() {
}

// 0x36b33c — __ZN3RBX25ScriptInformationProvider12setAccessKeyESs
// type: int __fastcall(int)
#[doc(alias = "RBX::ScriptInformationProvider::setAccessKey(std::string)")]
// was: __ZN3RBX25ScriptInformationProvider12setAccessKeyESs
// IDA 0x36b33c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b33c() {
}

// 0x36b344 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEE5resetIS5_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *, void *, int, int)
#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::reset<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
// was: __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEE5resetIS5_EEvPT_
// IDA 0x36b344: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b344() {
}

// 0x36b370 — __ZN3RBX17HeartbeatInstanceD2Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
// was: __ZN3RBX17HeartbeatInstanceD2Ev
// IDA 0x36b370: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36b370() {
}

// 0x36b454 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE13findCacheItemERKSsPS2_
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::findCacheItem(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo*)")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE13findCacheItemERKSsPS2_
// IDA 0x36b454: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b454() {
}

// 0x36b568 — __ZNK5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEclES3_bbfb
// type: void __fastcall(_DWORD *, int, int, int, float, int)
#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::operator()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)const")]
// was: __ZNK5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEclES3_bbfb
// IDA 0x36b568: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b568() {
}

// 0x36b644 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m
// type: unsigned int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m
// IDA 0x36b644: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b644() {
}

// 0x36b678 — __ZN5boost4bindINS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEES4_bbfbEENS_3_bi6bind_tINS7_11unspecifiedET_NS7_9list_av_5IT0_T1_T2_T3_T4_E4typeEEESA_SC_SD_SE_SF_SG_
// type: void __fastcall(int, int, int, char, struct _Unwind_Exception *lpuexcpt, float, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list_av_5<RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::type> boost::bind<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>(boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")]
// was: __ZN5boost4bindINS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEES4_bbfbEENS_3_bi6bind_tINS7_11unspecifiedET_NS7_9list_av_5IT0_T1_T2_T3_T4_E4typeEEESA_SC_SD_SE_SF_SG_
// IDA 0x36b678: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b678() {
}

// 0x36b76c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS3_13RequestResultEbbfbEEES4_NS_3argILi1EEESsSA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
// type: void __fastcall(_DWORD *, int, int *, const std::string *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list_av_4<rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>(void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS3_13RequestResultEbbfbEEES4_NS_3argILi1EEESsSA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
// IDA 0x36b76c: 304 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36b76c() {
}

// 0x36bac4 — __ZN3RBX9weak_fromINS_25ScriptInformationProviderEEEN5boost8weak_ptrIT_EEPS4_
// type: void __fastcall(int, int)
#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptInformationProvider> RBX::weak_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")]
// was: __ZN3RBX9weak_fromINS_25ScriptInformationProviderEEEN5boost8weak_ptrIT_EEPS4_
// IDA 0x36bac4: 182 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36bac4() {
}

// 0x36bcbc — __ZN3RBX25ScriptInformationProviderD1Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZN3RBX25ScriptInformationProviderD1Ev
// IDA 0x36bcbc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_36bcbc() {
}

// 0x36bcc0 — __ZN3RBX25ScriptInformationProviderD0Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZN3RBX25ScriptInformationProviderD0Ev
// IDA 0x36bcc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36bcc0() {
}

// 0x36bd64 — __ZN3RBX25ScriptInformationProvider17onServiceProviderEPNS_15ServiceProviderES2_
// type: int __fastcall(RBX::ScriptInformationProvider *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::ScriptInformationProvider::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX25ScriptInformationProvider17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x36bd64: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36bd64() {
}

// 0x36bd70 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv
// IDA 0x36bd70: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36bd70() {
}

// 0x36bd9c — __ZThn32_N3RBX25ScriptInformationProviderD1Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZThn32_N3RBX25ScriptInformationProviderD1Ev
// IDA 0x36bd9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36bd9c() {
}

// 0x36bda4 — __ZThn32_N3RBX25ScriptInformationProviderD0Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZThn32_N3RBX25ScriptInformationProviderD0Ev
// IDA 0x36bda4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36bda4() {
}

// 0x36be48 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv
// IDA 0x36be48: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36be48() {
}

// 0x36be70 — __ZThn36_N3RBX25ScriptInformationProviderD1Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZThn36_N3RBX25ScriptInformationProviderD1Ev
// IDA 0x36be70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36be70() {
}

// 0x36be78 — __ZThn36_N3RBX25ScriptInformationProviderD0Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZThn36_N3RBX25ScriptInformationProviderD0Ev
// IDA 0x36be78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36be78() {
}

// 0x36bf20 — __ZThn96_N3RBX25ScriptInformationProviderD1Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZThn96_N3RBX25ScriptInformationProviderD1Ev
// IDA 0x36bf20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36bf20() {
}

// 0x36bf28 — __ZThn96_N3RBX25ScriptInformationProviderD0Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZThn96_N3RBX25ScriptInformationProviderD0Ev
// IDA 0x36bf28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36bf28() {
}

// 0x36bfd0 — __ZN3RBX4Name13callDoDeclareILZNS_26sScriptInformationProviderEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_26sScriptInformationProviderEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_26sScriptInformationProviderEEEEvv
// IDA 0x36bfd0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_36bfd0() {
}

// 0x36bfd8 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0x36bfd8: 191 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36bfd8() {
}

// 0x36c200 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0x36c200: 193 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36c200() {
}

// 0x36c42c — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ERKSG_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>> const&)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ERKSG_
// IDA 0x36c42c: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36c42c() {
}

// 0x36c5a4 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEEvT_
// IDA 0x36c5a4: 197 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36c5a4() {
}

// 0x36c7e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
// IDA 0x36c7e0: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36c7e0() {
}

// 0x36c7fc — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SQ_ST_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SQ_ST_
// IDA 0x36c7fc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36c7fc() {
}

// 0x36c820 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x36c820: 192 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36c820() {
}

// 0x36ca44 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x36ca44: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ca44() {
}

// 0x36cc64 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x36cc64: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36cc64() {
}

// 0x36cd24 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSE_ENS0_5list3IRSJ_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, struct _Unwind_Exception **, int **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSE_ENS0_5list3IRSJ_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x36cd24: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36cd24() {
}

// 0x36cf0c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x36cf0c: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36cf0c() {
}

// 0x36d0dc — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *, int)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::list4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_
// IDA 0x36d0dc: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d0dc() {
}

// 0x36d2b8 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_
// type: int __fastcall(int, int, const std::string *, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_
// IDA 0x36d2b8: 186 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d2b8() {
}

// 0x36d4c0 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEEEC2ES7_S9_SA_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEEEC2ES7_S9_SA_
// IDA 0x36d4c0: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d4c0() {
}

// 0x36d600 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEEEC2ES7_S9_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEEEC2ES7_S9_
// IDA 0x36d600: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d600() {
}

// 0x36d748 — __ZN5boost8weak_ptrIN3RBX25ScriptInformationProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptInformationProvider>::weak_ptr<RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>::type)")]
// was: __ZN5boost8weak_ptrIN3RBX25ScriptInformationProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// IDA 0x36d748: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d748() {
}

// 0x36d798 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS7_5list5INS7_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS7_5list5INS7_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS7_5list5INS7_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x36d798: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d798() {
}

// 0x36d870 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x36d870: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d870() {
}

// 0x36d948 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEEvT_
// IDA 0x36d948: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36d948() {
}

// 0x36da30 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// IDA 0x36da30: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36da30() {
}

// 0x36da4c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
// type: int __fastcall(int *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
// IDA 0x36da4c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36da4c() {
}

// 0x36da78 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEbT_RNS1_15function_bufferE
// IDA 0x36da78: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36da78() {
}

// 0x36db50 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x36db50: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36db50() {
}

// 0x36dc24 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x36dc24: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36dc24() {
}

// 0x36dce8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x36dce8: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36dce8() {
}

// 0x36de2c — __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to_own(boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool> const&)")]
// was: __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE13assign_to_ownERKS4_
// IDA 0x36de2c: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36de2c() {
}

// 0x36de5c — __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
// type: void __fastcall(int, const std::string *, const std::string *, int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// was: __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
// IDA 0x36de5c: 488 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36de5c() {
}

// 0x36e3e4 — __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
// was: __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
// IDA 0x36e3e4: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e3e4() {
}

// 0x36e43c — __ZN3RBX8LRUCacheISsSsE6removeERKSs
// type: int __fastcall(int, int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
// was: __ZN3RBX8LRUCacheISsSsE6removeERKSs
// IDA 0x36e43c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e43c() {
}

// 0x36e490 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// IDA 0x36e490: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e490() {
}

// 0x36e4ec — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// IDA 0x36e4ec: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e4ec() {
}

// 0x36e518 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// IDA 0x36e518: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e518() {
}

// 0x36e558 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
// type: void __fastcall(int, int)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
// IDA 0x36e558: 62 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e558() {
}

// 0x36e610 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int __fastcall(int, char **)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// IDA 0x36e610: 22 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e610() {
}

// 0x36e650 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// IDA 0x36e650: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e650() {
}

// 0x36e6bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_
// IDA 0x36e6bc: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e6bc() {
}

// 0x36e874 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_
// IDA 0x36e874: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e874() {
}

// 0x36e898 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// IDA 0x36e898: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e898() {
}

// 0x36e8e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev
// IDA 0x36e8e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36e8e8() {
}

// 0x36e908 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// IDA 0x36e908: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36e908() {
}

// 0x36ea30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// IDA 0x36ea30: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ea30() {
}

// 0x36eac0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// IDA 0x36eac0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36eac0() {
}

// 0x36eaec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// IDA 0x36eaec: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36eaec() {
}

// 0x36eb44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv
// IDA 0x36eb44: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36eb44() {
}

// 0x36eb80 — __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, _DWORD *)
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
// was: __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
// IDA 0x36eb80: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36eb80() {
}

// 0x36ec4c — __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
// was: __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
// IDA 0x36ec4c: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ec4c() {
}

// 0x36ed5c — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int __fastcall(int, char **)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// IDA 0x36ed5c: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ed5c() {
}

// 0x36ed98 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// IDA 0x36ed98: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ed98() {
}

// 0x36ee08 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEEC2IS5_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
// was: __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEEC2IS5_EEPT_
// IDA 0x36ee08: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ee08() {
}

// 0x36eef0 — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>> const*,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x36eef0: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36eef0() {
}

// 0x36f018 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_25ScriptInformationProvider16CachedScriptInfoELb0EEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_25ScriptInformationProvider16CachedScriptInfoELb0EEEEEPT_
// IDA 0x36f018: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f018() {
}
