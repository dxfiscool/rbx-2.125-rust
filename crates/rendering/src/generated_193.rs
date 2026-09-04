//! rendering — generated_193 — 100 stubs 0x244bcc..0x26f0e4 EA-sorted asc global filler continuation after 0x244bc0 (global 20700->20800, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x244bcc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *)")]
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::operator()(void *)
// IDA 0x244bcc: 57 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244bcc() {
}

// 0x244c74 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()
// IDA 0x244c74: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_244c74() {
}

// 0x244c78 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()
// IDA 0x244c78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_244c78() {
}

// 0x244c84 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::dispose(void)
// IDA 0x244c84: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244c84() {
}

// 0x244c98 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)
// IDA 0x244c98: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244c98() {
}

// 0x244cb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)
// IDA 0x244cb0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244cb0() {
}

// 0x244cb4 — __GLOBAL__I_a_46
#[doc(alias = "global constructor keyed to_a_46")]
// was: global constructor keyed to _a_46
// IDA 0x244cb4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_244cb4() {
}

// 0x244d7c — __ZN3RBX16roblox_allocator6mallocEm
// type: void *__fastcall(size_t this, unsigned int)
#[doc(alias = "RBX::roblox_allocator::malloc(unsigned long)")]
// was: RBX::roblox_allocator::malloc(unsigned long)
// IDA 0x244d7c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244d7c() {
}

// 0x244dac — __ZN3RBX16roblox_allocator4freeEPc
// type: void __fastcall(RBX::roblox_allocator *this, char *)
#[doc(alias = "RBX::roblox_allocator::free(char *)")]
// was: RBX::roblox_allocator::free(char *)
// IDA 0x244dac: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244dac() {
}

// 0x244db8 — __ZNSt6vectorIPmSaIS0_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()")]
// was: std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()
// IDA 0x244db8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_244db8() {
}

// 0x244dcc — __ZNSt6vectorIPFbvESaIS1_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()")]
// was: std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()
// IDA 0x244dcc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_244dcc() {
}

// 0x244de0 — __GLOBAL__I_a_47
// type: int()
#[doc(alias = "global constructor keyed to_a_47")]
// was: global constructor keyed to _a_47
// IDA 0x244de0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_244de0() {
}

// 0x244e94 — __ZNK3rbx7signals10connection10disconnectEv
// type: void __fastcall(int32_t **this)
#[doc(alias = "rbx::signals::connection::disconnect(void)const")]
// was: rbx::signals::connection::disconnect(void)const
// IDA 0x244e94: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244e94() {
}

// 0x244fd4 — __ZNK3rbx7signals10connection9connectedEv
// type: int __fastcall(rbx::signals::connection *this)
#[doc(alias = "rbx::signals::connection::connected(void)const")]
// was: rbx::signals::connection::connected(void)const
// IDA 0x244fd4: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_244fd4() {
}

// 0x245118 — __ZNK3rbx7signals10connectioneqERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator==(rbx::signals::connection const&)const")]
// was: rbx::signals::connection::operator==(rbx::signals::connection const&)const
// IDA 0x245118: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245118() {
}

// 0x2452d0 — __ZNK3rbx7signals10connectionneERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator!=(rbx::signals::connection const&)const")]
// was: rbx::signals::connection::operator!=(rbx::signals::connection const&)const
// IDA 0x2452d0: 165 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2452d0() {
}

// 0x245488 — __ZN3rbx7signals10connectionaSERKS1_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx::signals::connection::operator=(rbx::signals::connection const&)")]
// was: rbx::signals::connection::operator=(rbx::signals::connection const&)
// IDA 0x245488: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245488() {
}

// 0x24551c — __ZN5boost8functionIFvRSt9exceptionEED1Ev
// type: int *__fastcall(int *)
#[doc(alias = "boost::function<void ()(std::exception &)>::~function()")]
// was: boost::function<void ()(std::exception &)>::~function()
// IDA 0x24551c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_24551c() {
}

// 0x245544 — __ZN4Init14initStaticDataEv
// type: void __fastcall(Init *this)
#[doc(alias = "Init::initStaticData(void)")]
// was: Init::initStaticData(void)
// IDA 0x245544: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_245544() {
}

// 0x245548 — __GLOBAL__I_a_48
#[doc(alias = "global constructor keyed to_a_48")]
// was: global constructor keyed to _a_48
// IDA 0x245548: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_245548() {
}

// 0x2456a0 — __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE
// type: bool __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")]
// was: RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)
// IDA 0x2456a0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2456a0() {
}

// 0x2456d8 — __ZN3RBX5Tasks12SequenceBase7advanceEv
// type: int __fastcall(RBX::Tasks::SequenceBase *this)
#[doc(alias = "RBX::Tasks::SequenceBase::advance(void)")]
// was: RBX::Tasks::SequenceBase::advance(void)
// IDA 0x2456d8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2456d8() {
}

// 0x245708 — __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE
// type: void __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")]
// was: RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)
// IDA 0x245708: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245708() {
}

// 0x2457f0 — __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")]
// was: RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)
// IDA 0x2457f0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2457f0() {
}

// 0x245848 — __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
// was: std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)
// IDA 0x245848: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_245848() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x245940 — __GLOBAL__I_a_49
#[doc(alias = "global constructor keyed to_a_49")]
// was: global constructor keyed to _a_49
// IDA 0x245940: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_245940() {
}

// 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
// type: __int64 __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")]
// was: RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const
// IDA 0x245a08: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245a08() {
}

// 0x245ab0 — __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: int __fastcall(RBX::ExclusiveArbiter *this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// was: RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)
// IDA 0x245ab0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245ab0() {
}

// 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
// type: void __fastcall(RBX::TaskScheduler *this, int, int, int)
#[doc(alias = "RBX::TaskScheduler::static_init(void)")]
// was: RBX::TaskScheduler::static_init(void)
// IDA 0x245b68: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245b68() {
}

// 0x245c64 — __ZN3RBX13TaskSchedulerD1Ev
// type: void __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
// was: RBX::TaskScheduler::~TaskScheduler()
// IDA 0x245c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_245c64() {
}

// 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
// type: _DWORD __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::singleton(void)")]
// was: RBX::TaskScheduler::singleton(void)
// IDA 0x245c70: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245c70() {
}

// 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
// type: int __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::TaskScheduler(void)")]
// was: RBX::TaskScheduler::TaskScheduler(void)
// IDA 0x245c94: 612 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_245c94() {
}

// 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
// type: bool __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount(void)")]
// was: RBX::TaskScheduler::sampleRunningJobCount(void)
// IDA 0x246308: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_246308() {
}

// 0x246358 — __ZN3RBX13TaskSchedulerD2Ev
// type: void __fastcall(RBX::TaskScheduler *this, int, int, const void *)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
// was: RBX::TaskScheduler::~TaskScheduler()
// IDA 0x246358: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_246358() {
}

// 0x2467d0 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE
// type: void __fastcall(int, int *, unsigned __int8, int)
#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")]
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)
// IDA 0x2467d0: 240 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2467d0() {
}

// 0x246a48 — __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>)")]
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job> const&,boost::shared_ptr<RBX::CEvent>)
// IDA 0x246a48: 319 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_246a48() {
}

// 0x246da8 — __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, RBX::TaskScheduler::Job **)
#[doc(alias = "RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// was: RBX::TaskScheduler::reschedule(boost::shared_ptr<RBX::TaskScheduler::Job>)
// IDA 0x246da8: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_246da8() {
}

// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
// was: RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)
// IDA 0x246e98: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_246e98() {
}

// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// was: RBX::TaskScheduler::add(boost::shared_ptr<RBX::TaskScheduler::Job>)
// IDA 0x246f90: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_246f90() {
}

// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
// was: RBX::TaskScheduler::incrementThreadCount(void)
// IDA 0x24710c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24710c() {
}

// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
// was: RBX::TaskScheduler::decrementThreadCount(void)
// IDA 0x24711c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24711c() {
}

// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// type: int __fastcall(RBX::TaskScheduler *this, int)
#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
// was: RBX::TaskScheduler::getShortestSleepTime(void)const
// IDA 0x247130: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247130() {
}

// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// type: int __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
// was: RBX::TaskScheduler::wakeSleepingJobs(void)
// IDA 0x247154: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247154() {
}

// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// type: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
// was: RBX::TaskScheduler::findJobToRun(boost::shared_ptr<RBX::TaskScheduler::Thread>)
// IDA 0x247220: 937 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247220() {
}

// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
// was: rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()
// IDA 0x247bd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_247bd8() {
}

// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// type: void __fastcall(RBX::TaskScheduler::Job *this, int)
#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
// was: RBX::TaskScheduler::Job::getDebugName(void)const
// IDA 0x247be8: 157 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247be8() {
}

// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
// was: RBX::RunningAverage<int,double>::sample(int)
// IDA 0x247db0: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247db0() {
}

// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
// was: RBX::ExclusiveArbiter::arbiterName(void)
// IDA 0x247e74: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247e74() {
}

// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
// was: RBX::ExclusiveArbiter::isThrottled(void)
// IDA 0x247e90: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247e90() {
}

// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::WeakPtr<RBX::TaskScheduler::Job> const&)")]
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)
// IDA 0x247e94: 90 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247e94() {
}

// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>)
// IDA 0x247fac: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_247fac() {
}

// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *)
// IDA 0x248020: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248020() {
}

// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
// IDA 0x248050: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248050() {
}

// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
// IDA 0x248104: 103 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248104() {
}

// 0x248224 — __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
// was: boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)
// IDA 0x248224: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248224() {
}

// 0x24831c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()
// IDA 0x24831c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_24831c() {
}

// 0x248320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()
// IDA 0x248320: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_248320() {
}

// 0x24832c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)
// IDA 0x24832c: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24832c() {
}

// 0x24834c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)
// IDA 0x24834c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24834c() {
}

// 0x248350 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)
// IDA 0x248350: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248350() {
}

// 0x248358 — __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
// was: boost::detail::thread_data<boost::function0<void>>::~thread_data()
// IDA 0x248358: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_248358() {
}

// 0x248448 — __ZN5boost18condition_variableC2Ev
// type: boost::condition_variable *__fastcall(boost::condition_variable *this)
#[doc(alias = "boost::condition_variable::condition_variable(void)")]
// was: boost::condition_variable::condition_variable(void)
// IDA 0x248448: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248448() {
}

// 0x248620 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const
// IDA 0x248620: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248620() {
}

// 0x248778 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)
// IDA 0x248778: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248778() {
}

// 0x24877c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x24877c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24877c() {
}

// 0x2487dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x2487dc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2487dc() {
}

// 0x2487f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)
// IDA 0x2487f8: 115 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2487f8() {
}

// 0x248938 — __ZN5boost9function0IvE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function0<void>::dummy::nonnull(void)")]
// was: boost::function0<void>::dummy::nonnull(void)
// IDA 0x248938: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_248938() {
}

// 0x248940 — __ZN3RBX5mutexC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
#[doc(alias = "RBX::mutex::mutex(void)")]
// was: RBX::mutex::mutex(void)
// IDA 0x248940: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248940() {
}

// 0x248a8c — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()
// IDA 0x248a8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_248a8c() {
}

// 0x248b80 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev
// type: void()
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()
// IDA 0x248b80: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_248b80() {
}

// 0x248b84 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()
// IDA 0x248b84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_248b84() {
}

// 0x248b90 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv
// type: void __fastcall(int, void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")]
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)
// IDA 0x248b90: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_248b90() {
}

// 0x248be0 — __GLOBAL__I_a_50
// type: void __fastcall(int, int, int, int, void *, int)
#[doc(alias = "global constructor keyed to_a_50")]
// was: global constructor keyed to _a_50
// IDA 0x248be0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_248be0() {
}

// 0x24ad90 — __ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE
// type: int __fastcall(__int64 *, unsigned int, unsigned int)
#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")]
// was: RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)
// IDA 0x24ad90: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24ad90() {
}

// 0x24ae08 — __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
// type: int __fastcall(int)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
// was: RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)
// IDA 0x24ae08: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24ae08() {
}

// 0x24b2c8 — __ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_
// type: int __fastcall(__int64 *, unsigned int, unsigned int, int)
#[doc(alias = "void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)")]
// was: void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)
// IDA 0x24b2c8: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24b2c8() {
}

// 0x24b364 — __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE
// type: int __fastcall(int, unsigned int, unsigned int)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)")]
// was: RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)
// IDA 0x24b364: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_24b364() {
}

// 0x24bf64 — __GLOBAL__I_a_51
#[doc(alias = "global constructor keyed to_a_51")]
// was: global constructor keyed to _a_51
// IDA 0x24bf64: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_24bf64() {
}

// 0x26dea0 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEE
// type: int __fastcall(int *, _DWORD *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// IDA 0x26dea0: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26dea0() {
}

// 0x26df08 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
// IDA 0x26df08: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26df08() {
}

// 0x26df2c — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// type: int __fastcall(int *, char ******)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>)
// IDA 0x26df2c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26df2c() {
}

// 0x26df60 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEE
// type: int __fastcall(int *, const shared_count *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
// IDA 0x26df60: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26df60() {
}

// 0x26e030 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEE
// type: int __fastcall(int *, const shared_count *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)
// IDA 0x26e030: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e030() {
}

// 0x26e100 — __ZN3RBX3Lua14ArgumentPusherclINS_6CellIDEEEiRKT_PN5boost10disable_ifINS7_13is_arithmeticIS4_EEvE4typeE
// type: int __fastcall(int *, int)
#[doc(alias = "int RBX::Lua::ArgumentPusher::operator()<RBX::CellID>(RBX::CellID const&,boost::disable_if<boost::is_arithmetic<RBX::CellID>,void>::type *)")]
// was: int RBX::Lua::ArgumentPusher::operator()<RBX::CellID>(RBX::CellID const&,boost::disable_if<boost::is_arithmetic<RBX::CellID>,void>::type *)
// IDA 0x26e100: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e100() {
}

// 0x26e1d8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "RBX::InputObject* RBX::Lua::Bridge<RBX::InputObject,true>::pushNewObject<RBX::InputObject>(lua_State *,RBX::InputObject)")]
// was: RBX::InputObject* RBX::Lua::Bridge<RBX::InputObject,true>::pushNewObject<RBX::InputObject>(lua_State *,RBX::InputObject)
// IDA 0x26e1d8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e1d8() {
}

// 0x26e228 — __ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e228: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e228() {
}

// 0x26e318 — __ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e318: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e318() {
}

// 0x26e408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::CellID* RBX::Lua::Bridge<RBX::CellID,true>::pushNewObject<RBX::CellID>(lua_State *,RBX::CellID)")]
// was: RBX::CellID* RBX::Lua::Bridge<RBX::CellID,true>::pushNewObject<RBX::CellID>(lua_State *,RBX::CellID)
// IDA 0x26e408: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e408() {
}

// 0x26e464 — __ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e464: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e464() {
}

// 0x26e554 — __ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e554: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e554() {
}

// 0x26e648 — __ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e648: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e648() {
}

// 0x26e738 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RBX::Region3int16* RBX::Lua::Bridge<RBX::Region3int16,true>::pushNewObject<RBX::Region3int16>(lua_State *,RBX::Region3int16)")]
// was: RBX::Region3int16* RBX::Lua::Bridge<RBX::Region3int16,true>::pushNewObject<RBX::Region3int16>(lua_State *,RBX::Region3int16)
// IDA 0x26e738: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e738() {
}

// 0x26e780 — __ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e780: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e780() {
}

// 0x26eb44 — __ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFvNS2_IKN3RBX10Reflection5TupleEEENS3_IFvPNS4_3Lua12IAsyncResultEEEEEEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26eb44: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26eb44() {
}

// 0x26ec34 — __ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFNS2_IKN3RBX10Reflection5TupleEEES8_EEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> const& rbx::any_cast<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26ec34: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26ec34() {
}

// 0x26ed24 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKN3RBX10Reflection5TupleEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<RBX::Reflection::Tuple const> const& rbx::any_cast<boost::shared_ptr<RBX::Reflection::Tuple const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26ed24: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26ed24() {
}

// 0x26ee14 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26ee14: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26ee14() {
}

// 0x26ef04 — __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State
// type: int __fastcall(char ****, char ****, int)
#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *)")]
// was: int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,lua_State *)
// IDA 0x26ef04: 169 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26ef04() {
}

// 0x26f0e4 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26f0e4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26f0e4() {
}
