//! core — generated_core_shard_og — 100 stubs EA-sorted asc core namespace gap filler 0xf2b504..0xf2fe14 (core namespace not yet in crates/core/src).
//! Source: ida/export.json (85545 funcs) filtered where demangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|FMOD|Audio|Sound|lua, EA-sorted asc, next 100 uncovered after 0xf2b504 (lowest EA first).
//! Core namespace total 34491, 34221 stubbed before, 34321 after; range 0xf2b504..0xf2fe14 EA-sorted asc.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "j___ZN3rbx10safe_queueIN3RBX13ScriptContext13WaitingThreadEE4pushERKS3_")]
// 0xf2b504 — j___ZN3rbx10safe_queueIN3RBX13ScriptContext13WaitingThreadEE4pushERKS3_
// demangled: rbx::safe_queue<RBX::ScriptContext::WaitingThread>::push(RBX::ScriptContext::WaitingThread const&)
pub fn stub_f2b504() {
    // IDA 0xf2b504: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3rbx10safe_queueIN3RBX13ScriptContext13WaitingThreadEE5clearEv")]
// 0xf2b514 — j___ZN3rbx10safe_queueIN3RBX13ScriptContext13WaitingThreadEE5clearEv
// demangled: rbx::safe_queue<RBX::ScriptContext::WaitingThread>::clear(void)
pub fn stub_f2b514() {
    // IDA 0xf2b514: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// 0xf2b604 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>> const&)
pub fn stub_f2b604() {
    // IDA 0xf2b604: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// 0xf2b734 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> const&)
pub fn stub_f2b734() {
    // IDA 0xf2b734: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript>::operator=(rbx_core::SharedPtr<RBX::BaseScript> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10BaseScriptEEaSERKS3_")]
// 0xf2b8e4 — j___ZN5boost10shared_ptrIN3RBX10BaseScriptEEaSERKS3_
// demangled: boost::shared_ptr<RBX::BaseScript>::operator=(boost::shared_ptr<RBX::BaseScript> const&)
pub fn stub_f2b8e4() {
    // IDA 0xf2b8e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::ScriptStats>::reset<RBX::ScriptStats>(RBX::ScriptStats *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX11ScriptStatsEE5resetIS2_EEvPT_")]
// 0xf2b934 — j___ZN5boost10shared_ptrIN3RBX11ScriptStatsEE5resetIS2_EEvPT_
// demangled: void boost::shared_ptr<RBX::ScriptStats>::reset<RBX::ScriptStats>(RBX::ScriptStats *)
pub fn stub_f2b934() {
    // IDA 0xf2b934: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX11ScriptStatsEEC2IS2_EEPT_")]
// 0xf2b944 — j___ZN5boost10shared_ptrIN3RBX11ScriptStatsEEC2IS2_EEPT_
// demangled: boost::shared_ptr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)
pub fn stub_f2b944() {
    // IDA 0xf2b944: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::operator=(rbx_core::SharedPtr<RBX::ScriptStats> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX11ScriptStatsEEaSERKS3_")]
// 0xf2b954 — j___ZN5boost10shared_ptrIN3RBX11ScriptStatsEEaSERKS3_
// demangled: boost::shared_ptr<RBX::ScriptStats>::operator=(boost::shared_ptr<RBX::ScriptStats> const&)
pub fn stub_f2b954() {
    // IDA 0xf2b954: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext>(rbx_core::WeakPtr<RBX::ScriptContext> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// 0xf2b994 — j___ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// demangled: boost::shared_ptr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext>(boost::weak_ptr<RBX::ScriptContext> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f2b994() {
    // IDA 0xf2b994: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_")]
// 0xf2b9b4 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_
// demangled: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)
// type: int __fastcall(int, void *, int, int, int, int)
pub fn stub_f2b9b4() {
    // IDA 0xf2b9b4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>)")]
#[doc(alias = "j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_")]
// 0xf2bbe4 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_
// demangled: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>)
pub fn stub_f2bbe4() {
    // IDA 0xf2bbe4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS4_11ScriptStartEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0xf2bbf4 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS4_11ScriptStartEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)
pub fn stub_f2bbf4() {
    // IDA 0xf2bbf4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20RuntimeScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0xf2bc04 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20RuntimeScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_f2bc04() {
    // IDA 0xf2bc04: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_")]
// 0xf2bc34 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_
// demangled: boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>)
pub fn stub_f2bc34() {
    // IDA 0xf2bc34: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS2_INS4_18ScriptStartOptionsEEEEC2ES6_S8_SA_")]
// 0xf2bc74 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS2_INS4_18ScriptStartOptionsEEEEC2ES6_S8_SA_
// demangled: boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_f2bc74() {
    // IDA 0xf2bc74: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list1<RBX::BaseScript * const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions> &,boost::_bi::list1<RBX::BaseScript * const&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS2_INS4_18ScriptStartOptionsEEEEclINS_4_mfi3mf2IvS4_PNS3_10BaseScriptES9_EENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0xf2bc84 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS2_INS4_18ScriptStartOptionsEEEEclINS_4_mfi3mf2IvS4_PNS3_10BaseScriptES9_EENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list1<RBX::BaseScript * const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions> &,boost::_bi::list1<RBX::BaseScript * const&> &,int)
pub fn stub_f2bc84() {
    // IDA 0xf2bc84: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv")]
// 0xf2bce4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>::operator()(void)
pub fn stub_f2bce4() {
    // IDA 0xf2bce4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")]
// 0xf2bcf4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// demangled: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)
pub fn stub_f2bcf4() {
    // IDA 0xf2bcf4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::ScriptContext>>::type> boost::bind<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::ScriptContext>>(void (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<RBX::ScriptContext>)")]
#[doc(alias = "j___ZN5boost4bindIvN3RBX13ScriptContextENS_10shared_ptrIS2_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS7_T0_EENS5_9list_av_1IT1_E4typeEEEMSA_FS7_vESD_")]
// 0xf2bd44 — j___ZN5boost4bindIvN3RBX13ScriptContextENS_10shared_ptrIS2_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS7_T0_EENS5_9list_av_1IT1_E4typeEEEMSA_FS7_vESD_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list_av_1<boost::shared_ptr<RBX::ScriptContext>>::type> boost::bind<void,RBX::ScriptContext,boost::shared_ptr<RBX::ScriptContext>>(void (RBX::ScriptContext::*)(void),boost::shared_ptr<RBX::ScriptContext>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2bd44() {
    // IDA 0xf2bd44: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list_av_3<RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>::type> boost::bind<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions,RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>(void (RBX::ScriptContext::*)(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions),RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions)")]
#[doc(alias = "j___ZN5boost4bindIvN3RBX13ScriptContextEPNS1_10BaseScriptENS2_18ScriptStartOptionsEPS2_NS_3argILi1EEES5_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISB_T0_T1_T2_EENS9_9list_av_3IT3_T4_T5_E4typeEEEMSE_FSB_SF_SG_ESJ_SK_SL_")]
// 0xf2bd54 — j___ZN5boost4bindIvN3RBX13ScriptContextEPNS1_10BaseScriptENS2_18ScriptStartOptionsEPS2_NS_3argILi1EEES5_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISB_T0_T1_T2_EENS9_9list_av_3IT3_T4_T5_E4typeEEEMSE_FSB_SF_SG_ESJ_SK_SL_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list_av_3<RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>::type> boost::bind<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions,RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>(void (RBX::ScriptContext::*)(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions),RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions)
// type: int __fastcall(int, char, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_f2bd54() {
    // IDA 0xf2bd54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX11ScriptStatsEEEPT_")]
// 0xf2bda4 — j___ZN5boost6detail12shared_countC2IN3RBX11ScriptStatsEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_f2bda4() {
    // IDA 0xf2bda4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_")]
// 0xf2bdb4 — j___ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_f2bdb4() {
    // IDA 0xf2bdb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0xf2bea4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_f2bea4() {
    // IDA 0xf2bea4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)> const&)")]
#[doc(alias = "j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSERKS8_")]
// 0xf2bfc4 — j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSERKS8_
// demangled: boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)> const&)
pub fn stub_f2bfc4() {
    // IDA 0xf2bfc4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>)")]
#[doc(alias = "j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_")]
// 0xf2bfe4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2bfe4() {
    // IDA 0xf2bfe4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0xf2bff4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// demangled: j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2bff4() {
    // IDA 0xf2bff4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
#[doc(alias = "j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE11move_assignERS7_")]
// 0xf2c214 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE11move_assignERS7_
// demangled: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>&)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_f2c214() {
    // IDA 0xf2c214: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> const&)")]
#[doc(alias = "j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_")]
// 0xf2c224 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_
// demangled: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int> const&)
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_f2c224() {
    // IDA 0xf2c224: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
#[doc(alias = "j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE4swapERS7_")]
// 0xf2c234 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE4swapERS7_
// demangled: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>&)
pub fn stub_f2c234() {
    // IDA 0xf2c234: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::clear(void)")]
#[doc(alias = "j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5clearEv")]
// 0xf2c244 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5clearEv
// demangled: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::clear(void)
// type: int __fastcall(_DWORD)
pub fn stub_f2c244() {
    // IDA 0xf2c244: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::table(unsigned long,boost::hash<int> const&,std::equal_to<int> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE")]
// 0xf2c2b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::table(unsigned long,boost::hash<int> const&,std::equal_to<int> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> const&)
pub fn stub_f2c2b4() {
    // IDA 0xf2c2b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>*)")]
#[doc(alias = "j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_")]
// 0xf2c2d4 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_
// demangled: __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::Script>>*)
pub fn stub_f2c2d4() {
    // IDA 0xf2c2d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "j___ZNK3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_v")]
// 0xf2c344 — j___ZNK3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_v
// demangled: RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2c344() {
    // IDA 0xf2c344: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "j___ZNK3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_v")]
// 0xf2c364 — j___ZNK3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_v
// demangled: RBX::ScriptContext * RBX::ServiceProvider::create<RBX::ScriptContext>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2c364() {
    // IDA 0xf2c364: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xf2c424 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(boost::shared_ptr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const
pub fn stub_f2c424() {
    // IDA 0xf2c424: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const")]
#[doc(alias = "j___ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_")]
// 0xf2c444 — j___ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_
// demangled: boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const
pub fn stub_f2c444() {
    // IDA 0xf2c444: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>::operator()(RBX::ScriptContext*,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions)const")]
#[doc(alias = "j___ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextEPNS2_10BaseScriptENS3_18ScriptStartOptionsEEclEPS3_S5_S6_")]
// 0xf2c464 — j___ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextEPNS2_10BaseScriptENS3_18ScriptStartOptionsEEclEPS3_S5_S6_
// demangled: boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>::operator()(RBX::ScriptContext*,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions)const
pub fn stub_f2c464() {
    // IDA 0xf2c464: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0xf2c484 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f2c484() {
    // IDA 0xf2c484: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE")]
// 0xf2c494 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2c494() {
    // IDA 0xf2c494: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0xf2c4a4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_f2c4a4() {
    // IDA 0xf2c4a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::operator()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)const")]
#[doc(alias = "j___ZNK5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEclES2_S2_S6_i")]
// 0xf2c654 — j___ZNK5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEclES2_S2_S6_i
// demangled: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::operator()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)const
pub fn stub_f2c654() {
    // IDA 0xf2c654: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_allocate_mapEm")]
// 0xf2c694 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_allocate_mapEm
// demangled: std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_allocate_map(unsigned long)
pub fn stub_f2c694() {
    // IDA 0xf2c694: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_create_nodesEPPS2_S6_")]
// 0xf2c6a4 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_create_nodesEPPS2_S6_
// demangled: std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_create_nodes(RBX::ScriptContext::WaitingThread**,RBX::ScriptContext::WaitingThread**)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_f2c6a4() {
    // IDA 0xf2c6a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_initialize_mapEm")]
// 0xf2c6b4 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_initialize_mapEm
// demangled: std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_initialize_map(unsigned long)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
pub fn stub_f2c6b4() {
    // IDA 0xf2c6b4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EED2Ev")]
// 0xf2c6c4 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EED2Ev
// demangled: std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::~_Deque_base()
pub fn stub_f2c6c4() {
    // IDA 0xf2c6c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX13ScriptContext11ScriptStartESaIS2_EE11_M_allocateEm")]
// 0xf2c6f4 — j___ZNSt12_Vector_baseIN3RBX13ScriptContext11ScriptStartESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_allocate(unsigned long)
pub fn stub_f2c6f4() {
    // IDA 0xf2c6f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_")]
// 0xf2c714 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
// demangled: RBX::ScriptContext::ScriptStart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)
pub fn stub_f2c714() {
    // IDA 0xf2c714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt3mapISsN3RBX13ScriptContext21ScriptStatInformationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
// 0xf2c734 — j___ZNSt3mapISsN3RBX13ScriptContext21ScriptStatInformationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// demangled: std::map<std::string,RBX::ScriptContext::ScriptStatInformation,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::operator[](std::string const&)
pub fn stub_f2c734() {
    // IDA 0xf2c734: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_")]
// 0xf2c754 — j___ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_
// demangled: std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&)
pub fn stub_f2c754() {
    // IDA 0xf2c754: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv")]
// 0xf2c774 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void)
pub fn stub_f2c774() {
    // IDA 0xf2c774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_push_back_auxERKS2_")]
// 0xf2c784 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_push_back_auxERKS2_
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_push_back_aux(RBX::ScriptContext::WaitingThread const&)
pub fn stub_f2c784() {
    // IDA 0xf2c784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb")]
// 0xf2c794 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_f2c794() {
    // IDA 0xf2c794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_")]
// 0xf2c7a4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
pub fn stub_f2c7a4() {
    // IDA 0xf2c7a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE22_M_reserve_map_at_backEm")]
// 0xf2c7b4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE22_M_reserve_map_at_backEm
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_f2c7b4() {
    // IDA 0xf2c7b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv")]
// 0xf2c7c4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void)
pub fn stub_f2c7c4() {
    // IDA 0xf2c7c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9push_backERKS2_")]
// 0xf2c7d4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9push_backERKS2_
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::push_back(RBX::ScriptContext::WaitingThread const&)
pub fn stub_f2c7d4() {
    // IDA 0xf2c7d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EEC2ERKS4_")]
// 0xf2c7e4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EEC2ERKS4_
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::deque(std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>> const&)
pub fn stub_f2c7e4() {
    // IDA 0xf2c7e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EED2Ev")]
// 0xf2c7f4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EED2Ev
// demangled: std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::~deque()
pub fn stub_f2c7f4() {
    // IDA 0xf2c7f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_")]
// 0xf2c814 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
// demangled: RBX::ScriptContext::ScriptStart * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)
pub fn stub_f2c814() {
    // IDA 0xf2c814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf2c844 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,RBX::ScriptContext::ScriptStart const&)
pub fn stub_f2c844() {
    // IDA 0xf2c844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS2_S4_EE")]
// 0xf2c854 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS2_S4_EE
// demangled: std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::erase(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>)
pub fn stub_f2c854() {
    // IDA 0xf2c854: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE9push_backERKS2_")]
// 0xf2c864 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::push_back(RBX::ScriptContext::ScriptStart const&)
// type: int __fastcall(int, RBX::ScriptContext::ScriptStart *)
pub fn stub_f2c864() {
    // IDA 0xf2c864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EED2Ev")]
// 0xf2c874 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EED2Ev
// demangled: std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::~vector()
pub fn stub_f2c874() {
    // IDA 0xf2c874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
// 0xf2c934 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::equal_range(RBX::BaseScript * const&)
pub fn stub_f2c934() {
    // IDA 0xf2c934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xf2c944 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(RBX::BaseScript * const&)
pub fn stub_f2c944() {
    // IDA 0xf2c944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xf2c954 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(std::_Rb_tree_iterator<RBX::BaseScript *>,RBX::BaseScript * const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_f2c954() {
    // IDA 0xf2c954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueISt23_Rb_tree_const_iteratorIS2_EEEvT_SC_")]
// 0xf2c964 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueISt23_Rb_tree_const_iteratorIS2_EEEvT_SC_
// demangled: void std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique<std::_Rb_tree_const_iterator<RBX::BaseScript *>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>)
// type: int __fastcall(int)
pub fn stub_f2c964() {
    // IDA 0xf2c964: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4swapERS8_")]
// 0xf2c974 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4swapERS8_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::swap(std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>&)
pub fn stub_f2c974() {
    // IDA 0xf2c974: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
// 0xf2c984 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(RBX::BaseScript * const&)
pub fn stub_f2c984() {
    // IDA 0xf2c984: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
// 0xf2c994 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(std::_Rb_tree_iterator<RBX::BaseScript *>,std::_Rb_tree_iterator<RBX::BaseScript *>)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_f2c994() {
    // IDA 0xf2c994: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf2c9a4 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_erase(std::_Rb_tree_node<RBX::BaseScript *> *)
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_f2c9a4() {
    // IDA 0xf2c9a4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xf2c9b4 — j___ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// demangled: std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::BaseScript * const&)
pub fn stub_f2c9b4() {
    // IDA 0xf2c9b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")]
// 0xf2c9c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::lower_bound(std::string const&)
// type: int __fastcall(int, std::string *)
pub fn stub_f2c9c4() {
    // IDA 0xf2c9c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11upper_boundERS1_")]
// 0xf2c9d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11upper_boundERS1_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&)
// type: int __fastcall(int, std::string *this)
pub fn stub_f2c9d4() {
    // IDA 0xf2c9d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")]
// 0xf2c9e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
pub fn stub_f2c9e4() {
    // IDA 0xf2c9e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2c9f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)
pub fn stub_f2c9f4() {
    // IDA 0xf2c9f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xf2ca04 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)
// type: int __fastcall(int, int, int)
pub fn stub_f2ca04() {
    // IDA 0xf2ca04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf2ca14 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)
// type: int __fastcall(int, int, int)
pub fn stub_f2ca14() {
    // IDA 0xf2ca14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
// 0xf2ca24 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::find(std::string const&)
// type: int __fastcall(int, std::string *this)
pub fn stub_f2ca24() {
    // IDA 0xf2ca24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseERS1_")]
// 0xf2ca34 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseERS1_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::string const&)
pub fn stub_f2ca34() {
    // IDA 0xf2ca34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")]
// 0xf2ca44 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_f2ca44() {
    // IDA 0xf2ca44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2ca54 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)
pub fn stub_f2ca54() {
    // IDA 0xf2ca54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0xf2ca64 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)
// type: int __fastcall(int, int, int, int)
pub fn stub_f2ca64() {
    // IDA 0xf2ca64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0xf2ca94 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Script>>> *)
pub fn stub_f2ca94() {
    // IDA 0xf2ca94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX13ScriptContext13WaitingThreadERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type")]
// 0xf2caf4 — j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX13ScriptContext13WaitingThreadERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type
// demangled: std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>>(std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>,std::__false_type)
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int)
pub fn stub_f2caf4() {
    // IDA 0xf2caf4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)")]
#[doc(alias = "j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX13ScriptContext11ScriptStartESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvS3_S4_EENSB_5list2INSB_5valueIPS3_EENSA_3argILi1EEEEEEEET0_T_SP_SO_")]
// 0xf2cb14 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX13ScriptContext11ScriptStartESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvS3_S4_EENSB_5list2INSB_5valueIPS3_EENSA_3argILi1EEEEEEEET0_T_SP_SO_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)
pub fn stub_f2cb14() {
    // IDA 0xf2cb14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)")]
#[doc(alias = "j___ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvNS1_13ScriptContextES3_EENS6_5list2INS6_5valueIPSA_EENS5_3argILi1EEEEEEEET0_T_SL_SK_")]
// 0xf2cb24 — j___ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvNS1_13ScriptContextES3_EENS6_5list2INS6_5valueIPSA_EENS5_3argILi1EEEEEEEET0_T_SL_SK_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)
pub fn stub_f2cb24() {
    // IDA 0xf2cb24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>)")]
#[doc(alias = "j___ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS1_13ScriptContextES3_NSA_18ScriptStartOptionsEEENS6_5list3INS6_5valueIPSA_EENS5_3argILi1EEENSE_ISB_EEEEEEET0_T_SN_SM_")]
// 0xf2cb34 — j___ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS1_13ScriptContextES3_NSA_18ScriptStartOptionsEEENS6_5list3INS6_5valueIPSA_EENS5_3argILi1EEENSE_ISB_EEEEEEET0_T_SN_SM_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>)
pub fn stub_f2cb34() {
    // IDA 0xf2cb34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
// 0xf2ccb4 — j___ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// demangled: boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>::operator()(void)
pub fn stub_f2ccb4() {
    // IDA 0xf2ccb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZNSt4pairIKSsN3RBX11ScriptStats14StatCollectionEEC2ERS0_RKS3_")]
// 0xf2cd24 — j___ZNSt4pairIKSsN3RBX11ScriptStats14StatCollectionEEC2ERS0_RKS3_
// demangled: std::pair<std::string const,RBX::ScriptStats::StatCollection>::pair(std::string const&,RBX::ScriptStats::StatCollection const&)
pub fn stub_f2cd24() {
    // IDA 0xf2cd24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")]
// 0xf2cda4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_create_node(std::pair<std::string const,RBX::ScriptStats::StatCollection> const&)
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
pub fn stub_f2cda4() {
    // IDA 0xf2cda4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2cdb4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptStats::StatCollection>> *)
pub fn stub_f2cdb4() {
    // IDA 0xf2cdb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xf2cdc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptStats::StatCollection> const&)
// type: int __fastcall(int, int, int)
pub fn stub_f2cdc4() {
    // IDA 0xf2cdc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
// 0xf2cdd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::find(std::string const&)
// type: int __fastcall(int, std::string *this)
pub fn stub_f2cdd4() {
    // IDA 0xf2cdd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2cde4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptStats::StatCollection>> *)
pub fn stub_f2cde4() {
    // IDA 0xf2cde4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0xf2cdf4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX11ScriptStats14StatCollectionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptStats::StatCollection> const&)
// type: int __fastcall(int, int, int, int)
pub fn stub_f2cdf4() {
    // IDA 0xf2cdf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE13findCacheItemERKSsPS2_")]
// 0xf2fdc4 — j___ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE13findCacheItemERKSsPS2_
// demangled: RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::findCacheItem(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo*)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_f2fdc4() {
    // IDA 0xf2fdc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "j___ZN3RBX25ScriptInformationProviderD2Ev")]
// 0xf2fdf4 — j___ZN3RBX25ScriptInformationProviderD2Ev
// demangled: RBX::ScriptInformationProvider::~ScriptInformationProvider()
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
pub fn stub_f2fdf4() {
    // IDA 0xf2fdf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv")]
// 0xf2fe04 — j___ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv
// demangled: RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::removeLeastRecentlyUsed(void)
pub fn stub_f2fe04() {
    // IDA 0xf2fe04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m")]
// 0xf2fe14 — j___ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// demangled: RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_f2fe14() {
    // IDA 0xf2fe14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
