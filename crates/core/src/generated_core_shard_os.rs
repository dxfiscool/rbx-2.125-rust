//! core shard os - 100 core stubs EA-sorted, 0x8bfd14..0x9c4974 (rbx:: namespace, EA-sorted asc, next 100 uncovered).
//! Source: ida/export.json filtered where demangled starts with rbx:: or thunk-to-rbx::, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> crate::SharedPtr, single quotes/backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::call(RBX::UserInputService::SwipeDirection)")]
// 0x8bfd14 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
pub fn stub_0x8bfd14() {
    // IDA 0x8bfd14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::remove(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot *)")]
// 0x8bfde0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6removeEPNS6_4slotE
pub fn stub_0x8bfde0() {
    // IDA 0x8bfde0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::safe_static_init_mutex(void)")]
// 0x8bfed0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot22safe_static_init_mutexEv
pub fn stub_0x8bfed0() {
    // IDA 0x8bfed0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::safe_static_do_get_mutex(void)")]
// 0x8bfed4 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x8bfed4() {
    // IDA 0x8bfed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::~callable()")]
// 0x8bffc4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
pub fn stub_0x8bffc4() {
    // IDA 0x8bffc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::~callable()")]
// 0x8c00d4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
pub fn stub_0x8c00d4() {
    // IDA 0x8c00d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::~slot()")]
// 0x8c0204 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotD1Ev
pub fn stub_0x8c0204() {
    // IDA 0x8c0204: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::~slot()")]
// 0x8c0230 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotD0Ev
pub fn stub_0x8c0230() {
    // IDA 0x8c0230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,bool)>::operator()(int,int,bool)")]
// 0x8d1ef0 — __ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib
pub fn stub_0x8d1ef0() {
    // IDA 0x8d1ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,bool)>::slot> &)")]
// 0x8d2040 — __ZN3rbx7signals6signalIFviibEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0x8d2040() {
    // IDA 0x8d2040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::on_error(std::exception &)")]
// 0x8d21a0 — __ZN3rbx7signals6signalIFviibEE8on_errorERSt9exception
pub fn stub_0x8d21a0() {
    // IDA 0x8d21a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::safe_static_init_mutex(void)")]
// 0x8d21ec — __ZN3rbx7signals6signalIFviibEE22safe_static_init_mutexEv
pub fn stub_0x8d21ec() {
    // IDA 0x8d21ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::safe_static_do_get_mutex(void)")]
// 0x8d21f0 — __ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv
pub fn stub_0x8d21f0() {
    // IDA 0x8d21f0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::disconnectAll(void)")]
// 0x8d3408 — __ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv
pub fn stub_0x8d3408() {
    // IDA 0x8d3408: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::remote_signal(void)")]
// 0x8d3854 — __ZN3rbx13remote_signalIFviibEEC2Ev
pub fn stub_0x8d3854() {
    // IDA 0x8d3854: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::disconnectAll(void)")]
// 0x8d39b0 — __ZN3rbx7signals6signalIFviibEE13disconnectAllEv
pub fn stub_0x8d39b0() {
    // IDA 0x8d39b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,bool)>::connect<boost::function<void ()(int,int,bool)>>(boost::function<void ()(int,int,bool)> const&)")]
// 0x8d5508 — __ZN3rbx7signals6signalIFviibEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_0x8d5508() {
    // IDA 0x8d5508: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::insert(rbx::signals::signal<void ()(int,int,bool)>::slot *)")]
// 0x8d55fc — __ZN3rbx7signals6signalIFviibEE6insertEPNS3_4slotE
pub fn stub_0x8d55fc() {
    // IDA 0x8d55fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::callable<rbx::signals::signal<void ()(int,int,bool)>*>(boost::function<void ()(int,int,bool)> const&,rbx::signals::signal<void ()(int,int,bool)>*)")]
// 0x8d582c — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_0x8d582c() {
    // IDA 0x8d582c: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::callable_slot<boost::function<void ()(int,int,bool)>>::~callable_slot()")]
// 0x8d5928 — __ZN3rbx7signals6signalIFviibEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_0x8d5928() {
    // IDA 0x8d5928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::callable_slot<boost::function<void ()(int,int,bool)>>::~callable_slot()")]
// 0x8d5a38 — __ZN3rbx7signals6signalIFviibEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_0x8d5a38() {
    // IDA 0x8d5a38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::disconnect(void)")]
// 0x8d5b68 — __ZN3rbx7signals6signalIFviibEE4slot10disconnectEv
pub fn stub_0x8d5b68() {
    // IDA 0x8d5b68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::connected(void)const")]
// 0x8d5c78 — __ZNK3rbx7signals6signalIFviibEE4slot9connectedEv
pub fn stub_0x8d5c78() {
    // IDA 0x8d5c78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::call(int,int,bool)")]
// 0x8d5c84 — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_E4callEiib
pub fn stub_0x8d5c84() {
    // IDA 0x8d5c84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::call(int,int,bool)")]
// 0x8d5c8c — __ZThn4_N3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_E4callEiib
pub fn stub_0x8d5c8c() {
    // IDA 0x8d5c8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::remove(rbx::signals::signal<void ()(int,int,bool)>::slot *)")]
// 0x8d5d60 — __ZN3rbx7signals6signalIFviibEE6removeEPNS3_4slotE
pub fn stub_0x8d5d60() {
    // IDA 0x8d5d60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::safe_static_init_mutex(void)")]
// 0x8d5e50 — __ZN3rbx7signals6signalIFviibEE4slot22safe_static_init_mutexEv
pub fn stub_0x8d5e50() {
    // IDA 0x8d5e50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::safe_static_do_get_mutex(void)")]
// 0x8d5e54 — __ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x8d5e54() {
    // IDA 0x8d5e54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::~callable()")]
// 0x8d5f48 — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_0x8d5f48() {
    // IDA 0x8d5f48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::~callable()")]
// 0x8d6058 — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_0x8d6058() {
    // IDA 0x8d6058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::~slot()")]
// 0x8d6188 — __ZN3rbx7signals6signalIFviibEE4slotD1Ev
pub fn stub_0x8d6188() {
    // IDA 0x8d6188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::~slot()")]
// 0x8d61b4 — __ZN3rbx7signals6signalIFviibEE4slotD0Ev
pub fn stub_0x8d61b4() {
    // IDA 0x8d61b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::insert(rbx::signals::signal<void ()(std::string,int,int)>::slot *)")]
// 0x8dc2d8 — __ZN3rbx7signals6signalIFvSsiiEE6insertEPNS3_4slotE
pub fn stub_0x8dc2d8() {
    // IDA 0x8dc2d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::callable<rbx::signals::signal<void ()(std::string,int,int)>*>(boost::function<void ()(std::string,int,int)> const&,rbx::signals::signal<void ()(std::string,int,int)>*)")]
// 0x8dc4e8 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_0x8dc4e8() {
    // IDA 0x8dc4e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x8dc5e8 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii
pub fn stub_0x8dc5e8() {
    // IDA 0x8dc5e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::remove(rbx::signals::signal<void ()(std::string,int,int)>::slot *)")]
// 0x8dc710 — __ZN3rbx7signals6signalIFvSsiiEE6removeEPNS3_4slotE
pub fn stub_0x8dc710() {
    // IDA 0x8dc710: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_init_mutex(void)")]
// 0x8dc800 — __ZN3rbx7signals6signalIFvSsiiEE4slot22safe_static_init_mutexEv
pub fn stub_0x8dc800() {
    // IDA 0x8dc800: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::~slot()")]
// 0x8dc808 — __ZN3rbx7signals6signalIFvSsiiEE4slotD0Ev
pub fn stub_0x8dc808() {
    // IDA 0x8dc808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")]
// 0x8e02d4 — __ZN3rbx13remote_signalIFviibEED2Ev
pub fn stub_0x8e02d4() {
    // IDA 0x8e02d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")]
// 0x8e06b8 — __ZN3rbx13remote_signalIFvSsiiEED2Ev
pub fn stub_0x8e06b8() {
    // IDA 0x8e06b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x926f24 — __ZN3rbx7signals6signalIFviSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_0x926f24() {
    // IDA 0x926f24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::insert(rbx::signals::signal<void ()(int,std::string)>::slot *)")]
// 0x926f98 — __ZN3rbx7signals6signalIFviSsEE6insertEPNS3_4slotE
pub fn stub_0x926f98() {
    // IDA 0x926f98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::safe_static_init_mutex(void)")]
// 0x9271ec — __ZN3rbx7signals6signalIFviSsEE22safe_static_init_mutexEv
pub fn stub_0x9271ec() {
    // IDA 0x9271ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::safe_static_do_get_mutex(void)")]
// 0x9271f0 — __ZN3rbx7signals6signalIFviSsEE24safe_static_do_get_mutexEv
pub fn stub_0x9271f0() {
    // IDA 0x9271f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x9272e8 — __ZN3rbx7signals6signalIFviSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
pub fn stub_0x9272e8() {
    // IDA 0x9272e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x927314 — __ZN3rbx7signals6signalIFviSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
pub fn stub_0x927314() {
    // IDA 0x927314: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::disconnect(void)")]
// 0x9273e8 — __ZN3rbx7signals6signalIFviSsEE4slot10disconnectEv
pub fn stub_0x9273e8() {
    // IDA 0x9273e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::connected(void)const")]
// 0x9274f8 — __ZNK3rbx7signals6signalIFviSsEE4slot9connectedEv
pub fn stub_0x9274f8() {
    // IDA 0x9274f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::call(int,std::string)")]
// 0x927504 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEiSs
pub fn stub_0x927504() {
    // IDA 0x927504: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::call(int,std::string)")]
// 0x927528 — __ZThn4_N3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEiSs
pub fn stub_0x927528() {
    // IDA 0x927528: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::remove(rbx::signals::signal<void ()(int,std::string)>::slot *)")]
// 0x9277ac — __ZN3rbx7signals6signalIFviSsEE6removeEPNS3_4slotE
pub fn stub_0x9277ac() {
    // IDA 0x9277ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::safe_static_init_mutex(void)")]
// 0x92789c — __ZN3rbx7signals6signalIFviSsEE4slot22safe_static_init_mutexEv
pub fn stub_0x92789c() {
    // IDA 0x92789c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::safe_static_do_get_mutex(void)")]
// 0x9278a0 — __ZN3rbx7signals6signalIFviSsEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x9278a0() {
    // IDA 0x9278a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::~slot()")]
// 0x927990 — __ZN3rbx7signals6signalIFviSsEE4slotD1Ev
pub fn stub_0x927990() {
    // IDA 0x927990: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::~slot()")]
// 0x9279bc — __ZN3rbx7signals6signalIFviSsEE4slotD0Ev
pub fn stub_0x9279bc() {
    // IDA 0x9279bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::~callable()")]
// 0x927a90 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
pub fn stub_0x927a90() {
    // IDA 0x927a90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::~callable()")]
// 0x927abc — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
pub fn stub_0x927abc() {
    // IDA 0x927abc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::remote_signal(void)")]
// 0x928768 — __ZN3rbx13remote_signalIFviSsEEC2Ev
pub fn stub_0x928768() {
    // IDA 0x928768: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::disconnectAll(void)")]
// 0x9288c4 — __ZN3rbx7signals6signalIFviSsEE13disconnectAllEv
pub fn stub_0x9288c4() {
    // IDA 0x9288c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(int,std::string)>::operator()(int,std::string)")]
// 0x92e824 — __ZN3rbx7signals16signal_with_argsILi2EFviSsEEclEiSs
pub fn stub_0x92e824() {
    // IDA 0x92e824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,std::string)>::slot> &)")]
// 0x92ea94 — __ZN3rbx7signals6signalIFviSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0x92ea94() {
    // IDA 0x92ea94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::on_error(std::exception &)")]
// 0x92ebf4 — __ZN3rbx7signals6signalIFviSsEE8on_errorERSt9exception
pub fn stub_0x92ebf4() {
    // IDA 0x92ebf4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::function<void ()(int,std::string)>>(boost::function<void ()(int,std::string)> const&)")]
// 0x92f5d4 — __ZN3rbx7signals6signalIFviSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_0x92f5d4() {
    // IDA 0x92f5d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::callable<rbx::signals::signal<void ()(int,std::string)>*>(boost::function<void ()(int,std::string)> const&,rbx::signals::signal<void ()(int,std::string)>*)")]
// 0x92f6c8 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_0x92f6c8() {
    // IDA 0x92f6c8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::function<void ()(int,std::string)>>::~callable_slot()")]
// 0x92f7c4 — __ZN3rbx7signals6signalIFviSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_0x92f7c4() {
    // IDA 0x92f7c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::function<void ()(int,std::string)>>::~callable_slot()")]
// 0x92f8d4 — __ZN3rbx7signals6signalIFviSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_0x92f8d4() {
    // IDA 0x92f8d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::call(int,std::string)")]
// 0x92fa04 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callEiSs
pub fn stub_0x92fa04() {
    // IDA 0x92fa04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::call(int,std::string)")]
// 0x92fb24 — __ZThn4_N3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callEiSs
pub fn stub_0x92fb24() {
    // IDA 0x92fb24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::~callable()")]
// 0x92fc84 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_0x92fc84() {
    // IDA 0x92fc84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::~callable()")]
// 0x92fd94 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_0x92fd94() {
    // IDA 0x92fd94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::~remote_signal()")]
// 0x9384f0 — __ZN3rbx13remote_signalIFviSsEED2Ev
pub fn stub_0x9384f0() {
    // IDA 0x9384f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,std::string)>::operator()(std::string,int,std::string)")]
// 0x96cf60 — __ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEEclESsiSs
pub fn stub_0x96cf60() {
    // IDA 0x96cf60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *,std::string,int,std::string)")]
// 0x970090 — __ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEE8fireItemEPNS0_6signalIS2_E4slotESsiSs
pub fn stub_0x970090() {
    // IDA 0x970090: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::disconnectAll(void)")]
// 0x971c04 — __ZN3rbx7signals6signalIFvSsiSsEE13disconnectAllEv
pub fn stub_0x971c04() {
    // IDA 0x971c04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::insert(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *)")]
// 0x97356c — __ZN3rbx7signals6signalIFvSsiSsEE6insertEPNS3_4slotE
pub fn stub_0x97356c() {
    // IDA 0x97356c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::callable_slot<boost::function<void ()(std::string,int,std::string)>>::~callable_slot()")]
// 0x9738d4 — __ZN3rbx7signals6signalIFvSsiSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_0x9738d4() {
    // IDA 0x9738d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::callable_slot<boost::function<void ()(std::string,int,std::string)>>::~callable_slot()")]
// 0x9738e0 — __ZN3rbx7signals6signalIFvSsiSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_0x9738e0() {
    // IDA 0x9738e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::disconnect(void)")]
// 0x973994 — __ZN3rbx7signals6signalIFvSsiSsEE4slot10disconnectEv
pub fn stub_0x973994() {
    // IDA 0x973994: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::connected(void)const")]
// 0x973b08 — __ZNK3rbx7signals6signalIFvSsiSsEE4slot9connectedEv
pub fn stub_0x973b08() {
    // IDA 0x973b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::call(std::string,int,std::string)")]
// 0x973b14 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs
pub fn stub_0x973b14() {
    // IDA 0x973b14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::call(std::string,int,std::string)")]
// 0x973cb8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs
pub fn stub_0x973cb8() {
    // IDA 0x973cb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::remove(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *)")]
// 0x973f50 — __ZN3rbx7signals6signalIFvSsiSsEE6removeEPNS3_4slotE
pub fn stub_0x973f50() {
    // IDA 0x973f50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::safe_static_init_mutex(void)")]
// 0x97403c — __ZN3rbx7signals6signalIFvSsiSsEE4slot22safe_static_init_mutexEv
pub fn stub_0x97403c() {
    // IDA 0x97403c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0x974120 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
pub fn stub_0x974120() {
    // IDA 0x974120: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0x9742b8 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_0x9742b8() {
    // IDA 0x9742b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0x9742c4 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_0x9742c4() {
    // IDA 0x9742c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::~slot()")]
// 0x974378 — __ZN3rbx7signals6signalIFvSsiSsEE4slotD1Ev
pub fn stub_0x974378() {
    // IDA 0x974378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::~slot()")]
// 0x9743d4 — __ZN3rbx7signals6signalIFvSsiSsEE4slotD0Ev
pub fn stub_0x9743d4() {
    // IDA 0x9743d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::safe_static_init_mutex(void)")]
// 0x975048 — __ZN3rbx7signals6signalIFvSsEE4slot22safe_static_init_mutexEv
pub fn stub_0x975048() {
    // IDA 0x975048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::~slot()")]
// 0x975130 — __ZN3rbx7signals6signalIFvSsEE4slotD0Ev
pub fn stub_0x975130() {
    // IDA 0x975130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot> &)")]
// 0x9828f0 — __ZN3rbx7signals6signalIFvSsiSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0x9828f0() {
    // IDA 0x9828f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::mutex(void)")]
// 0x982af8 — __ZN3rbx7signals6signalIFvSsiSsEE5mutexEv
pub fn stub_0x982af8() {
    // IDA 0x982af8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::safe_static_init_mutex(void)")]
// 0x982c10 — __ZN3rbx7signals6signalIFvSsiSsEE22safe_static_init_mutexEv
pub fn stub_0x982c10() {
    // IDA 0x982c10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Primitive *)>::operator()(RBX::Primitive *)")]
// 0x9bec1c — __ZN3rbx7signals16signal_with_argsILi1EFvPN3RBX9PrimitiveEEEclES4_
pub fn stub_0x9bec1c() {
    // IDA 0x9bec1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> &)")]
// 0x9bf028 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
pub fn stub_0x9bf028() {
    // IDA 0x9bf028: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::mutex(void)")]
// 0x9bf22c — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE5mutexEv
pub fn stub_0x9bf22c() {
    // IDA 0x9bf22c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::insert(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot *)")]
// 0x9c4168 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6insertEPNS7_4slotE
pub fn stub_0x9c4168() {
    // IDA 0x9c4168: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::mutex(void)")]
// 0x9c4420 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE5mutexEv
pub fn stub_0x9c4420() {
    // IDA 0x9c4420: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::disconnect(void)")]
// 0x9c4800 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot10disconnectEv
pub fn stub_0x9c4800() {
    // IDA 0x9c4800: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::connected(void)const")]
// 0x9c4974 — __ZNK3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot9connectedEv
pub fn stub_0x9c4974() {
    // IDA 0x9c4974: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
