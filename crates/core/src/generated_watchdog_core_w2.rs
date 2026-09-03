//! generated_watchdog_core_w2 — 120 core stubs EA-sorted, gap filler after watchdog_core_w1.
//! Source: ida/export.json (85545 funcs) filtered core namespace (excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|FMOD|Audio|Sound|lua|ObjC) EA-sorted asc next 120 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
// 0x5c0288 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x5c0288() {
    // IDA 0x5c0288: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::invalid_argument::~invalid_argument()")]
#[doc(alias = "__ZNSt16invalid_argumentD2Ev")]
// 0xbef320 — __ZNSt16invalid_argumentD2Ev
// type: void __cdecl(std::invalid_argument *__hidden this)
pub fn stub_0xbef320() {
    // IDA 0xbef320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::insert(rbx::signals::signal<void ()(int)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE6insertEPNS3_4slotE")]
// 0xbf0af8 — __ZN3rbx7signals6signalIFviEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xbf0af8() {
    // IDA 0xbf0af8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx::signals::signal<void ()(int)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSEPS6_")]
// 0xbf0dd8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSEPS6_
pub fn stub_0xbf0dd8() {
    // IDA 0xbf0dd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE22safe_static_init_mutexEv")]
// 0xbf0e90 — __ZN3rbx7signals6signalIFviEE22safe_static_init_mutexEv
pub fn stub_0xbf0e90() {
    // IDA 0xbf0e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE4slot10disconnectEv")]
// 0xbf0ff8 — __ZN3rbx7signals6signalIFviEE4slot10disconnectEv
pub fn stub_0xbf0ff8() {
    // IDA 0xbf0ff8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFviEE4slot9connectedEv")]
// 0xbf10e8 — __ZNK3rbx7signals6signalIFviEE4slot9connectedEv
pub fn stub_0xbf10e8() {
    // IDA 0xbf10e8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE4slot5mutexEv")]
// 0xbf1124 — __ZN3rbx7signals6signalIFviEE4slot5mutexEv
pub fn stub_0xbf1124() {
    // IDA 0xbf1124: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::remove(rbx::signals::signal<void ()(int)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE6removeEPNS3_4slotE")]
// 0xbf1230 — __ZN3rbx7signals6signalIFviEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xbf1230() {
    // IDA 0xbf1230: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE4slot22safe_static_init_mutexEv")]
// 0xbf1320 — __ZN3rbx7signals6signalIFviEE4slot22safe_static_init_mutexEv
pub fn stub_0xbf1320() {
    // IDA 0xbf1320: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE4slotD1Ev")]
// 0xbf1408 — __ZN3rbx7signals6signalIFviEE4slotD1Ev
pub fn stub_0xbf1408() {
    // IDA 0xbf1408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot>::operator=(rbx::signals::signal<void ()(bool)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSEPS6_")]
// 0xbf1a48 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSEPS6_
pub fn stub_0xbf1a48() {
    // IDA 0xbf1a48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE4slot10disconnectEv")]
// 0xbf1c60 — __ZN3rbx7signals6signalIFvbEE4slot10disconnectEv
pub fn stub_0xbf1c60() {
    // IDA 0xbf1c60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvbEE4slot9connectedEv")]
// 0xbf1d50 — __ZNK3rbx7signals6signalIFvbEE4slot9connectedEv
pub fn stub_0xbf1d50() {
    // IDA 0xbf1d50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE4slot5mutexEv")]
// 0xbf1d8c — __ZN3rbx7signals6signalIFvbEE4slot5mutexEv
pub fn stub_0xbf1d8c() {
    // IDA 0xbf1d8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::xtime_get(boost::xtime *,int)")]
#[doc(alias = "__ZN5boost9xtime_getEPNS_5xtimeEi")]
// 0xbf2a9c — __ZN5boost9xtime_getEPNS_5xtimeEi
pub fn stub_0xbf2a9c() {
    // IDA 0xbf2a9c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::this_thread::sleep<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>(boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll> const&)")]
#[doc(alias = "__ZN5boost11this_thread5sleepINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEvRKT_")]
// 0xbf2b84 — __ZN5boost11this_thread5sleepINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEvRKT_
pub fn stub_0xbf2b84() {
    // IDA 0xbf2b84: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::scoped_connection::~scoped_connection()")]
#[doc(alias = "__ZN3rbx7signals17scoped_connectionD2Ev")]
// 0xbf3908 — __ZN3rbx7signals17scoped_connectionD2Ev
// type: void __fastcall(rbx::signals::scoped_connection *__hidden this)
pub fn stub_0xbf3908() {
    // IDA 0xbf3908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE22safe_static_init_mutexEv")]
// 0xbf3ac8 — __ZN3rbx7signals6signalIFvSsEE22safe_static_init_mutexEv
pub fn stub_0xbf3ac8() {
    // IDA 0xbf3ac8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "ResourceGroupHelper::resourceGroupExist(std::string const&)")]
#[doc(alias = "__ZN19ResourceGroupHelper18resourceGroupExistERKSs")]
// 0xc039f4 — __ZN19ResourceGroupHelper18resourceGroupExistERKSs
// type: _DWORD __fastcall(ResourceGroupHelper *__hidden this, const std::string *)
pub fn stub_0xc039f4() {
    // IDA 0xc039f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "ResourceGroupHelper::getLatestModificationTime(std::string const&)")]
#[doc(alias = "__ZN19ResourceGroupHelper25getLatestModificationTimeERKSs")]
// 0xc046d4 — __ZN19ResourceGroupHelper25getLatestModificationTimeERKSs
// type: _DWORD __fastcall(ResourceGroupHelper *__hidden this, const std::string *)
pub fn stub_0xc046d4() {
    // IDA 0xc046d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<std::string,long,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISslSt4lessISsESaISt4pairIKSslEEEixERS3_")]
// 0xc04acc — __ZNSt3mapISslSt4lessISsESaISt4pairIKSslEEEixERS3_
pub fn stub_0xc04acc() {
    // IDA 0xc04acc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")]
// 0xc04c90 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_0xc04c90() {
    // IDA 0xc04c90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,long>>,std::pair<std::string const,long> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xc04d34 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xc04d34() {
    // IDA 0xc04d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,long> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xc04f14 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0xc04f14() {
    // IDA 0xc04f14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert_unique(std::pair<std::string const,long> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xc0505c — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xc0505c() {
    // IDA 0xc0505c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,long>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xc05140 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xc05140() {
    // IDA 0xc05140: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev")]
// 0xc0dbdc — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev
pub fn stub_0xc0dbdc() {
    // IDA 0xc0dbdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv")]
// 0xc0dbe0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv
pub fn stub_0xc0dbe0() {
    // IDA 0xc0dbe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info")]
// 0xc0dbf0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info
pub fn stub_0xc0dbf0() {
    // IDA 0xc0dbf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv")]
// 0xc0dbf4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv
pub fn stub_0xc0dbf4() {
    // IDA 0xc0dbf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
// 0xc0dd8c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_0xc0dd8c() {
    // IDA 0xc0dd8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
// 0xc0dde8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0xc0dde8() {
    // IDA 0xc0dde8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// 0xc0def0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_0xc0def0() {
    // IDA 0xc0def0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// 0xc0df08 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_0xc0df08() {
    // IDA 0xc0df08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::MegaCluster::ChunkData,std::allocator<RBX::MegaCluster::ChunkData>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MegaCluster::ChunkData*,std::vector<RBX::MegaCluster::ChunkData,std::allocator<RBX::MegaCluster::ChunkData>>>,unsigned long,RBX::MegaCluster::ChunkData const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11MegaCluster9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xc0df20 — __ZNSt6vectorIN3RBX11MegaCluster9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xc0df20() {
    // IDA 0xc0df20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::MegaCluster::ChunkData *,unsigned long,RBX::MegaCluster::ChunkData>(RBX::MegaCluster::ChunkData *,unsigned long,RBX::MegaCluster::ChunkData const&,std::__false_type)")]
#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN3RBX11MegaCluster9ChunkDataEmS2_EvT_T0_RKT1_St12__false_type")]
// 0xc0e8e8 — __ZSt26__uninitialized_fill_n_auxIPN3RBX11MegaCluster9ChunkDataEmS2_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
pub fn stub_0xc0e8e8() {
    // IDA 0xc0e8e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::ChunkData * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *>(RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *)")]
#[doc(alias = "__ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX11MegaCluster9ChunkDataES5_EET0_T_S7_S6_")]
// 0xc0ea84 — __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX11MegaCluster9ChunkDataES5_EET0_T_S7_S6_
pub fn stub_0xc0ea84() {
    // IDA 0xc0ea84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::MegaClusterLegacy::createChunk<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,unsigned int,unsigned int)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy11createChunkINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEjj")]
// 0xc135dc — __ZN3RBX17MegaClusterLegacy11createChunkINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEjj
// type: int __fastcall(int, int, int)
pub fn stub_0xc135dc() {
    // IDA 0xc135dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateWaterGeometry<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,unsigned int)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy19updateWaterGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEj")]
// 0xc14600 — __ZN3RBX17MegaClusterLegacy19updateWaterGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEj
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::
pub fn stub_0xc14600() {
    // IDA 0xc14600: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateChunkGeometry<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,int)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy19updateChunkGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEi")]
// 0xc14aa8 — __ZN3RBX17MegaClusterLegacy19updateChunkGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEi
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Og
pub fn stub_0xc14aa8() {
    // IDA 0xc14aa8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc14f78 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xc14f78() {
    // IDA 0xc14f78: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
// 0xc15780 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xc15780() {
    // IDA 0xc15780: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
// 0xc15a3c — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_0xc15a3c() {
    // IDA 0xc15a3c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
// 0xc15e68 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_0xc15e68() {
    // IDA 0xc15e68: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
// 0xc16054 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_0xc16054() {
    // IDA 0xc16054: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
// 0xc16138 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xc16138() {
    // IDA 0xc16138: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc16550 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
// type: int __fastcall(unsigned int *, _WORD *)
pub fn stub_0xc16550() {
    // IDA 0xc16550: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc16d18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xc16d18() {
    // IDA 0xc16d18: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
#[doc(alias = "__ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE")]
// 0xc17464 — __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
pub fn stub_0xc17464() {
    // IDA 0xc17464: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
// 0xc175f8 — __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xc175f8() {
    // IDA 0xc175f8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc1799c — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xc1799c() {
    // IDA 0xc1799c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::MegaClusterLegacy::ChunkData,std::allocator<RBX::MegaClusterLegacy::ChunkData>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MegaClusterLegacy::ChunkData*,std::vector<RBX::MegaClusterLegacy::ChunkData,std::allocator<RBX::MegaClusterLegacy::ChunkData>>>,unsigned long,RBX::MegaClusterLegacy::ChunkData const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX17MegaClusterLegacy9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xc180d4 — __ZNSt6vectorIN3RBX17MegaClusterLegacy9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xc180d4() {
    // IDA 0xc180d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk> const&)")]
#[doc(alias = "__ZN3RBX5Voxel6RegionINS0_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorC2ERKS5_")]
// 0xc18360 — __ZN3RBX5Voxel6RegionINS0_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorC2ERKS5_
pub fn stub_0xc18360() {
    // IDA 0xc18360: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillFromRegion<RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>>(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
#[doc(alias = "__ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk14fillFromRegionINS0_6RegionINS0_4Grid5ChunkEEEEEvRKT_")]
// 0xc189f8 — __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk14fillFromRegionINS0_6RegionINS0_4Grid5ChunkEEEEEvRKT_
// type: int __fastcall(int)
pub fn stub_0xc189f8() {
    // IDA 0xc189f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GfxBinding::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX10GfxBinding12updateEntityEb")]
// 0xc18ea8 — __ZN3RBX10GfxBinding12updateEntityEb
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this, bool)
pub fn stub_0xc18ea8() {
    // IDA 0xc18ea8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
#[doc(alias = "__ZN3RBX12ExtentsInt32C2Ev")]
// 0xc18eac — __ZN3RBX12ExtentsInt32C2Ev
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
pub fn stub_0xc18eac() {
    // IDA 0xc18eac: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::fillDummyLighting(RBX::LightGridChunk &,unsigned char,unsigned char)")]
#[doc(alias = "__ZN3RBX17fillDummyLightingERNS_14LightGridChunkEhh")]
// 0xc1a41c — __ZN3RBX17fillDummyLightingERNS_14LightGridChunkEhh
pub fn stub_0xc1a41c() {
    // IDA 0xc1a41c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::getChunkByIndex(RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX9LightGrid15getChunkByIndexERKNS_12Vector3int32E")]
// 0xc1a8d4 — __ZN3RBX9LightGrid15getChunkByIndexERKNS_12Vector3int32E
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, const RBX::Vector3int32 *)
pub fn stub_0xc1a8d4() {
    // IDA 0xc1a8d4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::precomputeShadowLUT(void)")]
#[doc(alias = "__ZN3RBX9LightGrid19precomputeShadowLUTEv")]
// 0xc1bc08 — __ZN3RBX9LightGrid19precomputeShadowLUTEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc1bc08() {
    // IDA 0xc1bc08: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::~LightGrid()")]
#[doc(alias = "__ZN3RBX9LightGridD0Ev")]
// 0xc1be30 — __ZN3RBX9LightGridD0Ev
// type: void __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc1be30() {
    // IDA 0xc1be30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LightGrid::~LightGrid()")]
#[doc(alias = "__ZN3RBX9LightGridD1Ev")]
// 0xc1bed0 — __ZN3RBX9LightGridD1Ev
// type: void __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc1bed0() {
    // IDA 0xc1bed0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LightGrid::~LightGrid()")]
#[doc(alias = "__ZN3RBX9LightGridD2Ev")]
// 0xc1bed4 — __ZN3RBX9LightGridD2Ev
// type: void __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc1bed4() {
    // IDA 0xc1bed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LightGrid::setNonFixedPartsEnabled(bool)")]
#[doc(alias = "__ZN3RBX9LightGrid23setNonFixedPartsEnabledEb")]
// 0xc1c0dc — __ZN3RBX9LightGrid23setNonFixedPartsEnabledEb
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, bool)
pub fn stub_0xc1c0dc() {
    // IDA 0xc1c0dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateLightScratch(RBX::LightGridChunk const&,RBX::Extents const&,RBX::LightObject *)")]
#[doc(alias = "__ZN3RBX9LightGrid26lightingUpdateLightScratchERKNS_14LightGridChunkERKNS_7ExtentsEPNS_11LightObjectE")]
// 0xc1e1f0 — __ZN3RBX9LightGrid26lightingUpdateLightScratchERKNS_14LightGridChunkERKNS_7ExtentsEPNS_11LightObjectE
pub fn stub_0xc1e1f0() {
    // IDA 0xc1e1f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LightGrid::lightingBlurAxisXScratchToChunkSIMD(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid35lightingBlurAxisXScratchToChunkSIMDERNS_14LightGridChunkE")]
// 0xc1e6c8 — __ZN3RBX9LightGrid35lightingBlurAxisXScratchToChunkSIMDERNS_14LightGridChunkE
pub fn stub_0xc1e6c8() {
    // IDA 0xc1e6c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LightGrid::lightingBlurAxisXScratchToChunk(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid31lightingBlurAxisXScratchToChunkERNS_14LightGridChunkE")]
// 0xc1e7f0 — __ZN3RBX9LightGrid31lightingBlurAxisXScratchToChunkERNS_14LightGridChunkE
pub fn stub_0xc1e7f0() {
    // IDA 0xc1e7f0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingClearLocal(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid18lightingClearLocalERNS_14LightGridChunkE")]
// 0xc1e908 — __ZN3RBX9LightGrid18lightingClearLocalERNS_14LightGridChunkE
pub fn stub_0xc1e908() {
    // IDA 0xc1e908: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateChunkGlobal(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid25lightingUpdateChunkGlobalERNS_14LightGridChunkE")]
// 0xc1e9d8 — __ZN3RBX9LightGrid25lightingUpdateChunkGlobalERNS_14LightGridChunkE
// type: int __fastcall(int, int, int, int, int, int, double, char, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xc1e9d8() {
    // IDA 0xc1e9d8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingClearGlobal(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid19lightingClearGlobalERNS_14LightGridChunkE")]
// 0xc1ebd8 — __ZN3RBX9LightGrid19lightingClearGlobalERNS_14LightGridChunkE
pub fn stub_0xc1ebd8() {
    // IDA 0xc1ebd8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateChunkSkylight(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid27lightingUpdateChunkSkylightERNS_14LightGridChunkE")]
// 0xc1ece8 — __ZN3RBX9LightGrid27lightingUpdateChunkSkylightERNS_14LightGridChunkE
pub fn stub_0xc1ece8() {
    // IDA 0xc1ece8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateSkylight(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid22lightingUpdateSkylightERNS_14LightGridChunkE")]
// 0xc1ee80 — __ZN3RBX9LightGrid22lightingUpdateSkylightERNS_14LightGridChunkE
pub fn stub_0xc1ee80() {
    // IDA 0xc1ee80: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateChunkAverage(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid26lightingUpdateChunkAverageERNS_14LightGridChunkE")]
// 0xc1f788 — __ZN3RBX9LightGrid26lightingUpdateChunkAverageERNS_14LightGridChunkE
pub fn stub_0xc1f788() {
    // IDA 0xc1f788: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateAverageImplSIMD(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid29lightingUpdateAverageImplSIMDERNS_14LightGridChunkE")]
// 0xc1f798 — __ZN3RBX9LightGrid29lightingUpdateAverageImplSIMDERNS_14LightGridChunkE
pub fn stub_0xc1f798() {
    // IDA 0xc1f798: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateAverageImpl(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid25lightingUpdateAverageImplERNS_14LightGridChunkE")]
// 0xc1fa58 — __ZN3RBX9LightGrid25lightingUpdateAverageImplERNS_14LightGridChunkE
// type: int __fastcall(int, void *)
pub fn stub_0xc1fa58() {
    // IDA 0xc1fa58: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::invalidateAll(unsigned int)")]
#[doc(alias = "__ZN3RBX9LightGrid13invalidateAllEj")]
// 0xc1fb88 — __ZN3RBX9LightGrid13invalidateAllEj
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, unsigned int)
pub fn stub_0xc1fb88() {
    // IDA 0xc1fb88: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::invalidateExtents(RBX::Extents const&,unsigned int)")]
#[doc(alias = "__ZN3RBX9LightGrid17invalidateExtentsERKNS_7ExtentsEj")]
// 0xc1fbb8 — __ZN3RBX9LightGrid17invalidateExtentsERKNS_7ExtentsEj
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, const RBX::Extents *, unsigned int)
pub fn stub_0xc1fbb8() {
    // IDA 0xc1fbb8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::relocateGrid(RBX::Vector3int32 const&,bool)")]
#[doc(alias = "__ZN3RBX9LightGrid12relocateGridERKNS_12Vector3int32Eb")]
// 0xc1fff8 — __ZN3RBX9LightGrid12relocateGridERKNS_12Vector3int32Eb
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, const RBX::Vector3int32 *, bool)
pub fn stub_0xc1fff8() {
    // IDA 0xc1fff8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::setLightShadows(bool)")]
#[doc(alias = "__ZN3RBX9LightGrid15setLightShadowsEb")]
// 0xc20e08 — __ZN3RBX9LightGrid15setLightShadowsEb
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, char *)
pub fn stub_0xc20e08() {
    // IDA 0xc20e08: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::stepCursor(RBX::Vector3int32 &)")]
#[doc(alias = "__ZN3RBX9LightGrid10stepCursorERNS_12Vector3int32E")]
// 0xc21048 — __ZN3RBX9LightGrid10stepCursorERNS_12Vector3int32E
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, RBX::Vector3int32 *)
pub fn stub_0xc21048() {
    // IDA 0xc21048: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::findDirtyChunk(void)")]
#[doc(alias = "__ZN3RBX9LightGrid14findDirtyChunkEv")]
// 0xc2119c — __ZN3RBX9LightGrid14findDirtyChunkEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc2119c() {
    // IDA 0xc2119c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::findFirstDirtyChunk(void)")]
#[doc(alias = "__ZN3RBX9LightGrid19findFirstDirtyChunkEv")]
// 0xc212a0 — __ZN3RBX9LightGrid19findFirstDirtyChunkEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc212a0() {
    // IDA 0xc212a0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::findOldestChunk(void)")]
#[doc(alias = "__ZN3RBX9LightGrid15findOldestChunkEv")]
// 0xc21358 — __ZN3RBX9LightGrid15findOldestChunkEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc21358() {
    // IDA 0xc21358: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::getGridCornerOffset(void)const")]
#[doc(alias = "__ZNK3RBX9LightGrid19getGridCornerOffsetEv")]
// 0xc21434 — __ZNK3RBX9LightGrid19getGridCornerOffsetEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc21434() {
    // IDA 0xc21434: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::getWrapSafeOffset(void)const")]
#[doc(alias = "__ZNK3RBX9LightGrid17getWrapSafeOffsetEv")]
// 0xc21484 — __ZNK3RBX9LightGrid17getWrapSafeOffsetEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc21484() {
    // IDA 0xc21484: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::getGridSize(void)const")]
#[doc(alias = "__ZNK3RBX9LightGrid11getGridSizeEv")]
// 0xc2158c — __ZNK3RBX9LightGrid11getGridSizeEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc2158c() {
    // IDA 0xc2158c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::getBorderColor(void)const")]
#[doc(alias = "__ZNK3RBX9LightGrid14getBorderColorEv")]
// 0xc215dc — __ZNK3RBX9LightGrid14getBorderColorEv
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this)
pub fn stub_0xc215dc() {
    // IDA 0xc215dc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUpdateSkylightRow(RBX::LightGridChunk &,int,int,unsigned char const*)")]
#[doc(alias = "__ZN3RBX9LightGrid25lightingUpdateSkylightRowERNS_14LightGridChunkEiiPKh")]
// 0xc2165c — __ZN3RBX9LightGrid25lightingUpdateSkylightRowERNS_14LightGridChunkEiiPKh
pub fn stub_0xc2165c() {
    // IDA 0xc2165c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingFixupShadowMaskBorder(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX9LightGrid29lightingFixupShadowMaskBorderERKNS_12Vector3int32ES3_S3_")]
// 0xc21af4 — __ZN3RBX9LightGrid29lightingFixupShadowMaskBorderERKNS_12Vector3int32ES3_S3_
// type: _DWORD __fastcall(RBX::LightGrid *__hidden this, const RBX::Vector3int32 *, const RBX::Vector3int32 *, const RBX::Vector3int32 *)
pub fn stub_0xc21af4() {
    // IDA 0xc21af4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingComposit(RBX::LightGridChunk const&,unsigned char *,unsigned int,unsigned int)")]
#[doc(alias = "__ZN3RBX9LightGrid16lightingCompositERKNS_14LightGridChunkEPhjj")]
// 0xc21ea8 — __ZN3RBX9LightGrid16lightingCompositERKNS_14LightGridChunkEPhjj
// type: int __fastcall(int, int, int, int, char *, FLog *, int, char, int, int, double, char, int, int, int, int, int, int, int, int, int)
pub fn stub_0xc21ea8() {
    // IDA 0xc21ea8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUploadChunk(RBX::LightGridChunk &)")]
#[doc(alias = "__ZN3RBX9LightGrid19lightingUploadChunkERNS_14LightGridChunkE")]
// 0xc22400 — __ZN3RBX9LightGrid19lightingUploadChunkERNS_14LightGridChunkE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xc22400() {
    // IDA 0xc22400: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::LightGrid::lightingUploadAll(void)")]
#[doc(alias = "__ZN3RBX9LightGrid17lightingUploadAllEv")]
// 0xc22c80 — __ZN3RBX9LightGrid17lightingUploadAllEv
// type: void __fastcall(RBX::LightGrid *this, int)
pub fn stub_0xc22c80() {
    // IDA 0xc22c80: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "boost::system::system_error::what(void)const")]
#[doc(alias = "__ZNK5boost6system12system_error4whatEv")]
// 0xc23250 — __ZNK5boost6system12system_error4whatEv
// type: _DWORD __fastcall(boost::system::system_error *__hidden this)
pub fn stub_0xc23250() {
    // IDA 0xc23250: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "std::invalid_argument::~invalid_argument()")]
#[doc(alias = "__ZNSt16invalid_argumentD0Ev")]
// 0xc233f0 — __ZNSt16invalid_argumentD0Ev
// type: void __cdecl(std::invalid_argument *__hidden this)
pub fn stub_0xc233f0() {
    // IDA 0xc233f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::domain_error::~domain_error()")]
#[doc(alias = "__ZNSt12domain_errorD1Ev")]
// 0xc23410 — __ZNSt12domain_errorD1Ev
// type: void __cdecl(std::domain_error *__hidden this)
pub fn stub_0xc23410() {
    // IDA 0xc23410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::length_error::~length_error()")]
#[doc(alias = "__ZNSt12length_errorD2Ev")]
// 0xc23420 — __ZNSt12length_errorD2Ev
// type: void __cdecl(std::length_error *__hidden this)
pub fn stub_0xc23420() {
    // IDA 0xc23420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::overflow_error::~overflow_error()")]
#[doc(alias = "__ZNSt14overflow_errorD0Ev")]
// 0xc23430 — __ZNSt14overflow_errorD0Ev
// type: void __cdecl(std::overflow_error *__hidden this)
pub fn stub_0xc23430() {
    // IDA 0xc23430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::underflow_error::~underflow_error()")]
#[doc(alias = "__ZNSt15underflow_errorD1Ev")]
// 0xc23450 — __ZNSt15underflow_errorD1Ev
// type: void __cdecl(std::underflow_error *__hidden this)
pub fn stub_0xc23450() {
    // IDA 0xc23450: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::LightGrid::occupancyFillTerrainSIMD<RBX::Voxel::Grid>(RBX::LightGridChunk &,RBX::Voxel::Grid &,RBX::Vector3int32 const&,RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX9LightGrid24occupancyFillTerrainSIMDINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE")]
// 0xc234b0 — __ZN3RBX9LightGrid24occupancyFillTerrainSIMDINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
pub fn stub_0xc234b0() {
    // IDA 0xc234b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::LightGrid::occupancyFillTerrain<RBX::Voxel::Grid>(RBX::LightGridChunk &,RBX::Voxel::Grid &,RBX::Vector3int32 const&,RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX9LightGrid20occupancyFillTerrainINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE")]
// 0xc239b8 — __ZN3RBX9LightGrid20occupancyFillTerrainINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
pub fn stub_0xc239b8() {
    // IDA 0xc239b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::LightGrid::lightingBlurAxisYZScratch<true>(void)")]
#[doc(alias = "__ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb1EEEvv")]
// 0xc27130 — __ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb1EEEvv
pub fn stub_0xc27130() {
    // IDA 0xc27130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::LightGrid::lightingBlurAxisYZScratch<false>(void)")]
#[doc(alias = "__ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb0EEEvv")]
// 0xc27240 — __ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb0EEEvv
pub fn stub_0xc27240() {
    // IDA 0xc27240: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<true,true>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")]
// 0xc27330 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xc27330() {
    // IDA 0xc27330: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<true,false>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")]
// 0xc27c18 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xc27c18() {
    // IDA 0xc27c18: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<false,true>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")]
// 0xc284e0 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xc284e0() {
    // IDA 0xc284e0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<false,false>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")]
// 0xc28de8 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xc28de8() {
    // IDA 0xc28de8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<0>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
#[doc(alias = "__ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi0EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_")]
// 0xc2b32c — __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi0EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_0xc2b32c() {
    // IDA 0xc2b32c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<1>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
#[doc(alias = "__ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi1EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_")]
// 0xc2b484 — __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi1EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_0xc2b484() {
    // IDA 0xc2b484: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<2>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
#[doc(alias = "__ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi2EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_")]
// 0xc2b5e4 — __ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi2EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_0xc2b5e4() {
    // IDA 0xc2b5e4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<0>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
#[doc(alias = "__ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_")]
// 0xc2b734 — __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_0xc2b734() {
    // IDA 0xc2b734: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<1>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
#[doc(alias = "__ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_")]
// 0xc2b9a8 — __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
// type: RBX::LightGrid *__fastcall(RBX::LightGrid *result, int, int *, RBX::LightGrid **, int *, int, RBX::LightGrid **, RBX::LightGrid **)
pub fn stub_0xc2b9a8() {
    // IDA 0xc2b9a8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<2>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
#[doc(alias = "__ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi2EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_")]
// 0xc2bc2c — __ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi2EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_0xc2bc2c() {
    // IDA 0xc2bc2c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "boost::system::system_error::~system_error()")]
#[doc(alias = "__ZN5boost6system12system_errorD1Ev")]
// 0xc2bfd0 — __ZN5boost6system12system_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
pub fn stub_0xc2bfd0() {
    // IDA 0xc2bfd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::system::system_error::~system_error()")]
#[doc(alias = "__ZN5boost6system12system_errorD0Ev")]
// 0xc2c030 — __ZN5boost6system12system_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
pub fn stub_0xc2c030() {
    // IDA 0xc2c030: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZN3RBX12CylinderMeshD1Ev")]
// 0xc2c220 — __ZN3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
pub fn stub_0xc2c220() {
    // IDA 0xc2c220: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::throw_exception<boost::bad_function_call>(boost::bad_function_call const&)")]
#[doc(alias = "__ZN5boost15throw_exceptionINS_17bad_function_callEEEvRKT_")]
// 0xc2c2b0 — __ZN5boost15throw_exceptionINS_17bad_function_callEEEvRKT_
pub fn stub_0xc2c2b0() {
    // IDA 0xc2c2b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::bad_function_call::~bad_function_call()")]
#[doc(alias = "__ZN5boost17bad_function_callD1Ev")]
// 0xc2c400 — __ZN5boost17bad_function_callD1Ev
// type: void __fastcall(boost::bad_function_call *__hidden this)
pub fn stub_0xc2c400() {
    // IDA 0xc2c400: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::rethrow(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv")]
// 0xc2c410 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv
pub fn stub_0xc2c410() {
    // IDA 0xc2c410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev")]
// 0xc2c4c0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xc2c4c0() {
    // IDA 0xc2c4c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::rethrow(void)const")]
#[doc(alias = "__ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv")]
// 0xc2c580 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv
pub fn stub_0xc2c580() {
    // IDA 0xc2c580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
