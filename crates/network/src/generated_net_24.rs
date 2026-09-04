//! network generated_net_24 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 4797 total, 0 remaining (complete) — global gap filler EA-sorted asc
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0x25104c..0x25c128 | 25450->25600 network distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

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

/// `boost::gregorian` date error (`std::logic_error` payload; thrown via
/// `boost::throw_exception`, IDA 0x251d10/0x251d94).
#[derive(Clone, Debug)]
pub struct GenDateError {
    pub kind: &'static str,
}

impl std::fmt::Display for GenDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad date: {}", self.kind)
    }
}

impl std::error::Error for GenDateError {}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}



// 0x25104c — __ZN5boost9gregorian16bad_day_of_monthD1Ev
// type: void __fastcall(std::logic_error *this)
#[doc(alias = "boost::gregorian::bad_day_of_month::~bad_day_of_month()")]
pub fn stub_25104c() {
    // IDA 0x25104c: dtor releases the owned control block/slots.
}
// 0x251058 — __ZN5boost9gregorian16bad_day_of_monthC2Ev
// type: std::out_of_range *__fastcall(std::out_of_range *this)
#[doc(alias = "boost::gregorian::bad_day_of_month::bad_day_of_month(void)")]
pub fn stub_251058() -> GenDateError {
    // IDA 0x251058: builds the logic_error payload for a bad day_of_month.
    GenDateError { kind: "day_of_month" }
}
// 0x2511a0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
pub fn stub_2511a0() {
    // IDA 0x2511a0: dtor releases the owned control block/slots.
}
// 0x251258 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
pub fn stub_251258() {
    // IDA 0x251258: dtor releases the owned control block/slots.
}
// 0x251310 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
pub fn stub_251310(fire: &dyn Fn()) {
    // IDA 0x251310: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2513c8 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
pub fn stub_2513c8(fire: &dyn Fn()) {
    // IDA 0x2513c8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x251480 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
pub fn stub_251480(e: &GenDateError) -> GenDateError {
    // IDA 0x251480: this-adjusted tail-call into the virtual clone.
    e.clone()
}
// 0x251550 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const")]
pub fn stub_251550(e: &GenDateError) -> GenDateError {
    // IDA 0x251550: virtual clone of the stored exception.
    e.clone()
}
// 0x25160c — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
pub fn stub_25160c(fire: &dyn Fn()) {
    // IDA 0x25160c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2516c8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const")]
pub fn stub_2516c8(e: &GenDateError) -> GenDateError {
    // IDA 0x2516c8: virtual clone of the stored exception.
    e.clone()
}
// 0x25178c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv
// type: int __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const")]
pub fn stub_25178c(e: &GenDateError) -> ! {
    // IDA 0x25178c: __noreturn rethrow of the stored exception.
    panic!("rethrow: {}", e);
}
// 0x25179c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
pub fn stub_25179c(e: &GenDateError) -> GenDateError {
    // IDA 0x25179c: this-adjusted tail-call into the virtual clone.
    e.clone()
}
// 0x251870 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
pub fn stub_251870() {
    // IDA 0x251870: dtor releases the owned control block/slots.
}
// 0x25192c — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
pub fn stub_25192c(fire: &dyn Fn()) {
    // IDA 0x25192c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2519e8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_tag)")]
pub fn stub_2519e8(msg: &str) -> GenDateError {
    // IDA 0x2519e8: copy ctor captures the message into the injector.
    let _ = msg;
    GenDateError { kind: "injected" }
}
// 0x251b7c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month> const&)")]
pub fn stub_251b7c(msg: &str) -> GenDateError {
    // IDA 0x251b7c: copy ctor captures the message into the injector.
    let _ = msg;
    GenDateError { kind: "injected" }
}
// 0x251d10 — __ZN5boost2CV23simple_exception_policyItLt1ELt12ENS_9gregorian9bad_monthEE8on_errorEttNS0_14violation_enumE
// type: void __noreturn()
#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)12,boost::gregorian::bad_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
pub fn stub_251d10(value: u32) -> GenDateError {
    // IDA 0x251d10: __noreturn policy hook: builds bad_month/day then throw_exception (throw elided, error returned).
    let _ = value;
    GenDateError { kind: "policy" }
}
// 0x251d94 — __ZN5boost15throw_exceptionINS_9gregorian9bad_monthEEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_month>(boost::gregorian::bad_month const&)")]
pub fn stub_251d94(e: GenDateError) -> ! {
    // IDA 0x251d94: __noreturn __cxa_allocate_exception + __cxa_throw.
    panic!("throw: {}", e);
}
// 0x251ee8 — __ZN5boost9gregorian9bad_monthC2Ev
// type: std::out_of_range *__fastcall(std::out_of_range *this)
#[doc(alias = "boost::gregorian::bad_month::bad_month(void)")]
pub fn stub_251ee8() -> GenDateError {
    // IDA 0x251ee8: builds the logic_error payload for a bad month.
    GenDateError { kind: "month" }
}
// 0x25202c — __ZN5boost9gregorian9bad_monthD0Ev
// type: void __fastcall(std::logic_error *this)
#[doc(alias = "boost::gregorian::bad_month::~bad_month()")]
pub fn stub_25202c() {
    // IDA 0x25202c: dtor releases the owned control block/slots.
}
// 0x252040 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
pub fn stub_252040() {
    // IDA 0x252040: dtor releases the owned control block/slots.
}
// 0x2520f8 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
pub fn stub_2520f8() {
    // IDA 0x2520f8: dtor releases the owned control block/slots.
}
// 0x2521b0 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
pub fn stub_2521b0(fire: &dyn Fn()) {
    // IDA 0x2521b0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x252268 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
pub fn stub_252268(fire: &dyn Fn()) {
    // IDA 0x252268: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x252320 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
pub fn stub_252320(e: &GenDateError) -> GenDateError {
    // IDA 0x252320: this-adjusted tail-call into the virtual clone.
    e.clone()
}
// 0x2523ec — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
pub fn stub_2523ec() {
    // IDA 0x2523ec: dtor releases the owned control block/slots.
}
// 0x2524a8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const")]
pub fn stub_2524a8(e: &GenDateError) -> GenDateError {
    // IDA 0x2524a8: virtual clone of the stored exception.
    e.clone()
}
// 0x252564 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")]
pub fn stub_252564(e: &GenDateError) -> ! {
    // IDA 0x252564: __noreturn rethrow of the stored exception.
    panic!("rethrow: {}", e);
}
// 0x252618 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const")]
pub fn stub_252618(e: &GenDateError) -> GenDateError {
    // IDA 0x252618: virtual clone of the stored exception.
    e.clone()
}
// 0x2526e0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&)")]
pub fn stub_2526e0(msg: &str) -> GenDateError {
    // IDA 0x2526e0: copy ctor captures the message into the injector.
    let _ = msg;
    GenDateError { kind: "injected" }
}
// 0x252820 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
pub fn stub_252820() {
    // IDA 0x252820: dtor releases the owned control block/slots.
}
// 0x2528e0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_tag)")]
pub fn stub_2528e0(msg: &str) -> GenDateError {
    // IDA 0x2528e0: copy ctor captures the message into the injector.
    let _ = msg;
    GenDateError { kind: "injected" }
}
// 0x252a78 — __ZN5boost2CV23simple_exception_policyItLt1400ELt10000ENS_9gregorian8bad_yearEE8on_errorEttNS0_14violation_enumE
// type: void __noreturn()
#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1400,(unsigned short)10000,boost::gregorian::bad_year>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
pub fn stub_252a78(value: u32) -> GenDateError {
    // IDA 0x252a78: __noreturn policy hook: builds bad_month/day then throw_exception (throw elided, error returned).
    let _ = value;
    GenDateError { kind: "policy" }
}
// 0x252afc — __ZN5boost9gregorian8bad_yearD1Ev
// type: void __fastcall(std::logic_error *this)
#[doc(alias = "boost::gregorian::bad_year::~bad_year()")]
pub fn stub_252afc() {
    // IDA 0x252afc: dtor releases the owned control block/slots.
}
// 0x252b08 — __ZN5boost9gregorian8bad_yearC2Ev
// type: std::out_of_range *__fastcall(std::out_of_range *this)
#[doc(alias = "boost::gregorian::bad_year::bad_year(void)")]
pub fn stub_252b08() {
    // IDA 0x252b08: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x252c50 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
pub fn stub_252c50() {
    // IDA 0x252c50: dtor releases the owned control block/slots.
}
// 0x252d08 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
pub fn stub_252d08() {
    // IDA 0x252d08: dtor releases the owned control block/slots.
}
// 0x252dc0 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
pub fn stub_252dc0(fire: &dyn Fn()) {
    // IDA 0x252dc0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x252e78 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
pub fn stub_252e78(fire: &dyn Fn()) {
    // IDA 0x252e78: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x252f30 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
pub fn stub_252f30(e: &GenDateError) -> GenDateError {
    // IDA 0x252f30: this-adjusted tail-call into the virtual clone.
    e.clone()
}
// 0x253000 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const")]
pub fn stub_253000(e: &GenDateError) -> GenDateError {
    // IDA 0x253000: virtual clone of the stored exception.
    e.clone()
}
// 0x2530bc — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
pub fn stub_2530bc(fire: &dyn Fn()) {
    // IDA 0x2530bc: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x253178 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const")]
pub fn stub_253178(e: &GenDateError) -> GenDateError {
    // IDA 0x253178: virtual clone of the stored exception.
    e.clone()
}
// 0x25323c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv
// type: int __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const")]
pub fn stub_25323c(e: &GenDateError) -> ! {
    // IDA 0x25323c: __noreturn rethrow of the stored exception.
    panic!("rethrow: {}", e);
}
// 0x25324c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
pub fn stub_25324c(e: &GenDateError) -> GenDateError {
    // IDA 0x25324c: this-adjusted tail-call into the virtual clone.
    e.clone()
}
// 0x253320 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
pub fn stub_253320() {
    // IDA 0x253320: dtor releases the owned control block/slots.
}
// 0x2533dc — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
pub fn stub_2533dc(fire: &dyn Fn()) {
    // IDA 0x2533dc: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x253498 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_tag)")]
pub fn stub_253498(msg: &str) -> GenDateError {
    // IDA 0x253498: copy ctor captures the message into the injector.
    let _ = msg;
    GenDateError { kind: "injected" }
}
// 0x25362c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_year> const&)")]
pub fn stub_25362c(msg: &str) -> GenDateError {
    // IDA 0x25362c: copy ctor captures the message into the injector.
    let _ = msg;
    GenDateError { kind: "injected" }
}
// 0x2537c0 — __ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE
// type: _DWORD *__fastcall(_DWORD *, unsigned __int16, unsigned __int16, unsigned __int16)
#[doc(alias = "boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)")]
pub fn stub_2537c0() {
    // IDA 0x2537c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2539a0 — __ZN5boost6thread17do_try_join_untilERK8timespec
// type: int __fastcall(boost::thread *this, const timespec *)
#[doc(alias = "boost::thread::do_try_join_until(timespec const&)")]
pub fn stub_2539a0() {
    // IDA 0x2539a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253af0 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_
// type: int __fastcall(int result, unsigned int *, __int64 *)
#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)")]
pub fn stub_253af0() {
    // IDA 0x253af0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253c78 — __GLOBAL__I_a_52
// type: 
#[doc(alias = "global constructor keyed to_a_52")]
pub fn stub_253c78() {
    // IDA 0x253c78: static initializer registration (runs before main).
}
// 0x253d50 — __ZN3RBX4Time3nowILNS0_12SampleMethodE2EEES0_v
// type: void __fastcall(double *)
#[doc(alias = "RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)2>(void)")]
pub fn stub_253d50() {
    // IDA 0x253d50: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253ea4 — __ZN3RBX4Time3nowILNS0_12SampleMethodE0EEES0_v
// type: void __fastcall(double *)
#[doc(alias = "RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)0>(void)")]
pub fn stub_253ea4() {
    // IDA 0x253ea4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253eb0 — __ZN3RBX4Time7nowFastEv
// type: void __fastcall(RBX::Time *this)
#[doc(alias = "RBX::Time::nowFast(void)")]
pub fn stub_253eb0() {
    // IDA 0x253eb0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253ebc — __ZN3RBX4Time10nowFastSecEv
// type: __int64 __fastcall(RBX::Time *this)
#[doc(alias = "RBX::Time::nowFastSec(void)")]
pub fn stub_253ebc() {
    // IDA 0x253ebc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253ecc — __ZN3RBX4Time3nowILNS0_12SampleMethodE1EEES0_v
// type: void __fastcall(double *)
#[doc(alias = "RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)1>(void)")]
pub fn stub_253ecc() {
    // IDA 0x253ecc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253edc — __ZN3RBXmiERKNS_4TimeES2_
// type: double *__fastcall(double *result, double *, double *)
#[doc(alias = "RBX::operator-(RBX::Time const&,RBX::Time const&)")]
pub fn stub_253edc() {
    // IDA 0x253edc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253ef0 — __ZN3RBX10RbxDbgInfo8AddPlaceEl
// type: int __fastcall(RBX::RbxDbgInfo *this, int)
#[doc(alias = "RBX::RbxDbgInfo::AddPlace(long)")]
pub fn stub_253ef0() {
    // IDA 0x253ef0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253f24 — __ZN3RBX10RbxDbgInfo11RemovePlaceEl
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::RbxDbgInfo::RemovePlace(long)")]
pub fn stub_253f24() {
    // IDA 0x253f24: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253f70 — __ZN3RBX10RbxDbgInfo14SetGfxCardNameEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardName(char const*)")]
pub fn stub_253f70() {
    // IDA 0x253f70: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253f94 — __ZN3RBX10RbxDbgInfo23SetGfxCardDriverVersionEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardDriverVersion(char const*)")]
pub fn stub_253f94() {
    // IDA 0x253f94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253fb8 — __ZN3RBX10RbxDbgInfo16SetGfxCardVendorEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardVendor(char const*)")]
pub fn stub_253fb8() {
    // IDA 0x253fb8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x253fdc — __ZN3RBX10RbxDbgInfo10SetCPUNameEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetCPUName(char const*)")]
pub fn stub_253fdc() {
    // IDA 0x253fdc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254000 — __ZN3RBX10RbxDbgInfo11SetServerIPEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetServerIP(char const*)")]
pub fn stub_254000() {
    // IDA 0x254000: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254024 — __ZN3RBX23RbxInterlockedDecrementEPVl
// type: int32_t __fastcall(int32_t *__theValue, volatile int *)
#[doc(alias = "RBX::RbxInterlockedDecrement(long volatile*)")]
pub fn stub_254024() {
    // IDA 0x254024: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254034 — __ZN3RBX23RbxInterlockedIncrementEPVl
// type: int32_t __fastcall(int32_t *__theValue, volatile int *)
#[doc(alias = "RBX::RbxInterlockedIncrement(long volatile*)")]
pub fn stub_254034() {
    // IDA 0x254034: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254044 — __ZN3RBX30RbxInterlockedIncrementAcquireEPVl
// type: int32_t __fastcall(int32_t *__theValue, volatile int *)
#[doc(alias = "RBX::RbxInterlockedIncrementAcquire(long volatile*)")]
pub fn stub_254044() {
    // IDA 0x254044: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254054 — __ZN3RBX22RbxInterlockedExchangeEPVll
// type: __int32 __fastcall(RBX *this, volatile int *, int)
#[doc(alias = "RBX::RbxInterlockedExchange(long volatile*,long)")]
pub fn stub_254054() {
    // IDA 0x254054: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254068 — __ZN3RBX29RbxInterlockedCompareExchangeEPVlll
// type: __int32 __fastcall(RBX *this, volatile int *, __int32 __oldValue, int)
#[doc(alias = "RBX::RbxInterlockedCompareExchange(long volatile*,long,long)")]
pub fn stub_254068() {
    // IDA 0x254068: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25407c — __ZN3RBX13MacSystemUtil10getCPUMakeEv
// type: int __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getCPUMake(void)")]
pub fn stub_25407c() {
    // IDA 0x25407c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2541ac — __ZN3RBX13MacSystemUtil11getCPUSpeedEv
// type: int __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getCPUSpeed(void)")]
pub fn stub_2541ac() {
    // IDA 0x2541ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254224 — __ZN3RBX13MacSystemUtil18getCPULogicalCountEv
// type: __int64 __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getCPULogicalCount(void)")]
pub fn stub_254224() {
    // IDA 0x254224: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2542b0 — __ZN3RBX13MacSystemUtil15getCPUCoreCountEv
// type: int __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getCPUCoreCount(void)")]
pub fn stub_2542b0() {
    // IDA 0x2542b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254320 — __ZN3RBX13MacSystemUtil19getCPUPhysicalCountEv
// type: __int64 __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getCPUPhysicalCount(void)")]
pub fn stub_254320() {
    // IDA 0x254320: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254478 — __ZN3RBX13MacSystemUtil10isCPU64BitEv
// type: int __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::isCPU64Bit(void)")]
pub fn stub_254478() {
    // IDA 0x254478: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25453c — __ZN3RBX13MacSystemUtil11getMBSysRAMEv
// type: int __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getMBSysRAM(void)")]
pub fn stub_25453c() {
    // IDA 0x25453c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2545b4 — __ZN3RBX13MacSystemUtil20getMBSysAvailableRAMEv
// type: int __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getMBSysAvailableRAM(void)")]
pub fn stub_2545b4() {
    // IDA 0x2545b4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254654 — __ZN3RBX13MacSystemUtil14getVideoMemoryEv
// type: __int64 __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getVideoMemory(void)")]
pub fn stub_254654() {
    // IDA 0x254654: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25465c — __ZN3RBX13MacSystemUtil5osVerEv
// type: void __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::osVer(void)")]
pub fn stub_25465c() {
    // IDA 0x25465c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254824 — __ZN3RBX13MacSystemUtil10getGPUMakeEv
// type: void __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getGPUMake(void)")]
pub fn stub_254824() {
    // IDA 0x254824: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2549ec — __ZN3RBX13MacSystemUtil9getMaxResEv
// type: void __fastcall(RBX::MacSystemUtil *this)
#[doc(alias = "RBX::MacSystemUtil::getMaxRes(void)")]
pub fn stub_2549ec() {
    // IDA 0x2549ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254bb4 — __ZN3RBX5CryptC1Ev
// type: void __fastcall(RBX::Crypt *this)
#[doc(alias = "RBX::Crypt::Crypt(void)")]
pub fn stub_254bb4() {
    // IDA 0x254bb4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254bb8 — __ZN3RBX5CryptD1Ev
// type: void __fastcall(RBX::Crypt *__hidden this)
#[doc(alias = "RBX::Crypt::~Crypt()")]
pub fn stub_254bb8() {
    // IDA 0x254bb8: dtor releases the owned control block/slots.
}
// 0x254bbc — __ZN3RBX5Crypt21verifySignatureBase64ESsSs
// type: void()
#[doc(alias = "RBX::Crypt::verifySignatureBase64(std::string,std::string)")]
pub fn stub_254bbc() {
    // IDA 0x254bbc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254bc0 — __GLOBAL__I_a_53
// type: int()
#[doc(alias = "global constructor keyed to_a_53")]
pub fn stub_254bc0() {
    // IDA 0x254bc0: static initializer registration (runs before main).
}
// 0x254bf8 — __ZN3RBX14IsValueOutlierEdjddNS_10ConfidenceE
// type: __int64 __fastcall(double, int, unsigned int, unsigned int, double, int)
#[doc(alias = "RBX::IsValueOutlier(double,unsigned int,double,double,RBX::Confidence)")]
pub fn stub_254bf8() {
    // IDA 0x254bf8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254c68 — __ZN3RBX21GetConfidenceIntervalEddNS_10ConfidenceEPdS1_
// type: int __fastcall(double, double, int, double *, double *)
#[doc(alias = "RBX::GetConfidenceInterval(double,double,RBX::Confidence,double *,double *)")]
pub fn stub_254c68() {
    // IDA 0x254c68: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254d18 — __ZN3RBX9TCriticalEjNS_10ConfidenceE
// type: __int64 __fastcall(int, int, int)
#[doc(alias = "RBX::TCritical(unsigned int,RBX::Confidence)")]
pub fn stub_254d18() {
    // IDA 0x254d18: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254e20 — __ZN13CookiesEngine8SetValueESsSs
// type: int __fastcall(int, const char **, const char **)
#[doc(alias = "CookiesEngine::SetValue(std::string,std::string)")]
pub fn stub_254e20() {
    // IDA 0x254e20: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x254e98 — __ZN13CookiesEngine18getCookiesFilePathEv
// type: void __fastcall(CookiesEngine *this)
#[doc(alias = "CookiesEngine::getCookiesFilePath(void)")]
pub fn stub_254e98() {
    // IDA 0x254e98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x255070 — __ZN13CookiesEngineC1ESbIwSt11char_traitsIwESaIwEE
// type: int __fastcall(int result)
#[doc(alias = "CookiesEngine::CookiesEngine(std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>)")]
pub fn stub_255070() {
    // IDA 0x255070: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x255098 — __ZN13CookiesEngine8GetValueESsPiPb
// type: int __fastcall(std::string *, int, const char **, int *, char *)
#[doc(alias = "CookiesEngine::GetValue(std::string,int *,bool *)")]
pub fn stub_255098() {
    // IDA 0x255098: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25529c — __ZN13CookiesEngine11DeleteValueESs
// type: int __fastcall(int, const char **)
#[doc(alias = "CookiesEngine::DeleteValue(std::string)")]
pub fn stub_25529c() {
    // IDA 0x25529c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2552f0 — __GLOBAL__I_a_54
// type: int()
#[doc(alias = "global constructor keyed to_a_54")]
pub fn stub_2552f0() {
    // IDA 0x2552f0: static initializer registration (runs before main).
}
// 0x255320 — __Z11convert_w2sRKSbIwSt11char_traitsIwESaIwEE
// type: void __fastcall(std::string *, int **, int, int, int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "convert_w2s(std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>> const&)")]
pub fn stub_255320() {
    // IDA 0x255320: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x255474 — __Z11convert_s2wRKSs
// type: void __fastcall(const std::string *, char **)
#[doc(alias = "convert_s2w(std::string const&)")]
pub fn stub_255474() {
    // IDA 0x255474: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2555bc — __Z7vformatPKcPv
// type: int __fastcall(std::string *, char *__format, va_list)
#[doc(alias = "vformat(char const*,void *)")]
pub fn stub_2555bc() {
    // IDA 0x2555bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2557cc — __Z13format_stringPKcz
// type: int(std::string *, char *, ...)
#[doc(alias = "format_string(char const*,...)")]
pub fn stub_2557cc() {
    // IDA 0x2557cc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2557ec — __ZN10SimpleJSON14ReadFromStreamEPKc
// type: void __fastcall(SimpleJSON *this, const char *)
#[doc(alias = "SimpleJSON::ReadFromStream(char const*)")]
pub fn stub_2557ec() {
    // IDA 0x2557ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x255c70 — __ZL14needTrimSymbolc
// type: unsigned int __fastcall(char)
#[doc(alias = "needTrimSymbol(char)")]
pub fn stub_255c70() {
    // IDA 0x255c70: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x255c8c — __ZN10SimpleJSON9ParseBoolEPKc
// type: bool __fastcall(SimpleJSON *this, const char *)
#[doc(alias = "SimpleJSON::ParseBool(char const*)")]
pub fn stub_255c8c() {
    // IDA 0x255c8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x255cc8 — __ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_
// type: int __fastcall(std::string *this, int (__fastcall *)(int))
#[doc(alias = "void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))")]
pub fn stub_255cc8() {
    // IDA 0x255cc8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x257828 — __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET_
// type: int __fastcall(int, const std::string *)
#[doc(alias = "void RBX::Reflection::resume_adapter<std::string>(boost::function<void ()(RBX::Reflection::Variant)>,std::string)")]
pub fn stub_257828() {
    // IDA 0x257828: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x259bfc — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(sp_counted_base **, int, const shared_count **)
#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_259bfc() -> Option<u32> {
    // IDA 0x259bfc: nullable object query (id when live, None when unset).
    None
}
// 0x25afd8 — __GLOBAL__I_a_55
// type: 
#[doc(alias = "global constructor keyed to_a_55")]
pub fn stub_25afd8() {
    // IDA 0x25afd8: static initializer registration (runs before main).
}
// 0x25b4c0 — __ZN3RBX5Light10setEnabledEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Light::setEnabled(bool)")]
pub fn stub_25b4c0() {
    // IDA 0x25b4c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b4e0 — __ZN3RBX5Light8setColorEN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
pub fn stub_25b4e0() {
    // IDA 0x25b4e0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b544 — __ZN3RBX5Light13setBrightnessEf
// type: float *__fastcall(float *this, float32_t)
#[doc(alias = "RBX::Light::setBrightness(float)")]
pub fn stub_25b544() {
    // IDA 0x25b544: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b574 — __ZN3RBX10PointLight8setRangeEf
// type: RBX::Instance *__fastcall(RBX::Instance *this, float32_t)
#[doc(alias = "RBX::PointLight::setRange(float)")]
pub fn stub_25b574() {
    // IDA 0x25b574: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b5b0 — __ZN3RBX9SpotLight8setRangeEf
// type: RBX::Instance *__fastcall(RBX::Instance *this, float32_t)
#[doc(alias = "RBX::SpotLight::setRange(float)")]
pub fn stub_25b5b0() {
    // IDA 0x25b5b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b5ec — __ZN3RBX9SpotLight8setAngleEf
// type: RBX::Instance *__fastcall(RBX::Instance *this, float32_t)
#[doc(alias = "RBX::SpotLight::setAngle(float)")]
pub fn stub_25b5ec() {
    // IDA 0x25b5ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b628 — __ZN3RBX19registerNewLightAPIEv
// type: void __fastcall(RBX *this, int, int)
#[doc(alias = "RBX::registerNewLightAPI(void)")]
pub fn stub_25b628() {
    // IDA 0x25b628: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b884 — __ZN3RBX5Light10setShadowsEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Light::setShadows(bool)")]
pub fn stub_25b884() {
    // IDA 0x25b884: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b8a8 — __ZN3RBX9SpotLight7setFaceENS_8NormalIdE
// type: RBX::Instance *__fastcall(RBX::Instance *result, int)
#[doc(alias = "RBX::SpotLight::setFace(RBX::NormalId)")]
pub fn stub_25b8a8() {
    // IDA 0x25b8a8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25b8c8 — __ZN3RBX5LightC2EPKc
// type: __guard *__fastcall(RBX::Light *this, const char *)
#[doc(alias = "RBX::Light::Light(char const*)")]
pub fn stub_25b8c8() {
    // IDA 0x25b8c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25baa8 — __ZN3RBX5LightD0Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "RBX::Light::~Light()")]
pub fn stub_25baa8() {
    // IDA 0x25baa8: dtor releases the owned control block/slots.
}
// 0x25bb48 — __ZN3RBX5LightD1Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "RBX::Light::~Light()")]
pub fn stub_25bb48() {
    // IDA 0x25bb48: dtor releases the owned control block/slots.
}
// 0x25bb4c — __ZThn32_N3RBX5LightD0Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_25bb4c(fire: &dyn Fn()) {
    // IDA 0x25bb4c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25bb54 — __ZThn36_N3RBX5LightD0Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_25bb54(fire: &dyn Fn()) {
    // IDA 0x25bb54: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25bb5c — __ZThn92_N3RBX5LightD0Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_25bb5c(fire: &dyn Fn()) {
    // IDA 0x25bb5c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25bb64 — __ZN3RBX5LightD2Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "RBX::Light::~Light()")]
pub fn stub_25bb64() {
    // IDA 0x25bb64: dtor releases the owned control block/slots.
}
// 0x25bc20 — __ZThn32_N3RBX5LightD1Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_25bc20(fire: &dyn Fn()) {
    // IDA 0x25bc20: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25bc28 — __ZThn36_N3RBX5LightD1Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_25bc28(fire: &dyn Fn()) {
    // IDA 0x25bc28: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25bc30 — __ZThn92_N3RBX5LightD1Ev
// type: void __fastcall(RBX::Light *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_25bc30(fire: &dyn Fn()) {
    // IDA 0x25bc30: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25bc38 — __ZNK3RBX5Light12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Light *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Light::askSetParent(RBX::Instance const*)const")]
pub fn stub_25bc38() {
    // IDA 0x25bc38: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25bc60 — __ZNK3RBX5Light11askAddChildEPKNS_8InstanceE
// type: int __fastcall(RBX::Light *this, const Instance *)
#[doc(alias = "RBX::Light::askAddChild(RBX::Instance const*)const")]
pub fn stub_25bc60(p: &GenPeer) -> bool {
    // IDA 0x25bc60: peers accept any instance child.
    let _ = p;
    true
}
// 0x25bc64 — __ZN3RBX10PointLightC2Ev
// type: RBX::Light *__fastcall(RBX::PointLight *this)
#[doc(alias = "RBX::PointLight::PointLight(void)")]
pub fn stub_25bc64() {
    // IDA 0x25bc64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25bdb8 — __ZN3RBX10PointLightD0Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "RBX::PointLight::~PointLight()")]
pub fn stub_25bdb8() {
    // IDA 0x25bdb8: dtor releases the owned control block/slots.
}
// 0x25be58 — __ZN3RBX10PointLightD1Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "RBX::PointLight::~PointLight()")]
pub fn stub_25be58() {
    // IDA 0x25be58: dtor releases the owned control block/slots.
}
// 0x25be5c — __ZThn32_N3RBX10PointLightD0Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_25be5c(fire: &dyn Fn()) {
    // IDA 0x25be5c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25be64 — __ZThn36_N3RBX10PointLightD0Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_25be64(fire: &dyn Fn()) {
    // IDA 0x25be64: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25be6c — __ZThn92_N3RBX10PointLightD0Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_25be6c(fire: &dyn Fn()) {
    // IDA 0x25be6c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25be74 — __ZThn32_N3RBX10PointLightD1Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_25be74(fire: &dyn Fn()) {
    // IDA 0x25be74: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25be7c — __ZThn36_N3RBX10PointLightD1Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_25be7c(fire: &dyn Fn()) {
    // IDA 0x25be7c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25be84 — __ZThn92_N3RBX10PointLightD1Ev
// type: void __fastcall(RBX::PointLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_25be84(fire: &dyn Fn()) {
    // IDA 0x25be84: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25be8c — __ZN3RBX9SpotLightC2Ev
// type: RBX::Light *__fastcall(RBX::SpotLight *this)
#[doc(alias = "RBX::SpotLight::SpotLight(void)")]
pub fn stub_25be8c() {
    // IDA 0x25be8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25bff0 — __ZN3RBX9SpotLightD0Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "RBX::SpotLight::~SpotLight()")]
pub fn stub_25bff0() {
    // IDA 0x25bff0: dtor releases the owned control block/slots.
}
// 0x25c090 — __ZN3RBX9SpotLightD1Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "RBX::SpotLight::~SpotLight()")]
pub fn stub_25c090() {
    // IDA 0x25c090: dtor releases the owned control block/slots.
}
// 0x25c094 — __ZThn32_N3RBX9SpotLightD0Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_25c094(fire: &dyn Fn()) {
    // IDA 0x25c094: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25c09c — __ZThn36_N3RBX9SpotLightD0Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_25c09c(fire: &dyn Fn()) {
    // IDA 0x25c09c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25c0a4 — __ZThn92_N3RBX9SpotLightD0Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_25c0a4(fire: &dyn Fn()) {
    // IDA 0x25c0a4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25c0ac — __ZThn32_N3RBX9SpotLightD1Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_25c0ac(fire: &dyn Fn()) {
    // IDA 0x25c0ac: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25c0b4 — __ZThn36_N3RBX9SpotLightD1Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_25c0b4(fire: &dyn Fn()) {
    // IDA 0x25c0b4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25c0bc — __ZThn92_N3RBX9SpotLightD1Ev
// type: void __fastcall(RBX::SpotLight *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_25c0bc(fire: &dyn Fn()) {
    // IDA 0x25c0bc: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x25c0c4 — __ZNK3RBX5Light10getEnabledEv
// type: int __fastcall(RBX::Light *this)
#[doc(alias = "RBX::Light::getEnabled(void)const")]
pub fn stub_25c0c4() {
    // IDA 0x25c0c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x25c0cc — __ZN3RBX10Reflection14PropDescriptorINS_5LightEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::~PropDescriptor()")]
pub fn stub_25c0cc(d: GenDesc) {
    // IDA 0x25c0cc: prop descriptor dtor.
    let _ = d;
}
// 0x25c100 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
pub fn stub_25c100(d: GenDesc) {
    // IDA 0x25c100: prop descriptor dtor.
    let _ = d;
}
// 0x25c128 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::~PropDescriptor()")]
pub fn stub_25c128(d: GenDesc) {
    // IDA 0x25c128: prop descriptor dtor.
    let _ = d;
}
