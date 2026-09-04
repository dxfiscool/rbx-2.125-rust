//! Auto-generated skeletons for rbx-network — RBX::Network|RakNet filtered EA-sorted ascending
//! Filter: RakNet|RBX::Network -> 4479 funcs, 4479 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0xa51310..0xa5bfec | existing 16829 -> 16929 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

use std::collections::HashMap;

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

use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// `boost::tokenizer<boost::char_separator<...>, ...>` (IDA 0xa51310/0xa51b40):
/// kept/dropped separator sets plus the token list. `end()` is one-past-last.
#[derive(Clone, Debug, Default)]
pub struct CharTokenizer {
    pub kept: String,
    pub dropped: String,
    pub tokens: Vec<String>,
}

/// One `XmlAttribute` node (IDA 0xa52198/0xa524a4): pooled 20-byte node
/// holding the name id, the value-kind tag (`2` = string), and the value.
#[derive(Clone, Debug, Default)]
pub struct XmlAttribute {
    pub name: u32,
    pub kind: u32,
    pub value: String,
}

/// `RBX::GuiBuilder::Data` (IDA 0xa530f8): string-valued payload; the field
/// count is engine-side, this keeps the owned strings the pair dtor releases.
#[derive(Clone, Debug, Default)]
pub struct GuiBuilderData {
    pub fields: Vec<String>,
}

/// One bound argument of a `Players` bound function (IDA 0xa53b04/0xa53c54/0xa53e38):
/// `SharedPtr<Instance>` becomes the registry id, strings stay strings.
#[derive(Clone, Debug)]
pub enum BoundFuncArg {
    Text(String),
    Instance(u32),
}

/// `RBX::Reflection::BoundFuncDesc<RBX::Network::Players, ...>` tear-down state
/// (IDA 0xa53b04/0xa53c54/0xa53e38): bound-arg holders plus the listener list
/// at +32 whose nodes are unlinked and freed one by one.
#[derive(Clone, Debug, Default)]
pub struct PlayersBoundFuncDesc {
    pub args: Vec<BoundFuncArg>,
    pub listeners: Vec<u32>,
}

/// `CheckSum` (IDA 0xa56c10): RakNet running checksum; `sum` at +0 with the
/// two multipliers at +2/+4 and the 32-bit accumulator at +8.
#[derive(Clone, Debug, Default)]
pub struct CheckSum {
    pub sum: u16,
    pub mult_a: u16,
    pub mult_b: u16,
    pub total: u32,
}

// 0xa51310 — __ZNK5boost9tokenizerINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsE3endEv
// demangled: boost::tokenizer<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::end(void)const
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::tokenizer<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::end(void)const")]
pub fn stub_a51310(tokenizer: &CharTokenizer) -> usize {
 // IDA 0xa51310: builds the end iterator by running the separator once
 // past the last token; that is one-past-last.
 tokenizer.tokens.len()
}

// 0xa514f0 — __ZNSt6vectorISsSaISsEEC2ERKS1_
// demangled: std::vector<std::string,std::allocator<std::string>>::vector(std::vector<std::string,std::allocator<std::string>> const&)
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::vector(std::vector<std::string,std::allocator<std::string>> const&)")]
pub fn stub_a514f0(src: &[String]) -> Vec<String> {
 // IDA 0xa514f0: empty init, then grow to the source length (throwing past
 // max_size 0x40000000) and copy each string; `to_vec` keeps that shape.
 src.to_vec()
}

// 0xa516a0 — __ZNSt6vectorISsSaISsEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS1_EERKSs
// demangled: std::vector<std::string,std::allocator<std::string>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,std::string const&)
// type: void __fastcall(struct _Unwind_Exception **, std::string *, const std::string *)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,std::string const&)")]
pub fn stub_a516a0(vec: &mut Vec<String>, pos: usize, value: String) {
 // IDA 0xa516a0: with spare capacity shift the tail back, otherwise
 // reallocate and copy around the new element; `insert` covers both arms.
 vec.insert(pos, value);
}

// 0xa51b40 — __ZN5boost14token_iteratorINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsEC2ES4_S9_S9_
// demangled: boost::token_iterator<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::token_iterator(boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::token_iterator<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::token_iterator(boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
pub fn stub_a51b40(kept: &str, dropped: &str, text: &str) -> CharTokenizer {
 // IDA 0xa51b40: copies both separators, then runs the separator predicate
 // once when begin != end to find the first token; that is a split.
 let is_sep = |c: char| kept.contains(c) || dropped.contains(c);
 let mut tokens = Vec::new();
 let mut cur = String::new();
 for c in text.chars() {
     if is_sep(c) {
         if !cur.is_empty() || kept.contains(c) {
             tokens.push(std::mem::take(&mut cur));
         }
     } else {
         cur.push(c);
     }
 }
 if !cur.is_empty() {
     tokens.push(cur);
 }
 CharTokenizer { kept: kept.to_owned(), dropped: dropped.to_owned(), tokens }
}

// 0xa51d24 — __ZN3RBX7Network11ChatMessageD2Ev
// demangled: RBX::Network::ChatMessage::~ChatMessage()
// type: void __fastcall(RBX::Network::ChatMessage *__hidden this)
#[doc(alias = "RBX::Network::ChatMessage::~ChatMessage()")]
pub fn stub_a51d24(msg: crate::player::ChatMessage) {
 // IDA 0xa51d24: releases the two strings at +0/+4 and the two shared
 // owners at +16/+24; the reduced `ChatMessage` drops the same owners.
 drop(msg);
}

// 0xa51ee0 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_
// demangled: std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::_M_insert_unique(std::pair<RBX::Guid::Data const,RBX::Instance *> const&)
// type: int __fastcall(int, _DWORD *, __int64 *)
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::_M_insert_unique(std::pair<RBX::Guid::Data const,RBX::Instance *> const&)")]
pub fn stub_a51ee0(map: &mut BTreeMap<u128, u32>, key: u128, value: u32) -> bool {
 // IDA 0xa51ee0: walk to the insertion point, return (it, false) when the
 // key exists, else allocate the node, rebalance, bump size, return
 // (it, true). Guid::Data (16 bytes) maps to u128, Instance* to the id;
 // rebalancing is intrinsic to BTreeMap. Reports the `inserted` half.
 use std::collections::btree_map::Entry;
 match map.entry(key) {
     Entry::Occupied(_) => false,
     Entry::Vacant(slot) => {
         slot.insert(value);
         true
     }
 }
}

// 0xa51fe4 — __ZN3RBX4HttpD2Ev
// demangled: RBX::Http::~Http()
// type: void __fastcall(RBX::Http *__hidden this)
#[doc(alias = "RBX::Http::~Http()")]
pub fn stub_a51fe4(http: crate::http::Http) {
 // IDA 0xa51fe4: erases the header map at +12, then releases the strings
 // at +0/+8/+36; see `Http::destroy`.
 http.destroy();
}

// 0xa52198 — __ZN10XmlElementD2Ev
// demangled: XmlElement::~XmlElement()
// type: void __fastcall(XmlElement *__hidden this)
#[doc(alias = "XmlElement::~XmlElement()")]
pub fn stub_a52198(attrs: Vec<XmlAttribute>) {
 // IDA 0xa52198: walks the attribute list at +28, clears each value, and
 // returns the nodes to the XmlAttribute singleton pool (pool engine-side);
 // dropping the vec releases the same values.
 drop(attrs);
}

// 0xa524a4 — __ZN10XmlElement12addAttributeISsEEvRKN3RBX4NameET_
// demangled: void XmlElement::addAttribute<std::string>(RBX::Name const&,std::string)
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "void XmlElement::addAttribute<std::string>(RBX::Name const&,std::string)")]
pub fn stub_a524a4(attrs: &mut Vec<XmlAttribute>, name: u32, value: String) {
 // IDA 0xa524a4: pool-allocates one 20-byte node, stores the name, the
 // kind tag 2 (string), and a copy of the value, then links it in.
 attrs.push(XmlAttribute { name, kind: 2, value });
}

// 0xa5275c — __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EEaSERKS5_
// demangled: std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::operator=(std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>> const&)
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::operator=(std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>> const&)")]
pub fn stub_a5275c(dst: &mut Vec<crate::player::ChatMessage>, src: &[crate::player::ChatMessage]) {
 // IDA 0xa5275c: destroys surplus tail nodes, copies over the overlap,
 // then inserts the remainder; `list` maps to Vec, node surgery elided.
 dst.clear();
 dst.extend(src.iter().cloned());
}

// 0xa52848 — __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_
// demangled: void std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::insert<std::_List_const_iterator<RBX::Network::AbuseReport::Message>>(std::_List_iterator<RBX::Network::AbuseReport::Message>,std::_List_const_iterator<RBX::Network::AbuseReport::Message>,std::_List_const_iterator<RBX::Network::AbuseReport::Message>)
// type: void __fastcall(int, std::_List_node_base *, int, int)
#[doc(alias = "void std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::insert<std::_List_const_iterator<RBX::Network::AbuseReport::Message>>(std::_List_iterator<RBX::Network::AbuseReport::Message>,std::_List_const_iterator<RBX::Network::AbuseReport::Message>,std::_List_const_iterator<RBX::Network::AbuseReport::Message>)")]
pub fn stub_a52848(dst: &mut Vec<crate::player::ChatMessage>, pos: usize, src: &[crate::player::ChatMessage]) {
 // IDA 0xa52848: builds the range in a temp list, splices it in at the
 // position, then destroys the temp shell; that is a range insert.
 dst.splice(pos..pos, src.iter().cloned());
}

// 0xa52ae4 — __ZNSt11_Deque_baseIN3RBX7Network11AbuseReportESaIS2_EE17_M_initialize_mapEm
// demangled: std::_Deque_base<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::_M_initialize_map(unsigned long)
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::_M_initialize_map(unsigned long)")]
pub fn stub_a52ae4(count: usize) -> VecDeque<crate::player::AbuseReport> {
 // IDA 0xa52ae4: sizes the chunk map for the element count and centers the
 // start/finish iterators; `with_capacity` keeps the observable shape.
 VecDeque::with_capacity(count)
}

// 0xa52cd0 — __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EEC2ERKS4_
// demangled: std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::deque(std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>> const&)
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::deque(std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>> const&)")]
pub fn stub_a52cd0(src: &VecDeque<crate::player::AbuseReport>) -> VecDeque<crate::player::AbuseReport> {
 // IDA 0xa52cd0: fresh map, then placement-copy each element chunk by chunk.
 src.clone()
}

// 0xa52e28 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX7Network11AbuseReportERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type
// demangled: std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>>(std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>,std::__false_type)
// type: void __fastcall(_DWORD *, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, _DWORD *, char, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>>(std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>,std::__false_type)")]
pub fn stub_a52e28(dst: &mut VecDeque<crate::player::AbuseReport>, src: &VecDeque<crate::player::AbuseReport>) {
 // IDA 0xa52e28: placement-new copy of the const-iterator range into
 // uninitialized deque storage; moves cover it, chunk pinning elided.
 dst.clear();
 dst.extend(src.iter().cloned());
}

// 0xa530c8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
pub fn stub_a530c8(map: &mut BTreeMap<String, GuiBuilderData>) {
 // IDA 0xa530c8: post-order walk from the node, destroying each pair and
 // freeing the node; whole-subtree erase is `clear`.
 map.clear();
}

// 0xa530f8 — __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEED2Ev
// demangled: std::pair<std::string const,RBX::GuiBuilder::Data>::~pair()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "std::pair<std::string const,RBX::GuiBuilder::Data>::~pair()")]
pub fn stub_a530f8(pair: (String, GuiBuilderData)) {
 // IDA 0xa530f8: releases the key string and each Data string, then the
 // node storage; dropping the pair releases the same owners.
 drop(pair);
}

// 0xa5327c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>>*)
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>>*)")]
pub fn stub_a5327c(src: &BTreeMap<String, GuiBuilderData>) -> BTreeMap<String, GuiBuilderData> {
 // IDA 0xa5327c: recursive deep copy of the subtree, allocating one node
 // per pair; balancing is intrinsic to BTreeMap.
 src.clone()
}

// 0xa533d0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_create_node(std::pair<std::string const,RBX::GuiBuilder::Data> const&)
// type: _DWORD *__fastcall(int, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, char, char, void *, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_create_node(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
pub fn stub_a533d0(value: (String, GuiBuilderData)) -> (String, GuiBuilderData) {
 // IDA 0xa533d0: allocates one node and copy-constructs the pair into it
 // (with the pool refcount bump); the owned pair is the node payload.
 value
}

// 0xa535ac — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::disconnectAll(void)
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::disconnectAll(void)")]
pub fn stub_a535ac(sig: &mut crate::signal::SlotList) {
 // IDA 0xa535ac: under the signal mutex, unlinks every slot (at most 11 per
 // pass, looping until the head is null) and releases the refs.
 sig.disconnect_all();
}

// 0xa53764 — __ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::disconnectAll(void)
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::disconnectAll(void)")]
pub fn stub_a53764(sig: &mut crate::signal::SlotList) {
 // IDA 0xa53764: same unlink-all loop as 0xa535ac for AbuseReport slots.
 sig.disconnect_all();
}

// 0xa5391c — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::disconnectAll(void)
// type: void __fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::disconnectAll(void)")]
pub fn stub_a5391c(sig: &mut crate::signal::SlotList) {
 // IDA 0xa5391c: same unlink-all loop for SystemAddress/BitStream slots;
 // see `SlotList::disconnect_all`.
 sig.disconnect_all();
}

// 0xa53ad4 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// demangled: std::_Rb_tree<int,std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>,std::_Select1st<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>,std::_Select1st<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>> *)")]
pub fn stub_a53ad4(map: &mut BTreeMap<i32, BTreeSet<String>>) {
 // IDA 0xa53ad4: post-order walk erasing each inner string set before
 // freeing the node; whole-subtree erase is `clear`.
 map.clear();
}

// 0xa53b04 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED2Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_a53b04(desc: PlayersBoundFuncDesc) {
 // IDA 0xa53b04: frees the bound Instance holder at +48, then unlinks and
 // frees each listener node from the list at +32; drop covers both.
 drop(desc);
}

// 0xa53c54 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED2Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
pub fn stub_a53c54(desc: PlayersBoundFuncDesc) {
 // IDA 0xa53c54: frees the two bound strings at +56/+52 and the bound
 // Instance holder at +48, then drains the listener list at +32.
 drop(desc);
}

// 0xa53e38 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED2Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_a53e38(desc: PlayersBoundFuncDesc) {
 // IDA 0xa53e38: frees the bound Instance holder at +52 and the bound
 // string at +48, then drains the listener list at +32.
 drop(desc);
}

// 0xa54018 — __GLOBAL__I_a_510
// demangled: global constructor keyed to_a_510
// type: int()
#[doc(alias = "global constructor keyed to_a_510")]
pub fn stub_a54018() {
    // IDA 0xa54018: static initializer registration (runs before main).
}
// 0xa5533c — __ZN6RakNet9BitStreamC1Ev
// demangled: RakNet::BitStream::BitStream(void)
// type: int __fastcall(int this)
#[doc(alias = "RakNet::BitStream::BitStream(void)")]
pub fn stub_a5533c() -> crate::bitstream::BitStream {
 // IDA 0xa5533c: default construct.
 crate::bitstream::BitStream::new()
}

// 0xa55354 — __ZN6RakNet9BitStreamC1Ej
// demangled: RakNet::BitStream::BitStream(unsigned int)
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, RakNet *)
#[doc(alias = "RakNet::BitStream::BitStream(unsigned int)")]
pub fn stub_a55354(bits: usize) -> crate::bitstream::BitStream {
 // IDA 0xa55354: capacity hint only.
 crate::bitstream::BitStream::with_capacity_bits(bits)
}

// 0xa553a0 — __ZN6RakNet9BitStreamC1EPhjb
// demangled: RakNet::BitStream::BitStream(unsigned char *,unsigned int,bool)
// type: int __fastcall(int this, unsigned __int8 *__src, unsigned int, int)
#[doc(alias = "RakNet::BitStream::BitStream(unsigned char *,unsigned int,bool)")]
pub fn stub_a553a0(bytes: &[u8], _copy: bool) -> crate::bitstream::BitStream {
 // IDA 0xa553a0: copy or view; the view lifetime is unmodelable so this copies.
 crate::bitstream::BitStream::from_bytes(bytes)
}

// 0xa55408 — __ZN6RakNet9BitStreamD1Ev
// demangled: RakNet::BitStream::~BitStream()
// type: void __fastcall(RakNet::BitStream *__hidden this)
#[doc(alias = "RakNet::BitStream::~BitStream()")]
pub fn stub_a55408(stream: crate::bitstream::BitStream) {
 // IDA 0xa55408: frees the buffer; Rust drops it.
 drop(stream);
}

// 0xa55440 — __ZN6RakNet9BitStream5ResetEv
// demangled: RakNet::BitStream::Reset(void)
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::BitStream::Reset(void)")]
pub fn stub_a55440(stream: &mut crate::bitstream::BitStream) {
 // IDA 0xa55440: both cursors to zero.
 stream.reset()
}

// 0xa55448 — __ZN6RakNet9BitStream5WriteEPKcj
// demangled: RakNet::BitStream::Write(char const*,unsigned int)
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, const char *, size_t __n)
#[doc(alias = "RakNet::BitStream::Write(char const*,unsigned int)")]
pub fn stub_a55448(stream: &mut crate::bitstream::BitStream, bytes: &[u8]) {
 // IDA 0xa55448: byte append.
 stream.write_bytes(bytes)
}

// 0xa55534 — __ZN6RakNet9BitStream20AddBitsAndReallocateEj
// demangled: RakNet::BitStream::AddBitsAndReallocate(unsigned int)
// type: unsigned int __fastcall(RakNet::BitStream *this, unsigned int)
#[doc(alias = "RakNet::BitStream::AddBitsAndReallocate(unsigned int)")]
pub fn stub_a55534(stream: &mut crate::bitstream::BitStream, bits: usize) {
 // IDA 0xa55534: capacity reservation.
 stream.add_bits_and_reallocate(bits)
}

// 0xa555e0 — __ZN6RakNet9BitStream9WriteBitsEPKhjb
// demangled: RakNet::BitStream::WriteBits(unsigned char const*,unsigned int,bool)
// type: unsigned int __fastcall(RakNet::BitStream *this, const unsigned __int8 *__src, unsigned int, int)
#[doc(alias = "RakNet::BitStream::WriteBits(unsigned char const*,unsigned int,bool)")]
pub fn stub_a555e0(stream: &mut crate::bitstream::BitStream, bytes: &[u8], count: usize) {
 // IDA 0xa555e0: low bits out MSB-first.
 stream.write_raw_bits(bytes, count)
}

// 0xa557e0 — __ZN6RakNet9BitStream5WriteEPS0_j
// demangled: RakNet::BitStream::Write(RakNet::BitStream*,unsigned int)
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *, unsigned int)
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream*,unsigned int)")]
pub fn stub_a557e0(stream: &mut crate::bitstream::BitStream, src: &mut crate::bitstream::BitStream, count: usize) {
 // IDA 0xa557e0: splice bits, consuming the source.
 stream.write_stream_bits(src, count)
}

// 0xa55940 — __ZN6RakNet9BitStream5WriteERS0_j
// demangled: RakNet::BitStream::Write(RakNet::BitStream&,unsigned int)
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *, unsigned int)
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream&,unsigned int)")]
pub fn stub_a55940(stream: &mut crate::bitstream::BitStream, src: &mut crate::bitstream::BitStream, count: usize) {
 // IDA 0xa55940: splice bits, consuming the source.
 stream.write_stream_bits(src, count)
}

// 0xa5594c — __ZN6RakNet9BitStream5WriteERS0_
// demangled: RakNet::BitStream::Write(RakNet::BitStream&)
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *)
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream&)")]
pub fn stub_a5594c(stream: &mut crate::bitstream::BitStream, src: &mut crate::bitstream::BitStream) {
 // IDA 0xa5594c: splice the remaining bits.
 stream.write_remaining_stream(src)
}

// 0xa5595c — __ZN6RakNet9BitStream4ReadEPcj
// demangled: RakNet::BitStream::Read(char *,unsigned int)
// type: int __fastcall(RakNet::BitStream *this, char *__dst, size_t)
#[doc(alias = "RakNet::BitStream::Read(char *,unsigned int)")]
pub fn stub_a5595c(stream: &mut crate::bitstream::BitStream, out: &mut [u8]) -> bool {
 // IDA 0xa5595c: byte block, nothing consumed on failure.
 stream.read_bytes(out)
}

// 0xa559a0 — __ZN6RakNet9BitStream8ReadBitsEPhjb
// demangled: RakNet::BitStream::ReadBits(unsigned char *,unsigned int,bool)
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *__b, unsigned int, int)
#[doc(alias = "RakNet::BitStream::ReadBits(unsigned char *,unsigned int,bool)")]
pub fn stub_a559a0(stream: &mut crate::bitstream::BitStream, out: &mut [u8], count: usize) -> bool {
 // IDA 0xa559a0: raw bit window, nothing consumed on failure.
 stream.read_raw_bits(out, count)
}

// 0xa55a70 — __ZN6RakNet9BitStream17ResetWritePointerEv
// demangled: RakNet::BitStream::ResetWritePointer(void)
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::BitStream::ResetWritePointer(void)")]
pub fn stub_a55a70(stream: &mut crate::bitstream::BitStream) {
 // IDA 0xa55a70: write cursor to zero.
 stream.reset_write_pointer()
}

// 0xa55a78 — __ZN6RakNet9BitStream6Write0Ev
// demangled: RakNet::BitStream::Write0(void)
// type: unsigned int __fastcall(RakNet::BitStream *this)
#[doc(alias = "RakNet::BitStream::Write0(void)")]
pub fn stub_a55a78(stream: &mut crate::bitstream::BitStream) {
 // IDA 0xa55a78: Write0.
 stream.write_bit(false)
}

// 0xa55b40 — __ZN6RakNet9BitStream6Write1Ev
// demangled: RakNet::BitStream::Write1(void)
// type: int __fastcall(RakNet::BitStream *this)
#[doc(alias = "RakNet::BitStream::Write1(void)")]
pub fn stub_a55b40(stream: &mut crate::bitstream::BitStream) {
 // IDA 0xa55b40: Write1.
 stream.write_bit(true)
}

// 0xa55c18 — __ZN6RakNet9BitStream7ReadBitEv
// demangled: RakNet::BitStream::ReadBit(void)
// type: bool __fastcall(RakNet::BitStream *this)
#[doc(alias = "RakNet::BitStream::ReadBit(void)")]
pub fn stub_a55c18(stream: &mut crate::bitstream::BitStream) -> Option<bool> {
 // IDA 0xa55c18: ReadBit.
 stream.read_bit()
}

// 0xa55c38 — __ZN6RakNet9BitStream17WriteAlignedBytesEPKhj
// demangled: RakNet::BitStream::WriteAlignedBytes(unsigned char const*,unsigned int)
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, const char *, size_t)
#[doc(alias = "RakNet::BitStream::WriteAlignedBytes(unsigned char const*,unsigned int)")]
pub fn stub_a55c38(stream: &mut crate::bitstream::BitStream, bytes: &[u8]) {
 // IDA 0xa55c38: align-up then raw bytes.
 stream.write_aligned_bytes(bytes)
}

// 0xa55c58 — __ZN6RakNet9BitStream16ReadAlignedBytesEPhj
// demangled: RakNet::BitStream::ReadAlignedBytes(unsigned char *,unsigned int)
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *__dst, size_t)
#[doc(alias = "RakNet::BitStream::ReadAlignedBytes(unsigned char *,unsigned int)")]
pub fn stub_a55c58(stream: &mut crate::bitstream::BitStream, out: &mut [u8]) -> bool {
 // IDA 0xa55c58: align-up then raw bytes.
 stream.read_aligned_bytes(out)
}

// 0xa55c9c — __ZN6RakNet9BitStream15WriteCompressedEPKhjb
// demangled: RakNet::BitStream::WriteCompressed(unsigned char const*,unsigned int,bool)
// type: unsigned int __fastcall(RakNet::BitStream *this, const unsigned __int8 *, unsigned int, int)
#[doc(alias = "RakNet::BitStream::WriteCompressed(unsigned char const*,unsigned int,bool)")]
pub fn stub_a55c9c(stream: &mut crate::bitstream::BitStream, data: &[u8]) {
    // IDA 0xa55c9c: core `WriteCompressed(src, nbits, rightAligned = 1)` — `Write1` per trailing zero byte, then the head.
    stream.write_compressed_raw(data);
}

// 0xa55d2c — __ZN6RakNet9BitStream14ReadCompressedEPhjb
// demangled: RakNet::BitStream::ReadCompressed(unsigned char *,unsigned int,bool)
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *__b, unsigned int, int)
#[doc(alias = "RakNet::BitStream::ReadCompressed(unsigned char *,unsigned int,bool)")]
pub fn stub_a55d2c(stream: &mut crate::bitstream::BitStream, out: &mut [u8]) -> bool {
    // IDA 0xa55d2c: core `ReadCompressed` — elided zero bytes read back as zero; `false` on short reads.
    stream.read_compressed_raw(out)
}

// 0xa55e08 — __ZNK6RakNet9BitStream24GetNumberOfBitsAllocatedEv
// demangled: RakNet::BitStream::GetNumberOfBitsAllocated(void)const
// type: int __fastcall(RakNet::BitStream *this)
#[doc(alias = "RakNet::BitStream::GetNumberOfBitsAllocated(void)const")]
pub fn stub_a55e08(stream: &crate::bitstream::BitStream) -> usize {
 // IDA 0xa55e08: allocated bit count.
 stream.bits_allocated()
}

// 0xa55e0c — __ZN6RakNet9BitStream23PadWithZeroToByteLengthEj
// demangled: RakNet::BitStream::PadWithZeroToByteLength(unsigned int)
// type: int __fastcall(RakNet::BitStream *this, unsigned int)
#[doc(alias = "RakNet::BitStream::PadWithZeroToByteLength(unsigned int)")]
pub fn stub_a55e0c(stream: &mut crate::bitstream::BitStream, len: usize) {
 // IDA 0xa55e0c: zero-fill to length, never shrinks.
 stream.pad_with_zero_to_byte_length(len)
}

// 0xa55ef0 — __ZNK6RakNet9BitStream8CopyDataEPPh
// demangled: RakNet::BitStream::CopyData(unsigned char **)const
// type: const void *__fastcall(const void **this, unsigned __int8 **)
#[doc(alias = "RakNet::BitStream::CopyData(unsigned char **)const")]
pub fn stub_a55ef0(stream: &crate::bitstream::BitStream) -> Vec<u8> {
 // IDA 0xa55ef0: the used bytes.
 stream.copy_data()
}

// 0xa55f30 — __ZN6RakNet9BitStream10IgnoreBitsEj
// demangled: RakNet::BitStream::IgnoreBits(unsigned int)
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::BitStream::IgnoreBits(unsigned int)")]
pub fn stub_a55f30(stream: &mut crate::bitstream::BitStream, count: usize) {
 // IDA 0xa55f30: skip the read cursor ahead.
 stream.ignore_bits(count)
}

// 0xa55f38 — __ZN6RakNet9BitStream11IgnoreBytesEj
// demangled: RakNet::BitStream::IgnoreBytes(unsigned int)
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::BitStream::IgnoreBytes(unsigned int)")]
pub fn stub_a55f38(stream: &mut crate::bitstream::BitStream, count: usize) {
 // IDA 0xa55f38: skip whole bytes ahead.
 stream.ignore_bytes(count)
}

// 0xa55f44 — __ZN6RakNet9BitStream14SetWriteOffsetEj
// demangled: RakNet::BitStream::SetWriteOffset(unsigned int)
// type: _DWORD *__fastcall(_DWORD *this, unsigned int)
#[doc(alias = "RakNet::BitStream::SetWriteOffset(unsigned int)")]
pub fn stub_a55f44(stream: &mut crate::bitstream::BitStream, bits: usize) {
 // IDA 0xa55f44: reposition the write cursor.
 stream.set_write_offset(bits)
}

// 0xa55f48 — __ZN6RakNet9BitStream22IsNetworkOrderInternalEv
// demangled: RakNet::BitStream::IsNetworkOrderInternal(void)
// type: int __fastcall(RakNet::BitStream *this)
#[doc(alias = "RakNet::BitStream::IsNetworkOrderInternal(void)")]
pub fn stub_a55f48() -> bool {
 // IDA 0xa55f48: internal check returns 0.
 crate::bitstream::BitStream::is_network_order()
}

// 0xa55f4c — __ZN6RakNet9BitStream12ReverseBytesEPhS1_j
// demangled: RakNet::BitStream::ReverseBytes(unsigned char *,unsigned char *,unsigned int)
// type: int __fastcall(int this, unsigned __int8 *, unsigned __int8 *, unsigned int)
#[doc(alias = "RakNet::BitStream::ReverseBytes(unsigned char *,unsigned char *,unsigned int)")]
pub fn stub_a55f4c(data: &[u8]) -> Vec<u8> {
 // IDA 0xa55f4c: byte reversal into a fresh buffer.
 crate::bitstream::reverse_bytes(data)
}

// 0xa55f64 — __ZN6RakNet9BitStream16WriteAlignedVar8EPKc
// demangled: RakNet::BitStream::WriteAlignedVar8(char const*)
// type: int __fastcall(RakNet::BitStream *this, const char *)
#[doc(alias = "RakNet::BitStream::WriteAlignedVar8(char const*)")]
pub fn stub_a55f64(stream: &mut crate::bitstream::BitStream, value: u8) {
 // IDA 0xa55f64: direct byte store.
 stream.write_aligned_var8(value)
}

// 0xa5602c — __ZN6RakNet9BitStream15ReadAlignedVar8EPc
// demangled: RakNet::BitStream::ReadAlignedVar8(char *)
// type: int __fastcall(RakNet::BitStream *this, char *)
#[doc(alias = "RakNet::BitStream::ReadAlignedVar8(char *)")]
pub fn stub_a5602c(stream: &mut crate::bitstream::BitStream) -> Option<u8> {
 // IDA 0xa5602c: direct byte load.
 stream.read_aligned_var8()
}

// 0xa56050 — __ZN6RakNet9BitStream17WriteAlignedVar16EPKc
// demangled: RakNet::BitStream::WriteAlignedVar16(char const*)
// type: int __fastcall(RakNet::BitStream *this, char *)
#[doc(alias = "RakNet::BitStream::WriteAlignedVar16(char const*)")]
pub fn stub_a56050(stream: &mut crate::bitstream::BitStream, value: u16) {
 // IDA 0xa56050: big-endian pair, direct store.
 stream.write_aligned_var16(value)
}

// 0xa5617c — __ZN6RakNet9BitStream16ReadAlignedVar16EPc
// demangled: RakNet::BitStream::ReadAlignedVar16(char *)
// type: int __fastcall(RakNet::BitStream *this, char *)
#[doc(alias = "RakNet::BitStream::ReadAlignedVar16(char *)")]
pub fn stub_a5617c(stream: &mut crate::bitstream::BitStream) -> Option<u16> {
 // IDA 0xa5617c: big-endian pair, direct load.
 stream.read_aligned_var16()
}

// 0xa5620c — __ZN6RakNet9BitStream17WriteAlignedVar32EPKc
// demangled: RakNet::BitStream::WriteAlignedVar32(char const*)
// type: int __fastcall(RakNet::BitStream *this, char *)
#[doc(alias = "RakNet::BitStream::WriteAlignedVar32(char const*)")]
pub fn stub_a5620c(stream: &mut crate::bitstream::BitStream, value: u32) {
 // IDA 0xa5620c: big-endian quad, direct store.
 stream.write_aligned_var32(value)
}

// 0xa56378 — __ZN6RakNet9BitStream16ReadAlignedVar32EPc
// demangled: RakNet::BitStream::ReadAlignedVar32(char *)
// type: int __fastcall(RakNet::BitStream *this, char *)
#[doc(alias = "RakNet::BitStream::ReadAlignedVar32(char *)")]
pub fn stub_a56378(stream: &mut crate::bitstream::BitStream) -> Option<u32> {
 // IDA 0xa56378: big-endian quad, direct load.
 stream.read_aligned_var32()
}

// 0xa56438 — __ZN6RakNet9BitStream11ReadFloat16ERfff
// demangled: RakNet::BitStream::ReadFloat16(float &,float,float)
// type: int __fastcall(RakNet::BitStream *this, float *, float, float)
#[doc(alias = "RakNet::BitStream::ReadFloat16(float &,float,float)")]
pub fn stub_a56438(stream: &mut crate::bitstream::BitStream, min: f32, max: f32) -> Option<f32> {
 // IDA 0xa56438: normalized 16-bit float.
 stream.read_float16(min, max)
}

// 0xa5653c — __ZN6RakNet9BitStream12WriteFloat16Efff
// demangled: RakNet::BitStream::WriteFloat16(float,float,float)
// type: unsigned int __fastcall(RakNet::BitStream *this, float32_t, float32_t, float32_t)
#[doc(alias = "RakNet::BitStream::WriteFloat16(float,float,float)")]
pub fn stub_a5653c(stream: &mut crate::bitstream::BitStream, value: f32, min: f32, max: f32) {
 // IDA 0xa5653c: normalized 16-bit float.
 stream.write_float16(value, min, max)
}

// 0xa565f0 — __GLOBAL__I_a_511
// demangled: global constructor keyed to_a_511
#[doc(alias = "global constructor keyed to_a_511")]
pub fn stub_a565f0() {
    // IDA 0xa565f0: static initializer registration (runs before main).
}
// 0xa56c10 — __ZN8CheckSum3AddEPhj
// demangled: CheckSum::Add(unsigned char *,unsigned int)
// type: unsigned __int16 *__fastcall(unsigned __int16 *this, unsigned __int8 *, unsigned int)
#[doc(alias = "CheckSum::Add(unsigned char *,unsigned int)")]
pub fn stub_a56c10(sum: &mut CheckSum, data: &[u8]) {
 // IDA 0xa56c10: per byte `t = b ^ (sum >> 8)`, `total += t`, then
 // `sum = mult_b + (t + sum) * mult_a` (disasm MLA, halfword ops); the
 // empty-range early-out is the loop not running.
 for &b in data {
     let t = (b ^ (sum.sum >> 8) as u8) as u16;
     sum.total = sum.total.wrapping_add(t as u32);
     sum.sum = sum.mult_b.wrapping_add(t.wrapping_add(sum.sum).wrapping_mul(sum.mult_a));
 }
}

// 0xa56c4c — __GLOBAL__I_a_512
// demangled: global constructor keyed to_a_512
#[doc(alias = "global constructor keyed to_a_512")]
pub fn stub_a56c4c() {
    // IDA 0xa56c4c: static initializer registration (runs before main).
}
// 0xa57260 — __GLOBAL__I_a_513
// demangled: global constructor keyed to_a_513
#[doc(alias = "global constructor keyed to_a_513")]
pub fn stub_a57260() {
    // IDA 0xa57260: static initializer registration (runs before main).
}
// 0xa57874 — __ZN6RakNet19HuffmanEncodingTreeC1Ev
// demangled: RakNet::HuffmanEncodingTree::HuffmanEncodingTree(void)
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::HuffmanEncodingTree::HuffmanEncodingTree(void)")]
pub fn stub_a57874() -> crate::huffman::HuffmanTree {
 // IDA 0xa57874: default construct.
 crate::huffman::HuffmanTree::new()
}

// 0xa5787c — __ZN6RakNet19HuffmanEncodingTreeD1Ev
// demangled: RakNet::HuffmanEncodingTree::~HuffmanEncodingTree()
// type: void __fastcall(RakNet::HuffmanEncodingTree *__hidden this)
#[doc(alias = "RakNet::HuffmanEncodingTree::~HuffmanEncodingTree()")]
pub fn stub_a5787c(tree: crate::huffman::HuffmanTree) {
 // IDA 0xa5787c: frees the tree; Rust drops it.
 drop(tree);
}

// 0xa5788c — __ZN6RakNet19HuffmanEncodingTree10FreeMemoryEv
// demangled: RakNet::HuffmanEncodingTree::FreeMemory(void)
// type: void __fastcall(RakNet::HuffmanEncodingTree *this)
#[doc(alias = "RakNet::HuffmanEncodingTree::FreeMemory(void)")]
pub fn stub_a5788c(tree: &mut crate::huffman::HuffmanTree) {
 // IDA 0xa5788c: release the tree.
 tree.clear()
}

// 0xa57a3c — __ZN6RakNet19HuffmanEncodingTree26GenerateFromFrequencyTableEPj
// demangled: RakNet::HuffmanEncodingTree::GenerateFromFrequencyTable(unsigned int *)
// type: int __fastcall(RakNet::HuffmanEncodingTree *this, unsigned int *)
#[doc(alias = "RakNet::HuffmanEncodingTree::GenerateFromFrequencyTable(unsigned int *)")]
pub fn stub_a57a3c(tree: &mut crate::huffman::HuffmanTree, freq: &[u32; 256]) {
 // IDA 0xa57a3c: build from the frequency table.
 tree.generate(freq)
}

// 0xa58090 — __ZN6RakNet19HuffmanEncodingTree11EncodeArrayEPhmPNS_9BitStreamE
// demangled: RakNet::HuffmanEncodingTree::EncodeArray(unsigned char *,unsigned long,RakNet::BitStream *)
// type: const unsigned __int8 **__fastcall(const unsigned __int8 **this, unsigned __int8 *, unsigned int, RakNet::BitStream *)
#[doc(alias = "RakNet::HuffmanEncodingTree::EncodeArray(unsigned char *,unsigned long,RakNet::BitStream *)")]
pub fn stub_a58090(tree: &crate::huffman::HuffmanTree, stream: &mut crate::bitstream::BitStream, data: &[u8]) {
 // IDA 0xa58090: code bits plus pad.
 tree.encode(stream, data)
}

// 0xa580f0 — __ZN6RakNet19HuffmanEncodingTree11DecodeArrayEPNS_9BitStreamEjmPh
// demangled: RakNet::HuffmanEncodingTree::DecodeArray(RakNet::BitStream *,unsigned int,unsigned long,unsigned char *)
// type: unsigned int __fastcall(RakNet::HuffmanEncodingTree *this, RakNet::BitStream *, unsigned int, unsigned int, unsigned __int8 *)
#[doc(alias = "RakNet::HuffmanEncodingTree::DecodeArray(RakNet::BitStream *,unsigned int,unsigned long,unsigned char *)")]
pub fn stub_a580f0(tree: &crate::huffman::HuffmanTree, stream: &mut crate::bitstream::BitStream, max_bits: usize, out: &mut [u8]) -> usize {
 // IDA 0xa580f0: walk to leaves, count symbols.
 tree.decode(stream, max_bits, out)
}

// 0xa58150 — __ZN14DataStructures5QueueIP23HuffmanEncodingTreeNodeE4PushERKS2_PKcj
// demangled: DataStructures::Queue<HuffmanEncodingTreeNode *>::Push(HuffmanEncodingTreeNode * const&,char const*,unsigned int)
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<HuffmanEncodingTreeNode *>::Push(HuffmanEncodingTreeNode * const&,char const*,unsigned int)")]
pub fn stub_a58150(queue: &mut VecDeque<u32>, node: u32) {
 // IDA 0xa58150: circular-buffer push (16-entry first alloc at 0xa581fc,
 // doubling realloc preserving order at 0xa5819e..0xa581f4); growth is
 // intrinsic to VecDeque, node identity maps to the registry id.
 queue.push_back(node);
}

// 0xa58224 — __GLOBAL__I_a_514
// demangled: global constructor keyed to_a_514
#[doc(alias = "global constructor keyed to_a_514")]
pub fn stub_a58224() {
    // IDA 0xa58224: static initializer registration (runs before main).
}
// 0xa58844 — __ZN6RakNet7GetTimeEv
// demangled: RakNet::GetTime(void)
// type: unsigned __int64 __fastcall(RakNet *this)
#[doc(alias = "RakNet::GetTime(void)")]
pub fn stub_a58844() -> u64 {
 // IDA 0xa58844: milliseconds since first call.
 crate::time::raknet_time_ms()
}

// 0xa588c4 — __ZN6RakNet9GetTimeUSEv
// demangled: RakNet::GetTimeUS(void)
// type: __int64 __fastcall(RakNet *this)
#[doc(alias = "RakNet::GetTimeUS(void)")]
pub fn stub_a588c4() -> u64 {
 // IDA 0xa588c4: microseconds since first call.
 crate::time::raknet_time_us()
}

// 0xa58938 — __ZN6RakNet9GetTimeMSEv
// demangled: RakNet::GetTimeMS(void)
// type: unsigned __int64 __fastcall(RakNet *this)
#[doc(alias = "RakNet::GetTimeMS(void)")]
pub fn stub_a58938() -> u64 {
 // IDA 0xa58938: milliseconds since first call.
 crate::time::raknet_time_millis()
}

// 0xa589b8 — __GLOBAL__I_a_515
// demangled: global constructor keyed to_a_515
#[doc(alias = "global constructor keyed to_a_515")]
pub fn stub_a589b8() {
    // IDA 0xa589b8: static initializer registration (runs before main).
}
// 0xa58fcc — _Itoa
// demangled: _Itoa
// type: _BYTE *__fastcall(int, _BYTE *, int)
#[doc(alias = "_Itoa")]
pub fn stub_a58fcc(value: i32, radix: u32) -> String {
 // IDA 0xa58fcc: radix outside 2..=16 yields the empty string; otherwise
 // repeated div/mod digits (negated for the table), a '-' for negative
 // base-10 values, then an in-place reverse of the digit run.
 if !(2..=16).contains(&radix) {
     return String::new();
 }
 const DIGITS: &[u8; 16] = b"0123456789abcdef";
 let mut out: Vec<u8> = Vec::new();
 let mut v = value;
 loop {
     let d = (v % radix as i32).abs() as usize;
     out.push(DIGITS[d]);
     v /= radix as i32;
     if v == 0 {
         break;
     }
 }
 if value < 0 && radix == 10 {
     out.push(b'-');
 }
 out.reverse();
 String::from_utf8(out).unwrap_or_default()
}

// 0xa59064 — __GLOBAL__I_a_516
// demangled: global constructor keyed to_a_516
#[doc(alias = "global constructor keyed to_a_516")]
pub fn stub_a59064() {
    // IDA 0xa59064: static initializer registration (runs before main).
}
// 0xa59678 — __GLOBAL__I_a_517
// demangled: global constructor keyed to_a_517
#[doc(alias = "global constructor keyed to_a_517")]
pub fn stub_a59678() {
    // IDA 0xa59678: static initializer registration (runs before main).
}
// 0xa59c8c — __GLOBAL__I_a_518
// demangled: global constructor keyed to_a_518
#[doc(alias = "global constructor keyed to_a_518")]
pub fn stub_a59c8c() {
    // IDA 0xa59c8c: static initializer registration (runs before main).
}
// 0xa5a2ac — __ZN6RakNet16PluginInterface2C2Ev
// demangled: RakNet::PluginInterface2::PluginInterface2(void)
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::PluginInterface2::PluginInterface2(void)")]
pub fn stub_a5a2ac() -> crate::socket::PluginInterface2 {
 // IDA 0xa5a2ac: default construct.
 crate::socket::PluginInterface2::new()
}

// 0xa5a2c4 — __ZN6RakNet16PluginInterface2D0Ev
// demangled: RakNet::PluginInterface2::~PluginInterface2()
// type: void __fastcall(RakNet::PluginInterface2 *__hidden this)
#[doc(alias = "RakNet::PluginInterface2::~PluginInterface2()")]
pub fn stub_a5a2c4(plugin: crate::socket::PluginInterface2) {
 // IDA 0xa5a2c4: frees; Rust drops it.
 drop(plugin);
}

// 0xa5a2d0 — __ZN6RakNet16PluginInterface2D1Ev
// demangled: RakNet::PluginInterface2::~PluginInterface2()
// type: void __fastcall(RakNet::PluginInterface2 *__hidden this)
#[doc(alias = "RakNet::PluginInterface2::~PluginInterface2()")]
pub fn stub_a5a2d0(plugin: crate::socket::PluginInterface2) {
 // IDA 0xa5a2d0: frees; Rust drops it.
 drop(plugin);
}

// 0xa5a2d4 — __ZN6RakNet16PluginInterface2D2Ev
// demangled: RakNet::PluginInterface2::~PluginInterface2()
// type: void __fastcall(RakNet::PluginInterface2 *__hidden this)
#[doc(alias = "RakNet::PluginInterface2::~PluginInterface2()")]
pub fn stub_a5a2d4(plugin: crate::socket::PluginInterface2) {
 // IDA 0xa5a2d4: frees; Rust drops it.
 drop(plugin);
}

// 0xa5a2d8 — __ZN6RakNet16PluginInterface219SetRakPeerInterfaceEPNS_16RakPeerInterfaceE
// demangled: RakNet::PluginInterface2::SetRakPeerInterface(RakNet::RakPeerInterface *)
// type: int __fastcall(int result, int)
#[doc(alias = "RakNet::PluginInterface2::SetRakPeerInterface(RakNet::RakPeerInterface *)")]
pub fn stub_a5a2d8(plugin: &mut crate::socket::PluginInterface2, peer: Option<u32>) {
 // IDA 0xa5a2d8: store the peer handle.
 plugin.set_rak_peer_interface(peer)
}

// 0xa5a2dc — __GLOBAL__I_a_519
// demangled: global constructor keyed to_a_519
#[doc(alias = "global constructor keyed to_a_519")]
pub fn stub_a5a2dc() {
    // IDA 0xa5a2dc: static initializer registration (runs before main).
}
// 0xa5a8fc — __Z25DefaultOutOfMemoryHandlerPKcl
// demangled: DefaultOutOfMemoryHandler(char const*,long)
// type: void __fastcall(const char *, int)
#[doc(alias = "DefaultOutOfMemoryHandler(char const*,long)")]
pub fn stub_a5a8fc(_file: Option<&str>, _line: i32) {
 // IDA 0xa5a8fc: empty body, returns immediately.
 // BUG: the OOM handler ignores the out-of-memory condition instead of
 // aborting or freeing emergency memory, so allocation failure silently
 // continues into likely-corrupt state.
}

// 0xa5a900 — __ZN6RakNet13_RakMalloc_ExEmPKcj
// demangled: RakNet::_RakMalloc_Ex(unsigned long,char const*,unsigned int)
// type: void *__fastcall(RakNet *this, unsigned int, const char *, unsigned int)
#[doc(alias = "RakNet::_RakMalloc_Ex(unsigned long,char const*,unsigned int)")]
pub fn stub_a5a900(size: usize) -> Vec<u8> {
 // IDA 0xa5a900: zeroed allocation.
 crate::socket::rak_malloc(size)
}

// 0xa5a90c — __ZN6RakNet14_RakRealloc_ExEPvmPKcj
// demangled: RakNet::_RakRealloc_Ex(void *,unsigned long,char const*,unsigned int)
// type: void *__fastcall(RakNet *this, size_t, unsigned int, const char *, unsigned int)
#[doc(alias = "RakNet::_RakRealloc_Ex(void *,unsigned long,char const*,unsigned int)")]
pub fn stub_a5a90c(buf: Vec<u8>, size: usize) -> Vec<u8> {
 // IDA 0xa5a90c: resize, tails zero-fill.
 crate::socket::rak_realloc(buf, size)
}

// 0xa5a918 — __ZN6RakNet11_RakFree_ExEPvPKcj
// demangled: RakNet::_RakFree_Ex(void *,char const*,unsigned int)
// type: void __fastcall(RakNet *this, void *, const char *, unsigned int)
#[doc(alias = "RakNet::_RakFree_Ex(void *,char const*,unsigned int)")]
pub fn stub_a5a918(buf: Vec<u8>) {
 // IDA 0xa5a918: free.
 crate::socket::rak_free(buf)
}

// 0xa5a924 — __GLOBAL__I_a_520
// demangled: global constructor keyed to_a_520
#[doc(alias = "global constructor keyed to_a_520")]
pub fn stub_a5a924() {
    // IDA 0xa5a924: static initializer registration (runs before main).
}
// 0xa5af38 — __ZN6RakNet12RakNetSocketC1Ev
// demangled: RakNet::RakNetSocket::RakNetSocket(void)
// type: RakNet::RakNetSocket *__fastcall(RakNet::RakNetSocket *this)
#[doc(alias = "RakNet::RakNetSocket::RakNetSocket(void)")]
pub fn stub_a5af38() {
 // IDA 0xa5af38: descriptor init stays engine-side.
 crate::socket::init_raknet_socket()
}

// 0xa5af50 — __ZN6RakNet12RakNetSocketD1Ev
// demangled: RakNet::RakNetSocket::~RakNetSocket()
// type: void __fastcall(RakNet::RakNetSocket *__hidden this)
#[doc(alias = "RakNet::RakNetSocket::~RakNetSocket()")]
pub fn stub_a5af50() {
 // IDA 0xa5af50: descriptor close stays engine-side.
 crate::socket::free_raknet_socket()
}

// 0xa5af7c — __GLOBAL__I_a_521
// demangled: global constructor keyed to_a_521
#[doc(alias = "global constructor keyed to_a_521")]
pub fn stub_a5af7c() {
    // IDA 0xa5af7c: static initializer registration (runs before main).
}
// 0xa5b5b0 — __ZN6RakNet18StatisticsToStringEPKNS_16RakNetStatisticsEPci
// demangled: RakNet::StatisticsToString(RakNet::RakNetStatistics const*,char *,int)
// type: int __fastcall(unsigned int *, char *, int)
#[doc(alias = "RakNet::StatisticsToString(RakNet::RakNetStatistics const*,char *,int)")]
pub fn stub_a5b5b0(present: bool, verbose: u32, sent_per_sec: u64, received_per_sec: u64, packetloss: f32, full: &mut dyn FnMut() -> String) -> String {
 // IDA 0xa5b5b0: null/brief/full verbosity arms.
 crate::socket::statistics_to_string(present, verbose, sent_per_sec, received_per_sec, packetloss, full)
}

// 0xa5b9cc — __GLOBAL__I_a_522
// demangled: global constructor keyed to_a_522
#[doc(alias = "global constructor keyed to_a_522")]
pub fn stub_a5b9cc() {
    // IDA 0xa5b9cc: static initializer registration (runs before main).
}
// 0xa5bfec — __ZN6RakNet13SystemAddressC1Ev
// demangled: RakNet::SystemAddress::SystemAddress(void)
// type: int __fastcall(int this)
#[doc(alias = "RakNet::SystemAddress::SystemAddress(void)")]
pub fn stub_a5bfec() -> crate::socket::SystemAddress {
 // IDA 0xa5bfec: zeroed with the IPv4 family byte.
 crate::socket::SystemAddress::new()
}
