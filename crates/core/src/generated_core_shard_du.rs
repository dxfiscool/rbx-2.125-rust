//! core shard DU — 100 core stubs EA-sorted, next uncovered after DT 0x842fa8 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::~callable()")]
// 0x842fd4 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_ED0Ev
pub fn stub_842fd4() {
    // IDA 0x842fd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0x8430a8 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_8430a8() {
    // IDA 0x8430a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::insert(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot *)")]
// 0x84311c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE6insertEPNS6_4slotE
pub fn stub_84311c() {
    // IDA 0x84311c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot*)")]
// 0x843328 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot*)
pub fn stub_843328() {
    // IDA 0x843328: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x84334c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEED1Ev
pub fn stub_84334c() {
    // IDA 0x84334c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x843378 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEED0Ev
pub fn stub_843378() {
    // IDA 0x843378: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::disconnect(void)")]
// 0x84344c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot10disconnectEv
pub fn stub_84344c() {
    // IDA 0x84344c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::connected(void)const")]
// 0x84355c — __ZNK3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot9connectedEv
pub fn stub_84355c() {
    // IDA 0x84355c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::call(int,int,RBX::FriendService::FriendStatus)")]
// 0x843568 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_E4callEiiS5_
pub fn stub_843568() {
    // IDA 0x843568: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::call(int,int,RBX::FriendService::FriendStatus)")]
// 0x843594 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_E4callEiiS5_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::call(int,int,RBX::FriendService::FriendStatus)
pub fn stub_843594() {
    // IDA 0x843594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::FriendService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list3<int &,int &,RBX::FriendService::FriendStatus&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus> &,boost::_bi::list3<int &,int &,RBX::FriendService::FriendStatus&> &,int)")]
// 0x8435c0 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13FriendServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_iiNS4_12FriendStatusEEENS0_5list3IRiSI_RSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_8435c0() {
    // IDA 0x8435c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::remove(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot *)")]
// 0x8435f4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE6removeEPNS6_4slotE
pub fn stub_8435f4() {
    // IDA 0x8435f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::safe_static_init_mutex(void)")]
// 0x8436e4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot22safe_static_init_mutexEv
pub fn stub_8436e4() {
    // IDA 0x8436e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::safe_static_do_get_mutex(void)")]
// 0x8436e8 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot24safe_static_do_get_mutexEv
pub fn stub_8436e8() {
    // IDA 0x8436e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::~slot()")]
// 0x8437d8 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotD1Ev
pub fn stub_8437d8() {
    // IDA 0x8437d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::~slot()")]
// 0x843804 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotD0Ev
pub fn stub_843804() {
    // IDA 0x843804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::~callable()")]
// 0x8438d8 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_ED1Ev
pub fn stub_8438d8() {
    // IDA 0x8438d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::~callable()")]
// 0x843904 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_ED0Ev
pub fn stub_843904() {
    // IDA 0x843904: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendStatus)>::remote_signal(void)")]
// 0x8448c0 — __ZN3rbx13remote_signalIFviiN3RBX13FriendService12FriendStatusEEEC2Ev
pub fn stub_8448c0() {
    // IDA 0x8448c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::disconnectAll(void)")]
// 0x844a1c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13disconnectAllEv
pub fn stub_844a1c() {
    // IDA 0x844a1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendEventType)>::remote_signal(void)")]
// 0x844b94 — __ZN3rbx13remote_signalIFviiN3RBX13FriendService15FriendEventTypeEEEC2Ev
pub fn stub_844b94() {
    // IDA 0x844b94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::disconnectAll(void)")]
// 0x844cf0 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13disconnectAllEv
pub fn stub_844cf0() {
    // IDA 0x844cf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendStatus>::clear(void)")]
// 0x845600 — __ZN5boost9function3IviiN3RBX13FriendService12FriendStatusEE5clearEv
pub fn stub_845600() {
    // IDA 0x845600: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::connect<boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>>(boost::function<void ()(int,int,RBX::FriendService::FriendStatus)> const&)")]
// 0x845d54 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_845d54() {
    // IDA 0x845d54: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>*>(boost::function<void ()(int,int,RBX::FriendService::FriendStatus)> const&,rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>*)")]
// 0x845e48 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
pub fn stub_845e48() {
    // IDA 0x845e48: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>>::~callable_slot()")]
// 0x845f44 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13callable_slotIN5boost8functionIS5_EEED1Ev
pub fn stub_845f44() {
    // IDA 0x845f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>>::~callable_slot()")]
// 0x846054 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_846054() {
    // IDA 0x846054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::call(int,int,RBX::FriendService::FriendStatus)")]
// 0x846184 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEiiS5_
pub fn stub_846184() {
    // IDA 0x846184: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::call(int,int,RBX::FriendService::FriendStatus)")]
// 0x84618c — __ZThn4_N3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEiiS5_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::call(int,int,RBX::FriendService::FriendStatus)
pub fn stub_84618c() {
    // IDA 0x84618c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendStatus>::operator()(int,int,RBX::FriendService::FriendStatus)const")]
// 0x846194 — __ZNK5boost9function3IviiN3RBX13FriendService12FriendStatusEEclEiiS3_
pub fn stub_846194() {
    // IDA 0x846194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::~callable()")]
// 0x846260 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
pub fn stub_846260() {
    // IDA 0x846260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::~callable()")]
// 0x846370 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
pub fn stub_846370() {
    // IDA 0x846370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendStatus>::assign_to_own(boost::function3<void,int,int,RBX::FriendService::FriendStatus> const&)")]
// 0x8464a0 — __ZN5boost9function3IviiN3RBX13FriendService12FriendStatusEE13assign_to_ownERKS4_
pub fn stub_8464a0() {
    // IDA 0x8464a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendEventType>::clear(void)")]
// 0x846da0 — __ZN5boost9function3IviiN3RBX13FriendService15FriendEventTypeEE5clearEv
pub fn stub_846da0() {
    // IDA 0x846da0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::connect<boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>>(boost::function<void ()(int,int,RBX::FriendService::FriendEventType)> const&)")]
// 0x8474f4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_8474f4() {
    // IDA 0x8474f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>*>(boost::function<void ()(int,int,RBX::FriendService::FriendEventType)> const&,rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>*)")]
// 0x8475e8 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
pub fn stub_8475e8() {
    // IDA 0x8475e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>>::~callable_slot()")]
// 0x8476e4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13callable_slotIN5boost8functionIS5_EEED1Ev
pub fn stub_8476e4() {
    // IDA 0x8476e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::callable_slot<boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>>::~callable_slot()")]
// 0x8477f4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_8477f4() {
    // IDA 0x8477f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::call(int,int,RBX::FriendService::FriendEventType)")]
// 0x847924 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEiiS5_
pub fn stub_847924() {
    // IDA 0x847924: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::call(int,int,RBX::FriendService::FriendEventType)")]
// 0x84792c — __ZThn4_N3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEiiS5_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::call(int,int,RBX::FriendService::FriendEventType)
pub fn stub_84792c() {
    // IDA 0x84792c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendEventType>::operator()(int,int,RBX::FriendService::FriendEventType)const")]
// 0x847934 — __ZNK5boost9function3IviiN3RBX13FriendService15FriendEventTypeEEclEiiS3_
pub fn stub_847934() {
    // IDA 0x847934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::~callable()")]
// 0x847a00 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
pub fn stub_847a00() {
    // IDA 0x847a00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::~callable()")]
// 0x847b10 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
pub fn stub_847b10() {
    // IDA 0x847b10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendEventType>::assign_to_own(boost::function3<void,int,int,RBX::FriendService::FriendEventType> const&)")]
// 0x847c40 — __ZN5boost9function3IviiN3RBX13FriendService15FriendEventTypeEE13assign_to_ownERKS4_
pub fn stub_847c40() {
    // IDA 0x847c40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FriendService::~FriendService()")]
// 0x848734 — __ZN3RBX13FriendServiceD2Ev
pub fn stub_848734() {
    // IDA 0x848734: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendEventType)>::~remote_signal()")]
// 0x848974 — __ZN3rbx13remote_signalIFviiN3RBX13FriendService15FriendEventTypeEEED2Ev
pub fn stub_848974() {
    // IDA 0x848974: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendStatus)>::~remote_signal()")]
// 0x848ac0 — __ZN3rbx13remote_signalIFviiN3RBX13FriendService12FriendStatusEEED2Ev
pub fn stub_848ac0() {
    // IDA 0x848ac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>> *)")]
// 0x848c0c — __ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
pub fn stub_848c0c() {
    // IDA 0x848c0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::setControlMode(RBX::GameBasicSettings::ControlMode)")]
// 0x8490fc — __ZN3RBX17GameBasicSettings14setControlModeENS0_11ControlModeE
pub fn stub_8490fc() {
    // IDA 0x8490fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::setUploadVideoSetting(RBX::GameSettings::UploadSetting)")]
// 0x849118 — __ZN3RBX17GameBasicSettings21setUploadVideoSettingENS_12GameSettings13UploadSettingE
pub fn stub_849118() {
    // IDA 0x849118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::setRenderQuality(RBX::GameBasicSettings::RenderQualitySetting)")]
// 0x849134 — __ZN3RBX17GameBasicSettings16setRenderQualityENS0_20RenderQualitySettingE
pub fn stub_849134() {
    // IDA 0x849134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::getTutorialState(std::string)")]
// 0x849150 — __ZN3RBX17GameBasicSettings16getTutorialStateESs
pub fn stub_849150() {
    // IDA 0x849150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::setTutorialState(std::string,bool)")]
// 0x849380 — __ZN3RBX17GameBasicSettings16setTutorialStateESsb
pub fn stub_849380() {
    // IDA 0x849380: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::getCompletedTutorials(void)const")]
// 0x8494e8 — __ZNK3RBX17GameBasicSettings21getCompletedTutorialsEv
pub fn stub_8494e8() {
    // IDA 0x8494e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::setCompletedTutorials(std::string)")]
// 0x8495fc — __ZN3RBX17GameBasicSettings21setCompletedTutorialsESs
pub fn stub_8495fc() {
    // IDA 0x8495fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::setAllTutorialsDisabled(bool)")]
// 0x849768 — __ZN3RBX17GameBasicSettings23setAllTutorialsDisabledEb
pub fn stub_849768() {
    // IDA 0x849768: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::GameBasicSettings(void)")]
// 0x849c0c — __ZN3RBX17GameBasicSettingsC1Ev
pub fn stub_849c0c() {
    // IDA 0x849c0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::GameBasicSettings(void)")]
// 0x849c10 — __ZN3RBX17GameBasicSettingsC2Ev
pub fn stub_849c10() {
    // IDA 0x849c10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::setMouseLock(bool)")]
// 0x849f38 — __ZN3RBX17GameBasicSettings12setMouseLockEb
pub fn stub_849f38() {
    // IDA 0x849f38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::reset(void)")]
// 0x849f40 — __ZN3RBX17GameBasicSettings5resetEv
pub fn stub_849f40() {
    // IDA 0x849f40: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::getControlMode(void)const")]
// 0x849f90 — __ZNK3RBX17GameBasicSettings14getControlModeEv
pub fn stub_849f90() {
    // IDA 0x849f90: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::getUploadVideoSetting(void)const")]
// 0x849fb8 — __ZNK3RBX17GameBasicSettings21getUploadVideoSettingEv
pub fn stub_849fb8() {
    // IDA 0x849fb8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::getRenderQuality(void)const")]
// 0x849fe0 — __ZNK3RBX17GameBasicSettings16getRenderQualityEv
pub fn stub_849fe0() {
    // IDA 0x849fe0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::getAllTutorialsDisabled(void)const")]
// 0x84a0b4 — __ZNK3RBX17GameBasicSettings23getAllTutorialsDisabledEv
pub fn stub_84a0b4() {
    // IDA 0x84a0b4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::getFullScreen(void)const")]
// 0x84a0e0 — __ZNK3RBX17GameBasicSettings13getFullScreenEv
pub fn stub_84a0e0() {
    // IDA 0x84a0e0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::setFullScreen(bool)")]
// 0x84a0e8 — __ZN3RBX17GameBasicSettings13setFullScreenEb
pub fn stub_84a0e8() {
    // IDA 0x84a0e8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::inFullScreen(void)")]
// 0x84a10c — __ZN3RBX17GameBasicSettings12inFullScreenEv
pub fn stub_84a10c() {
    // IDA 0x84a10c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameBasicSettings::inStudioMode(void)")]
// 0x84a138 — __ZN3RBX17GameBasicSettings12inStudioModeEv
pub fn stub_84a138() {
    // IDA 0x84a138: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::map<std::string,bool,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::operator[](std::string const&)")]
// 0x84aa94 — __ZNSt3mapISsbSt4lessISsESaISt4pairIKSsbEEEixERS3_
pub fn stub_84aa94() {
    // IDA 0x84aa94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84abe4 — __ZN3RBX17GameBasicSettingsD1Ev
pub fn stub_84abe4() {
    // IDA 0x84abe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84abe8 — __ZN3RBX17GameBasicSettingsD0Ev
pub fn stub_84abe8() {
    // IDA 0x84abe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84acd4 — __ZThn32_N3RBX17GameBasicSettingsD1Ev
// was: non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()
pub fn stub_84acd4() {
    // IDA 0x84acd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84acdc — __ZThn32_N3RBX17GameBasicSettingsD0Ev
// was: non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()
pub fn stub_84acdc() {
    // IDA 0x84acdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84ad90 — __ZThn36_N3RBX17GameBasicSettingsD1Ev
// was: non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()
pub fn stub_84ad90() {
    // IDA 0x84ad90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84ad98 — __ZThn36_N3RBX17GameBasicSettingsD0Ev
// was: non-virtual thunk toRBX::GameBasicSettings::~GameBasicSettings()
pub fn stub_84ad98() {
    // IDA 0x84ad98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,bool>>,std::pair<std::string const,bool> const&)")]
// 0x84b444 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_84b444() {
    // IDA 0x84b444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,bool> const&)")]
// 0x84b530 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_84b530() {
    // IDA 0x84b530: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_insert_unique(std::pair<std::string const,bool> const&)")]
// 0x84b580 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_84b580() {
    // IDA 0x84b580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_create_node(std::pair<std::string const,bool> const&)")]
// 0x84b604 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_84b604() {
    // IDA 0x84b604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::lower_bound(std::string const&)")]
// 0x84b6e0 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
pub fn stub_84b6e0() {
    // IDA 0x84b6e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::find(std::string const&)")]
// 0x84b710 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_84b710() {
    // IDA 0x84b710: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GlobalBasicSettings::Item::reset(void)")]
// 0x84b880 — __ZN3RBX19GlobalBasicSettings4Item5resetEv
pub fn stub_84b880() {
    // IDA 0x84b880: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::resize(unsigned long,RBX::GameBasicSettings::RenderQualitySetting)")]
// 0x84bb18 — __ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE6resizeEmS2_
pub fn stub_84bb18() {
    // IDA 0x84bb18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::push_back(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0x84bb4c — __ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE9push_backERKS2_
pub fn stub_84bb4c() {
    // IDA 0x84bb4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameBasicSettings::RenderQualitySetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::operator[](RBX::Name const* const&)")]
// 0x84bb74 — __ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings20RenderQualitySettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_84bb74() {
    // IDA 0x84bb74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// 0x84bbcc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_84bbcc() {
    // IDA 0x84bbcc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// 0x84bc80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_84bc80() {
    // IDA 0x84bc80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// 0x84bcd8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_84bcd8() {
    // IDA 0x84bcd8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0x84bd40 — __ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_84bd40() {
    // IDA 0x84bd40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_allocate(unsigned long)")]
// 0x84be24 — __ZNSt12_Vector_baseIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE11_M_allocateEm
pub fn stub_84be24() {
    // IDA 0x84be24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *>(RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *)")]
// 0x84be3c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17GameBasicSettings20RenderQualitySettingES6_EET0_T_S8_S7_
pub fn stub_84be3c() {
    // IDA 0x84be3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,unsigned long,RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0x84be78 — __ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_84be78() {
    // IDA 0x84be78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameBasicSettings::ControlMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::operator[](RBX::Name const* const&)")]
// 0x84c008 — __ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings11ControlModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_84c008() {
    // IDA 0x84c008: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode> const&)")]
// 0x84c060 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_84c060() {
    // IDA 0x84c060: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode> const&)")]
// 0x84c114 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_84c114() {
    // IDA 0x84c114: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode> const&)")]
// 0x84c16c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_84c16c() {
    // IDA 0x84c16c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::resize(unsigned long,RBX::GameBasicSettings::ControlMode)")]
// 0x84c1d4 — __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE6resizeEmS2_
pub fn stub_84c1d4() {
    // IDA 0x84c1d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::push_back(RBX::GameBasicSettings::ControlMode const&)")]
// 0x84c208 — __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE9push_backERKS2_
pub fn stub_84c208() {
    // IDA 0x84c208: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::ControlMode*,std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>>,RBX::GameBasicSettings::ControlMode const&)")]
// 0x84c230 — __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_84c230() {
    // IDA 0x84c230: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::_M_allocate(unsigned long)")]
// 0x84c314 — __ZNSt12_Vector_baseIN3RBX17GameBasicSettings11ControlModeESaIS2_EE11_M_allocateEm
pub fn stub_84c314() {
    // IDA 0x84c314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
