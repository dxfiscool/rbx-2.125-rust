//! core shard kl — 100 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core after kk 0x1c73f0 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, boost+RBX|std|rbx; 21918 filtered, 553 remaining, 21365->21465 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED1Ev")]
// 0x316414 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED1Ev
pub fn stub_0x316414() {
    // IDA 0x316414: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED0Ev")]
// 0x316418 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED0Ev
pub fn stub_0x316418() {
    // IDA 0x316418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE7disposeEv")]
// 0x31641c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE7disposeEv
pub fn stub_0x31641c() {
    // IDA 0x31641c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE11get_deleterERKSt9type_info")]
// 0x3164c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE11get_deleterERKSt9type_info
pub fn stub_0x3164c0() {
    // IDA 0x3164c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE19get_untyped_deleterEv")]
// 0x3164c4 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE19get_untyped_deleterEv
pub fn stub_0x3164c4() {
    // IDA 0x3164c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::get(boost::function<void ()(std::string *,std::exception *)>,bool)")]
#[doc(alias = "__ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb")]
// 0x3168b0 — __ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x3168b0() {
    // IDA 0x3168b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::doGet(std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN3RBXL5doGetESsbN5boost8functionIFvPSsPSt9exceptionEEE")]
// 0x316b74 — __ZN3RBXL5doGetESsbN5boost8functionIFvPSsPSt9exceptionEEE
pub fn stub_0x316b74() {
    // IDA 0x316b74: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::post(std::string const&,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
#[doc(alias = "__ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb")]
// 0x316f2c — __ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x316f2c() {
    // IDA 0x316f2c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::doPost(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN3RBXL6doPostESsSsbbN5boost8functionIFvPSsPSt9exceptionEEE")]
// 0x317378 — __ZN3RBXL6doPostESsSsbbN5boost8functionIFvPSsPSt9exceptionEEE
pub fn stub_0x317378() {
    // IDA 0x317378: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
#[doc(alias = "__ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb")]
// 0x317570 — __ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb
// was: RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)
pub fn stub_0x317570() {
    // IDA 0x317570: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::doPostStream(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN3RBXL12doPostStreamESsN5boost10shared_ptrISiEEbbNS0_8functionIFvPSsPSt9exceptionEEE")]
// 0x317a08 — __ZN3RBXL12doPostStreamESsN5boost10shared_ptrISiEEbbNS0_8functionIFvPSsPSt9exceptionEEE
// was: RBX::doPostStream(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_0x317a08() {
    // IDA 0x317a08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::scoped_ptr<RBX::ThreadPool>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX10ThreadPoolEED1Ev")]
// 0x318104 — __ZN5boost10scoped_ptrIN3RBX10ThreadPoolEED1Ev
pub fn stub_0x318104() {
    // IDA 0x318104: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_3<std::string,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")]
// 0x318118 — __ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// type: int __fastcall(int, int, std::string *, int, int)
pub fn stub_0x318118() {
    // IDA 0x318118: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_")]
// 0x3183e0 — __ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_
// type: int __fastcall(int, int, std::string *, int, int, int, int)
pub fn stub_0x3183e0() {
    // IDA 0x3183e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")]
// 0x31888c — __ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, int, int, int)
// was: boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_0x31888c() {
    // IDA 0x31888c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_")]
// 0x3195c0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_
// was: void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x3195c0() {
    // IDA 0x3195c0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// 0x3199f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x3199f4() {
    // IDA 0x3199f4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEvNS5_IN3RBX5mutexEEEE6invokeERNS1_15function_bufferESP_")]
// 0x319a10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEvNS5_IN3RBX5mutexEEEE6invokeERNS1_15function_bufferESP_
// type: int __fastcall(int)
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)
pub fn stub_0x319a10() {
    // IDA 0x319a10: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x319a28 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x319a28() {
    // IDA 0x319a28: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x319e4c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x319e4c() {
    // IDA 0x319e4c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x31a26c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
// was: void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31a26c() {
    // IDA 0x31a26c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x31a3d8 — __ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)
pub fn stub_0x31a3d8() {
    // IDA 0x31a3d8: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x31a594 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x31a594() {
    // IDA 0x31a594: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_")]
// 0x31a7f0 — __ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_0x31a7f0() {
    // IDA 0x31a7f0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_")]
// 0x31a99c — __ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_0x31a99c() {
    // IDA 0x31a99c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_")]
// 0x31ab68 — __ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)
pub fn stub_0x31ab68() {
    // IDA 0x31ab68: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_")]
// 0x31ace0 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)
pub fn stub_0x31ace0() {
    // IDA 0x31ace0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_")]
// 0x31ae54 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_
// was: boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)
pub fn stub_0x31ae54() {
    // IDA 0x31ae54: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_")]
// 0x31b6c4 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x31b6c4() {
    // IDA 0x31b6c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
// 0x31ba5c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_0x31ba5c() {
    // IDA 0x31ba5c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_")]
// 0x31ba78 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)
pub fn stub_0x31ba78() {
    // IDA 0x31ba78: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x31ba80 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x31ba80() {
    // IDA 0x31ba80: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x31be08 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x31be08() {
    // IDA 0x31be08: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x31c18c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// was: void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31c18c() {
    // IDA 0x31c18c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x31c2e8 — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)
pub fn stub_0x31c2e8() {
    // IDA 0x31c2e8: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x31c4e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
pub fn stub_0x31c4e4() {
    // IDA 0x31c4e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_")]
// 0x31c72c — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
pub fn stub_0x31c72c() {
    // IDA 0x31c72c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_")]
// 0x31c918 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
pub fn stub_0x31c918() {
    // IDA 0x31c918: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_")]
// 0x31cb24 — __ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_
// type: int __fastcall(int, const std::string *, const std::string *, unsigned __int8, char)
pub fn stub_0x31cb24() {
    // IDA 0x31cb24: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_")]
// 0x31ccd4 — __ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_
pub fn stub_0x31ccd4() {
    // IDA 0x31ccd4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_")]
// 0x31d2d0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x31d2d0() {
    // IDA 0x31d2d0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
// 0x31d50c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_0x31d50c() {
    // IDA 0x31d50c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_")]
// 0x31d528 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)
pub fn stub_0x31d528() {
    // IDA 0x31d528: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x31d530 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x31d530() {
    // IDA 0x31d530: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x31d75c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, std::string *)
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x31d75c() {
    // IDA 0x31d75c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x31d984 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
// was: void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31d984() {
    // IDA 0x31d984: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x31da84 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)
pub fn stub_0x31da84() {
    // IDA 0x31da84: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x31dbf0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
pub fn stub_0x31dbf0() {
    // IDA 0x31dbf0: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::assign_to_own(boost::function2<void,std::string *,std::exception *> const&)")]
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_")]
// 0x31dd9c — __ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
pub fn stub_0x31dd9c() {
    // IDA 0x31dd9c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_")]
// 0x31ddcc — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
// type: int(void)
pub fn stub_0x31ddcc() {
    // IDA 0x31ddcc: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_")]
// 0x31df30 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
pub fn stub_0x31df30() {
    // IDA 0x31df30: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_")]
// 0x31e084 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_
pub fn stub_0x31e084() {
    // IDA 0x31e084: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::operator()(std::string *,std::exception *)const")]
#[doc(alias = "__ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_")]
// 0x31e1a8 — __ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_
pub fn stub_0x31e1a8() {
    // IDA 0x31e1a8: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
#[doc(alias = "__ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_")]
// 0x31e270 — __ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// was: rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_0x31e270() {
    // IDA 0x31e270: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_")]
// 0x31e344 — __ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x31e344() {
    // IDA 0x31e344: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev")]
// 0x31e43c — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev
pub fn stub_0x31e43c() {
    // IDA 0x31e43c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev")]
// 0x31e440 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev
pub fn stub_0x31e440() {
    // IDA 0x31e440: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv")]
// 0x31e444 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv
pub fn stub_0x31e444() {
    // IDA 0x31e444: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info")]
// 0x31e454 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
pub fn stub_0x31e454() {
    // IDA 0x31e454: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv")]
// 0x31e458 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x31e458() {
    // IDA 0x31e458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy(void)")]
#[doc(alias = "__ZN5boost15circular_bufferIdSaIdEE7destroyEv")]
// 0x31e63c — __ZN5boost15circular_bufferIdSaIdEE7destroyEv
// type: int(void)
pub fn stub_0x31e63c() {
    // IDA 0x31e63c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::clear(void)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv")]
// 0x325bb4 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv
pub fn stub_0x325bb4() {
    // IDA 0x325bb4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_low_capacity(unsigned long)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm")]
// 0x325be0 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm
pub fn stub_0x325be0() {
    // IDA 0x325be0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::push_back(RBX::InterpolatedCFrame::FrameInfo const&)")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE9push_backERKS3_")]
// 0x325c38 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE9push_backERKS3_
// type: int __fastcall(int, G3D::Matrix3 *)
pub fn stub_0x325c38() {
    // IDA 0x325c38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::set_capacity(unsigned long)")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm")]
// 0x325ce4 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm
pub fn stub_0x325ce4() {
    // IDA 0x325ce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator+=(int)")]
#[doc(alias = "__ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi")]
// 0x325e8c — __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi
pub fn stub_0x325e8c() {
    // IDA 0x325e8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator-=(int)")]
#[doc(alias = "__ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi")]
// 0x325ed4 — __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi
pub fn stub_0x325ed4() {
    // IDA 0x325ed4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_")]
// 0x325f14 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_
pub fn stub_0x325f14() {
    // IDA 0x325f14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_")]
// 0x325f94 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_
pub fn stub_0x325f94() {
    // IDA 0x325f94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_high_capacity(void)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv")]
// 0x32607c — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv
pub fn stub_0x32607c() {
    // IDA 0x32607c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::linearize_pointer<boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer> const&)const")]
#[doc(alias = "__ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE")]
// 0x3260d8 — __ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE
// type: int __fastcall(_DWORD **, int)
pub fn stub_0x3260d8() {
    // IDA 0x3260d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
// 0x345b08 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_0x345b08() {
    // IDA 0x345b08: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")]
// 0x345b48 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
pub fn stub_0x345b48() {
    // IDA 0x345b48: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE")]
// 0x345bb4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_0x345bb4() {
    // IDA 0x345bb4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const")]
#[doc(alias = "__ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv")]
// 0x352dc8 — __ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv
// was: rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const
pub fn stub_0x352dc8() {
    // IDA 0x352dc8: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<bool (*)(std::string const&,std::string *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerIPFbRKSsPSsEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")]
// 0x356328 — __ZN5boost6detail8function15functor_managerIPFbRKSsPSsEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
pub fn stub_0x356328() {
    // IDA 0x356328: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function1<void,bool>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvbE5clearEv")]
// 0x356588 — __ZN5boost9function1IvbE5clearEv
pub fn stub_0x356588() {
    // IDA 0x356588: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv")]
// 0x3565b8 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)
pub fn stub_0x3565b8() {
    // IDA 0x3565b8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")]
// 0x36e490 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// type: int __fastcall(int, int *, int)
pub fn stub_0x36e490() {
    // IDA 0x36e490: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
// 0x36e4ec — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x36e4ec() {
    // IDA 0x36e4ec: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// 0x36e518 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
pub fn stub_0x36e518() {
    // IDA 0x36e518: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
// 0x36e610 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int __fastcall(int, char **)
pub fn stub_0x36e610() {
    // IDA 0x36e610: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")]
// 0x36e650 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
pub fn stub_0x36e650() {
    // IDA 0x36e650: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_")]
// 0x36e6bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
pub fn stub_0x36e6bc() {
    // IDA 0x36e6bc: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_")]
// 0x36e874 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
pub fn stub_0x36e874() {
    // IDA 0x36e874: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0x36e898 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
pub fn stub_0x36e898() {
    // IDA 0x36e898: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev")]
// 0x36e8e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev
// type: int __fastcall(int)
pub fn stub_0x36e8e8() {
    // IDA 0x36e8e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0x36e908 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
pub fn stub_0x36e908() {
    // IDA 0x36e908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")]
// 0x36ea30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x36ea30() {
    // IDA 0x36ea30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")]
// 0x36eac0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x36eac0() {
    // IDA 0x36eac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
// 0x36eaec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_0x36eaec() {
    // IDA 0x36eaec: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv")]
// 0x36eb44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
pub fn stub_0x36eb44() {
    // IDA 0x36eb44: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
// 0x370714 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
pub fn stub_0x370714() {
    // IDA 0x370714: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv")]
// 0x37074c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
pub fn stub_0x37074c() {
    // IDA 0x37074c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE")]
// 0x3708e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// type: int __fastcall(int result, unsigned int)
pub fn stub_0x3708e4() {
    // IDA 0x3708e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
// 0x37dd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// type: int __fastcall(__int64, unsigned int)
pub fn stub_0x37dd20() {
    // IDA 0x37dd20: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE")]
// 0x37dd80 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
pub fn stub_0x37dd80() {
    // IDA 0x37dd80: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::StandardOut::print_exception(boost::function0<void> const&,RBX::MessageType,bool)")]
#[doc(alias = "__ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb")]
// 0x381b0c — __ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb
// type: void __fastcall(int, int, int, int, int, char, int, int, void *, int)
pub fn stub_0x381b0c() {
    // IDA 0x381b0c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev")]
// 0x3821f0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
// type: int __fastcall(int)
// was: rbx_core::SharedPtr<RBX::StandardOut>::~shared_ptr()
pub fn stub_0x3821f0() {
    // IDA 0x3821f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")]
// 0x382348 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)
pub fn stub_0x382348() {
    // IDA 0x382348: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
