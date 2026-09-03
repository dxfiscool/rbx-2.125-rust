//! generated_core_h — 150 core stubs EA-sorted, next after generated_core_g.
//! Source: ida/export.json filtered where demangled/mangled contains "boost" or "rbx::signals", excluding Reflection/Instance/Ogre/RakNet/Network, sorted by EA, next 150 uncovered (lowest EA first).
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.

#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
pub fn stub_cb94() {
    // IDA 0xcb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
pub fn stub_efd8() {
    // IDA 0xefd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "joinGameWithJoinScript(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
// 0x26990 — __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE
// was: joinGameWithJoinScript(std::string const&,boost::shared_ptr<RBX::Game>)
pub fn stub_26990() {
    // IDA 0x26990: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "joinLocalGame(int,std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
// 0x26dd4 — __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE
// was: joinLocalGame(int,std::string const&,boost::shared_ptr<RBX::Game>)
pub fn stub_26dd4() {
    // IDA 0x26dd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "loadLocalApp(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
// 0x27268 — __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE
// was: loadLocalApp(std::string const&,boost::shared_ptr<RBX::Game>)
pub fn stub_27268() {
    // IDA 0x27268: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "joinGamePlaceId(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
// 0x278a8 — __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest
// was: joinGamePlaceId(int,boost::shared_ptr<RBX::Game>,JoinGameRequest)
pub fn stub_278a8() {
    // IDA 0x278a8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "joinGamePlaceIdSolo(int,rbx_core::SharedPtr<RBX::Game>)")]
// 0x28d98 — __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE
// was: joinGamePlaceIdSolo(int,boost::shared_ptr<RBX::Game>)
pub fn stub_28d98() {
    // IDA 0x28d98: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "joinGameTeleport(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>)")]
// 0x2a350 — __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE
// was: joinGameTeleport(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>)
pub fn stub_2a350() {
    // IDA 0x2a350: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
// 0x2c8c0 — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_2c8c0() {
    // IDA 0x2c8c0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
// 0x2c9a8 — __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_
// was: boost::shared_ptr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)
pub fn stub_2c9a8() {
    // IDA 0x2c9a8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string const&,rbx_core::SharedPtr<RBX::Game>,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),char const*,rbx_core::SharedPtr<RBX::Game>)")]
// 0x2ca7c — __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// was: boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string const&,boost::shared_ptr<RBX::Game>,char const*,boost::shared_ptr<RBX::Game>>(void (*)(std::string const&,boost::shared_ptr<RBX::Game>),char const*,boost::shared_ptr<RBX::Game>)
pub fn stub_2ca7c() {
    // IDA 0x2ca7c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_3<int,char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,std::string const&,rbx_core::SharedPtr<RBX::Game>,int,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),int,char const*,rbx_core::SharedPtr<RBX::Game>)")]
// 0x2cb64 — __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// was: boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_3<int,char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,std::string const&,boost::shared_ptr<RBX::Game>,int,char const*,boost::shared_ptr<RBX::Game>>(void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),int,char const*,boost::shared_ptr<RBX::Game>)
pub fn stub_2cb64() {
    // IDA 0x2cb64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>(void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
// 0x2cc54 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
// was: boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,boost::shared_ptr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,JoinGameRequest,int,boost::shared_ptr<RBX::Game>,JoinGameRequest>(void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),int,boost::shared_ptr<RBX::Game>,JoinGameRequest)
pub fn stub_2cc54() {
    // IDA 0x2cc54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<int,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,int,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,rbx_core::SharedPtr<RBX::Game>),int,rbx_core::SharedPtr<RBX::Game>)")]
// 0x2cd44 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// was: boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<int,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,int,boost::shared_ptr<RBX::Game>>(void (*)(int,boost::shared_ptr<RBX::Game>),int,boost::shared_ptr<RBX::Game>)
pub fn stub_2cd44() {
    // IDA 0x2cd44: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x2e2a0 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn stub_2e2a0() {
    // IDA 0x2e2a0: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>> *)const")]
// 0x2e518 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)const
pub fn stub_2e518() {
    // IDA 0x2e518: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>> *)")]
// 0x2e5ec — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// was: boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)
pub fn stub_2e5ec() {
    // IDA 0x2e5ec: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
// 0x2e6e0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()
pub fn stub_2e6e0() {
    // IDA 0x2e6e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
// 0x2e6e4 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()
pub fn stub_2e6e4() {
    // IDA 0x2e6e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::dispose(void)")]
// 0x2e6e8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::dispose(void)
pub fn stub_2e6e8() {
    // IDA 0x2e6e8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::get_deleter(std::type_info const&)")]
// 0x2e6f8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_deleter(std::type_info const&)
pub fn stub_2e6f8() {
    // IDA 0x2e6f8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::get_untyped_deleter(void)")]
// 0x2e6fc — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_untyped_deleter(void)
pub fn stub_2e6fc() {
    // IDA 0x2e6fc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
// 0x2e700 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn stub_2e700() {
    // IDA 0x2e700: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
// 0x2e970 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn stub_2e970() {
    // IDA 0x2e970: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")]
// 0x2ebbc — __ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
pub fn stub_2ebbc() {
    // IDA 0x2ebbc: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
// 0x2edec — __ZN5boost3_bi8storage3INS0_5valueISsEES3_S3_EC2ES3_S3_S3_
pub fn stub_2edec() {
    // IDA 0x2edec: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
// 0x2efb4 — __ZN5boost3_bi8storage2INS0_5valueISsEES3_EC2ES3_S3_
pub fn stub_2efb4() {
    // IDA 0x2efb4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
// 0x2f1d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
pub fn stub_2f1d8() {
    // IDA 0x2f1d8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x2f2d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_2f2d0() {
    // IDA 0x2f2d0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2f2ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_2f2ec() {
    // IDA 0x2f2ec: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
// 0x2f300 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
pub fn stub_2f300() {
    // IDA 0x2f300: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2f3e8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_2f3e8() {
    // IDA 0x2f3e8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x2f4fc — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn stub_2f4fc() {
    // IDA 0x2f4fc: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x2f5d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_2f5d4() {
    // IDA 0x2f5d4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
// 0x2f708 — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S8_
// was: boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn stub_2f708() {
    // IDA 0x2f708: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)")]
// 0x2f8bc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)
pub fn stub_2f8bc() {
    // IDA 0x2f8bc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x2f9bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_2f9bc() {
    // IDA 0x2f9bc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2f9d8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_2f9d8() {
    // IDA 0x2f9d8: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const")]
// 0x2f9ec — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const
pub fn stub_2f9ec() {
    // IDA 0x2f9ec: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2fad8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_2fad8() {
    // IDA 0x2fad8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)")]
// 0x2fbf4 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEclIPFviS7_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)
pub fn stub_2fbf4() {
    // IDA 0x2fbf4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x2fcd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_2fcd4() {
    // IDA 0x2fcd4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
// 0x2fe0c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
// was: boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)
pub fn stub_2fe0c() {
    // IDA 0x2fe0c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
// 0x2fec4 — __ZN5boost3_bi8storage3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
// was: boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)
pub fn stub_2fec4() {
    // IDA 0x2fec4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
// 0x30080 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
pub fn stub_30080() {
    // IDA 0x30080: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3017c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_3017c() {
    // IDA 0x3017c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x30198 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_30198() {
    // IDA 0x30198: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
// 0x301ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
pub fn stub_301ac() {
    // IDA 0x301ac: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x30298 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_30298() {
    // IDA 0x30298: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x303b8 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviRKSsSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn stub_303b8() {
    // IDA 0x303b8: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x30534 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_30534() {
    // IDA 0x30534: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
// 0x3066c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S6_SB_
// was: boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn stub_3066c() {
    // IDA 0x3066c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")]
// 0x3073c — __ZN5boost6threadC2INS_9function0IvEEEEOT_
pub fn stub_3073c() {
    // IDA 0x3073c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")]
// 0x30878 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2EOS3_
pub fn stub_30878() {
    // IDA 0x30878: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
// 0x30a24 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
pub fn stub_30a24() {
    // IDA 0x30a24: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x30b1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_30b1c() {
    // IDA 0x30b1c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x30b38 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_30b38() {
    // IDA 0x30b38: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
// 0x30b40 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
pub fn stub_30b40() {
    // IDA 0x30b40: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x30c28 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_30c28() {
    // IDA 0x30c28: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x30d3c — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn stub_30d3c() {
    // IDA 0x30d3c: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x30eac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_30eac() {
    // IDA 0x30eac: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
// 0x30fe0 — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// was: boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn stub_30fe0() {
    // IDA 0x30fe0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
// 0x310a8 — __ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_
pub fn stub_310a8() {
    // IDA 0x310a8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
// 0x3119c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev
pub fn stub_3119c() {
    // IDA 0x3119c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
// 0x311a0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev
pub fn stub_311a0() {
    // IDA 0x311a0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)")]
// 0x311a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE7disposeEv
pub fn stub_311a4() {
    // IDA 0x311a4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)")]
// 0x311b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE11get_deleterERKSt9type_info
pub fn stub_311b4() {
    // IDA 0x311b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)")]
// 0x311b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE19get_untyped_deleterEv
pub fn stub_311b8() {
    // IDA 0x311b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)")]
// 0x311bc — __ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_
pub fn stub_311bc() {
    // IDA 0x311bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
// 0x312b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED1Ev
pub fn stub_312b0() {
    // IDA 0x312b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
// 0x312b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED0Ev
pub fn stub_312b4() {
    // IDA 0x312b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)")]
// 0x312b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE7disposeEv
pub fn stub_312b8() {
    // IDA 0x312b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)")]
// 0x312c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE11get_deleterERKSt9type_info
pub fn stub_312c8() {
    // IDA 0x312c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)")]
// 0x312cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE19get_untyped_deleterEv
pub fn stub_312cc() {
    // IDA 0x312cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string)>::slot*)")]
// 0x31e24 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSEPS6_
pub fn stub_31e24() {
    // IDA 0x31e24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::callable<rbx::signals::signal<void ()(std::string)>*>(boost::function<void ()(std::string)> const&,rbx::signals::signal<void ()(std::string)>*)")]
// 0x31fc0 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
pub fn stub_31fc0() {
    // IDA 0x31fc0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()")]
// 0x320bc — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_320bc() {
    // IDA 0x320bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
// 0x32194 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
pub fn stub_32194() {
    // IDA 0x32194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
// 0x3219c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
pub fn stub_3219c() {
    // IDA 0x3219c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
// 0x2afa28 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
pub fn stub_2afa28() {
    // IDA 0x2afa28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
// 0x2afc80 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
pub fn stub_2afc80() {
    // IDA 0x2afc80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::disconnect(void)")]
// 0x2afe78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv
pub fn stub_2afe78() {
    // IDA 0x2afe78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::connected(void)const")]
// 0x2aff88 — __ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv
pub fn stub_2aff88() {
    // IDA 0x2aff88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
// 0x2affbc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
pub fn stub_2affbc() {
    // IDA 0x2affbc: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_init_mutex(void)")]
// 0x2b00ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv
pub fn stub_2b00ac() {
    // IDA 0x2b00ac: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
// 0x2b00b0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
pub fn stub_2b00b0() {
    // IDA 0x2b00b0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
// 0x2b01a0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev
pub fn stub_2b01a0() {
    // IDA 0x2b01a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
// 0x2b01cc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev
pub fn stub_2b01cc() {
    // IDA 0x2b01cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Heartbeat const&)>::operator()(RBX::Heartbeat const&)")]
// 0x361c20 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_
pub fn stub_361c20() {
    // IDA 0x361c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(double)>::operator()(double)")]
// 0x361d64 — __ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd
pub fn stub_361d64() {
    // IDA 0x361d64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Stepped const&)>::operator()(RBX::Stepped const&)")]
// 0x361eb0 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_
pub fn stub_361eb0() {
    // IDA 0x361eb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(double,double)>::operator()(double,double)")]
// 0x361ff4 — __ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd
pub fn stub_361ff4() {
    // IDA 0x361ff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::RunTransition)>::operator()(RBX::RunTransition)")]
// 0x362158 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_
pub fn stub_362158() {
    // IDA 0x362158: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::on_error(std::exception &)")]
// 0x362924 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception
pub fn stub_362924() {
    // IDA 0x362924: libstdc++ template instantiation (mangled-only context). Std container/algorithm — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::safe_static_init_mutex(void)")]
// 0x3631a8 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE22safe_static_init_mutexEv
pub fn stub_3631a8() {
    // IDA 0x3631a8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")]
// 0x363384 — __ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception
pub fn stub_363384() {
    // IDA 0x363384: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_init_mutex(void)")]
// 0x3633d0 — __ZN3rbx7signals6signalIFvddEE22safe_static_init_mutexEv
pub fn stub_3633d0() {
    // IDA 0x3633d0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")]
// 0x3633d4 — __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv
pub fn stub_3633d4() {
    // IDA 0x3633d4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")]
// 0x36362c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception
pub fn stub_36362c() {
    // IDA 0x36362c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_init_mutex(void)")]
// 0x363678 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE22safe_static_init_mutexEv
pub fn stub_363678() {
    // IDA 0x363678: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")]
// 0x36367c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv
pub fn stub_36367c() {
    // IDA 0x36367c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::on_error(std::exception &)")]
// 0x3638d4 — __ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception
pub fn stub_3638d4() {
    // IDA 0x3638d4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_init_mutex(void)")]
// 0x363920 — __ZN3rbx7signals6signalIFvdEE22safe_static_init_mutexEv
pub fn stub_363920() {
    // IDA 0x363920: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")]
// 0x363924 — __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv
pub fn stub_363924() {
    // IDA 0x363924: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")]
// 0x363b7c — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception
pub fn stub_363b7c() {
    // IDA 0x363b7c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_init_mutex(void)")]
// 0x363ba8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE22safe_static_init_mutexEv
pub fn stub_363ba8() {
    // IDA 0x363ba8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::disconnectAll(void)")]
// 0x364cdc — __ZN3rbx7signals6signalIFvdEE13disconnectAllEv
pub fn stub_364cdc() {
    // IDA 0x364cdc: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")]
// 0x366184 — __ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE
pub fn stub_366184() {
    // IDA 0x366184: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::disconnect(void)")]
// 0x3666f0 — __ZN3rbx7signals6signalIFvdEE4slot10disconnectEv
pub fn stub_3666f0() {
    // IDA 0x3666f0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::connected(void)const")]
// 0x366800 — __ZNK3rbx7signals6signalIFvdEE4slot9connectedEv
pub fn stub_366800() {
    // IDA 0x366800: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")]
// 0x3668e8 — __ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE
pub fn stub_3668e8() {
    // IDA 0x3668e8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_init_mutex(void)")]
// 0x3669d8 — __ZN3rbx7signals6signalIFvdEE4slot22safe_static_init_mutexEv
pub fn stub_3669d8() {
    // IDA 0x3669d8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")]
// 0x3669dc — __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv
pub fn stub_3669dc() {
    // IDA 0x3669dc: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::~slot()")]
// 0x366d10 — __ZN3rbx7signals6signalIFvdEE4slotD1Ev
pub fn stub_366d10() {
    // IDA 0x366d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::~slot()")]
// 0x366d3c — __ZN3rbx7signals6signalIFvdEE4slotD0Ev
pub fn stub_366d3c() {
    // IDA 0x366d3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::disconnectAll(void)")]
// 0x3673ac — __ZN3rbx7signals6signalIFvddEE13disconnectAllEv
pub fn stub_3673ac() {
    // IDA 0x3673ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::insert(rbx::signals::signal<void ()(double,double)>::slot *)")]
// 0x367fe8 — __ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE
pub fn stub_367fe8() {
    // IDA 0x367fe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::disconnect(void)")]
// 0x368554 — __ZN3rbx7signals6signalIFvddEE4slot10disconnectEv
pub fn stub_368554() {
    // IDA 0x368554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::connected(void)const")]
// 0x368664 — __ZNK3rbx7signals6signalIFvddEE4slot9connectedEv
pub fn stub_368664() {
    // IDA 0x368664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::remove(rbx::signals::signal<void ()(double,double)>::slot *)")]
// 0x368778 — __ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE
pub fn stub_368778() {
    // IDA 0x368778: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_init_mutex(void)")]
// 0x368868 — __ZN3rbx7signals6signalIFvddEE4slot22safe_static_init_mutexEv
pub fn stub_368868() {
    // IDA 0x368868: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_do_get_mutex(void)")]
// 0x36886c — __ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv
pub fn stub_36886c() {
    // IDA 0x36886c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::~slot()")]
// 0x368b9c — __ZN3rbx7signals6signalIFvddEE4slotD1Ev
pub fn stub_368b9c() {
    // IDA 0x368b9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::~slot()")]
// 0x368bc8 — __ZN3rbx7signals6signalIFvddEE4slotD0Ev
pub fn stub_368bc8() {
    // IDA 0x368bc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::disconnectAll(void)")]
// 0x369c54 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv
pub fn stub_369c54() {
    // IDA 0x369c54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::disconnectAll(void)")]
// 0x369dcc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv
pub fn stub_369dcc() {
    // IDA 0x369dcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")]
// 0x369f44 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv
pub fn stub_369f44() {
    // IDA 0x369f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
// 0x382204 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
pub fn stub_382204() {
    // IDA 0x382204: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")]
// 0x3824a8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
pub fn stub_3824a8() {
    // IDA 0x3824a8: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")]
// 0x38290c — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
pub fn stub_38290c() {
    // IDA 0x38290c: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// 0x385024 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
pub fn stub_385024() {
    // IDA 0x385024: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::disconnect(void)")]
// 0x385354 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot10disconnectEv
pub fn stub_385354() {
    // IDA 0x385354: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::connected(void)const")]
// 0x385464 — __ZNK3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot9connectedEv
pub fn stub_385464() {
    // IDA 0x385464: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// 0x385498 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
pub fn stub_385498() {
    // IDA 0x385498: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_init_mutex(void)")]
// 0x385588 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot22safe_static_init_mutexEv
pub fn stub_385588() {
    // IDA 0x385588: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")]
// 0x38558c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
pub fn stub_38558c() {
    // IDA 0x38558c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
// 0x38567c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD1Ev
pub fn stub_38567c() {
    // IDA 0x38567c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
// 0x3856a8 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD0Ev
pub fn stub_3856a8() {
    // IDA 0x3856a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
// 0x39cb28 — __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
pub fn stub_39cb28() {
    // IDA 0x39cb28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
// 0x39cde8 — __ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
pub fn stub_39cde8() {
    // IDA 0x39cde8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")]
// 0x39ce34 — __ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv
pub fn stub_39ce34() {
    // IDA 0x39ce34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
// 0x39ce38 — __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
pub fn stub_39ce38() {
    // IDA 0x39ce38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
// 0x39d260 — __ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
pub fn stub_39d260() {
    // IDA 0x39d260: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
// 0x39d53c — __ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
pub fn stub_39d53c() {
    // IDA 0x39d53c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")]
// 0x39d588 — __ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv
pub fn stub_39d588() {
    // IDA 0x39d588: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
// 0x39d58c — __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
pub fn stub_39d58c() {
    // IDA 0x39d58c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// 0x39d774 — __ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
pub fn stub_39d774() {
    // IDA 0x39d774: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")]
// 0x39daa4 — __ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv
pub fn stub_39daa4() {
    // IDA 0x39daa4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")]
// 0x39dbb4 — __ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv
pub fn stub_39dbb4() {
    // IDA 0x39dbb4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// 0x39dc54 — __ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE
pub fn stub_39dc54() {
    // IDA 0x39dc54: threading primitive. std::thread/parking_lot — carrier no-op.
}
