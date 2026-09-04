//! platform generated_04 — next 100 stubs EA-sorted high-EA iOS region not yet in crates/platform/src/*.rs
//! Filter: ObjC|iOS|Roblox|GVC|UIKit (2763 ObjC total + platform) | 117 remaining, 100 taken high-EA 0xf55c84..0x29a68c desc | SharedPtr = rbx_core::SharedPtr (Arc) not boost::shared_ptr
//! Batch: 100 stubs EA-sorted desc, rbx_core::SharedPtr not boost | skeleton batch

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
/// Rust model of `RobloxExtraSpace::Shared` (IDA `0xf55c84`/`0xf55c74` family):
/// the reference-counted payload behind the intrusive set. `SharedPtr` is
/// `rbx_core::SharedPtr` (`Arc`), never `boost::shared_ptr`.
#[derive(Debug, Default)]
pub struct ExtraSpaceShared {
    pub refs: u32,
}

/// One node of the `RBX::Intrusive::Set<RobloxExtraSpace, ...>` store (IDA
/// `0xf55c14`/`0xf55c54`/`0xf2ce14` family). The intrusive hook collapses
/// into the `linked` flag; the set itself is the `Vec` in `RobloxExtraSpace`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExtraSpaceNode {
    pub id: u32,
    pub refs: u32,
    pub linked: bool,
}

/// Rust model of `RobloxExtraSpace` (IDA `0xf55c34`/`0xf55c44`/`0xf55bf4`):
/// the intrusive-set node store behind `createNewNode`/`eraseRefsFromAllNodes`.
#[derive(Debug, Default)]
pub struct RobloxExtraSpace {
    pub nodes: Vec<ExtraSpaceNode>,
    pub next_id: u32,
}

/// Borrowed cursor over the intrusive set (IDA `...::Iterator` family:
/// `0xf55c64`/`0xf2b4d4`/`0xf2b4c4`/`0xf2b4b4`). The raw node pointer of the
/// original collapses into an index; out-of-range is the end iterator.
#[derive(Debug, Clone, Copy)]
pub struct ExtraSpaceIter {
    pub index: usize,
}

/// The page-launch slot bound at IDA `0xf269b4`: `void(std::string,
/// std::string, std::string, NSObject*, SharedPtr<RBX::Game>)` with the
/// `RobloxPageViewController *` receiver. `boost::bind`/`list5`/`storage5`
/// collapse into this tuple (bind/function become closures); the `NSObject *`
/// page and `SharedPtr<RBX::Game>` game stay opaque addresses (`0` is
/// `nil`/empty) with no host UIKit runtime here.
#[derive(Debug, Default)]
pub struct PageLaunchTask {
    pub first: String,
    pub second: String,
    pub third: String,
    pub page: usize,
    pub game: usize,
}

impl PageLaunchTask {
    /// Hosted page-launch dispatch (IDA `0xf268e4` `list5::operator()`);
    /// without the UIKit host this keeps the call shape only.
    pub fn run(&self) {
        let _ = (&self.first, &self.second, &self.third, self.page, self.game);
    }
}

/// First four bound args of the page-launch slot (IDA `0xf26954`
/// `storage4`): the three strings plus the page receiver.
#[derive(Debug, Default)]
pub struct PageLaunchPrefix {
    pub first: String,
    pub second: String,
    pub third: String,
    pub page: usize,
}

/// Unsent-block queue behind `FlurryDataSenderBase` (IDA `0xf129a0`):
/// `-[FlurryDataSenderBase networkStatusChanged:]` re-fires
/// `performRetransmitNotSentBlocks` when the network returns.
#[derive(Debug, Default)]
pub struct FlurryDataSender {
    pub pending: Vec<Vec<u8>>,
}

impl FlurryDataSender {
    /// Rust model of `performRetransmitNotSentBlocks`: drains the unsent
    /// queue and reports how many blocks were retransmitted.
    pub fn perform_retransmit_not_sent_blocks(&mut self) -> usize {
        let n = self.pending.len();
        self.pending.clear();
        n
    }
}
/// Mutable analytics session behind the `FlurryImpl` `__block_invoke`
/// family (IDA `0xf11c1c`..`0xf0fb7c`): every block captures the session
/// (`a1 + 20/24`) and forwards one selector, optionally storing the result
/// through a `__block_byref` slot. Object pointers stay opaque `usize`
/// (`0` is `nil`) with no host Flurry/UIKit runtime here.
#[derive(Debug, Default)]
pub struct FlurrySession {
    pub age: i32,
    pub gender: i32,
    pub page_view_count: i32,
    pub pause_time: i64,
    pub accuracy: f64,
    pub longitude: f64,
    pub latitude: f64,
    pub push_token: usize,
    pub age_years: i32,
    pub gender_id: usize,
    pub crash_reporting: bool,
    pub reports_on_pause: bool,
    pub session_open: bool,
    pub resuming: bool,
    pub page_views: u32,
    pub pending_session_sends: u32,
    pub events: Vec<FlurryEvent>,
    pub errors: Vec<FlurryErrorReport>,
    pub purchases: Vec<usize>,
}

/// One `recordEvent:withParameters:[timed:]` entry (IDA `0xf102a0`/`0xf10160`/
/// `0xf103d4`): opaque event/parameter ids plus the timed flag and the
/// end marker set by `endTimedEvent:withParameters:`.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlurryEvent {
    pub name: usize,
    pub params: usize,
    pub timed: bool,
    pub ended: bool,
}

/// One `recordError:...` entry (IDA `0xf1082c`/`0xf10698`/`0xf10518`): the
/// captured selector args as opaque ids; `live` selects the live report path.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlurryErrorReport {
    pub first: usize,
    pub second: usize,
    pub third: usize,
    pub fourth: usize,
    pub fifth: usize,
    pub live: bool,
}

// 0xf55c84 — j___ZN5boost6detail12shared_countC2IN16RobloxExtraSpace6SharedEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
pub fn stub_f55c84(ptr: *const ExtraSpaceShared) -> u32 {
    // IDA 0xf55c84: `__picsymbolstub4` PLT trampoline (LDR R12 / ADD R12,PC /
    // LDR PC); decompiler marks `// attributes: thunk` with a single tail
    // call to the real `shared_count` ctor. Models `__shared_count(p)`
    // adopting `p`: null starts empty, otherwise the use count starts at one
    // (the atomic itself collapses into `Arc`).
    if ptr.is_null() { 0 } else { 1 }
}

// 0xf55c74 — j___ZN5boost10shared_ptrIN16RobloxExtraSpace6SharedEEC2IS2_EEPT_
// was: boost::shared_ptr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
pub fn stub_f55c74(shared: ExtraSpaceShared) -> SharedPtr<ExtraSpaceShared> {
    // IDA 0xf55c74: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling the real `shared_ptr` ctor. `Arc::new` allocates object +
    // control block together, matching `shared_ptr(p)` adopting a fresh object.
    SharedPtr::new(shared)
}

// 0xf55c64 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator*(void)")]
pub fn stub_f55c64(set: &RobloxExtraSpace, it: ExtraSpaceIter) -> Option<ExtraSpaceNode> {
    // IDA 0xf55c64: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Iterator::operator*`. Dereferences the cursor; an
    // out-of-range cursor is the end iterator (`None`).
    set.nodes.get(it.index).copied()
}

// 0xf55c54 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE
// type: int __fastcall(int, void *)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::erase(RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator)")]
pub fn stub_f55c54(set: &mut RobloxExtraSpace, it: ExtraSpaceIter) -> bool {
    // IDA 0xf55c54: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Set::erase(Iterator)`. Unhooks the node at the cursor;
    // the intrusive unlink collapses into the `Vec` removal.
    if it.index < set.nodes.len() {
        set.nodes.remove(it.index);
        true
    } else {
        false
    }
}

// 0xf55c44 — j___ZN16RobloxExtraSpaceD2Ev
// type: void __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::~RobloxExtraSpace()")]
pub fn stub_f55c44(space: &mut RobloxExtraSpace) {
    // IDA 0xf55c44: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `~RobloxExtraSpace` (0xf55c4c). The member-by-member
    // destruction collapses: dropping the node store runs the same release.
    space.nodes.clear();
}

// 0xf55c34 — j___ZN16RobloxExtraSpaceC2Ev
// type: RobloxExtraSpace *__fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::RobloxExtraSpace(void)")]
pub fn stub_f55c34() -> RobloxExtraSpace {
    // IDA 0xf55c34: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `RobloxExtraSpace::RobloxExtraSpace()` (0xf55c3c). The
    // empty intrusive-set head is the default store.
    RobloxExtraSpace::default()
}

// 0xf55c14 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::insert(RobloxExtraSpace&)")]
pub fn stub_f55c14(set: &mut RobloxExtraSpace, node: ExtraSpaceNode) {
    // IDA 0xf55c14: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Set::insert(RobloxExtraSpace&)`. Links the node; the
    // red-black hook collapses into the `Vec` push with `linked` set.
    let mut node = node;
    node.linked = true;
    set.nodes.push(node);
}

// 0xf55c04 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Hook::remove(void)")]
pub fn stub_f55c04(hook: &mut ExtraSpaceNode) {
    // IDA 0xf55c04: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Hook::remove()`. Unhooks one node; the sibling/parent
    // pointer fixups collapse into clearing `linked`.
    hook.linked = false;
}

// 0xf55bf4 — j___ZN16RobloxExtraSpaceC2EPS_
// type: RobloxExtraSpace *__fastcall(RobloxExtraSpace *__hidden this, RobloxExtraSpace *)
#[doc(alias = "RobloxExtraSpace::RobloxExtraSpace(RobloxExtraSpace*)")]
pub fn stub_f55bf4(other: &RobloxExtraSpace) -> RobloxExtraSpace {
    // IDA 0xf55bf4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `RobloxExtraSpace(RobloxExtraSpace*)` (0xf55bfc). Copies
    // the node store; the fresh copy restarts id allocation past the max id.
    let next_id = other.nodes.iter().map(|n| n.id.wrapping_add(1)).max().unwrap_or(0);
    RobloxExtraSpace { nodes: other.nodes.clone(), next_id }
}

// 0xf2ce14 — j___ZN16RobloxExtraSpace13createNewNodeEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::createNewNode(void)")]
pub fn stub_f2ce14(space: &mut RobloxExtraSpace) -> u32 {
    // IDA 0xf2ce14: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `RobloxExtraSpace::createNewNode()`. Allocates one node
    // and returns its id; the slab free-list collapses into the `Vec`.
    let id = space.next_id;
    space.next_id = space.next_id.wrapping_add(1);
    space.nodes.push(ExtraSpaceNode { id, refs: 0, linked: false });
    id
}

// 0xf2b4d4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")]
pub fn stub_f2b4d4(set: &RobloxExtraSpace, it: ExtraSpaceIter) -> Option<&ExtraSpaceNode> {
    // IDA 0xf2b4d4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Iterator::operator->`. Borrows the node at the cursor;
    // the end iterator yields `None`.
    set.nodes.get(it.index)
}

// 0xf2b4c4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")]
pub fn stub_f2b4c4(it: &mut ExtraSpaceIter) {
    // IDA 0xf2b4c4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Iterator::operator++`. Advances the cursor by one node;
    // the tree-successor walk collapses into the index step.
    it.index = it.index.wrapping_add(1);
}

// 0xf2b4b4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")]
pub fn stub_f2b4b4(index: usize) -> ExtraSpaceIter {
    // IDA 0xf2b4b4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `Iterator(RobloxExtraSpace*)`. Wraps the node position;
    // the raw pointer collapses into the index.
    ExtraSpaceIter { index }
}

// 0xf2ad54 — j___ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::eraseRefsFromAllNodes(void)")]
pub fn stub_f2ad54(space: &mut RobloxExtraSpace) {
    // IDA 0xf2ad54: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `eraseRefsFromAllNodes()`. Walks every node clearing the
    // back-refs; the per-node unlink collapses into zeroing `refs`.
    for node in space.nodes.iter_mut() {
        node.refs = 0;
    }
}

// 0xf26ca4 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)const (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>> *)const")]
pub fn stub_f26ca4(task: &SharedPtr<PageLaunchTask>) -> std::sync::Weak<PageLaunchTask> {
    // IDA 0xf26ca4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `enable_shared_from_this<thread_data_base>::
    // _internal_accept_owner` for the page-launch `thread_data`. Arms the
    // internal weak owner from the live `SharedPtr` (bind/function become
    // closures; `Arc::downgrade` is the weak arm).
    std::sync::Arc::downgrade(task)
}

// 0xf26ad4 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
// type: int __fastcall(int, int)
// was: boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>> &&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>> &&)")]
pub fn stub_f26ad4(task: PageLaunchTask) -> std::thread::JoinHandle<()> {
    // IDA 0xf26ad4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `boost::thread(thread_data<bind_t<...>>&&)` for the page
    // launch `(string, string, string, NSObject*, SharedPtr<Game>)`. Starts
    // detached work via `std::thread` (`boost::thread` maps to `std::thread`).
    std::thread::spawn(move || task.run())
}

// 0xf26a34 — j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>> *)")]
pub fn stub_f26a34(task: *const PageLaunchTask) -> u32 {
    // IDA 0xf26a34: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling the `shared_count` ctor for the page-launch `thread_data`.
    // Same adoption shape as 0xf55c84: null starts empty, else count is one.
    if task.is_null() { 0 } else { 1 }
}

// 0xf269f4 — j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>&&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>&&)")]
pub fn stub_f269f4(task: PageLaunchTask) -> PageLaunchTask {
    // IDA 0xf269f4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling the `thread_data<bind_t<...>>` move ctor. Takes ownership
    // of the bound page-launch args; the move collapses into the return.
    task
}

// 0xf269b4 — j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
// was: boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_f269b4(first: String, second: String, third: String, page: usize, game: usize) -> PageLaunchTask {
    // IDA 0xf269b4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `boost::bind<void,...>` for the page launch `(string,
    // string, string, NSObject*, SharedPtr<Game>)`. Binds the callable plus
    // the five args into one slot; the `bind_t`/`list_av_5` wrappers collapse
    // into the `PageLaunchTask` tuple.
    PageLaunchTask { first, second, third, page, game }
}

// 0xf26964 — j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// type: int __fastcall(int, int, int, int, int, int)
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_f26964(first: String, second: String, third: String, page: usize, game: usize) -> PageLaunchTask {
    // IDA 0xf26964: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling the `storage5<value<string> x3, value<page*>,
    // value<SharedPtr<Game>>>` ctor. Packs all five bound values; same tuple
    // as the `bind` at 0xf269b4.
    PageLaunchTask { first, second, third, page, game }
}

// 0xf26954 — j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")]
pub fn stub_f26954(prefix: PageLaunchPrefix) -> PageLaunchPrefix {
    // IDA 0xf26954: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling the `storage4<value<string> x3, value<page*>>` ctor. Packs
    // the first four bound values; the copy collapses into the return.
    prefix
}

// 0xf268e4 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_f268e4(task: &PageLaunchTask) {
    // IDA 0xf268e4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling `list5<...>::operator()<page-launch-fn, list0>`: invokes
    // the bound slot with the stored args. The unwrap-and-call collapses
    // into `run`.
    task.run();
}

// 0xf268d4 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// type: int __fastcall(int, int, int, int, int, int)
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_f268d4(first: String, second: String, third: String, page: usize, game: usize) -> PageLaunchTask {
    // IDA 0xf268d4: `__picsymbolstub4` PLT trampoline; `// attributes: thunk`
    // tail-calling the `list5<...>` ctor. Same five-value pack as `storage5`
    // at 0xf26964 (list5 owns a storage5).
    PageLaunchTask { first, second, third, page, game }
}

// 0xf129a0 — ___45-[FlurryDataSenderBase networkStatusChanged:]_block_invoke_0
#[doc(alias = "___45-[FlurryDataSenderBase networkStatusChanged:]_block_invoke_0")]
pub fn stub_f129a0(sender: &mut FlurryDataSender) -> usize {
    // IDA 0xf129a0: `__block_invoke_0` for `-[FlurryDataSenderBase
    // networkStatusChanged:]`; body is one `objc_msgSend` (0xf129b6) to
    // `performRetransmitNotSentBlocks` on the captured sender (`a1 + 20`).
    // The block/literal scaffolding collapses; `id` result is the count.
    sender.perform_retransmit_not_sent_blocks()
}

// 0xf12774 — ___59-[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]_block_invoke_0
// type: void __cdecl(id)
#[doc(alias = "___59-[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]_block_invoke_0")]
pub fn stub_f12774(log_level: u32) -> bool {
    // IDA 0xf12774: `__block_invoke_0` for `-[FlurryDataSenderBase
    // startBackgroundTaskTrackingIfNeeded]`; logs the expiration note via
    // `NSLog` (0xf127a4) only when `[FlurryUtil logLevel] >= 2` (0xf12796).
    // Returns whether the log fired.
    log_level >= 2
}

// 0xf11f68 — ___35+[FlurryImpl registerBackgoundTask]_block_invoke_0
// type: void __cdecl(id)
#[doc(alias = "___35+[FlurryImpl registerBackgoundTask]_block_invoke_0")]
pub fn stub_f11f68(log_level: u32) -> bool {
    // IDA 0xf11f68: `__block_invoke_0` for `+[FlurryImpl
    // registerBackgoundTask]`; logs the expiration-handler note (0xf11f98)
    // only when `[FlurryUtil logLevel] == 3` (0xf11f8a). Returns whether the
    // log fired.
    log_level == 3
}

// 0xf11e54 — ___23-[FlurryImpl pauseTime]_block_invoke_0
#[doc(alias = "___23-[FlurryImpl pauseTime]_block_invoke_0")]
pub fn stub_f11e54(session_pause: i64, out_slot: &mut i64) -> i64 {
    // IDA 0xf11e54: `__block_invoke_0` for `-[FlurryImpl pauseTime]`; reads
    // `[[capture+24] session] pauseTime` (0xf11e68/0xf11e78) and stores it
    // through the `__block_byref` slot (`*(a1+20)+4` at 0xf11e80), returning
    // the value (0xf11e82). The ObjC sends collapse into the read.
    *out_slot = session_pause;
    session_pause
}

// 0xf11c1c — ___27-[FlurryImpl pageViewCount]_block_invoke_0
#[doc(alias = "___27-[FlurryImpl pageViewCount]_block_invoke_0")]
pub fn stub_f11c1c(session: &FlurrySession, out_slot: &mut i32) -> i32 {
    // IDA 0xf11c1c: `__block_invoke_0` for `-[FlurryImpl pageViewCount]`;
    // reads `[[capture+24] session] pageViewCount` (0xf11c30/0xf11c40) and
    // stores it through the `__block_byref` slot (`+16` at 0xf11c48),
    // returning the value (0xf11c4a).
    *out_slot = session.page_view_count;
    session.page_view_count
}

// 0xf11a00 — ___17-[FlurryImpl age]_block_invoke_0
#[doc(alias = "___17-[FlurryImpl age]_block_invoke_0")]
pub fn stub_f11a00(session: &FlurrySession, out_slot: &mut i32) -> i32 {
    // IDA 0xf11a00: `__block_invoke_0` for `-[FlurryImpl age]`; reads
    // `[[capture+24] session] age` (0xf11a14/0xf11a24) and stores it through
    // the `__block_byref` slot (`+24` at 0xf11a2c), returning it (0xf11a2e).
    *out_slot = session.age;
    session.age
}

// 0xf117a8 — ___20-[FlurryImpl gender]_block_invoke_0
#[doc(alias = "___20-[FlurryImpl gender]_block_invoke_0")]
pub fn stub_f117a8(session: &FlurrySession, out_slot: &mut i32) -> i32 {
    // IDA 0xf117a8: `__block_invoke_0` for `-[FlurryImpl gender]`; reads
    // `[[capture+24] session] gender` (0xf117bc/0xf117cc) and stores it
    // through the `__block_byref` slot (`+16` at 0xf117d4), returning it
    // (0xf117d6).
    *out_slot = session.gender;
    session.gender
}

// 0xf11580 — ___22-[FlurryImpl accuracy]_block_invoke_0
#[doc(alias = "___22-[FlurryImpl accuracy]_block_invoke_0")]
pub fn stub_f11580(session: &FlurrySession, out_slot: &mut f64) -> *mut f64 {
    // IDA 0xf11580: `__block_invoke_0` for `-[FlurryImpl accuracy]`; reads
    // `[[capture+24] session] accuracy` as a double (0xf11594/0xf115ae) and
    // stores it through the `__block_byref` slot (`+16` at 0xf115b4). Unlike
    // its siblings it returns the slot itself (0xf115b8), hence the pointer.
    *out_slot = session.accuracy;
    out_slot
}

// 0xf11348 — ___23-[FlurryImpl longitude]_block_invoke_0
#[doc(alias = "___23-[FlurryImpl longitude]_block_invoke_0")]
pub fn stub_f11348(session: &FlurrySession, out_slot: &mut f64) -> f64 {
    // IDA 0xf11348: `__block_invoke_0` for `-[FlurryImpl longitude]`; reads
    // `[[capture+24] session] longitude` (0xf1135c/0xf1136c, 64-bit value)
    // and stores it through the `__block_byref` slot (`+16/+20` at
    // 0xf11374/0xf11378), returning the value (0xf1137a).
    *out_slot = session.longitude;
    session.longitude
}

// 0xf11118 — ___22-[FlurryImpl latitude]_block_invoke_0
#[doc(alias = "___22-[FlurryImpl latitude]_block_invoke_0")]
pub fn stub_f11118(session: &FlurrySession, out_slot: &mut f64) -> f64 {
    // IDA 0xf11118: `__block_invoke_0` for `-[FlurryImpl latitude]`; same
    // shape as longitude above (`session` at 0xf1112c, `latitude` at
    // 0xf1113c, byref store at 0xf11144/0xf11148, return at 0xf1114a).
    *out_slot = session.latitude;
    session.latitude
}

// 0xf10f10 — ___50-[FlurryImpl sendSessionsToServerForCreateSession]_block_invoke_0
#[doc(alias = "___50-[FlurryImpl sendSessionsToServerForCreateSession]_block_invoke_0")]
pub fn stub_f10f10(session: &mut FlurrySession) -> u32 {
    // IDA 0xf10f10: `__block_invoke_0` for `-[FlurryImpl
    // sendSessionsToServerForCreateSession]`; forwards `session` (0xf10f22)
    // to `sendSessionsToServerForCreateSession` (0xf10f36). Flushes the
    // queued session sends and reports how many went out.
    let sent = session.pending_session_sends;
    session.pending_session_sends = 0;
    sent
}

// 0xf10e28 — ___24-[FlurryImpl endSession]_block_invoke_0
#[doc(alias = "___24-[FlurryImpl endSession]_block_invoke_0")]
pub fn stub_f10e28(session: &mut FlurrySession) {
    // IDA 0xf10e28: `__block_invoke_0` for `-[FlurryImpl endSession]`;
    // forwards `session` (0xf10e3a) to `endSession` (0xf10e4e). Closes the
    // session; the server flush rides the session batch.
    session.session_open = false;
    session.resuming = false;
}

// 0xf10d40 — ___27-[FlurryImpl resumeSession]_block_invoke_0
#[doc(alias = "___27-[FlurryImpl resumeSession]_block_invoke_0")]
pub fn stub_f10d40(session: &mut FlurrySession) -> bool {
    // IDA 0xf10d40: `__block_invoke_0` for `-[FlurryImpl resumeSession]`;
    // forwards `session` (0xf10d52) to `resumeSession` (0xf10d66). Reopens
    // the session and reports the open state.
    session.session_open = true;
    session.resuming = false;
    session.session_open
}

// 0xf10c58 — ___35-[FlurryImpl markSessionAsResuming]_block_invoke_0
#[doc(alias = "___35-[FlurryImpl markSessionAsResuming]_block_invoke_0")]
pub fn stub_f10c58(session: &mut FlurrySession) {
    // IDA 0xf10c58: `__block_invoke_0` for `-[FlurryImpl
    // markSessionAsResuming]`; forwards `session` (0xf10c6a) to
    // `markSessionAsResuming` (0xf10c7e). Flags the resume without reopening.
    session.resuming = true;
}

// 0xf10b28 — ___26-[FlurryImpl pauseSession]_block_invoke_0136
#[doc(alias = "___26-[FlurryImpl pauseSession]_block_invoke_0136")]
pub fn stub_f10b28(session: &mut FlurrySession, bg_task: u32) -> u32 {
    // IDA 0xf10b28: `__block_invoke_0` for `-[FlurryImpl pauseSession]`;
    // forwards `session` (0xf10b3c) to `pauseSession` (0xf10b4c), then ends
    // the captured background task unless it is `UIBackgroundTaskInvalid`
    // (0xf10b5e/0xf10b62) via `endBackgroundTask:` (0xf10b7e/0xf10b90).
    // `UIBackgroundTaskInvalid` is 0 [INFERENCE: iOS SDK constant value].
    // Returns the invalid sentinel after ending, else the surviving task.
    session.session_open = false;
    if bg_task != 0 {
        0
    } else {
        bg_task
    }
}

// 0xf10af0 — ___26-[FlurryImpl pauseSession]_block_invoke_0
// type: void __cdecl(id)
#[doc(alias = "___26-[FlurryImpl pauseSession]_block_invoke_0")]
pub fn stub_f10af0(log_level: u32) -> bool {
    // IDA 0xf10af0: `__block_invoke_0` for the `pauseSession` logger; emits
    // the session-state note via `NSLog` (0xf10b20) only when
    // `[FlurryUtil logLevel] >= 2` (0xf10b12). Returns whether it fired.
    log_level >= 2
}

// 0xf10990 — ___33-[FlurryImpl recordPurchaseItem:]_block_invoke_0
#[doc(alias = "___33-[FlurryImpl recordPurchaseItem:]_block_invoke_0")]
pub fn stub_f10990(session: &mut FlurrySession, item: usize) {
    // IDA 0xf10990: `__block_invoke_0` for `-[FlurryImpl recordPurchaseItem:]`;
    // forwards `session` (0xf109a4) to `recordPurchaseItem:` with the
    // captured item (`a1 + 24` at 0xf109ba). Queues the purchase record.
    session.purchases.push(item);
}

// 0xf1082c — ___71-[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]_block_invoke_0
#[doc(alias = "___71-[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]_block_invoke_0")]
pub fn stub_f1082c(session: &mut FlurrySession, error: usize, message: usize, exc_str: usize, err_type: usize, live: bool) {
    // IDA 0xf1082c: `__block_invoke_0` for `-[FlurryImpl
    // recordError:message:exceptionString:errorType:liveReport:]`; forwards
    // `session` (0xf10842) with the five captured args (`a1 + 24..40` at
    // 0xf10870). Queues the five-word error report.
    session.errors.push(FlurryErrorReport { first: error, second: message, third: exc_str, fourth: err_type, fifth: 0, live });
}

// 0xf10698 — ___51-[FlurryImpl recordError:message:error:liveReport:]_block_invoke_0
#[doc(alias = "___51-[FlurryImpl recordError:message:error:liveReport:]_block_invoke_0")]
pub fn stub_f10698(session: &mut FlurrySession, error: usize, message: usize, error_obj: usize, live: bool) {
    // IDA 0xf10698: `__block_invoke_0` for `-[FlurryImpl
    // recordError:message:error:liveReport:]`; forwards `session` (0xf106b2)
    // with the four captured args (`a1 + 24..36` at 0xf106d8). Queues the
    // report; the `NSError` object stays an opaque id.
    session.errors.push(FlurryErrorReport { first: error, second: message, third: error_obj, fourth: 0, fifth: 0, live });
}

// 0xf10518 — ___55-[FlurryImpl recordError:message:exception:liveReport:]_block_invoke_0
#[doc(alias = "___55-[FlurryImpl recordError:message:exception:liveReport:]_block_invoke_0")]
pub fn stub_f10518(session: &mut FlurrySession, error: usize, message: usize, exception: usize, live: bool) {
    // IDA 0xf10518: `__block_invoke_0` for `-[FlurryImpl
    // recordError:message:exception:liveReport:]`; forwards `session`
    // (0xf10532) with the four captured args (`a1 + 24..36` at 0xf10558).
    // Same queue shape as 0xf10698 with an `NSException` in the third slot.
    session.errors.push(FlurryErrorReport { first: error, second: message, third: exception, fourth: 0, fifth: 0, live });
}

// 0xf103d4 — ___43-[FlurryImpl endTimedEvent:withParameters:]_block_invoke_0
#[doc(alias = "___43-[FlurryImpl endTimedEvent:withParameters:]_block_invoke_0")]
pub fn stub_f103d4(session: &mut FlurrySession, event: usize, params: usize) -> bool {
    // IDA 0xf103d4: `__block_invoke_0` for `-[FlurryImpl
    // endTimedEvent:withParameters:]`; forwards `session` (0xf103e8) to
    // `endTimedEvent:withParameters:` with the captured event/params
    // (`a1 + 24/28` at 0xf10400). Marks the newest matching open timed
    // event ended; the param merge rides the event batch.
    for entry in session.events.iter_mut().rev() {
        if entry.name == event && entry.timed && !entry.ended {
            entry.params = params;
            entry.ended = true;
            return true;
        }
    }
    false
}

// 0xf102a0 — ___47-[FlurryImpl recordEvent:withParameters:timed:]_block_invoke_0
#[doc(alias = "___47-[FlurryImpl recordEvent:withParameters:timed:]_block_invoke_0")]
pub fn stub_f102a0(session: &mut FlurrySession, event: usize, params: usize, timed: bool) {
    // IDA 0xf102a0: `__block_invoke_0` for `-[FlurryImpl
    // recordEvent:withParameters:timed:]`; forwards `session` (0xf102b6) to
    // `recordEvent:withParameters:timed:` with the captured event/params and
    // the `char` timed flag (`a1 + 24/28/32` at 0xf102d6). Queues the event.
    session.events.push(FlurryEvent { name: event, params, timed, ended: false });
}

// 0xf10160 — ___41-[FlurryImpl recordEvent:withParameters:]_block_invoke_0
#[doc(alias = "___41-[FlurryImpl recordEvent:withParameters:]_block_invoke_0")]
pub fn stub_f10160(session: &mut FlurrySession, event: usize, params: usize) {
    // IDA 0xf10160: `__block_invoke_0` for `-[FlurryImpl
    // recordEvent:withParameters:]`; forwards `session` (0xf10174) with the
    // captured event/params (`a1 + 24/28` at 0xf1018c). Untimed variant of
    // 0xf102a0.
    session.events.push(FlurryEvent { name: event, params, timed: false, ended: false });
}

// 0xf1004c — ___27-[FlurryImpl setPushToken:]_block_invoke_0
#[doc(alias = "___27-[FlurryImpl setPushToken:]_block_invoke_0")]
pub fn stub_f1004c(session: &mut FlurrySession, token: usize) {
    // IDA 0xf1004c: `__block_invoke_0` for `-[FlurryImpl setPushToken:]`;
    // forwards `session` (0xf10060) to `setPushToken:` with the captured
    // token (`a1 + 24` at 0xf10076). The `NSData` token stays an opaque id.
    session.push_token = token;
}

// 0xf0ff5c — ___28-[FlurryImpl setAgeInYears:]_block_invoke_0
#[doc(alias = "___28-[FlurryImpl setAgeInYears:]_block_invoke_0")]
pub fn stub_f0ff5c(session: &mut FlurrySession, years: i32) {
    // IDA 0xf0ff5c: `__block_invoke_0` for `-[FlurryImpl setAgeInYears:]`;
    // forwards `session` (0xf0ff70) to `setAgeInYears:` with the captured
    // value (`a1 + 24` at 0xf0ff86).
    session.age_years = years;
}

// 0xf0fe50 — ___32-[FlurryImpl setGenderAsString:]_block_invoke_0
#[doc(alias = "___32-[FlurryImpl setGenderAsString:]_block_invoke_0")]
pub fn stub_f0fe50(session: &mut FlurrySession, gender_id: usize) {
    // IDA 0xf0fe50: `__block_invoke_0` for `-[FlurryImpl setGenderAsString:]`;
    // forwards `session` (0xf0fe64) to `setGenderAsString:` with the captured
    // string (`a1 + 24` at 0xf0fe7a). The `NSString` stays an opaque id.
    session.gender_id = gender_id;
}

// 0xf0fd64 — ___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0
#[doc(alias = "___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0")]
pub fn stub_f0fd64(session: &mut FlurrySession) -> u32 {
    // IDA 0xf0fd64: `__block_invoke_0` for `-[FlurryImpl
    // maybeIncrementPageView]`; forwards `session` (0xf0fd76) to
    // `maybeIncrementPageView` (0xf0fd8a). The "maybe" gating was decided by
    // queueing this block, so the body increments and reports the count.
    session.page_views = session.page_views.wrapping_add(1);
    session.page_views
}

// 0xf0fc74 — ___39-[FlurryImpl setCrashReportingEnabled:]_block_invoke_0
#[doc(alias = "___39-[FlurryImpl setCrashReportingEnabled:]_block_invoke_0")]
pub fn stub_f0fc74(session: &mut FlurrySession, enabled: bool) {
    // IDA 0xf0fc74: `__block_invoke_0` for `-[FlurryImpl
    // setCrashReportingEnabled:]`; forwards `session` (0xf0fc88) to
    // `setCrashReportingEnabled:` with the captured `char` (`a1 + 24` at
    // 0xf0fca0).
    session.crash_reporting = enabled;
}

// 0xf0fb7c — ___46-[FlurryImpl setSessionReportsOnPauseEnabled:]_block_invoke_0
#[doc(alias = "___46-[FlurryImpl setSessionReportsOnPauseEnabled:]_block_invoke_0")]
pub fn stub_f0fb7c(session: &mut FlurrySession, enabled: bool) {
    // IDA 0xf0fb7c: `__block_invoke_0` for `-[FlurryImpl
    // setSessionReportsOnPauseEnabled:]`; forwards `session` (0xf0fb90) to
    // `setSessionReportsOnPauseEnabled:` with the captured `char`
    // (`a1 + 24` at 0xf0fba8). Same `BOOL`-forward shape as 0xf0fc74.
    session.reports_on_pause = enabled;
}

// 0xf0fa84 — ___46-[FlurryImpl setSessionReportsOnCloseEnabled:]_block_invoke_0
#[doc(alias = "___46-[FlurryImpl setSessionReportsOnCloseEnabled:]_block_invoke_0")]
pub fn stub_f0fa84() -> ! {
    todo!("0xf0fa84 ___46-[FlurryImpl setSessionReportsOnCloseEnabled:]_block_invoke_0")
}

// 0xf0f974 — ___24-[FlurryImpl setUserID:]_block_invoke_0
#[doc(alias = "___24-[FlurryImpl setUserID:]_block_invoke_0")]
pub fn stub_f0f974() -> ! {
    todo!("0xf0f974 ___24-[FlurryImpl setUserID:]_block_invoke_0")
}

// 0xf0f880 — ___37-[FlurryImpl setEventLoggingEnabled:]_block_invoke_0
#[doc(alias = "___37-[FlurryImpl setEventLoggingEnabled:]_block_invoke_0")]
pub fn stub_f0f880() -> ! {
    todo!("0xf0f880 ___37-[FlurryImpl setEventLoggingEnabled:]_block_invoke_0")
}

// 0xf0f768 — ___72-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]_block_invoke_0
#[doc(alias = "___72-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]_block_invoke_0")]
pub fn stub_f0f768() -> ! {
    todo!("0xf0f768 ___72-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]_block_invoke_0")
}

// 0xf0f564 — ___29-[FlurryImpl setupForApiKey:]_block_invoke_0
#[doc(alias = "___29-[FlurryImpl setupForApiKey:]_block_invoke_0")]
pub fn stub_f0f564() -> ! {
    todo!("0xf0f564 ___29-[FlurryImpl setupForApiKey:]_block_invoke_0")
}

// 0xf0daa0 — ___51-[FlurryDataSender requestSuccessful:withResponse:]_block_invoke_0
#[doc(alias = "___51-[FlurryDataSender requestSuccessful:withResponse:]_block_invoke_0")]
pub fn stub_f0daa0() -> ! {
    todo!("0xf0daa0 ___51-[FlurryDataSender requestSuccessful:withResponse:]_block_invoke_0")
}

// 0xf0d9a8 — ___50-[FlurryDataSender requestDidCancel:withResponse:]_block_invoke_0
#[doc(alias = "___50-[FlurryDataSender requestDidCancel:withResponse:]_block_invoke_0")]
pub fn stub_f0d9a8() -> ! {
    todo!("0xf0d9a8 ___50-[FlurryDataSender requestDidCancel:withResponse:]_block_invoke_0")
}

// 0xf0d85c — ___48-[FlurryDataSender requestDidFail:withResponse:]_block_invoke_0
#[doc(alias = "___48-[FlurryDataSender requestDidFail:withResponse:]_block_invoke_0")]
pub fn stub_f0d85c() -> ! {
    todo!("0xf0d85c ___48-[FlurryDataSender requestDidFail:withResponse:]_block_invoke_0")
}

// 0xf0d5b0 — ___44-[FlurryDataSender sendData:withIdentifier:]_block_invoke_0
#[doc(alias = "___44-[FlurryDataSender sendData:withIdentifier:]_block_invoke_0")]
pub fn stub_f0d5b0() -> ! {
    todo!("0xf0d5b0 ___44-[FlurryDataSender sendData:withIdentifier:]_block_invoke_0")
}

// 0xe8b490 — __ZNK4Ogre14EAGLES2Context10getContextEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::getContext(void)const")]
pub fn stub_e8b490() -> ! {
    todo!("0xe8b490 Ogre::EAGLES2Context::getContext(void)const")
}

// 0xe8b48c — __ZNK4Ogre14EAGLES2Context5cloneEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::clone(void)const")]
pub fn stub_e8b48c() -> ! {
    todo!("0xe8b48c Ogre::EAGLES2Context::clone(void)const")
}

// 0xe8b488 — __ZN4Ogre14EAGLES2Context10endCurrentEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::endCurrent(void)")]
pub fn stub_e8b488() -> ! {
    todo!("0xe8b488 Ogre::EAGLES2Context::endCurrent(void)")
}

// 0xe8b298 — __ZN4Ogre14EAGLES2Context10setCurrentEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::setCurrent(void)")]
pub fn stub_e8b298() -> ! {
    todo!("0xe8b298 Ogre::EAGLES2Context::setCurrent(void)")
}

// 0xe8ac58 — __ZN4Ogre14EAGLES2Context17createFramebufferEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::createFramebuffer(void)")]
pub fn stub_e8ac58() -> ! {
    todo!("0xe8ac58 Ogre::EAGLES2Context::createFramebuffer(void)")
}

// 0xe8abf8 — __ZN4Ogre14EAGLES2Context18destroyFramebufferEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::destroyFramebuffer(void)")]
pub fn stub_e8abf8() -> ! {
    todo!("0xe8abf8 Ogre::EAGLES2Context::destroyFramebuffer(void)")
}

// 0xe8aab4 — __ZN4Ogre14EAGLES2ContextD1Ev
// type: void __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::~EAGLES2Context()")]
pub fn stub_e8aab4() -> ! {
    todo!("0xe8aab4 Ogre::EAGLES2Context::~EAGLES2Context()")
}

// 0xe8a970 — __ZN4Ogre14EAGLES2ContextD0Ev
// type: void __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "Ogre::EAGLES2Context::~EAGLES2Context()")]
pub fn stub_e8a970() -> ! {
    todo!("0xe8a970 Ogre::EAGLES2Context::~EAGLES2Context()")
}

// 0xe8a6a4 — __ZN4Ogre14EAGLES2ContextC2EP11CAEAGLLayerP14EAGLSharegroup
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this, CAEAGLLayer *, EAGLSharegroup *)
#[doc(alias = "Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)")]
pub fn stub_e8a6a4() -> ! {
    todo!("0xe8a6a4 Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)")
}

// 0xe8a698 — __ZN4Ogre14EAGLES2ContextC1EP11CAEAGLLayerP14EAGLSharegroup
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this, CAEAGLLayer *, EAGLSharegroup *)
#[doc(alias = "Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)")]
pub fn stub_e8a698() -> ! {
    todo!("0xe8a698 Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)")
}

// 0x82ea28 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::get_untyped_deleter(void)")]
pub fn stub_82ea28() -> ! {
    todo!("0x82ea28 boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::get_untyped_deleter(void)")
}

// 0x82ea24 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::get_deleter(std::type_info const&)")]
pub fn stub_82ea24() -> ! {
    todo!("0x82ea24 boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::get_deleter(std::type_info const&)")
}

// 0x82e9f0 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::dispose(void)")]
pub fn stub_82e9f0() -> ! {
    todo!("0x82e9f0 boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::dispose(void)")
}

// 0x82e9ec — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::~sp_counted_impl_p()")]
pub fn stub_82e9ec() -> ! {
    todo!("0x82e9ec boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::~sp_counted_impl_p()")
}

// 0x82e9e8 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::~sp_counted_impl_p()")]
pub fn stub_82e9e8() -> ! {
    todo!("0x82e9e8 boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::~sp_counted_impl_p()")
}

// 0x82e834 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator*(void)")]
pub fn stub_82e834() -> ! {
    todo!("0x82e834 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator*(void)")
}

// 0x82e808 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE
// type: int __fastcall(int, void *)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::erase(RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator)")]
pub fn stub_82e808() -> ! {
    todo!("0x82e808 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::erase(RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator)")
}

// 0x82e6f8 — __ZN5boost6detail12shared_countC2IN16RobloxExtraSpace6SharedEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
pub fn stub_82e6f8() -> ! {
    todo!("0x82e6f8 boost::detail::shared_count::shared_count<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")
}

// 0x82e624 — __ZN5boost10shared_ptrIN16RobloxExtraSpace6SharedEEC2IS2_EEPT_
// was: boost::shared_ptr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
pub fn stub_82e624() -> ! {
    todo!("0x82e624 boost::shared_ptr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")
}

// 0x82e530 — __ZN16RobloxExtraSpaceC2Ev
// type: RobloxExtraSpace *__fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::RobloxExtraSpace(void)")]
pub fn stub_82e530() -> ! {
    todo!("0x82e530 RobloxExtraSpace::RobloxExtraSpace(void)")
}

// 0x82e308 — __ZN16RobloxExtraSpaceD2Ev
// type: void __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::~RobloxExtraSpace()")]
pub fn stub_82e308() -> ! {
    todo!("0x82e308 RobloxExtraSpace::~RobloxExtraSpace()")
}

// 0x823d98 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Hook::remove(void)")]
pub fn stub_823d98() -> ! {
    todo!("0x823d98 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Hook::remove(void)")
}

// 0x823b10 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::insert(RobloxExtraSpace&)")]
pub fn stub_823b10() -> ! {
    todo!("0x823b10 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::insert(RobloxExtraSpace&)")
}

// 0x8238a8 — __ZN16RobloxExtraSpaceC2EPS_
// type: RobloxExtraSpace *__fastcall(RobloxExtraSpace *__hidden this, RobloxExtraSpace *)
#[doc(alias = "RobloxExtraSpace::RobloxExtraSpace(RobloxExtraSpace*)")]
pub fn stub_8238a8() -> ! {
    todo!("0x8238a8 RobloxExtraSpace::RobloxExtraSpace(RobloxExtraSpace*)")
}

// 0x7ec350 — __ZN3RBX15ContentProvider21isValidRobloxAssetUrlENS_9ContentIdE
#[doc(alias = "RBX::ContentProvider::isValidRobloxAssetUrl(RBX::ContentId)")]
pub fn stub_7ec350() -> ! {
    todo!("0x7ec350 RBX::ContentProvider::isValidRobloxAssetUrl(RBX::ContentId)")
}

// 0x7dd3c0 — __Z16rbx_isRobloxSitePKc
// type: _DWORD __fastcall(const char *)
#[doc(alias = "rbx_isRobloxSite(char const*)")]
pub fn stub_7dd3c0() -> ! {
    todo!("0x7dd3c0 rbx_isRobloxSite(char const*)")
}

// 0x7034c4 — __ZNK3RBX8Instance15getRobloxLockedEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::getRobloxLocked(void)const")]
pub fn stub_7034c4() -> ! {
    todo!("0x7034c4 RBX::Instance::getRobloxLocked(void)const")
}

// 0x6fee38 — __ZN3RBX8Instance15setRobloxLockedEb
// type: _DWORD __fastcall(RBX::Instance *__hidden this, bool)
#[doc(alias = "RBX::Instance::setRobloxLocked(bool)")]
pub fn stub_6fee38() -> ! {
    todo!("0x6fee38 RBX::Instance::setRobloxLocked(bool)")
}

// 0x5fca54 — __ZN3RBX14CoreGuiService21createRobloxScreenGuiEv
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "RBX::CoreGuiService::createRobloxScreenGui(void)")]
pub fn stub_5fca54() -> ! {
    todo!("0x5fca54 RBX::CoreGuiService::createRobloxScreenGui(void)")
}

// 0x41be14 — __ZN3RBX9DataModel12saveToRobloxEN5boost8functionIFvbEEENS2_IFvSsEEE
// type: void __fastcall(RBX::DataModel *, const RBX::Instance *)
#[doc(alias = "RBX::DataModel::saveToRoblox(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_41be14() -> ! {
    todo!("0x41be14 RBX::DataModel::saveToRoblox(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x3180dc — __ZN3RBX4Http12isRobloxSiteEPKc
// type: _DWORD __fastcall(RBX::Http *__hidden this, const char *)
#[doc(alias = "RBX::Http::isRobloxSite(char const*)")]
pub fn stub_3180dc() -> ! {
    todo!("0x3180dc RBX::Http::isRobloxSite(char const*)")
}

// 0x316590 — __ZN3RBX4Http21getRobloxResponceLockEv
// type: _DWORD __fastcall(RBX::Http *__hidden this)
#[doc(alias = "RBX::Http::getRobloxResponceLock(void)")]
pub fn stub_316590() -> ! {
    todo!("0x316590 RBX::Http::getRobloxResponceLock(void)")
}

// 0x2cbc40 — __ZN16RobloxExtraSpace13createNewNodeEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::createNewNode(void)")]
pub fn stub_2cbc40() -> ! {
    todo!("0x2cbc40 RobloxExtraSpace::createNewNode(void)")
}

// 0x2c3e54 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")]
pub fn stub_2c3e54() -> ! {
    todo!("0x2c3e54 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")
}

// 0x2c3ca4 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")]
pub fn stub_2c3ca4() -> ! {
    todo!("0x2c3ca4 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")
}

// 0x2c3af0 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")]
pub fn stub_2c3af0() -> ! {
    todo!("0x2c3af0 RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")
}

// 0x2a4c6c — __ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::eraseRefsFromAllNodes(void)")]
pub fn stub_2a4c6c() -> ! {
    todo!("0x2a4c6c RobloxExtraSpace::eraseRefsFromAllNodes(void)")
}

// 0x29aa08 — __ZN3RBX13ScriptContext14setRobloxPlaceEb
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, char *)
#[doc(alias = "RBX::ScriptContext::setRobloxPlace(bool)")]
pub fn stub_29aa08() -> ! {
    todo!("0x29aa08 RBX::ScriptContext::setRobloxPlace(bool)")
}

// 0x29a68c — __ZN3RBX13ScriptContext17loadRobloxLibraryEP9lua_State
#[doc(alias = "RBX::ScriptContext::loadRobloxLibrary(lua_State *)")]
pub fn stub_29a68c() -> ! {
    todo!("0x29a68c RBX::ScriptContext::loadRobloxLibrary(lua_State *)")
}
