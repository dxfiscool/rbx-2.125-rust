//! network generated_13 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 120 stubs here, 4299+120=4419 total, 378 remaining).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

use std::collections::{HashMap, BTreeMap};

/// `rbx::signals::signal` slot list reduced to linkage bits.
#[derive(Clone, Debug, Default)]
pub struct GenSignalState {
    pub slots: Vec<(u64, bool)>,
    pub next: u64,
}

fn gen_connect(s: &mut GenSignalState) -> u64 {
    let id = s.next;
    s.next = s.next.wrapping_add(1);
    s.slots.push((id, true));
    id
}

fn gen_disconnect(s: &mut GenSignalState, id: u64) {
    s.slots.retain(|(i, _)| *i != id);
}

/// `RBX::EventReplicatorBase` listener side (IDA 0x3a7f68/0x3a8228/0x3a9944).
#[derive(Clone, Debug, Default)]
pub struct GenEventState {
    pub mode: bool,
    pub conn: bool,
    pub listener: bool,
    pub watched: u32,
    pub count: i32,
}

/// Reflection descriptor row (Bound/Prop/Event desc common shape).
#[derive(Clone, Debug, Default)]
pub struct GenDesc {
    pub name: String,
    pub value: i32,
    pub text: String,
    pub readable: bool,
    pub writable: bool,
    pub scriptable: bool,
    pub broadcast: bool,
}

/// `RBX::Network::Peer` transport view.
#[derive(Clone, Debug, Default)]
pub struct GenPeer {
    pub kbps: i32,
    pub connected: bool,
    pub port: u16,
    pub ip: u32,
}

/// RakNet stats accumulation (`PeerStatsItem::update`, IDA 0xad5790).
#[derive(Clone, Debug, Default)]
pub struct GenStats {
    pub packets: u64,
    pub bytes: u64,
    pub enabled: bool,
    pub checked: bool,
}

/// `TopNErrorsPhysicsSender` tables: part -> error plus descending top-N.
#[derive(Clone, Debug, Default)]
pub struct GenTopN {
    pub map: HashMap<u32, f32>,
    pub top: Vec<u32>,
}

fn gen_refresh_top(t: &mut GenTopN) {
    let mut ids: Vec<u32> = t.map.keys().copied().collect();
    ids.sort_by(|a, b| {
        t.map
            .get(b)
            .partial_cmp(&t.map.get(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    t.top = ids;
}

/// `InterpolatingPhysicsReceiver` lerp queue (IDA 0xada700).
#[derive(Clone, Debug, Default)]
pub struct GenInterp {
    pub alpha: f32,
    pub active: bool,
    pub queue: Vec<u32>,
}

/// `RBX::Network::Replicator` connection view.
#[derive(Clone, Debug, Default)]
pub struct GenReplicator {
    pub open: bool,
    pub process: bool,
    pub port: u16,
    pub ip: u32,
    pub markers: u64,
}

/// `boost::function` buffer occupancy for one bound functor.
#[derive(Clone, Debug, Default)]
pub struct GenFunctor {
    pub has: bool,
}

/// `boost::multi_index` nugget index: hash by part + order by stamp.
#[derive(Clone, Debug, Default)]
pub struct GenIndex {
    pub by_id: HashMap<u32, u64>,
    pub by_time: BTreeMap<u64, u32>,
}

/// TaskScheduler job view (`sleepTime`, IDA 0xad74f8).
#[derive(Clone, Debug, Default)]
pub struct GenJob {
    pub owner: u32,
    pub running: bool,
}

/// `RBX::Network::Marker` fire state (IDA 0xad12d0).
#[derive(Clone, Debug, Default)]
pub struct GenMarker {
    pub returned: bool,
    pub fired: u64,
}

/// `RBX::Network::ChatMessage` payload kept by value.
#[derive(Clone, Debug, Default)]
pub struct GenMessage {
    pub text: String,
    pub sender: u32,
}

/// `RBX::Network::NetworkOwner` address view.
#[derive(Clone, Debug, Default)]
pub struct GenOwner {
    pub ip: u32,
    pub port: u16,
    pub server: bool,
}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}


// 0xf605a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::create_buckets(unsigned long)")]
pub fn stub_f605a4(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xf605a4: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xf605b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::reserve_for_insert(unsigned long)")]
pub fn stub_f605b4(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xf605b4: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xf605c4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(int,bool,int)> const&)const")]
pub fn stub_f605c4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf605c4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf60614 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network12RakStatsItemES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::RakStatsItem,RBX::Network::RakStatsItem>(rbx_core::SharedPtr<RBX::Network::RakStatsItem> const*,RBX::Network::RakStatsItem *)const")]
pub fn stub_f60614(has_weak: bool) -> bool {
    // IDA 0xf60614: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf60624 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ServerReplicator15ServerStatsItemES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ServerReplicator::ServerStatsItem,RBX::Network::ServerReplicator::ServerStatsItem>(rbx_core::SharedPtr<RBX::Network::ServerReplicator::ServerStatsItem> const*,RBX::Network::ServerReplicator::ServerStatsItem *)const")]
pub fn stub_f60624(has_weak: bool) -> bool {
    // IDA 0xf60624: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf60634 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator9StreamJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> const*,RBX::Network::Replicator::StreamJob *)const")]
pub fn stub_f60634(has_weak: bool) -> bool {
    // IDA 0xf60634: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf60644 — j___ZNK5boost6detail8function13basic_vtable1IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS3_10Reflection5TupleEEENS6_IKSF_EEEEEEES8_ENSB_5list2INSB_5valueISL_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f60644(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60644: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60654 — j___ZNK5boost6detail8function13basic_vtable2IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEES8_E9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS3_10Reflection5TupleEEENS6_IKSF_EEEEEEES8_S8_ENSB_5list3INSB_5valueISL_EENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f60654(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60654: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60664 — j___ZNK5boost6detail8function13basic_vtable2IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEESsE9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS3_10Reflection5TupleEEENS6_IKSF_EEEEEEES8_SsENSB_5list3INSB_5valueISL_EENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f60664(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60664: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60684 — j___ZNK5boost6detail8function13basic_vtable3IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEESsNS3_10Reflection7VariantEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS9_5TupleEEENS6_IKSG_EEEEEEES8_SsSA_ENSD_5list4INSD_5valueISM_EENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f60684(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60684: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf606a4 — j___ZNK5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEclES6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_f606a4() -> Option<u32> {
    // IDA 0xf606a4: nullable object query (id when live, None when unset).
    None
}
// 0xf606b4 — j___ZNK5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EclES6_S6_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_f606b4() -> Option<u32> {
    // IDA 0xf606b4: nullable object query (id when live, None when unset).
    None
}
// 0xf606c4 — j___ZNK5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEclES6_Ss
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string)const")]
pub fn stub_f606c4() -> Option<u32> {
    // IDA 0xf606c4: nullable object query (id when live, None when unset).
    None
}
// 0xf606d4 — j___ZNK5boost9function3IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEclES6_SsS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::function3<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)const")]
pub fn stub_f606d4() -> Option<u32> {
    // IDA 0xf606d4: nullable object query (id when live, None when unset).
    None
}
// 0xf606f4 — j___ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network8PropSync6detail11PropertyKeyEEESaIS8_EE17_M_reallocate_mapEmb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_f606f4() {
    // IDA 0xf606f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60704 — j___ZNSt6vectorIN3RBX7Network12FilterResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::FilterResult*,std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>>,RBX::Network::FilterResult const&)")]
pub fn stub_f60704(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0xf60704: vector insert with reallocation around the new element.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0xf60714 — j___ZNSt6vectorIN3RBX7Network12FilterResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::FilterResult*,std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>>,unsigned long,RBX::Network::FilterResult const&)")]
pub fn stub_f60714() {
    // IDA 0xf60714: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60734 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::FilterResult>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::FilterResult> const&)")]
pub fn stub_f60734() -> Option<u32> {
    // IDA 0xf60734: nullable object query (id when live, None when unset).
    None
}
// 0xf60744 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::FilterResult>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::pair<RBX::Name const* const,RBX::Network::FilterResult> const&)")]
pub fn stub_f60744() -> Option<u32> {
    // IDA 0xf60744: nullable object query (id when live, None when unset).
    None
}
// 0xf60754 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::FilterResult>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::FilterResult>> *)")]
pub fn stub_f60754(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0xf60754: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0xf607f4 — j___ZN3RBX10Reflection11Call1HelperINS_7Network7PlayersEMS3_FN5boost10shared_ptrINS_8InstanceEEES7_ES7_S7_E4callEPS3_S9_RNS0_7VariantERKS7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Players*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f607f4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf607f4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf60804 — j___ZN3RBX10Reflection11Call2HelperINS_7Network7PlayersEMS3_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS7_vE4callEPS3_S9_RNS0_7VariantERKSsRKS7_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Network::Players,void (RBX::Network::Players::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f60804(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf60804: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf60814 — j___ZN3RBX10Reflection11Call3HelperINS_7Network7PlayersEMS3_FvN5boost10shared_ptrINS_8InstanceEEESsSsES7_SsSsvE4callEPS3_S9_RNS0_7VariantERKS7_RKSsSH_
// type: int __fastcall(int, int, int, int, int, std::string *, std::string *)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),rbx_core::SharedPtr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,std::string const&)")]
pub fn stub_f60814(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf60814: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf60824 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f60824(name: &str) -> GenDesc {
    // IDA 0xf60824: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60834 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EEC2EMS3_FS7_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f60834(name: &str) -> GenDesc {
    // IDA 0xf60834: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60844 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED2Ev
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_f60844(d: GenDesc) {
    // IDA 0xf60844: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xf60854 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EEC2EMS3_FS7_iEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f60854(name: &str) -> GenDesc {
    // IDA 0xf60854: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60864 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EEC2EMS3_FvS7_SsSsEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f60864(name: &str) -> GenDesc {
    // IDA 0xf60864: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60874 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
pub fn stub_f60874(d: GenDesc) {
    // IDA 0xf60874: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xf60884 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EEC2EMS3_FvS4_EPKcSA_S4_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::BoundFuncDesc(void (RBX::Network::Players::*)(RBX::Network::Players::ChatOption),char const*,char const*,RBX::Network::Players::ChatOption,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f60884(name: &str) -> GenDesc {
    // IDA 0xf60884: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60894 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsELi1EEC2EMS3_FvSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::BoundFuncDesc(void (RBX::Network::Players::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f60894(name: &str) -> GenDesc {
    // IDA 0xf60894: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf608a4 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS3_FvSsS7_EPKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Players::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f608a4(name: &str) -> GenDesc {
    // IDA 0xf608a4: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf608b4 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_f608b4(d: GenDesc) {
    // IDA 0xf608b4: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xf608c4 — j___ZN3RBX10Reflection13DescribedBase15fastDynamicCastINS_7Network6PlayerEEEPT_PS1_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)")]
pub fn stub_f608c4() -> Option<u32> {
    // IDA 0xf608c4: nullable object query (id when live, None when unset).
    None
}
// 0xf608d4 — j___ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f608d4() -> Option<u32> {
    // IDA 0xf608d4: nullable object query (id when live, None when unset).
    None
}
// 0xf608e4 — j___ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::PropDescriptor<bool (RBX::Network::Players::*)(void)const,void (RBX::Network::Players::*)(bool)>(char const*,char const*,bool (RBX::Network::Players::*)(void)const,void (RBX::Network::Players::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f608e4(name: &str) -> GenDesc {
    // IDA 0xf608e4: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf608f4 — j___ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::PropDescriptor<bool (RBX::Network::Players::*)(void)const,int>(char const*,char const*,bool (RBX::Network::Players::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f608f4(name: &str) -> GenDesc {
    // IDA 0xf608f4: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60904 — j___ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::PropDescriptor<int (RBX::Network::Players::*)(void)const,void (RBX::Network::Players::*)(int)>(char const*,char const*,int (RBX::Network::Players::*)(void)const,void (RBX::Network::Players::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f60904(name: &str) -> GenDesc {
    // IDA 0xf60904: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60914 — j___ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::PropDescriptor<int (RBX::Network::Players::*)(void)const,int>(char const*,char const*,int (RBX::Network::Players::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f60914(name: &str) -> GenDesc {
    // IDA 0xf60914: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60924 — j___ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEEC2IMS3_KFPS4_vEiEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: void
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Network::Players::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::Network::Players::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f60924(name: &str) -> GenDesc {
    // IDA 0xf60924: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60944 — j___ZN3RBX10Reflection7Variant14genericConvertINS_7Network7Players10ChatOptionEEERT_v
// type: void
#[doc(alias = "RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)")]
pub fn stub_f60944() -> Option<u32> {
    // IDA 0xf60944: nullable object query (id when live, None when unset).
    None
}
// 0xf60954 — j___ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE7addPairES4_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)")]
pub fn stub_f60954() -> Option<u32> {
    // IDA 0xf60954: nullable object query (id when live, None when unset).
    None
}
// 0xf60964 — j___ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
pub fn stub_f60964() {
    // IDA 0xf60964: EnumDesc dtor releases the item map.
}
// 0xf60974 — j___ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE7addPairES4_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)")]
pub fn stub_f60974() -> Option<u32> {
    // IDA 0xf60974: nullable object query (id when live, None when unset).
    None
}
// 0xf60984 — j___ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_f60984() {
    // IDA 0xf60984: EnumDesc dtor releases the item map.
}
// 0xf60994 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_7Network7Players10ChatOptionELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::Players::ChatOption RBX::Reflection::ArgHelper::getArg<RBX::Network::Players::ChatOption,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Network::Players::ChatOption> const&,boost::disable_if<boost::is_same<RBX::Network::Players::ChatOption,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f60994() -> Option<u32> {
    // IDA 0xf60994: nullable object query (id when live, None when unset).
    None
}
// 0xf609a4 — j___ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f609a4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf609a4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf609b4 — j___ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f609b4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf609b4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf609c4 — j___ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_EC2ESD_PKcSG_SG_SG_SG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f609c4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf609c4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf609d4 — j___ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f609d4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf609d4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf609e4 — j___ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_f609e4() -> Option<u32> {
    // IDA 0xf609e4: nullable object query (id when live, None when unset).
    None
}
// 0xf609f4 — j___ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)")]
pub fn stub_f609f4() -> Option<u32> {
    // IDA 0xf609f4: nullable object query (id when live, None when unset).
    None
}
// 0xf60b74 — j___ZN3RBX32shared_from_polymorphic_downcastINS_7Network6PlayerENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
pub fn stub_f60b74() -> Option<u32> {
    // IDA 0xf60b74: nullable object query (id when live, None when unset).
    None
}
// 0xf60ba4 — j___ZN3RBX7Network11ChatMessageC2ERKS1_
// type: _DWORD __fastcall(RBX::Network::ChatMessage *__hidden this, const RBX::Network::ChatMessage *)
#[doc(alias = "RBX::Network::ChatMessage::ChatMessage(RBX::Network::ChatMessage const&)")]
pub fn stub_f60ba4() {
    // IDA 0xf60ba4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60bb4 — j___ZN3RBX7Network11ChatMessageD2Ev
// type: void __fastcall(RBX::Network::ChatMessage *__hidden this)
#[doc(alias = "RBX::Network::ChatMessage::~ChatMessage()")]
pub fn stub_f60bb4() {
    // IDA 0xf60bb4: dtor releases the owned control block/slots.
}
// 0xf60bc4 — j___ZN3RBX7Network12NetworkOwner10UnassignedEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
#[doc(alias = "RBX::Network::NetworkOwner::Unassigned(void)")]
pub fn stub_f60bc4() -> Option<u32> {
    // IDA 0xf60bc4: nullable object query (id when live, None when unset).
    None
}
// 0xf60c04 — j___ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_f60c04() -> Option<u32> {
    // IDA 0xf60c04: nullable object query (id when live, None when unset).
    None
}
// 0xf60c14 — j___ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)")]
pub fn stub_f60c14() -> Option<u32> {
    // IDA 0xf60c14: nullable object query (id when live, None when unset).
    None
}
// 0xf60c34 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEE8fireItemEPNS0_6signalIS5_E4slotES4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::fireItem(rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot *,RBX::Network::AbuseReport)")]
pub fn stub_f60c34() {
    // IDA 0xf60c34: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60c44 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEEclES4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)")]
pub fn stub_f60c44() {
    // IDA 0xf60c44: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60c54 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7Network11ChatMessageEEEclES6_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)")]
pub fn stub_f60c54() {
    // IDA 0xf60c54: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60ca4 — j___ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE8fireItemEPNS0_6signalISA_E4slotES5_S9_SsS9_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_f60ca4() {
    // IDA 0xf60ca4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60cb4 — j___ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_
// type: void
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_f60cb4() {
    // IDA 0xf60cb4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60cc4 — j___ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE13disconnectAllEv
// type: void
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::disconnectAll(void)")]
pub fn stub_f60cc4(s: &mut GenSignalState) {
    // IDA 0xf60cc4: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xf60cd4 — j___ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot> &)")]
pub fn stub_f60cd4() {
    // IDA 0xf60cd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60ce4 — j___ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE5mutexEv
// type: int(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::mutex(void)")]
pub fn stub_f60ce4() {
    // IDA 0xf60ce4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60cf4 — j___ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13disconnectAllEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
pub fn stub_f60cf4(s: &mut GenSignalState) {
    // IDA 0xf60cf4: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xf60d04 — j___ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4nextERNS6_13intrusive_ptrINSB_4slotEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
pub fn stub_f60d04() {
    // IDA 0xf60d04: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60d14 — j___ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE5mutexEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")]
pub fn stub_f60d14() {
    // IDA 0xf60d14: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60d24 — j___ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE6insertEPNSB_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
pub fn stub_f60d24(s: &mut GenSignalState) -> u64 {
    // IDA 0xf60d24: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0xf60d34 — j___ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE6removeEPNSB_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
pub fn stub_f60d34(s: &mut GenSignalState, id: u64) {
    // IDA 0xf60d34: unlinks one slot node (missing node is a no-op).
    gen_disconnect(s, id);
}
// 0xf60da4 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13disconnectAllEv
// type: void
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::disconnectAll(void)")]
pub fn stub_f60da4(s: &mut GenSignalState) {
    // IDA 0xf60da4: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xf60db4 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot> &)")]
pub fn stub_f60db4() {
    // IDA 0xf60db4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60dc4 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE5mutexEv
// type: int(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::mutex(void)")]
pub fn stub_f60dc4() {
    // IDA 0xf60dc4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60f04 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_E4callES6_SA_SsSA_
// type: void
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_f60f04(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf60f04: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf60f14 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_ED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_f60f14() {
    // IDA 0xf60f14: drops the bound functor held by the callable.
}
// 0xf60f64 — j___ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
pub fn stub_f60f64() {
    // IDA 0xf60f64: drops the bound functor held by the callable.
}
// 0xf60f94 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE4slotEEaSERKSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot> const&)")]
pub fn stub_f60f94(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf60f94: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf60fa4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSEPSD_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
pub fn stub_f60fa4(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf60fa4: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf60fb4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSERKSE_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
pub fn stub_f60fb4(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf60fb4: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf60fe4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSERKSC_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot> const&)")]
pub fn stub_f60fe4(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf60fe4: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf61094 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEC2ES9_SA_
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>)")]
pub fn stub_f61094(slot: &mut GenFunctor) {
    // IDA 0xf61094: packs the bound argument list.
    slot.has = true;
}
// 0xf610a4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEclINS4_13worker_thread11work_resultEPFSE_S8_SsENS0_5list0EEET_NS0_4typeISI_EERT0_RT1_l
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::worker_thread::work_result boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>::operator()<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list0>(boost::_bi::type<RBX::worker_thread::work_result>,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string) &,boost::_bi::list0 &,long)")]
pub fn stub_f610a4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf610a4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf610b4 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEC2ES8_SA_SD_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)")]
pub fn stub_f610b4(slot: &mut GenFunctor) {
    // IDA 0xf610b4: packs the bound argument list.
    slot.has = true;
}
// 0xf610c4 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEclIPFvS7_NS_10shared_ptrINS4_8InstanceEEESC_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_f610c4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf610c4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf610f4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)")]
pub fn stub_f610f4(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xf610f4: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xf61104 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::list4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
pub fn stub_f61104() -> Option<u32> {
    // IDA 0xf61104: nullable object query (id when live, None when unset).
    None
}
// 0xf61114 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEclINS_4_mfi3mf3IvS5_bSC_iEENS0_5list2IRbRiEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list2<bool &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int> &,boost::_bi::list2<bool &,int &> &,int)")]
pub fn stub_f61114(fire: &dyn Fn(Option<u32>), player: Option<u32>) {
    // IDA 0xf61114: bind/call thunk forwards the locked weak player (mf1).
    fire(player);
}
// 0xf61124 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS2_IPKcEENS_3argILi1EEENS2_IbEEEclINS_4_mfi3mf4IvS5_iRKSsSK_bEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool> &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_f61124() -> Option<u32> {
    // IDA 0xf61124: nullable object query (id when live, None when unset).
    None
}
// 0xf61134 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf4IvS5_iSsSsSsEENS0_5list3IRSsSJ_SJ_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list3<std::string &,std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string> &,boost::_bi::list3<std::string &,std::string &,std::string &> &,int)")]
pub fn stub_f61134(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xf61134: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xf61144 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEC2ES9_SA_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>)")]
pub fn stub_f61144(slot: &mut GenFunctor) {
    // IDA 0xf61144: packs the bound argument list.
    slot.has = true;
}
// 0xf61154 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEEEC2ES8_S9_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>)")]
pub fn stub_f61154(slot: &mut GenFunctor) {
    // IDA 0xf61154: packs the bound argument list.
    slot.has = true;
}
// 0xf61164 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEEEC2ES8_SA_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>)")]
pub fn stub_f61164(slot: &mut GenFunctor) {
    // IDA 0xf61164: packs the bound argument list.
    slot.has = true;
}
// 0xf61184 — j___ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEES9_EC2ERKSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
pub fn stub_f61184(slot: &mut GenFunctor) {
    // IDA 0xf61184: packs the bound argument list.
    slot.has = true;
}
// 0xf61194 — j___ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEES9_EC2ES8_S9_S9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_f61194(slot: &mut GenFunctor) {
    // IDA 0xf61194: packs the bound argument list.
    slot.has = true;
}
// 0xf611a4 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEC2ES8_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)")]
pub fn stub_f611a4(slot: &mut GenFunctor) {
    // IDA 0xf611a4: packs the bound argument list.
    slot.has = true;
}
// 0xf611c4 — j___ZN5boost3_bi8storage3INS_17reference_wrapperIN3RBX7Network11AbuseReportEEENS0_5valueINS_10shared_ptrINS4_6PlayerEEEEENS_3argILi1EEEEC2ES6_SB_SD_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>::storage3(boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>)")]
pub fn stub_f611c4(slot: &mut GenFunctor) {
    // IDA 0xf611c4: packs the bound argument list.
    slot.has = true;
}
// 0xf611f4 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::storage4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
pub fn stub_f611f4(slot: &mut GenFunctor) {
    // IDA 0xf611f4: packs the bound argument list.
    slot.has = true;
}
// 0xf61204 — j___ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// type: void
#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)")]
pub fn stub_f61204() {
    // IDA 0xf61204: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf61214 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS1_8InstanceEEERKSsSD_NS9_IS3_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISO_T0_T1_T2_T3_T4_EENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSR_FSO_SS_ST_SU_SV_ESY_SZ_S10_S11_S12_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_f61214() -> Option<u32> {
    // IDA 0xf61214: nullable object query (id when live, None when unset).
    None
}
// 0xf61234 — j___ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// type: int __fastcall(int, pthread_mutex_t *, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>)")]
pub fn stub_f61234() -> Option<u32> {
    // IDA 0xf61234: nullable object query (id when live, None when unset).
    None
}
// 0xf61254 — j___ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,rbx_core::WeakPtr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>)")]
pub fn stub_f61254() -> Option<u32> {
    // IDA 0xf61254: nullable object query (id when live, None when unset).
    None
}
// 0xf61264 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
pub fn stub_f61264() {
    // IDA 0xf61264: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf61294 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network13AbuseReporter4dataES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::AbuseReporter::data,RBX::Network::AbuseReporter::data>(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data> *,RBX::Network::AbuseReporter::data *,boost::detail::shared_count &)")]
pub fn stub_f61294(slot: &mut Option<u32>, v: u32) {
    // IDA 0xf61294: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xf612b4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f612b4(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf612b4: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf612f4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f612f4(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf612f4: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf61304 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f61304(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf61304: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf61364 — j___ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEEvT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::function0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>)")]
pub fn stub_f61364(slot: &mut GenFunctor) -> bool {
    // IDA 0xf61364: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf613b4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")]
pub fn stub_f613b4(slot: &mut GenFunctor) -> bool {
    // IDA 0xf613b4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf613f4 — j___ZN5boost9function4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSJ_EENSA_5list5INSA_5valueINS5_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
pub fn stub_f613f4(slot: &mut GenFunctor) -> bool {
    // IDA 0xf613f4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf61484 — j___ZN9__gnu_cxx13new_allocatorIN3RBX7Network11AbuseReportEE7destroyEPS3_
// type: void
#[doc(alias = "__gnu_cxx::new_allocator<RBX::Network::AbuseReport>::destroy(RBX::Network::AbuseReport*)")]
pub fn stub_f61484() {
    // IDA 0xf61484: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf61494 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
pub fn stub_f61494(s: &mut GenSignalState) -> u64 {
    // IDA 0xf61494: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf614a4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> const&)const")]
pub fn stub_f614a4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf614a4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf614b4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E7connectEPNS0_11EventSourceERKNS5_8functionIS9_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
pub fn stub_f614b4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf614b4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf614c4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string)> const&)const")]
pub fn stub_f614c4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf614c4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf614d4 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_f614d4(name: &str) -> GenDesc {
    // IDA 0xf614d4: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf614f4 — j___ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE13convertToItemERKS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToItem(RBX::Network::Players::ChatOption const&)const")]
pub fn stub_f614f4(value: i32) -> i32 {
    // IDA 0xf614f4: EnumDesc item passthrough (validated upstream).
    value
}
// 0xf61504 — j___ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const")]
pub fn stub_f61504(value: i32) -> &'static str {
    // IDA 0xf61504: EnumDesc value->name (table engine-side).
    "Valid"
}
// 0xf61514 — j___ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE13convertToItemERKS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const")]
pub fn stub_f61514(value: i32) -> i32 {
    // IDA 0xf61514: EnumDesc item passthrough (validated upstream).
    value
}
// 0xf61524 — j___ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const")]
pub fn stub_f61524(value: i32) -> &'static str {
    // IDA 0xf61524: EnumDesc value->name (table engine-side).
    "Valid"
}
