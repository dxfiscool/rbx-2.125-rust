//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 1 remaining before batch (next 0xecd6e8 _TFCreateCrashSocket); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x3e8b8..0xecd6e8 | existing 17809 -> 17909 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
// 0x3e8b8 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv
// demangled: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const")]
pub fn stub_3e8b8(rethrow: &mut dyn FnMut(), destroy: &mut dyn FnMut()) {
    // IDA 0x3e8b8: virtual thunk adjusts; rethrow then ~clone_impl.
    rethrow();
    destroy();
}

// 0x3e8c8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev
// demangled: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
pub fn stub_3e8c8(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3e8c8: virtual thunk adjusts; member/base dtors + delete.
    destroy();
    free();
}

// 0x3e900 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE
// demangled: boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_tag)
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_tag)")]
pub fn stub_3e900(dst: usize, src: usize, copy: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x3e900: clone_impl<bad_alloc_> copy construct (below truncation).
    copy(dst, src);
    dst
}

// 0x3ea80 — __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev
// demangled: non-virtual thunk to boost::exception_detail::bad_alloc_::~bad_alloc_()
// type: void __fastcall(boost::exception_detail::bad_alloc_ *__hidden this)
#[doc(alias = "non-virtual thunk to boost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_3ea80(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3ea80: bad_alloc_ thunk dtor + delete.
    destroy();
    free();
}

// 0x3eab0 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_
// demangled: boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)
#[doc(alias = "rbx_core::SharedPtr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
pub fn stub_3eab0(make: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3eab0: shared_ptr<clone_base> construct from clone_impl (below truncation).
    make()
}

// 0x3eb98 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::dispose(void)")]
pub fn stub_3eb98(px: usize, destroy: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3eb98: null px -> 0 else virtual destroy (+4).
    if px != 0 {
        destroy(px)
    } else {
        0
    }
}

// 0x3eba8 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_deleter(std::type_info const&)")]
pub fn stub_3eba8() -> usize {
    // IDA 0x3eba8: plain impl_p has no deleter -> 0.
    0
}

// 0x3ebb0 — __ZN3RBX5Tasks8Sequence9onPreStepEPNS_13TaskScheduler3JobE
// demangled: RBX::Tasks::Sequence::onPreStep(RBX::TaskScheduler::Job *)
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::Sequence::onPreStep(RBX::TaskScheduler::Job *)")]
pub fn stub_3ebb0(advance: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x3ebb0: Sequence::onPreStep thunk tail-calls SequenceBase::advance.
    advance()
}

// 0x3ebb4 — __ZN3RBX5Tasks17ExclusiveSequence10onPostStepEPNS_13TaskScheduler3JobE
// demangled: RBX::Tasks::ExclusiveSequence::onPostStep(RBX::TaskScheduler::Job *)
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::ExclusiveSequence::onPostStep(RBX::TaskScheduler::Job *)")]
pub fn stub_3ebb4(advance: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x3ebb4: ExclusiveSequence::onPostStep thunk tail-calls SequenceBase::advance.
    advance()
}

// 0x3ebb8 — __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// demangled: void boost::intrusive_ptr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
// type: int(void)
#[doc(alias = "void rbx_core::SharedPtr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
pub fn stub_3ebb8(weak: &mut u32, strong: u32, check_asserts: bool, destroy: &mut dyn FnMut()) {
    // IDA 0x3ebb8: weak--; at zero ReleaseAssert(strong == 0) then destroy.
    if *weak > 0 {
        *weak -= 1;
    }
    if *weak == 0 {
        if check_asserts {
            assert!(strong == 0, "strong == 0");
        }
        destroy();
    }
}

// 0x3ec30 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
// demangled: boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
pub fn stub_3ec30(destroy: &mut dyn FnMut()) {
    // IDA 0x3ec30: scoped_ptr<LogManager> D1 thunk tail-calls D2.
    destroy();
}

// 0x3ec34 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
// demangled: boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
pub fn stub_3ec34(px: usize, destroy: &mut dyn FnMut(usize), dealloc: &mut dyn FnMut(usize)) {
    // IDA 0x3ec34: scoped_ptr D2 — LogManager dtor + dealloc when set.
    if px != 0 {
        destroy(px);
        dealloc(px);
    }
}

// 0x3eccc — __ZN17QuitEventListenerD0Ev
// demangled: QuitEventListener::~QuitEventListener()
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener()")]
pub fn stub_3eccc(obj: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x3eccc: QuitEventListener D0 thunk tail-calls operator delete.
    free(obj);
}

// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// demangled: Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
pub fn stub_3ecd0() {
    // IDA 0x3ecd0: empty windowMoved body.
}

// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// demangled: Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
pub fn stub_3ecd4() {
    // IDA 0x3ecd4: empty windowResized body.
}

// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// demangled: Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
pub fn stub_3ecd8() -> i32 {
    // IDA 0x3ecd8: windowClosing returns 1 (allow close).
    1
}

// 0x3ecdc — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
// demangled: QuitEventListener::windowClosed(Ogre::RenderWindow *)
// type: _DWORD __fastcall(QuitEventListener *__hidden this, RenderWindow *)
#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
pub fn stub_3ecdc(log: &mut dyn FnMut(&str)) {
    // IDA 0x3ecdc: puts "Request to close OGRE render window received".
    log("Request to close OGRE render window received");
}

// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// demangled: Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
pub fn stub_3ecec() {
    // IDA 0x3ecec: empty windowFocusChange body.
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// demangled: RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
pub fn stub_3ecf0(job: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x3ecf0: RenderJob::RenderJob (below truncation).
    init(job);
    job
}

// 0x3ee80 — __ZN10RobloxView9RenderJobD1Ev
// demangled: RobloxView::RenderJob::~RenderJob()
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
pub fn stub_3ee80(destroy: &mut dyn FnMut()) {
    // IDA 0x3ee80: RenderJob D2 — CEvent/weak/Job destroys (below truncation).
    destroy();
}

// 0x3ef40 — __ZN10RobloxView9RenderJobD0Ev
// demangled: RobloxView::RenderJob::~RenderJob()
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
pub fn stub_3ef40(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3ef40: RenderJob D0 — destroys + operator delete.
    destroy();
    free();
}

// 0x3f008 — __ZN10RobloxView9RenderJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// demangled: RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_3f008(has_stats: bool, sleep: &mut dyn FnMut(f64) -> f64, no_sleep: &mut dyn FnMut()) -> f64 {
    // IDA 0x3f008: stats flag ? computeStandardSleepTime(stats, 60.0) : store +inf.
    if has_stats {
        sleep(60.0)
    } else {
        no_sleep();
        f64::INFINITY
    }
}

// 0x3f058 — __ZN10RobloxView9RenderJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// demangled: RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_3f058(has_stats: bool, error: &mut dyn FnMut(f64) -> f64, clear: &mut dyn FnMut()) -> f64 {
    // IDA 0x3f058: stats flag ? computeStandardError(stats, 30.0) : zero.
    if has_stats {
        error(30.0)
    } else {
        clear();
        0.0
    }
}

// 0x3f090 — __ZNK3RBX13TaskScheduler3Job26getDesiredConcurrencyCountEv
// demangled: RBX::TaskScheduler::Job::getDesiredConcurrencyCount(void)const
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::getDesiredConcurrencyCount(void)const")]
pub fn stub_3f090() -> i32 {
    // IDA 0x3f090: getDesiredConcurrencyCount returns 1.
    1
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// demangled: RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_3f094(step: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x3f094: RenderJob::stepDataModelJob (below truncation).
    step()
}

// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
// demangled: RobloxView::RenderJob::getMetricValue(std::string const&)const
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn stub_3f598(name: &str, lookup: &mut dyn FnMut(&str) -> f64) -> f64 {
    // IDA 0x3f598: metric-name dispatch ("Render FPS" -> avg steps/s, "Render Duty" -> ..., ...).
    lookup(name)
}

// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
// demangled: RobloxView::RenderJob::getMetric(std::string const&)const
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn stub_3f700(name: &str, format: &mut dyn FnMut(&str) -> String) -> String {
    // IDA 0x3f700: getMetric — format metric value as string (below truncation).
    format(name)
}

// 0x3f904 — __ZThn480_N10RobloxView9RenderJobD1Ev
// demangled: non-virtual thunk to RobloxView::RenderJob::~RenderJob()
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
pub fn stub_3f904(destroy: &mut dyn FnMut()) {
    // IDA 0x3f904: thunk adjusts (-480) then ~RenderJob.
    destroy();
}

// 0x3f9c8 — __ZThn480_N10RobloxView9RenderJobD0Ev
// demangled: non-virtual thunk to RobloxView::RenderJob::~RenderJob()
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
pub fn stub_3f9c8(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3f9c8: thunk adjusts then ~RenderJob + delete.
    destroy();
    free();
}

// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// demangled: non-virtual thunk to RobloxView::RenderJob::getMetric(std::string const&)const
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn stub_3fa94(job: usize, name: &str, out: &mut String, get: &mut dyn FnMut(usize, &str, &mut String)) {
    // IDA 0x3fa94: thunk adjusts then getMetric.
    get(job, name, out);
}

// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// demangled: non-virtual thunk to RobloxView::RenderJob::getMetricValue(std::string const&)const
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn stub_3faa4(job: usize, name: &str, get: &mut dyn FnMut(usize, &str) -> f64) -> f64 {
    // IDA 0x3faa4: thunk adjusts then getMetricValue.
    get(job, name)
}

// 0x3faac — __ZN10RobloxView9RenderJob21scheduleRenderPrepareEPS0_PN3RBX8ViewBaseE
// demangled: RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RenderJob *, ViewBase *)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)")]
pub fn stub_3faac(scheduled: bool, this: usize, prepare: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x3faac: unscheduled ? schedule prepare : this.
    if !scheduled {
        prepare(this)
    } else {
        this
    }
}

// 0x3fac4 — __ZN10RobloxView9RenderJob21scheduleRenderPerformEPS0_PN3RBX8ViewBaseEd
// demangled: RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RobloxView::RenderJob *, RBX::ViewBase *, double)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)")]
pub fn stub_3fac4(perform: &mut dyn FnMut()) {
    // IDA 0x3fac4: RenderJob::scheduleRenderPerform (below truncation).
    perform();
}

// 0x3fb9c — __ZN10RobloxView9RenderJob4wakeEv
// demangled: RobloxView::RenderJob::wake(void)
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::wake(void)")]
pub fn stub_3fb9c(wake: &mut dyn FnMut()) {
    // IDA 0x3fb9c: RenderJob::wake — mutex + signal (below truncation).
    wake();
}

// 0x3fcf8 — __ZN5boost12bad_weak_ptrD0Ev
// demangled: boost::bad_weak_ptr::~bad_weak_ptr()
// type: void __fastcall(boost::bad_weak_ptr *__hidden this)
#[doc(alias = "boost::bad_weak_ptr::~bad_weak_ptr()")]
pub fn stub_3fcf8(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3fcf8: bad_weak_ptr dtor + delete.
    destroy();
    free();
}

// 0x3fd10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
// demangled: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_3fd10(obj: usize, destroy: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x3fd10: clone_impl dtor (no delete); return this.
    destroy(obj);
    obj
}

// 0x3fd38 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
// demangled: boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
pub fn stub_3fd38(destroy: &mut dyn FnMut()) {
    // IDA 0x3fd38: error_info_injector dtor (no delete).
    destroy();
}

// 0x3fd60 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
// demangled: non-virtual thunk to boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
pub fn stub_3fd60(destroy: &mut dyn FnMut()) {
    // IDA 0x3fd60: non-virtual thunk runs the injector dtor.
    destroy();
}

// 0x3fd88 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
// demangled: non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_3fd88(destroy: &mut dyn FnMut()) {
    // IDA 0x3fd88: non-virtual thunk runs the clone dtor.
    destroy();
}

// 0x3fdb8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
// demangled: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const
// type: int(void)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
pub fn stub_3fdb8() -> ! {
    // IDA 0x3fdb8: allocate + throw bad_weak_ptr clone (noreturn).
    panic!("bad_weak_ptr rethrow");
}

// 0x3fee0 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// demangled: non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_3fee0(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3fee0: thunk clone dtor + delete.
    destroy();
    free();
}

// 0x3ff18 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
// demangled: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
pub fn stub_3ff18(rethrow: &mut dyn FnMut(), destroy: &mut dyn FnMut()) {
    // IDA 0x3ff18: virtual thunk — rethrow then ~clone_impl.
    rethrow();
    destroy();
}

// 0x3ff28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// demangled: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_3ff28(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3ff28: virtual thunk clone dtor + delete.
    destroy();
    free();
}

// 0x3ff60 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
// demangled: boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
pub fn stub_3ff60(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3ff60: error_info_injector D0 + operator delete.
    destroy();
    free();
}

// 0x3ff90 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
// demangled: non-virtual thunk to boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
pub fn stub_3ff90(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3ff90: thunk injector D0 + operator delete.
    destroy();
    free();
}

// 0x3ffc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE
// demangled: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_tag)
// type: int __fastcall(int, int, int, int, char, std::exception *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_tag)")]
pub fn stub_3ffc0(dst: usize, src: usize, copy: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x3ffc0: clone_impl copy construct (below truncation).
    copy(dst, src);
    dst
}

// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_40160(op: u32, manage: &mut dyn FnMut(u32) -> usize) -> usize {
    // IDA 0x40160: functor_manager::manage — clone/move/destroy by op (below truncation).
    manage(op)
}

// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_401dc(f: usize, a: usize, b: usize, c: f64, invoke: &mut dyn FnMut(usize, usize, usize, f64)) {
    // IDA 0x401dc: invoker calls fn(job, view, double).
    invoke(f, a, b, c);
}

// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_401f0(op: u32, manage: &mut dyn FnMut(u32) -> usize) -> usize {
    // IDA 0x401f0: functor_manager::manage — clone/move/destroy by op (below truncation).
    manage(op)
}

// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_40270(obj: usize, is_virtual: bool, metric: usize, dt: f64, call: &mut dyn FnMut(usize, bool, usize, f64)) {
    // IDA 0x40270: mf2 dispatch (virtual adjust); obj->method(metric, dt).
    call(obj, is_virtual, metric, dt);
}

// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
pub fn stub_4027c() -> ! {
    todo!("0x4027c void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")
}

// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_402a8() -> ! {
    todo!("0x402a8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_40308() -> ! {
    todo!("0x40308 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// demangled: boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)
#[doc(alias = "rbx_core::WeakPtr<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
pub fn stub_40318() -> ! {
    todo!("0x40318 boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")
}

// 0x403f0 — __ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// demangled: RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")]
pub fn stub_403f0() -> ! {
    todo!("0x403f0 RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")
}

// 0x404f0 — __ZN10RobloxView13ViewUpdateJobD1Ev
// demangled: RobloxView::ViewUpdateJob::~ViewUpdateJob()
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob()")]
pub fn stub_404f0() -> ! {
    todo!("0x404f0 RobloxView::ViewUpdateJob::~ViewUpdateJob()")
}

// 0x4059c — __ZN10RobloxView13ViewUpdateJobD0Ev
// demangled: RobloxView::ViewUpdateJob::~ViewUpdateJob()
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob()")]
pub fn stub_4059c() -> ! {
    todo!("0x4059c RobloxView::ViewUpdateJob::~ViewUpdateJob()")
}

// 0x40650 — __ZN10RobloxView13ViewUpdateJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// demangled: RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_40650() -> ! {
    todo!("0x40650 RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x40680 — __ZN10RobloxView13ViewUpdateJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// demangled: RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_40680() -> ! {
    todo!("0x40680 RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x406a8 — __ZN10RobloxView13ViewUpdateJob17getPriorityFactorEv
// demangled: RobloxView::ViewUpdateJob::getPriorityFactor(void)
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::getPriorityFactor(void)")]
pub fn stub_406a8() -> ! {
    todo!("0x406a8 RobloxView::ViewUpdateJob::getPriorityFactor(void)")
}

// 0x406b4 — __ZN10RobloxView13ViewUpdateJob4stepERKN3RBX13TaskScheduler3Job5StatsE
// demangled: RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)
#[doc(alias = "RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_406b4() -> ! {
    todo!("0x406b4 RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x406e0 — __ZN5boost9function0IvE5clearEv
// demangled: boost::function0<void>::clear(void)
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::function0<void>::clear(void)")]
pub fn stub_406e0() -> ! {
    todo!("0x406e0 boost::function0<void>::clear(void)")
}

// 0x4070c — __GLOBAL__I_a_10
// demangled: global constructor keyed to_a_10
#[doc(alias = "global constructor keyed to_a_10")]
pub fn stub_4070c() -> ! {
    todo!("0x4070c global constructor keyed to_a_10")
}

// 0x40984 — -[UserInfo init]
// type: UserInfo *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo init]")]
pub fn stub_40984() -> ! {
    todo!("0x40984 -[UserInfo init]")
}

// 0x409b0 — -[UserInfo setUserLoggedIn:]
// type: void __cdecl(UserInfo *self, SEL, char)
#[doc(alias = "-[UserInfo setUserLoggedIn:]")]
pub fn stub_409b0() -> ! {
    todo!("0x409b0 -[UserInfo setUserLoggedIn:]")
}

// 0x40ab4 — -[UserInfo userLoggedIn]
// type: char __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userLoggedIn]")]
pub fn stub_40ab4() -> ! {
    todo!("0x40ab4 -[UserInfo userLoggedIn]")
}

// 0x40ac4 — -[UserInfo UpdatePlayerInfo]
// type: void __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo UpdatePlayerInfo]")]
pub fn stub_40ac4() -> ! {
    todo!("0x40ac4 -[UserInfo UpdatePlayerInfo]")
}

// 0x40c58 — ___28-[UserInfo UpdatePlayerInfo]_block_invoke
#[doc(alias = "___28-[UserInfo UpdatePlayerInfo]_block_invoke")]
pub fn stub_40c58() -> ! {
    todo!("0x40c58 ___28-[UserInfo UpdatePlayerInfo]_block_invoke")
}

// 0x41104 — ___copy_helper_block__6
#[doc(alias = "___copy_helper_block__6")]
pub fn stub_41104() -> ! {
    todo!("0x41104 ___copy_helper_block__6")
}

// 0x41128 — ___destroy_helper_block__6
#[doc(alias = "___destroy_helper_block__6")]
pub fn stub_41128() -> ! {
    todo!("0x41128 ___destroy_helper_block__6")
}

// 0x41144 — +[UserInfo CurrentPlayer]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UserInfo CurrentPlayer]")]
pub fn stub_41144() -> ! {
    todo!("0x41144 +[UserInfo CurrentPlayer]")
}

// 0x4118c — -[UserInfo Robux]
// type: id __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo Robux]")]
pub fn stub_4118c() -> ! {
    todo!("0x4118c -[UserInfo Robux]")
}

// 0x411a0 — __Z23convertToFriendlyStringP8NSNumber
// demangled: convertToFriendlyString(NSNumber *)
// type: _DWORD __fastcall(id)
#[doc(alias = "convertToFriendlyString(NSNumber *)")]
pub fn stub_411a0() -> ! {
    todo!("0x411a0 convertToFriendlyString(NSNumber *)")
}

// 0x41288 — -[UserInfo Tix]
// type: id __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo Tix]")]
pub fn stub_41288() -> ! {
    todo!("0x41288 -[UserInfo Tix]")
}

// 0x4129c — +[UserInfo clearAllRobloxCookie]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo clearAllRobloxCookie]")]
pub fn stub_4129c() -> ! {
    todo!("0x4129c +[UserInfo clearAllRobloxCookie]")
}

// 0x41580 — +[UserInfo printCookies]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo printCookies]")]
pub fn stub_41580() -> ! {
    todo!("0x41580 +[UserInfo printCookies]")
}

// 0x419c8 — +[UserInfo logout]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo logout]")]
pub fn stub_419c8() -> ! {
    todo!("0x419c8 +[UserInfo logout]")
}

// 0x419f4 — -[UserInfo userInfoDict]
// type: NSDictionary *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userInfoDict]")]
pub fn stub_419f4() -> ! {
    todo!("0x419f4 -[UserInfo userInfoDict]")
}

// 0x41a04 — -[UserInfo setUserInfoDict:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUserInfoDict:]")]
pub fn stub_41a04() -> ! {
    todo!("0x41a04 -[UserInfo setUserInfoDict:]")
}

// 0x41a28 — -[UserInfo userinfo]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userinfo]")]
pub fn stub_41a28() -> ! {
    todo!("0x41a28 -[UserInfo userinfo]")
}

// 0x41a38 — -[UserInfo setUserinfo:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUserinfo:]")]
pub fn stub_41a38() -> ! {
    todo!("0x41a38 -[UserInfo setUserinfo:]")
}

// 0x41a5c — -[UserInfo rbxBal]
// type: NSNumber *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo rbxBal]")]
pub fn stub_41a5c() -> ! {
    todo!("0x41a5c -[UserInfo rbxBal]")
}

// 0x41a6c — -[UserInfo setRbxBal:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setRbxBal:]")]
pub fn stub_41a6c() -> ! {
    todo!("0x41a6c -[UserInfo setRbxBal:]")
}

// 0x41a90 — -[UserInfo tikBal]
// type: NSNumber *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo tikBal]")]
pub fn stub_41a90() -> ! {
    todo!("0x41a90 -[UserInfo tikBal]")
}

// 0x41aa0 — -[UserInfo setTikBal:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setTikBal:]")]
pub fn stub_41aa0() -> ! {
    todo!("0x41aa0 -[UserInfo setTikBal:]")
}

// 0x41ac4 — -[UserInfo userThumbNailUrl]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userThumbNailUrl]")]
pub fn stub_41ac4() -> ! {
    todo!("0x41ac4 -[UserInfo userThumbNailUrl]")
}

// 0x41ad4 — -[UserInfo setUserThumbNailUrl:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUserThumbNailUrl:]")]
pub fn stub_41ad4() -> ! {
    todo!("0x41ad4 -[UserInfo setUserThumbNailUrl:]")
}

// 0x41af8 — -[UserInfo bcMember]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo bcMember]")]
pub fn stub_41af8() -> ! {
    todo!("0x41af8 -[UserInfo bcMember]")
}

// 0x41b08 — -[UserInfo setBcMember:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setBcMember:]")]
pub fn stub_41b08() -> ! {
    todo!("0x41b08 -[UserInfo setBcMember:]")
}

// 0x41b2c — -[UserInfo encodedPassword]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo encodedPassword]")]
pub fn stub_41b2c() -> ! {
    todo!("0x41b2c -[UserInfo encodedPassword]")
}

// 0x41b3c — -[UserInfo setEncodedPassword:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setEncodedPassword:]")]
pub fn stub_41b3c() -> ! {
    todo!("0x41b3c -[UserInfo setEncodedPassword:]")
}

// 0x41b60 — -[UserInfo encodedUsername]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo encodedUsername]")]
pub fn stub_41b60() -> ! {
    todo!("0x41b60 -[UserInfo encodedUsername]")
}

// 0x41b70 — -[UserInfo setEncodedUsername:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setEncodedUsername:]")]
pub fn stub_41b70() -> ! {
    todo!("0x41b70 -[UserInfo setEncodedUsername:]")
}

// 0x41b94 — -[UserInfo username]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo username]")]
pub fn stub_41b94() -> ! {
    todo!("0x41b94 -[UserInfo username]")
}

// 0x41ba4 — -[UserInfo setUsername:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUsername:]")]
pub fn stub_41ba4() -> ! {
    todo!("0x41ba4 -[UserInfo setUsername:]")
}

// 0x41bc8 — -[UserInfo password]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo password]")]
pub fn stub_41bc8() -> ! {
    todo!("0x41bc8 -[UserInfo password]")
}

// 0x41bd8 — -[UserInfo setPassword:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setPassword:]")]
pub fn stub_41bd8() -> ! {
    todo!("0x41bd8 -[UserInfo setPassword:]")
}

// 0x41bfc — __GLOBAL__I_a_11
// demangled: global constructor keyed to_a_11
#[doc(alias = "global constructor keyed to_a_11")]
pub fn stub_41bfc() -> ! {
    todo!("0x41bfc global constructor keyed to_a_11")
}

// 0x41cc4 — +[RobloxGoogleAnalytics initialize]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[RobloxGoogleAnalytics initialize]")]
pub fn stub_41cc4() -> ! {
    todo!("0x41cc4 +[RobloxGoogleAnalytics initialize]")
}

// 0xecd6e8 — _TFCreateCrashSocket
#[doc(alias = "_TFCreateCrashSocket")]
pub fn stub_ecd6e8() -> ! {
    todo!("0xecd6e8 _TFCreateCrashSocket")
}
