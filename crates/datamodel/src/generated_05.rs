// Auto-generated skeletons for rbx-datamodel — shard b (RBX::Instance property accessors)
// Filter: demangled/mangled contains RBX::Instance (subset of 10215 wide), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x703444..0x709ff0 | total filtered 8021 Instance, remaining 4280 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::WeakPtr;
use rbx_core::signal::Signal;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::any::Any;

/// Rust model of the `RBX::Instance` header fields read by this shard's leaves.
/// The original C++ object is larger; only the slots used below are modelled,
/// with original offsets noted per accessor.
pub struct Instance {
    /// Original `*(this + 13)` (byte `0x34`); unretained, hence "dangerous".
    pub parent: *const Instance,
    /// Original name store at `*(this + 17)` (byte `0x44`).
    pub name: InstanceName,
    /// Original flag byte at name store `+ 0x16`.
    pub roblox_locked: bool,
    /// Parent-lock flag at name store `+ 21` (IDA `0x6ffcfc`); distinct from
    /// `roblox_locked` (`+ 22`). Throws out of `setParentInternal`.
    pub parent_locked: bool,
    /// Class name behind the instance vtable's `classDescriptor` slot
    /// (read as `*(this + 12)` words at e.g. IDA `0x70376e`); drives the
    /// `ClassDescriptor::isA` checks until a hierarchy model exists.
    pub class_name: &'static str,
    /// Owned children (`RBX::Instance` child list); backs
    /// `visitDescendants` (IDA `0x70430c`) and the `ServiceProvider::find`
    /// scan (IDA `0x7039cc`).
    pub children: Vec<SharedPtr<Instance>>,
    /// Re-entrancy guard at byte `+64` (IDA `0x6ffe86`); set for the body of
    /// `setParentInternal`.
    pub in_set_parent: bool,
    /// Combined child-added/removed signal at `+80` (IDA `0x70013e` kind `1`,
    /// IDA `0x7001d6` kind `0`; see `CombinedSignal`).
    pub combined: CombinedSignal,
    /// Virtual-hook table for the reparenting path (`setParentInternal`,
    /// IDA `0x6ffc98`) and the tree queries; base-class slots are no-ops
    /// (`None`) until subclass overrides are modelled.
    pub hooks: InstanceHooks,
    /// Lazily allocated block from `RBX::Instance::onDemandWrite` (IDA `0x7010ac`).
    pub write: Option<Box<InstanceWrite>>,
    /// Embedded `boost::enable_shared_from_this` weak owner at `this + 40`
    /// (IDA `0x7039e4` reads px at `+40`, pi at `+44`).
    pub weak_owner: WeakPtr<Instance>,
}
/// Two-pointer virtual hook (`RBX::Instance` vtable slots `+56`, `+64`,
/// `+100`, `+104`, `+108` in `setParentInternal`, IDA `0x6ffc98`).
pub type Hook2 = fn(*mut Instance, *const Instance);
/// Three-pointer virtual hook (slots `+60`, `+68`, `+88`).
pub type Hook3 = fn(*mut Instance, *const Instance, *const Instance);
/// overridable `getPersistentDataCost` virtual (slot `+32`, IDA `0x6ff898`).
pub type CostHook = fn(*const Instance) -> i32;
/// `onChildChanged` propagation virtual (slot `+112`, IDA `0x6ff8ac`).
pub type ChildChangedHook = fn(*mut Instance) -> i32;
/// Subclass `readProperty`/`read` virtual (slot `+120`, IDA `0x6ff02c`).
pub type ReadNodeHook = fn(*mut Instance, &XmlElement, &mut ReferenceBinder);
/// Property identity for `raisePropertyChanged` (IDA `0x700222`,
/// `0x6fee64`): only the descriptors referenced so far are modelled.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    Parent,
    RobloxLocked,
}
/// Virtual-hook table; see `Instance::hooks`.
#[derive(Default)]
pub struct InstanceHooks {
    pub changing: Option<Hook2>,
    pub ancestry_changing: Option<Hook3>,
    pub child_added: Option<Hook2>,
    pub descendant_added: Option<Hook3>,
    pub added: Option<Hook2>,
    pub removing: Option<Hook2>,
    pub child_removed: Option<Hook2>,
    pub ancestry_changed: Option<Hook3>,
    pub property_changed: Option<fn(*mut Instance, PropertyKind)>,
    pub on_child_changed: Option<ChildChangedHook>,
    pub data_cost: Option<CostHook>,
    pub read_node: Option<ReadNodeHook>,
}
/// XML tag/attribute names used by the `Instance::read*` family
/// (IDA `0x6feebc`, `0x6ff018`, `0x6ff03e`, `0x6ff092`).
pub const TAG_ITEM: &str = "Item";
pub const TAG_PROPERTIES: &str = "Properties";
pub const ATTR_CLASS: &str = "class";
pub const ATTR_REFERENT: &str = "referent";
/// Rust model of one `XmlElement` for the `Instance::read*` family (IDA
/// `0x6fefd0` et al.): tag word at `+12`, attribute list, child list.
/// `Name*` tag identity collapses into interned strings.
#[derive(Default)]
pub struct XmlElement {
    pub tag: String,
    pub attrs: Vec<XmlAttr>,
    pub children: Vec<XmlElement>,
}
/// Rust model of one `XmlElement` attribute (`XmlNameValuePair`, IDA
/// `0x6feebc`): presence is resolution (`getValue() == 1`).
#[derive(Default)]
pub struct XmlAttr {
    pub name: String,
    pub value: String,
}
impl XmlElement {
    /// Rust model of `XmlElement::findAttribute` (IDA `0x6feebc`,
    /// `0x6feff0`): linear scan by name.
    pub fn find_attribute(&self, name: &str) -> Option<&XmlAttr> {
        self.attrs.iter().find(|a| a.name == name)
    }
    /// Rust model of `XmlElement::findFirstChildByTag` (IDA `0x6ff092`).
    pub fn find_first_child_by_tag(&self, tag: &str) -> Option<&XmlElement> {
        self.children.iter().find(|c| c.tag == tag)
    }
    /// Rust model of `XmlElement::findNextChildWithSameTag` (IDA `0x6ff0a0`):
    /// first same-tag sibling after `prev` by identity.
    pub fn find_next_child_with_same_tag(&self, prev: *const XmlElement, tag: &str) -> Option<&XmlElement> {
        let mut after = false;
        for child in self.children.iter() {
            if after && child.tag == tag {
                return Some(child);
            }
            if (child as *const XmlElement) == prev {
                after = true;
            }
        }
        None
    }
}
/// Rust model of `RBX::IReferenceBinder` (IDA `0x6ff002`, `0x6fef64`): maps
/// `referent` names to instances. The `(**a3)(a3, name, target)` calls are
/// `bind`.
#[derive(Default)]
pub struct ReferenceBinder {
    pub entries: HashMap<String, *const Instance>,
}
impl ReferenceBinder {
    pub fn bind(&mut self, name: &str, inst: *const Instance) {
        self.entries.insert(name.to_string(), inst);
    }
}
/// Collapse of `RBX::shared_from<Instance>` retains (IDA `0x6ffeba`,
/// `0x6ffec6`): mints a borrower `SharedPtr` from a pointer the caller
/// guarantees comes from a live `Arc`. The `from_raw` ownership is
/// immediately forgotten, so the net effect is exactly one clone.
/// # Safety
/// `ptr` must point into a live `SharedPtr<Instance>` allocation that
/// outlives the returned handle's clones.
pub unsafe fn borrow_shared(ptr: *const Instance) -> SharedPtr<Instance> {
    let owned = SharedPtr::from_raw(ptr);
    let out = owned.clone();
    core::mem::forget(owned);
    out
}

/// Embedded name store. Original `getName` returns `*(this + 17) + 24`
/// (IDA `0x703484`), i.e. the address of the name within the store.
#[derive(Default)]
pub struct InstanceName {
    pub text: String,
}

/// Lazily allocated signal block from `RBX::Instance::onDemandWrite`.
/// Original byte offsets from the block base: `+4` child-added, `+8`
/// child-removed, `+12` descendant-added, `+16` descendant-removing.
#[derive(Default)]
pub struct InstanceWrite {
    pub child_added: Signal<SharedPtr<Instance>>,
    pub child_removed: Signal<SharedPtr<Instance>>,
    pub descendant_added: Signal<SharedPtr<Instance>>,
    pub descendant_removing: Signal<SharedPtr<Instance>>,
}

/// Rust model of `RBX::Instance::onDemandWrite` (IDA `0x7010ac`).
/// Returns the live write block, allocating it on first use. The original
/// builds the block through a vtable factory and asserts via `FLog::Asserts`
/// when that yields null; allocation cannot fail here, so that path is
/// unreachable.
/// # Safety
/// `this` must point to a valid `Instance`.
unsafe fn on_demand_write(this: *mut Instance) -> *mut InstanceWrite {
    let inst = &mut *this;
    if inst.write.is_none() {
        inst.write = Some(Box::default());
    }
    inst.write.as_deref_mut().unwrap() as *mut InstanceWrite
}

/// Destructor payload for the child-removed signal slot.
/// Original holds one retained `shared_ptr` at `this + 8` (IDA `0x703da4`).
pub struct ChildRemovedSignalData {
    pub slot: Option<SharedPtr<Instance>>,
}

/// Destructor payload for the child-added signal slot.
/// Original holds one retained `shared_ptr` at `this + 8` (IDA `0x703eac`).
pub struct ChildAddedSignalData {
    pub slot: Option<SharedPtr<Instance>>,
}

/// Destructor payload for the ancestry-changed signal.
/// Original holds two retained `shared_ptr`s, released `+16` first then `+8`
/// (IDA `0x703ed0`).
pub struct AncestryChangedSignalData {
    pub slot_hi: Option<SharedPtr<Instance>>,
    pub slot_lo: Option<SharedPtr<Instance>>,
}

/// Rust model of `RBX::Guid::Data` (the 8 bytes at `GuidItem + 12`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GuidData {
    pub lo: u32,
    pub hi: u32,
}

/// Sequence counter for `GuidData::new`.
static GUID_SEQ: AtomicU32 = AtomicU32::new(0);

impl GuidData {
    /// Rust model of `RBX::Guid::Guid(void)` (IDA `0x32281c`, via disasm):
    /// `lo` copies the init-once process seed word, `hi` is the
    /// `RbxInterlockedIncrementAcquire` sequence.
    pub fn new() -> Self {
        Self {
            lo: std::process::id(),
            hi: GUID_SEQ.fetch_add(1, Ordering::Acquire).wrapping_add(1),
        }
    }
}

/// Rust model of `RBX::GuidItem<RBX::Instance>::Registry`: the guid→instance
/// map at `+8` guarded by the mutex at `+32` (IDA `0x704ee8`).
pub struct GuidRegistry {
    pub map: Mutex<HashMap<GuidData, *const Instance>>,
}

/// Rust model of `RBX::GuidItem<RBX::Instance>` (IDA `0x704414`): registry link
/// at `+4` (raw) / `+8` (retained), guid at `+12`.
pub struct GuidItem {
    pub registry: Option<SharedPtr<GuidRegistry>>,
    pub guid: GuidData,
}

/// Rust model of `RBX::Instance::PropertyChangedSignalData` (IDA `0x704b68`):
/// D1 emits no code (`BX LR`), so there is nothing to drop; any payload is
/// trivially droppable.
pub struct PropertyChangedSignalData;

/// Rust model of `RBX::AbstractFactoryProduct<RBX::Instance>` (IDA `0x7053f8`):
/// D1 emits no code (`BX LR`); the deleting dtor only frees.
pub struct AbstractFactoryProduct;

/// Event carried by the combined child-added/removed signal. `kind` is `0`
/// for child-added and `1` for child-removed: IDA `0x7001d6` passes `0` with
/// the added-data vtable, IDA `0x70013e` passes `1` with the removed-data
/// vtable.
#[derive(Clone)]
pub struct CombinedEvent {
    pub kind: u32,
    pub child: SharedPtr<Instance>,
}

/// Rust model of `rbx::signals::signal_with_args<2, ...>` at `Instance + 80`
/// (IDA `0x703fb0`): slot iteration with per-slot retained arg copies
/// collapses into `Signal::fire`.
#[derive(Default)]
pub struct CombinedSignal {
    pub slots: Signal<CombinedEvent>,
}

/// was: `RBX::InstanceHandle` (a `shared_ptr`-like retained handle).
pub type InstanceHandle = SharedPtr<Instance>;

/// Rust model of `XmlNameValuePair` for an `InstanceHandle` (IDA `0x706198`):
/// packed name word at `+0`, retained handle box at `+8`.
pub struct XmlNameValuePair {
    pub packed: u64,
    pub handle: SharedPtr<Instance>,
}
/// Rust model of `XmlAttribute<RBX::InstanceHandle>` (IDA `0x706094`):
/// allocator state at `+0` (always `0`), name/value pair at `+4`.
pub struct XmlAttribute {
    pub alloc_state: u32,
    pub pair: XmlNameValuePair,
}

/// Rust model of `RBX::Guid::Data::operator<` (IDA `0x322b10`): lexicographic
/// comparison of the two words (first word, tie-broken by `+4`).
pub fn guid_data_less(a: &GuidData, b: &GuidData) -> bool {
    (a.lo, a.hi) < (b.lo, b.hi)
}

/// Rust model of `std::_Rb_tree<Guid::Data, pair<const Guid::Data, Instance*>,
/// ...>` — the guid→instance map at `GuidRegistry + 8` (IDA `0x705088`).
/// Keys are unique, so ordering only matters for `equal_range` bounds and the
/// tree collapses into a `HashMap` (same entry type as `GuidRegistry::map`).
pub type GuidTree = HashMap<GuidData, *const Instance>;

/// Rust model of a `std::_Rb_tree_iterator` over the guid map (IDA `0x7050b0`):
/// the key at the cursor, `None` for the end iterator.
pub type GuidIter = Option<GuidData>;

/// Operation selector for `boost::detail::function::functor_manager::manage`
/// (IDA `0x705780`/`0x706298` discriminants: 0 clone, 1 move, 2 destroy,
/// 3 check-type, 4 get-type).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctorOp {
    Clone,
    Move,
    Destroy,
    CheckType,
    GetType,
}

/// Rust model of `boost::_bi::bind_t<bool, bool (*)(Instance*, Instance*),
/// list2<value<Instance*>, arg<1>>>` (IDA `0x705780`): bound target + predicate.
/// The 8-byte `function_buffer` holds exactly these two words (disasm
/// `LDRD/STRD` at `0x705794`).
#[derive(Clone, Copy)]
pub struct BindPredicate {
    pub target: *const Instance,
    pub func: fn(*const Instance, *const Instance) -> bool,
}

/// Mangled type name `strcmp`ed by the check path (disasm `0x7057aa`-`0x7057ba`).
pub const BIND_PREDICATE_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIbPFbPN3RBX8InstanceES4_ENS0_5list2INS0_5valueIS4_EENS_3argILi1EEEEEEE";

/// Rust model of a `bool (*)(RBX::Instance *)` functor buffer (IDA `0x706298`).
#[derive(Clone, Copy)]
pub struct BoolPredicate {
    pub func: fn(*const Instance) -> bool,
}

/// Mangled type name `strcmp`ed by the check path (disasm `0x7062d2`-`0x7062e2`).
pub const BOOL_PREDICATE_TYPE_NAME: &str = "PFbPN3RBX8InstanceEE";

/// Rust model of an `rbx::signals::signal<...>::slot` link walked by `next`
/// (IDA `0x7057f0`): the intrusive `+8` successor becomes `next`;
/// retain/release become `clone`/`drop`.
pub struct SlotNode {
    pub next: Option<SharedPtr<SlotNode>>,
}

/// Process-wide mutex behind `safe_static_do_get_mutex` (IDA `0x7059a0`).
static SIGNAL_STATIC_MUTEX: Mutex<()> = Mutex::new(());

/// Global slot-exception handler for the 2-arg Instance signal (IDA `0x705950`
/// `rbx::signals::slot_exception_handler`); owned by other translation units.
pub static SLOT_EXCEPTION_HANDLER: Mutex<Option<fn(&str)>> = Mutex::new(None);
/// Rust model of one `SignatureDescriptor::Item` in an `EventDesc` /
/// `BoundYieldFuncDesc` signature list (IDA `0x70347a`, `0x7034f4`): the
/// `std::list` node + allocator reclaim collapse into a `Vec` entry holding
/// the reflected type name.
pub struct SignatureItem {
    pub type_name: &'static str,
}
/// Rust model of
/// `RBX::Reflection::BoundYieldFuncDesc<RBX::Instance, SharedPtr<Instance>(std::string), SharedPtr<Instance>, 1>`
/// (IDA `0x703444`): signature list at `+8`, bound `scoped_ptr<std::string>`
/// at `+12`.
pub struct BoundYieldFuncDesc {
    pub items: Vec<SignatureItem>,
    pub bound: Option<String>,
}
/// Rust model of `RBX::Reflection::RefPropDescriptor<RBX::Instance, RBX::Instance>`
/// (IDA `0x703498`): the conditionally-deleted heap payload at `+11`
/// (`if (v2) operator delete(v2)`); vtable resets are compiler-managed here.
pub struct RefPropDescriptor {
    pub owned: Option<Box<RefPropExtra>>,
}
/// Opaque heap payload owned by `RefPropDescriptor`.
pub struct RefPropExtra {
    pub words: [u32; 8],
}
/// Shared payload behind the `RBX::Reflection::EventDesc<RBX::Instance, ...>`
/// family (IDA `0x7034d8`/`0x703520`/`0x703544` D1, `0x7064c0`/`0x707d18` D0,
/// `0x70633c`/`0x707b28` C2): signature list at `+8` plus the connected
/// generic slots. The original inserts each slot into the *source's* member
/// signal (`*(desc + 40) + (source - 36)`, IDA `0x706738`/`0x7080d6`); with no
/// member-signal table yet the connections live on the descriptor, which
/// preserves the observable connect/fire/disconnectAll behavior.
pub struct EventDescPayload {
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
    pub items: Vec<SignatureItem>,
    pub connections: Mutex<Vec<SharedPtr<GenericSlotWrapper>>>,
    /// Direct-connect member signal for the 1-arg `SharedPtr<Instance>`
    /// family, exposed by `getSignal` (IDA `0x709e7c`) and fired alongside
    /// the generic wrappers by `fireEvent` (IDA `0x709cf4`). It models the
    /// source's `signal<void ()(SharedPtr<Instance>)>` member that
    /// `connectGeneric` (IDA `0x709b88`) and `disconnectAll` (IDA `0x709e50`)
    /// resolve through the `+40` member pointer.
    pub single: Signal<SharedPtr<Instance>>,
    /// Direct-connect member signal for the 3-arg `(string, string, Instance)`
    /// `ScriptContext` family (IDA `0x2b8c9c` fire, `0x2b8f38` disconnect):
    /// same `+40` member-pointer pattern as `single`.
    pub triple: Signal<(String, String, SharedPtr<Instance>)>,
    /// Direct-connect member signal for the 3-arg `(Instance, string,
    /// Instance)` family (IDA `0x2be4d4` fire, `0x2be728` disconnect):
    /// same pattern; the `(string, string, Instance)` spelling above is the
    /// distinct `0x2b8838` event.
    pub triple_isi: Signal<(SharedPtr<Instance>, String, SharedPtr<Instance>)>,
}
/// Rust model of `RBX::Reflection::GenericSlotWrapper` (IDA `0x708378`): the
/// marshalled script callback behind `connectGeneric`. Native handlers stand
/// in for the Lua frames until the script bridge exists.
pub struct GenericSlotWrapper {
    pub on_prop: Option<fn(*const PropertyDescriptor)>,
    pub on_pair: Option<fn(&SharedPtr<Instance>, &SharedPtr<Instance>)>,
    pub on_single: Option<fn(&SharedPtr<Instance>)>,
    pub on_triple: Option<fn(&str, &str, &SharedPtr<Instance>)>,
    pub on_triple_isi: Option<fn(&SharedPtr<Instance>, &str, &SharedPtr<Instance>)>,
}
/// Rust model of `RBX::Reflection::PropertyDescriptor` (IDA `0x706742`): only
/// pointer identity / name cross the `fireEvent` boundary here.
pub struct PropertyDescriptor {
    pub name: &'static str,
}
/// Rust model of `RBX::Reflection::Variant` values crossing `fireEvent`
/// (IDA `0x706742`, `0x707f20`): the 1-arg event carries a property
/// descriptor, the 2-arg event carries two retained instances.
pub enum Variant {
    Property(*const PropertyDescriptor),
    Instance(SharedPtr<Instance>),
    Text(String),
}
/// Rust model of `RBX::Instance::SaveFilter` (IDA `0x703748` discriminants:
/// `1` takes the service-exclusion chain, `0` the workspace chain, any other
/// value allows the write outright at `0x703804`-`0x703808`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SaveFilter(pub u32);
/// Rust model of `RBX::CreatorRole` (IDA `0x703568` `a3`, matched against the
/// creator entry's role word).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CreatorRole(pub u32);
/// Rust model of one `AbstractFactoryProduct<Instance>` creator entry (IDA
/// `0x703568`): name-sorted table node at `unk_1221848` with the role word at
/// `+4`. The table starts empty and fills as products register; lookup miss
/// takes the original `*out = 0` path.
pub struct CreatorEntry {
    pub name: &'static str,
    pub role: u32,
    pub create: fn() -> SharedPtr<Instance>,
}
/// Creator table behind `AbstractFactoryProduct<Instance>::getCreators`
/// (IDA `0x70358a`); kept sorted by name for the binary search below.
pub static CREATOR_TABLE: Mutex<Vec<CreatorEntry>> = Mutex::new(Vec::new());
/// Rust model of `boost::_bi::bind_t<void, mf2 execute2 on GenericSlotWrapper>`
/// (IDA `0x70825c`): retained wrapper (the `shared_count` copy at bind time)
/// plus late-bound instance args.
#[derive(Clone)]
pub struct BindWrapper2 {
    pub target: SharedPtr<GenericSlotWrapper>,
}
/// Rust model of `boost::function2<void, SharedPtr<Instance>,
/// SharedPtr<Instance>>` holding the `execute2` bind (IDA `0x7084e0` et al.):
/// the vtable word (`*a1`, low bit = heap tag per disasm `0x7084f2`)
/// collapses into nullability of the retained wrapper.
#[derive(Clone, Default)]
pub struct PairFunction {
    pub target: Option<SharedPtr<GenericSlotWrapper>>,
}
/// Rust model of `boost::function1<void, SharedPtr<Instance>>` holding a
/// 1-arg wrapper bind (IDA `0x709ef0` `assign_to_own`): same collapse as
/// `PairFunction`, one arg fewer.
#[derive(Clone, Default)]
pub struct SingleFunction {
    pub target: Option<SharedPtr<GenericSlotWrapper>>,
}
/// Rust model of an `rbx::signals::signal<void
/// ()(SharedPtr<Instance>, SharedPtr<Instance>)>::slot` link (IDA `0x708d24`
/// ctor, `0x709170` connected): the intrusive `+8` successor becomes `next`
/// (cf. 1-arg `SlotNode`); retain/release become `clone`/`drop`.
pub struct PairSlotNode {
    pub next: Option<SharedPtr<PairSlotNode>>,
    pub func: PairFunction,
}
/// Rust model of an `rbx::signals::signal<void ()(SharedPtr<Instance>)>::slot`
/// link (IDA `0x709ef0` ctor, `0x709ff0` remove): 1-arg twin of `PairSlotNode`.
pub struct SingleSlotNode {
    pub next: Option<SharedPtr<SingleSlotNode>>,
    pub func: SingleFunction,
}
/// Connection handle returned by the 2-arg `signal::connect` (IDA `0x708c08`).
/// The original hands out an intrusive slot pointer unlinked by
/// `slot::disconnect` (IDA `0x709060`); here the handle owns the closure's
/// strong ref, so dropping it expires the signal's weak slot — the same
/// unlink. Kept alive by the connecter until disconnect.
pub struct PairConnection {
    pub keep: SharedPtr<dyn Any + Send + Sync>,
}
/// Process-wide mutex behind the 2-arg slot's `safe_static_do_get_mutex`
/// (IDA `0x7094e4`); twin of `SIGNAL_STATIC_MUTEX` (IDA `0x7059a0`).
static PAIR_SLOT_STATIC_MUTEX: Mutex<()> = Mutex::new(());
/// `ClassDescriptor::isA` (IDA `0x703782` and siblings): walks the descriptor
/// base chain in the original; with no hierarchy modelled yet only exact
/// class-name matches report true.
pub fn instance_is_a(this: *const Instance, class: &'static str) -> bool {
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe { (*this).class_name == class }
}

// 0x703444 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED1Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()
pub fn stub_0x703444(this: *mut BoundYieldFuncDesc) {
    // IDA 0x703444: vtable reset (compiler-managed here),
    // `scoped_ptr<string>::~scoped_ptr(a1 + 12)`, then the base
    // `SignatureDescriptor` D1 (`_M_clear(a1 + 8)`); the D1 keeps storage.
    // SAFETY: `this` must point to a valid `BoundYieldFuncDesc`.
    unsafe {
        (*this).bound = None;
        (*this).items.clear();
    }
}
// 0x703484 — __ZNK3RBX8Instance7getNameEv
#[doc(alias = "RBX::Instance::getName(void)const")]
pub fn stub_0x703484(this: *const Instance) -> *const InstanceName {
    // IDA 0x703484: LDR R0,[R0,#0x44]; ADDS R0,#0x18; BX LR.
    // Returns the address of the embedded name: `*(this + 17) + 24`.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe { core::ptr::addr_of!((*this).name) }
}
// 0x70348c — __ZNK3RBX8Instance18getParentDangerousEv
#[doc(alias = "RBX::Instance::getParentDangerous(void)const")]
pub fn stub_0x70348c(this: *const Instance) -> *const Instance {
    // IDA 0x70348c: LDR R0,[R0,#0x34]; BX LR.
    // Unretained parent pointer: `*(this + 13)`.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe { (*this).parent }
}
// 0x703490 — __ZN3RBX8Instance9setParentEPS0_
#[doc(alias = "RBX::Instance::setParent(RBX::Instance*)")]
pub fn stub_0x703490(this: *mut Instance, new_parent: *const Instance) -> bool {
    // IDA 0x703490: `MOVS R2,#0; B.W setParentInternal` — pure delegation
    // with `a3 = 0` (lock check enabled).
    // SAFETY: same contract as `crate::generated_86::stub_6ffc98`.
    crate::generated_86::stub_6ffc98(this, new_parent, false)
}
// 0x703498 — __ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_0x703498(this: *mut RefPropDescriptor) {
    // IDA 0x703498: two vtable resets (compiler-managed here), then the
    // conditional `operator delete` of the heap word at `+11`.
    // SAFETY: `this` must point to a valid `RefPropDescriptor`.
    unsafe {
        (*this).owned = None;
    }
}
// 0x7034c4 — __ZNK3RBX8Instance15getRobloxLockedEv
#[doc(alias = "RBX::Instance::getRobloxLocked(void)const")]
pub fn stub_0x7034c4(this: *const Instance) -> bool {
    // IDA 0x7034c4: LDR R0,[R0,#0x44]; LDRB R0,[R0,#0x16]; BX LR.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe { (*this).roblox_locked }
}
// 0x7034cc — __ZN3RBX8Instance27getOrCreateChildAddedSignalEv
#[doc(alias = "RBX::Instance::getOrCreateChildAddedSignal(void)")]
pub fn stub_0x7034cc(this: *mut Instance) -> *mut Signal<SharedPtr<Instance>> {
    // IDA 0x7034cc: onDemandWrite(this) + 4.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe { &mut (*on_demand_write(this)).child_added }
}
// 0x7034d8 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()
pub fn stub_0x7034d8(this: *mut EventDescPayload) {
    // IDA 0x7034d8: vtable reset (compiler-managed here) +
    // `_M_clear(a1 + 8)`; the D1 keeps storage (cf. D0 at 0x7064c0).
    // SAFETY: `this` must point to a valid `EventDescPayload`.
    unsafe {
        (*this).items.clear();
    }
}
// 0x7034fc — __ZN3RBX8Instance29getOrCreateChildRemovedSignalEv
#[doc(alias = "RBX::Instance::getOrCreateChildRemovedSignal(void)")]
pub fn stub_0x7034fc(this: *mut Instance) -> *mut Signal<SharedPtr<Instance>> {
    // IDA 0x7034fc: onDemandWrite(this) + 8.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe { &mut (*on_demand_write(this)).child_removed }
}
// 0x703508 — __ZN3RBX8Instance32getOrCreateDescendantAddedSignalEv
#[doc(alias = "RBX::Instance::getOrCreateDescendantAddedSignal(void)")]
pub fn stub_0x703508(this: *mut Instance) -> *mut Signal<SharedPtr<Instance>> {
    // IDA 0x703508: onDemandWrite(this) + 12.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe { &mut (*on_demand_write(this)).descendant_added }
}
// 0x703514 — __ZN3RBX8Instance35getOrCreateDescendantRemovingSignalEv
#[doc(alias = "RBX::Instance::getOrCreateDescendantRemovingSignal(void)")]
pub fn stub_0x703514(this: *mut Instance) -> *mut Signal<SharedPtr<Instance>> {
    // IDA 0x703514: onDemandWrite(this) + 16.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe { &mut (*on_demand_write(this)).descendant_removing }
}
// 0x703520 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()
pub fn stub_0x703520(this: *mut EventDescPayload) {
    // IDA 0x703520: same shape as 0x7034d8 (2-arg `EventDesc` D1): vtable
    // reset + `_M_clear(a1 + 8)`, storage kept.
    // SAFETY: `this` must point to a valid `EventDescPayload`.
    unsafe {
        (*this).items.clear();
    }
}
// 0x703544 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")]
pub fn stub_0x703544(this: *mut EventDescPayload) {
    // IDA 0x703544: same shape as 0x7034d8 (property-descriptor `EventDesc`
    // D1): vtable reset + `_M_clear(a1 + 8)`, storage kept.
    // SAFETY: `this` must point to a valid `EventDescPayload`.
    unsafe {
        (*this).items.clear();
    }
}
// 0x703568 — __ZN3RBX22AbstractFactoryProductINS_8InstanceEE6createERKNS_4NameENS_11CreatorRoleE
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::create(RBX::Name const&,RBX::CreatorRole)")]
pub fn stub_0x703568(out: *mut Option<SharedPtr<Instance>>, name: &str, role: CreatorRole) -> bool {
    // IDA 0x703568: `getCreators` (table init, collapses into `CREATOR_TABLE`),
    // binary search of the name-sorted table (`v14[4] >= a2`, disasm
    // 0x7035d2-0x7035de), exact-name + role match, then the creator's
    // `shared_ptr` factory call; miss writes `*a1 = 0` and returns.
    // SAFETY: `out` must be writable.
    unsafe {
        let table = CREATOR_TABLE.lock();
        let found = table
            .binary_search_by(|e| e.name.cmp(&name))
            .ok()
            .filter(|&i| table[i].role == role.0)
            .map(|i| (table[i].create)());
        core::ptr::write(out, found);
        (*out).is_some()
    }
}
// 0x703748 — __ZN10Serializer13canWriteChildEN5boost10shared_ptrIN3RBX8InstanceEEENS3_10SaveFilterE
#[doc(alias = "Serializer::canWriteChild(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance::SaveFilter)")]
// was: Serializer::canWriteChild(boost::shared_ptr<RBX::Instance>,RBX::Instance::SaveFilter)
pub fn stub_0x703748(child: &SharedPtr<Instance>, filter: SaveFilter) -> bool {
    // IDA 0x703748: `roblox_locked` gate (`*(name_store + 0x17)`, disasm
    // 0x703752-0x703758) fails everything; `filter == 1` (disasm 0x70375c)
    // walks the service chain — StarterGui, StarterPack, ServerScript,
    // Workspace, Lighting, ServerStorage — hitting any `isA` returns true,
    // else the shared tail (disasm 0x7037ea-0x703802) returns the
    // ReplicatedStorage `isA` (the `R5 != 0x24` word there is just the
    // non-null `shared_ptr`, always true for a borrow); any other nonzero
    // filter allows outright (disasm 0x703804-0x703808); filter `0` walks the
    // Workspace/Lighting/ServerStorage chain into the same tail.
    // SAFETY: `SharedPtr` deref is valid by construction.
    let this: *const Instance = SharedPtr::as_ptr(child);
    unsafe {
        if !(*this).roblox_locked {
            return false;
        }
        if filter == SaveFilter(1) {
            for class in [
                "StarterGuiService",
                "StarterPackService",
                "ServerScriptService",
                "Workspace",
                "Lighting",
                "ServerStorage",
            ] {
                if instance_is_a(this, class) {
                    return true;
                }
            }
            return instance_is_a(this, "ReplicatedStorage");
        }
        if filter != SaveFilter(0) {
            return true;
        }
        for class in ["Workspace", "Lighting", "ServerStorage"] {
            if instance_is_a(this, class) {
                return true;
            }
        }
        instance_is_a(this, "ReplicatedStorage")
    }
}
// 0x7039cc — __ZN3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(RBX::Instance const*)")]
pub fn stub_0x7039cc(instance: *const Instance) -> *const Instance {
    // IDA 0x7039cc: `findServiceProvider(instance)` (disasm 0x7039d0), null
    // yields `0` (disasm 0x7039d8); else `find<ScriptService>()` inside that
    // provider (tail `B.W shim` at 0x7039e0). No provider marker is modelled
    // yet, so the provider is the tree root and the lookup is a pre-order
    // scan of its subtree for the `ScriptService` class; miss returns null
    // on the same path as the original's `return 0`.
    // SAFETY: `instance` must be null or point to a valid `Instance` whose
    // whole ancestry/subtree outlives the call.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack = vec![root];
        while let Some(cur) = stack.pop() {
            if instance_is_a(cur, "ScriptService") {
                return cur;
            }
            for child in (*cur).children.iter().rev() {
                stack.push(SharedPtr::as_ptr(child));
            }
        }
        core::ptr::null()
    }
}
// 0x7039e4 — __ZN3RBX9weak_fromINS_8InstanceEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::WeakPtr<RBX::Instance> RBX::weak_from<RBX::Instance>(RBX::Instance*)")]
// was: boost::weak_ptr<RBX::Instance> RBX::weak_from<RBX::Instance>(RBX::Instance*)
pub fn stub_0x7039e4(out: *mut WeakPtr<Instance>, this: *const Instance) {
    // IDA 0x7039e4: null `this` yields an empty weak; otherwise the embedded
    // `enable_shared_from_this` weak (`this + 40`, px adjusted by `-36` for
    // multiple inheritance, which collapses here) is copied with a locked
    // `weak_add_ref` (`Weak::clone`). A dead (never-owned or expired) owner
    // throws `boost::bad_weak_ptr`, mapped to a panic.
    // SAFETY: `out` must be writable; `this` must be null or valid.
    unsafe {
        let weak = match this.as_ref() {
            None => WeakPtr::new(),
            Some(inst) => inst.weak_owner.clone(),
        };
        if !this.is_null() && weak.upgrade().is_none() {
            panic!("0x7039e4 RBX::weak_from<RBX::Instance>: bad_weak_ptr");
        }
        core::ptr::write(out, weak);
    }
}
// 0x703cc0 — __ZN3RBX8Instance18childRemovedSignalERN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::childRemovedSignal(rbx_core::SharedPtr<RBX::Instance> &)")]
// was: RBX::Instance::childRemovedSignal(boost::shared_ptr<RBX::Instance> &)
pub fn stub_0x703cc0(this: *mut Instance, child: &SharedPtr<Instance>) {
    // IDA 0x703cc0: if (*(a1 + 19)) emit onDemandWrite(a1) + 8 with a cloned
    // shared_ptr, then release the clone.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        if (*this).write.is_some() {
            let block = on_demand_write(this);
            // `clone` + end-of-scope drop mirrors the original `shared_count`
            // copy + `sp_counted_base::release`.
            (*block).child_removed.fire(child.clone());
        }
    }
}
// 0x703da4 — __ZN3RBX8Instance22ChildRemovedSignalDataD1Ev
#[doc(alias = "RBX::Instance::ChildRemovedSignalData::~ChildRemovedSignalData()")]
pub fn stub_0x703da4(this: *mut ChildRemovedSignalData) {
    // IDA 0x703da4: reset vtable, then release the retained ref at this + 8.
    // Rust drops the Arc here, which is the same release; the vtable is
    // compiler-managed.
    // SAFETY: `this` must point to a valid `ChildRemovedSignalData`.
    unsafe {
        (*this).slot = None;
    }
}
// 0x703dc8 — __ZN3RBX8Instance16childAddedSignalERN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::childAddedSignal(rbx_core::SharedPtr<RBX::Instance> &)")]
// was: RBX::Instance::childAddedSignal(boost::shared_ptr<RBX::Instance> &)
pub fn stub_0x703dc8(this: *mut Instance, child: &SharedPtr<Instance>) {
    // IDA 0x703dc8: if (*(a1 + 19)) emit onDemandWrite(a1) + 4 with a cloned
    // shared_ptr, then release the clone.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        if (*this).write.is_some() {
            let block = on_demand_write(this);
            // `clone` + end-of-scope drop mirrors the original `shared_count`
            // copy + `sp_counted_base::release`.
            (*block).child_added.fire(child.clone());
        }
    }
}
// 0x703eac — __ZN3RBX8Instance20ChildAddedSignalDataD1Ev
#[doc(alias = "RBX::Instance::ChildAddedSignalData::~ChildAddedSignalData()")]
pub fn stub_0x703eac(this: *mut ChildAddedSignalData) {
    // IDA 0x703eac: reset vtable, then release the retained ref at this + 8.
    // Rust drops the Arc here, which is the same release; the vtable is
    // compiler-managed.
    // SAFETY: `this` must point to a valid `ChildAddedSignalData`.
    unsafe {
        (*this).slot = None;
    }
}
// 0x703ed0 — __ZN3RBX8Instance25AncestryChangedSignalDataD1Ev
#[doc(alias = "RBX::Instance::AncestryChangedSignalData::~AncestryChangedSignalData()")]
pub fn stub_0x703ed0(this: *mut AncestryChangedSignalData) {
    // IDA 0x703ed0: reset vtable, release *(this + 4), then *(this + 2).
    // Rust drops each Arc in the same order; the vtable is compiler-managed.
    // SAFETY: `this` must point to a valid `AncestryChangedSignalData`.
    unsafe {
        (*this).slot_hi = None;
        (*this).slot_lo = None;
    }
}
// 0x703fb0 — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EEclES6_S6_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x703fb0(this: *mut CombinedSignal, kind: u32, child: &SharedPtr<Instance>) {
    // IDA 0x703fb0: no-op when the slot list is empty (`*a1 == 0`); else an
    // `FLog::SignalPrints` log line, then `next()` walks each live slot and
    // calls it with retained arg copies. The log, per-slot retain/release
    // pairs, and slot-call unwind tables collapse into `Signal::fire`, which
    // already skips dead/empty slot lists.
    // SAFETY: `this` must point to a valid `CombinedSignal`.
    unsafe {
        (*this).slots.fire(CombinedEvent { kind, child: child.clone() });
    }
}
// 0x704228 — __ZN3RBX8Instance24descendantRemovingSignalERKN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::descendantRemovingSignal(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Instance::descendantRemovingSignal(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x704228(this: *mut Instance, child: &SharedPtr<Instance>) {
    // IDA 0x704228: if (*(a1 + 19)) emit onDemandWrite(a1) + 16 with a cloned
    // shared_ptr, then release the clone.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        if (*this).write.is_some() {
            let block = on_demand_write(this);
            // `clone` + end-of-scope drop mirrors the original `shared_count`
            // copy + `sp_counted_base::release`.
            (*block).descendant_removing.fire(child.clone());
        }
    }
}
// 0x70430c — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPiENS3_5list2INS2_3argILi1EEENS3_5valueIS7_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>> const&)const
pub fn stub_0x70430c(
    this: *const Instance,
    func: fn(SharedPtr<Instance>, *mut i32),
    counter: *mut i32,
) {
    // IDA 0x70430c: retains the `list2` binder's bound `int *` (the
    // `shared_count` copy at `a1 + 14`), snapshots the child vector, then
    // applies `func(child, counter)` to every descendant; the recursion over
    // nested child vectors collapses into one explicit pre-order stack, and
    // each per-child `shared_ptr` copy/release pair is a clone/drop.
    // SAFETY: `this` must point to a valid `Instance` whose subtree outlives
    // the call; `counter` must be writable.
    unsafe {
        let mut stack: Vec<SharedPtr<Instance>> = (*this).children.clone();
        stack.reverse();
        while let Some(child) = stack.pop() {
            let nested: Vec<SharedPtr<Instance>> =
                (*SharedPtr::as_ptr(&child)).children.clone();
            for grand in nested.into_iter().rev() {
                stack.push(grand);
            }
            func(child, counter);
        }
    }
}
// 0x704414 — __ZN3RBX8GuidItemINS_8InstanceEEC2Ev
#[doc(alias = "RBX::GuidItem<RBX::Instance>::GuidItem(void)")]
pub fn stub_0x704414(this: *mut GuidItem) -> *mut GuidItem {
    // IDA 0x704414: vtable store (compiler-managed here), zero `+4`/`+8`,
    // then `RBX::Guid::Guid(this + 12)` (disasm `BL __ZN3RBX4GuidC1Ev`).
    // SAFETY: `this` must point to valid uninitialized `GuidItem` storage.
    unsafe {
        core::ptr::write(this, GuidItem { registry: None, guid: GuidData::new() });
        this
    }
}
// 0x7045b0 — __ZN3RBX8GuidItemINS_8InstanceEED2Ev
#[doc(alias = "RBX::GuidItem<RBX::Instance>::~GuidItem()")]
pub fn stub_0x7045b0(this: *mut GuidItem) {
    // IDA 0x7045b0: vtable reset (compiler-managed here); when the registry
    // link at `+4` is set, `Registry::unregister` runs and clears `+4`/`+8`,
    // so the trailing `+8` release is dead — the `Arc` drop inside
    // `unregister` is that same release.
    // SAFETY: `this` must point to a valid `GuidItem`.
    unsafe {
        let registry = (*this).registry.clone();
        if let Some(registry) = registry {
            stub_0x704ee8(SharedPtr::as_ptr(&registry), this);
        }
    }
}
// 0x704748 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf0IvS5_EENSD_5list1INS2_3argILi1EEEEEEEET0_T_SO_SN_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>)
pub fn stub_0x704748(
    items: &[SharedPtr<Instance>],
    func: fn(&SharedPtr<Instance>),
) -> fn(&SharedPtr<Instance>) {
    // IDA 0x704748: `for (i = first; i != last; i += 2)` over the
    // `shared_ptr` vector applying the `mf0` binder; the `(a5 & 1)` branch is
    // the virtual-thunk alternative, which collapses into the direct call
    // here. Returns the functor copy (`*a1 = a4`, disasm 0x704784).
    for item in items {
        func(item);
    }
    func
}
// 0x704794 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKPN3RBX10Reflection15EventDescriptorESt6vectorIS5_SaIS5_EEEEN5boost3_bi6bind_tIvNSC_4_mfi4cmf1IvS4_PNS3_11EventSourceEEENSD_5list2INSC_3argILi1EEENSD_5valueIPNS2_8InstanceEEEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)
pub fn stub_0x704794(
    items: &[*const EventDescPayload],
    source: *const Instance,
    func: fn(*const EventDescPayload, *const Instance),
) -> fn(*const EventDescPayload, *const Instance) {
    // IDA 0x704794: empty-range early-out (disasm 0x7047a8), then per
    // `EventDescriptor *` the `cmf1` binder call with the bound `Instance *`
    // adjusted to its `EventSource *` (`a6 ? a6 + 36 : a6`, disasm
    // 0x7047ae-0x7047b4 — the +36 member offset collapses since the source
    // pointer itself is threaded through here). Returns the functor copy.
    for &item in items {
        func(item, source);
    }
    func
}
// 0x704b68 — __ZN3RBX8Instance25PropertyChangedSignalDataD1Ev
#[doc(alias = "RBX::Instance::PropertyChangedSignalData::~PropertyChangedSignalData()")]
pub fn stub_0x704b68(_this: *mut PropertyChangedSignalData) {
    // IDA 0x704b68: `BX LR` — empty.
}
// 0x704ee8 — __ZN3RBX8GuidItemINS_8InstanceEE8Registry10unregisterEPS2_
#[doc(alias = "RBX::GuidItem<RBX::Instance>::Registry::unregister(RBX::GuidItem<RBX::Instance>*)")]
pub fn stub_0x704ee8(registry: *const GuidRegistry, item: *mut GuidItem) {
    // IDA 0x704ee8: `ReleaseAssert(item->registry.get() == this)` (Guid.h:144,
    // gated by `FLog::Asserts`, hence `debug_assert`); lock the `+32` mutex;
    // `_Rb_tree::erase(+8, guid)`; `ReleaseAssert(num == 1)` (Guid.h:150);
    // unlock; clear `+4` and release `+8`. `HashMap::remove` under the lock is
    // the same erase, and clearing the `Option` is the same release.
    // SAFETY: `registry` must be valid; `item` must be valid and mutable.
    unsafe {
        let item = &mut *item;
        debug_assert!(item.registry.as_ref().is_some_and(|r| SharedPtr::as_ptr(r) == registry));
        let removed = (*registry).map.lock().remove(&item.guid);
        debug_assert!(removed.is_some());
        item.registry = None;
    }
}
// 0x705088 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseERS4_
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::erase(RBX::Guid::Data const&)")]
pub fn stub_0x705088(tree: *mut GuidTree, key: &GuidData) -> usize {
    // IDA 0x705088: `equal_range(key)` into a stack pair (disasm `BL equal_range`
    // at 0x705096), snapshot `size` (`LDR R5,[R4,#0x14]`), `erase(first, last)`
    // (disasm 0x7050a2), return `old - new` (`SUBS R0,R5,R0`).
    // SAFETY: `tree` must point to a valid `GuidTree`; `key` must be readable.
    unsafe {
        let (first, last) = stub_0x7050b0(tree, key);
        let old = (*tree).len();
        stub_0x705110(tree, first, last);
        old - (*tree).len()
    }
}
// 0x7050b0 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE11equal_rangeERS4_
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::equal_range(RBX::Guid::Data const&)")]
pub fn stub_0x7050b0(tree: *const GuidTree, key: &GuidData) -> (GuidIter, GuidIter) {
    // IDA 0x7050b0: two descending walks from the root (`+8`): the lower walk
    // keeps the last node with `!(node < key)` (disasm 0x7050ca-0x7050e4), the
    // upper walk keeps the last node with `key < node` (disasm 0x7050e8-0x705102),
    // both via `Guid::Data::operator<` (IDA 0x322b10, see `guid_data_less`).
    // Keys are unique so the range holds at most `key` itself; linear scans
    // find the same lower/upper bounds.
    // SAFETY: `tree` must point to a valid `GuidTree`; `key` must be readable.
    unsafe {
        let mut lower: GuidIter = None;
        let mut upper: GuidIter = None;
        for k in (*tree).keys() {
            if !guid_data_less(k, key) && lower.is_none_or(|l| guid_data_less(k, &l)) {
                lower = Some(*k);
            }
            if guid_data_less(key, k) && upper.is_none_or(|u| guid_data_less(&u, k)) {
                upper = Some(*k);
            }
        }
        (lower, upper)
    }
}
// 0x705110 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,RBX::Instance *>>)")]
pub fn stub_0x705110(tree: *mut GuidTree, first: GuidIter, last: GuidIter) {
    // IDA 0x705110: `first == begin && last == end` (disasm 0x705124-0x70512a)
    // takes the `_M_erase(root)` + header-reset path (disasm 0x705154-0x705168);
    // else loop `increment; rebalance_for_erase; operator delete; size--`
    // (disasm 0x705130-0x705150). C++03 returns `void` here; the decompiler's
    // trailing value is a leftover register. The fast path and the loop agree
    // on observable state, so one bounded-removal loop covers both (equal
    // bounds erase nothing, matching the original's `(end, end)` no-op);
    // per-node `operator delete` is allocator reclaim.
    // SAFETY: `tree` must point to a valid `GuidTree`.
    unsafe {
        if first == last {
            return;
        }
        let doomed: Vec<GuidData> = (*tree)
            .keys()
            .copied()
            .filter(|k| {
                first.is_none_or(|f| !guid_data_less(k, &f))
                    && last.is_none_or(|l| guid_data_less(k, &l))
            })
            .collect();
        for k in doomed {
            (*tree).remove(&k);
        }
    }
}
// 0x705170 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,RBX::Instance *>> *)")]
pub fn stub_0x705170(tree: *mut GuidTree, node: GuidIter) {
    // IDA 0x705170: null node returns at once (`CMP R5,#0; POPEQ`, disasm
    // 0x705176-0x70517c); else post-order `_M_erase(left at +12)`, take right
    // at `+8`, `operator delete(node)`, continue with right (disasm
    // 0x70517e-0x705192). The flat `HashMap` model has no child links or
    // manual nodes: destroying one node is removing its entry, and the
    // per-node `operator delete` is allocator reclaim.
    // SAFETY: `tree` must point to a valid `GuidTree`.
    unsafe {
        if let Some(k) = node {
            (*tree).remove(&k);
        }
    }
}
// 0x7053f8 — __ZN3RBX22AbstractFactoryProductINS_8InstanceEED1Ev
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::~AbstractFactoryProduct()")]
pub fn stub_0x7053f8(_this: *mut AbstractFactoryProduct) {
    // IDA 0x7053f8: `BX LR` — empty.
}
// 0x7053fc — __ZN3RBX22AbstractFactoryProductINS_8InstanceEED0Ev
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::~AbstractFactoryProduct()")]
pub fn stub_0x7053fc(this: *mut AbstractFactoryProduct) {
    // IDA 0x7053fc: the deleting dtor is a bare `B.W __ZdlPv$shim` (D1 is
    // empty, so nothing runs before `operator delete`). Reclaiming the box
    // frees the same global-heap allocation; there is no `Drop` to run.
    // SAFETY: `this` must be a live box-allocated `AbstractFactoryProduct`.
    unsafe {
        drop(Box::from_raw(this));
    }
}
// 0x70566c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPiEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEES5_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,int *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0x70566c(
    func: fn(SharedPtr<Instance>, *mut i32),
    bound: *mut i32,
    arg: &SharedPtr<Instance>,
) {
    // IDA 0x70566c: `shared_count` copy of the incoming `list1` arg
    // (`BL shared_count::shared_count`, disasm 0x70569e), call
    // `f(retained, *value<int*>)` (decomp `v13(&pi, *a1)`), then `release` on
    // scope exit — with SjLj unwind tables. `clone` + call + end-of-scope drop
    // is the same pair; a panic unwinds through the caller instead.
    func(arg.clone(), bound);
}
// 0x705780 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbPN3RBX8InstanceES7_ENS3_5list2INS3_5valueIS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x705780(src: &BindPredicate, dst: &mut BindPredicate, op: FunctorOp) -> bool {
    // IDA 0x705780 (disasm): clone/move (0/1) copy the 8-byte buffer when the
    // destination is set (`LDRD/STRD`, 0x705794-0x705798) — move leaves the
    // source stale, with no zeroing. destroy (2) is a bare return: the bind
    // owns nothing. check (3) `strcmp`s the stored name
    // (`BIND_PREDICATE_TYPE_NAME`, 0x7057aa-0x7057ba) and keeps the buffer on
    // match, else zeroes it — a mismatch is unreachable in this single-type
    // model. get (4) stores the `typeinfo` pointer + zero half — the compiler
    // owns types here. Returns whether `dst` holds a live functor afterwards.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = *src;
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = *src;
            true
        }
        FunctorOp::GetType => true,
    }
}
// 0x7057e0 — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIbPFbPN3RBX8InstanceES7_ENS3_5list2INS3_5valueIS7_EENS_3argILi1EEEEEEEbS7_E6invokeERNS1_15function_bufferES7_
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>,bool,RBX::Instance *>::invoke(boost::detail::function::function_buffer &,RBX::Instance *)")]
// was: boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>,bool,RBX::Instance *>::invoke(boost::detail::function::function_buffer &,RBX::Instance *)
pub fn stub_0x7057e0(bound: &BindPredicate, arg: *const Instance) -> bool {
    // IDA 0x7057e0: `LDRD R2,R3,[R0]` loads the fn ptr + bound
    // `value<Instance*>` from the buffer, `BLX R2` with the `Instance*` arg.
    (bound.func)(bound.target, arg)
}
// 0x7057f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot> &)
pub fn stub_0x7057f0(
    head: &Option<SharedPtr<SlotNode>>,
    cursor: &mut Option<SharedPtr<SlotNode>>,
) -> bool {
    // IDA 0x7057f0: add_ref the incoming cursor (disasm 0x705840-0x70584a);
    // `call_once` static init + `safe_static_do_get_mutex` (0x70584e-0x705876);
    // lock; live cursor → `operator=(*cursor + 8)` (0x70588a-0x705898), empty
    // cursor → `operator=(head)` (0x70589e-0x7058a4); unlock; release the old
    // ref; return non-null (0x7058d6-0x7058f2). Clone-then-assign +
    // end-of-scope drop is the same retain/release pairing.
    let _guard = stub_0x7059a0().lock();
    *cursor = match cursor.as_ref() {
        Some(node) => node.next.clone(),
        None => head.clone(),
    };
    cursor.is_some()
}
// 0x705950 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
pub fn stub_0x705950(err: &str) {
    // IDA 0x705950: null handler → return the handler slot untouched; set
    // handler → `function1::operator()(exc)` after the `dummy::nonnull` check
    // (disasm 0x705968-0x705972). `err` carries `std::exception::what()`.
    if let Some(handler) = *SLOT_EXCEPTION_HANDLER.lock() {
        handler(err);
    }
}
// 0x705978 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0x705978(
    dst: *mut Option<SharedPtr<SlotNode>>,
    src: &Option<SharedPtr<SlotNode>>,
) {
    // IDA 0x705978: add_ref(src) (disasm 0x705980-0x705986), store over `dst`,
    // release(old) (0x70598e-0x705992). Clone-then-assign is the same order
    // and is self-assignment safe via the temporary.
    // SAFETY: `dst` must be writable; `src` must be readable.
    unsafe {
        *dst = src.clone();
    }
}
// 0x7059a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_0x7059a0() -> &'static Mutex<()> {
    // IDA 0x7059a0: guard-checked once-init (`__cxa_guard_acquire`, disasm
    // 0x7059fa-0x705a00), `operator new(0x2c)` + `mutex::mutex` (disasm
    // 0x705a08-0x705a16), `__cxa_guard_release`. A `static` with `const` init
    // is the same once-init; the 0x2c-byte pthread object lives inside `Mutex`.
    &SIGNAL_STATIC_MUTEX
}
// 0x705a98 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SC_SC_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")]
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance> const&,std::random_access_iterator_tag)
pub fn stub_0x705a98(haystack: &[SharedPtr<Instance>], needle: &SharedPtr<Instance>) -> usize {
    // IDA 0x705a98: 4-wide unrolled scan (`ASR#5`, stride 0x20 = four 8-byte
    // shared_ptrs, disasm 0x705aa8-0x705ada) comparing px words, then a
    // 3/2/1-element tail (disasm 0x705adc-0x705b22); returns `last` on miss.
    // `position` is the same search; the unrolling is codegen.
    haystack
        .iter()
        .position(|item| SharedPtr::ptr_eq(item, needle))
        .unwrap_or(haystack.len())
}
// 0x705b28 — __ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEaSERKS7_
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::operator=(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> const&)")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::operator=(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)
pub fn stub_0x705b28(
    this: *mut SharedPtr<Vec<SharedPtr<Instance>>>,
    other: *const SharedPtr<Vec<SharedPtr<Instance>>>,
) -> *mut SharedPtr<Vec<SharedPtr<Instance>>> {
    // IDA 0x705b28: retain `other`, copy px/pi over `this`, release the old
    // control block. `Arc` clone-then-assign is the same ordering.
    // SAFETY: `this` must be writable and `other` readable; both valid.
    unsafe {
        *this = (*other).clone();
        this
    }
}
// 0x705b60 — __ZN5boost8weak_ptrIN3RBX8InstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::Instance>::weak_ptr<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&,boost::detail::sp_enable_if_convertible<RBX::Instance,RBX::Instance>::type)")]
// was: boost::weak_ptr<RBX::Instance>::weak_ptr<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&,boost::detail::sp_enable_if_convertible<RBX::Instance,RBX::Instance>::type)
pub fn stub_0x705b60(dst: *mut WeakPtr<Instance>, src: *const SharedPtr<Instance>) {
    // IDA 0x705b60 (`weak_ptr(shared_ptr const&)`): copy px/pi, then
    // `weak_add_ref` under the spinlock pool. `Arc::downgrade` is the same.
    // SAFETY: `dst` must be writable `WeakPtr` storage; `src` a valid `SharedPtr`.
    unsafe {
        core::ptr::write(dst, SharedPtr::downgrade(&*src));
    }
}
// 0x705fd0 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKNS4_10Reflection13DescribedBaseEEEET_SH_SH_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Reflection::DescribedBase const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Reflection::DescribedBase const> const&,std::random_access_iterator_tag)")]
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Reflection::DescribedBase const>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Reflection::DescribedBase const> const&,std::random_access_iterator_tag)
pub fn stub_0x705fd0(haystack: &[SharedPtr<Instance>], needle: *const Instance) -> usize {
    // IDA 0x705fd0: same 4-wide scan, but each element is adjusted to its
    // `DescribedBase` subobject (`+36`, `ADDNE #0x24`; null stays null, disasm
    // 0x705fe4-0x705fec) before comparing with the `DescribedBase const*` key.
    // Each `Instance` owns exactly one such subobject, so the key is already
    // the owner's identity in this model; null elements are unrepresentable
    // (`SharedPtr`), so a null key matches nothing.
    // SAFETY: `needle` must be null or carry the `DescribedBase` identity of a
    // live `Instance` (i.e. its address in this model).
    if needle.is_null() {
        return haystack.len();
    }
    haystack
        .iter()
        .position(|item| SharedPtr::as_ptr(item) == needle)
        .unwrap_or(haystack.len())
}
// 0x706094 — __ZN12XmlAttributeC2IN3RBX14InstanceHandleEEERKNS1_4NameET_
#[doc(alias = "XmlAttribute::XmlAttribute<RBX::InstanceHandle>(RBX::Name const&,RBX::InstanceHandle)")]
// was: XmlAttribute::XmlAttribute<RBX::InstanceHandle>(RBX::Name const&,RBX::InstanceHandle)
pub fn stub_0x706094(this: *mut XmlAttribute, name: u32, handle: &SharedPtr<Instance>) -> *mut XmlAttribute {
    // IDA 0x706094: `*a1 = 0`; `XmlNameValuePair(a1 + 1, name, handle)`;
    // `RBX::Allocator<XmlAttribute>::Allocator` is empty. `name` is the
    // 32-bit `RBX::Name` id (`unsigned int` at IDA 0x7061bc).
    // SAFETY: `this` must point to valid uninitialized `XmlAttribute` storage.
    unsafe {
        core::ptr::addr_of_mut!((*this).alloc_state).write(0);
        stub_0x706198(core::ptr::addr_of_mut!((*this).pair), name, handle);
        this
    }
}
// 0x706198 — __ZN16XmlNameValuePairC2ERKN3RBX4NameENS0_14InstanceHandleE
#[doc(alias = "XmlNameValuePair::XmlNameValuePair(RBX::Name const&,RBX::InstanceHandle)")]
pub fn stub_0x706198(this: *mut XmlNameValuePair, name: u32, handle: &SharedPtr<Instance>) -> *mut XmlNameValuePair {
    // IDA 0x706198: `*(u64 *)a1 = name | 0x800000000`; the handle is cloned
    // into a fresh 8-byte box stored at `a1 + 8` (the box only carries the
    // retained control block; the `Arc` clone is that same retain).
    // SAFETY: `this` must point to valid uninitialized `XmlNameValuePair` storage.
    unsafe {
        core::ptr::write(
            this,
            XmlNameValuePair { packed: u64::from(name) | 0x8_0000_0000, handle: handle.clone() },
        );
        this
    }
}
// 0x706298 — __ZN5boost6detail8function15functor_managerIPFbPN3RBX8InstanceEEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<bool (*)(RBX::Instance *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<bool (*)(RBX::Instance *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x706298(src: &BoolPredicate, dst: &mut BoolPredicate, op: FunctorOp) -> bool {
    // IDA 0x706298 (disasm): clone (0) copies the word (`LDR/STR`, 0x7062ae);
    // move (1) copies then zeroes the source (0x7062c8-0x7062ce) — unobservable
    // here since the source is dead after a move; destroy (2) is a bare return
    // (0x7062ee): the fn ptr owns nothing. check (3) `strcmp`s the stored name
    // (`BOOL_PREDICATE_TYPE_NAME`, 0x7062d2-0x7062e2) and keeps the buffer on
    // match, else zeroes it — a mismatch is unreachable in this single-type
    // model. get (default/4) stores the `typeinfo` pointer + zero half
    // (0x7062b4-0x7062c4) — the compiler owns types here. Returns whether
    // `dst` holds a live functor afterwards.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = *src;
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = *src;
            true
        }
        FunctorOp::GetType => true,
    }
}
// 0x7062f8 — __ZN5boost20dynamic_pointer_castIN3RBX8InstanceENS1_6ObjectEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> boost::dynamic_pointer_cast<RBX::Instance,RBX::Object>(rbx_core::SharedPtr<RBX::Object> const&)")]
// was: boost::shared_ptr<RBX::Instance> boost::dynamic_pointer_cast<RBX::Instance,RBX::Object>(boost::shared_ptr<RBX::Object> const&)
pub fn stub_0x7062f8() -> ! {
    todo!("0x7062f8 rbx_core::SharedPtr<RBX::Instance> boost::dynamic_pointer_cast<RBX::Instance,RBX::Object>(rbx_core::SharedPtr<RBX::Object> const&)")
}
// 0x70633c — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x70633c(
    this: *mut EventDescPayload,
    name: &str,
    permissions: u32,
    attributes: u32,
) {
    // IDA 0x70633c: `Described<Instance>::classDescriptor()` (static, no
    // state — collapses), `EventDescriptor::EventDescriptor(a1, ...)` (base
    // init), then one signature item for the `PropertyDescriptor const *`
    // arg (the single `Type` lookup + `_M_insert`, cf. the two-item 0x707b28).
    // SAFETY: `this` must point to valid uninitialized `EventDescPayload` storage.
    unsafe {
        core::ptr::write(
            this,
            EventDescPayload {
                name: name.to_string(),
                permissions,
                attributes,
                items: vec![SignatureItem { type_name: "PropertyDescriptor const*" }],
                connections: Mutex::new(Vec::new()),
                single: Signal::new(),
                triple: Signal::new(),
                triple_isi: Signal::new(),
            },
        );
    }
}
// 0x7064c0 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")]
pub fn stub_0x7064c0(this: *mut EventDescPayload) {
    // IDA 0x7064c0: D0 — the D1 body (`*a1 = EventDescriptor vtable`,
    // `_M_clear(a1 + 8)`, disasm 0x7064fe-0x706524) plus `operator delete(a1)`
    // (disasm 0x70652a). Reclaiming the box runs the field drops (the clear)
    // and frees storage (the delete) together.
    // SAFETY: `this` must be a live box pointer that is never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}
// 0x706574 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x706574(desc: *const EventDescPayload, slot: &SharedPtr<GenericSlotWrapper>) {
    // IDA 0x706574: retains the wrapper `shared_ptr` (`shared_count` copy at
    // `a4 + 4`, disasm 0x7065a0) and inserts the new slot into the source's
    // member signal (`*(a1 + 40) + (a2 - 36)`); the member-signal addressing
    // collapses into the payload-side connection list (see
    // `EventDescPayload`), so connect is a retained clone + push.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().push(slot.clone());
    }
}
// 0x7066c8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x7066c8(desc: *const EventDescPayload, args: &[Variant]) {
    // IDA 0x7066c8: `ReleaseAssert(args.size() == 1)` (Event.h:320, disasm
    // 0x7066e2-0x70672c), `any_cast<PropertyDescriptor const*>(args[0])`
    // (disasm 0x706742), then `signal_with_args<1>::operator()(signal, desc)`
    // (disasm 0x706738-0x706742) — each connected wrapper's `execute1` runs
    // with the cast descriptor.
    // SAFETY: `desc` must point to a valid `EventDescPayload`; any
    // `Variant::Property` pointer must stay valid through dispatch.
    assert!(args.len() == 1, "0x7066c8: args.size() == 1");
    let prop = match &args[0] {
        Variant::Property(p) => *p,
        _ => panic!("0x7066c8: any_cast<PropertyDescriptor const*> failed"),
    };
    unsafe {
        let slots = (*desc).connections.lock().clone();
        for slot in slots.iter() {
            if let Some(cb) = slot.on_prop {
                cb(prop);
            }
        }
    }
}
// 0x706754 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x706754(desc: *const EventDescPayload) {
    // IDA 0x706754: `source ? source - 36 : 0` (disasm 0x706754-0x70675a)
    // selects the member signal at `*(a1 + 40) + v10`, then
    // `signal::disconnectAll` on it; the addressing collapses into the
    // payload-side list, so this clears the connections.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().clear();
    }
}
// 0x707b28 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x707b28(
    this: *mut EventDescPayload,
    name: &str,
    permissions: u32,
    attributes: u32,
) {
    // IDA 0x707b28: same ctor shape as 0x70633c for the 2-`shared_ptr`
    // `EventDesc`: `classDescriptor()` (collapses), base
    // `EventDescriptor::EventDescriptor`, then two signature items (the two
    // `Type` lookups + `_M_insert` pair).
    // SAFETY: `this` must point to valid uninitialized `EventDescPayload` storage.
    unsafe {
        core::ptr::write(
            this,
            EventDescPayload {
                name: name.to_string(),
                permissions,
                attributes,
                items: vec![
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                ],
                connections: Mutex::new(Vec::new()),
                single: Signal::new(),
                triple: Signal::new(),
                triple_isi: Signal::new(),
            },
        );
    }
}
// 0x707d18 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()
pub fn stub_0x707d18(this: *mut EventDescPayload) {
    // IDA 0x707d18: D0 — D1 body (`*a1` vtable reset + `_M_clear`,
    // disasm 0x707d56-0x707d7c) plus `operator delete` (disasm 0x707d82);
    // the box reclaim is both.
    // SAFETY: `this` must be a live box pointer that is never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}
// 0x707dcc — __ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x707dcc(desc: *const EventDescPayload, slot: &SharedPtr<GenericSlotWrapper>) {
    // IDA 0x707dcc: 2-arg twin of 0x706574 — retain the wrapper
    // (`shared_count` copy) and insert into the member signal; collapses to
    // a retained clone + push onto the payload-side list.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().push(slot.clone());
    }
}
// 0x707f20 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x707f20(desc: *const EventDescPayload, args: &[Variant]) {
    // IDA 0x707f20: 2-arg twin of 0x7066c8 — assert `args.size() == 2`,
    // `any_cast` both `shared_ptr<Instance>` args (the two `shared_count`
    // copies at `[bp-9Ch]`/`[bp-94h]` are retains), then
    // `signal_with_args<2>::operator()` fans out to each connected
    // wrapper's `execute2`.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    assert!(args.len() == 2, "0x707f20: args.size() == 2");
    let (a, b) = match (&args[0], &args[1]) {
        (Variant::Instance(a), Variant::Instance(b)) => (a.clone(), b.clone()),
        _ => panic!("0x707f20: any_cast<shared_ptr<Instance>> failed"),
    };
    unsafe {
        let slots = (*desc).connections.lock().clone();
        for slot in slots.iter() {
            stub_0x708378(slot, &a, &b);
        }
    }
}
// 0x7080d0 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x7080d0(desc: *const EventDescPayload) {
    // IDA 0x7080d0: 2-arg twin of 0x706754 — `source ? source - 36 : 0`
    // (disasm 0x7080d0-0x7080d6) into `*(a1 + 40) + v2`, then
    // `signal::disconnectAll`; collapses to clearing the payload-side list.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().clear();
    }
}
// 0x7080e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_0x7080e4(sig: *mut Signal<(SharedPtr<Instance>, SharedPtr<Instance>)>) {
    // IDA 0x7080e4: mutex acquisition (`safe_static` guard dance) then every
    // slot unlinked and released; `Signal::disconnect_all` holds the same
    // lock and drops the same slot list.
    // SAFETY: `sig` must point to a valid `Signal`.
    unsafe {
        (*sig).disconnect_all();
    }
}
// 0x70825c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEES8_NS4_IS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_0x70825c(target: &SharedPtr<GenericSlotWrapper>) -> BindWrapper2 {
    // IDA 0x70825c: `boost::bind(execute2-mf2, wrapper, _1, _2)` — the
    // wrapper `shared_ptr` is retained into the `bind_t` object (the
    // `shared_count` copy) while `_1`/`_2` stay late-bound; the `list_av_3`
    // storage collapses into the retained target.
    BindWrapper2 { target: target.clone() }
}
// 0x708378 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEES6_EEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x708378(
    wrapper: &SharedPtr<GenericSlotWrapper>,
    a: &SharedPtr<Instance>,
    b: &SharedPtr<Instance>,
) {
    // IDA 0x708378: `GenericSlotWrapper::execute2` unpacks the marshalled
    // 2-arg functor from the wrapper (`a1` + slots) and invokes it with the
    // two retained `shared_ptr` args; the Lua frame underneath is the
    // `on_pair` handler until the script bridge exists.
    if let Some(cb) = wrapper.on_pair {
        cb(a, b);
    }
}
// 0x7084e0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E5clearEv
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::clear(void)
pub fn stub_0x7084e0(func: &mut PairFunction) {
    // IDA 0x7084e0: null vtable returns at once (`CMP R0,#0`, disasm
    // 0x7084e6-0x7084ec); else the heap-tag check (`TST R0,#1`, disasm
    // 0x7084f2) routes to the manager destroy op (`v3(a1 + 1, a1 + 1, 2)`,
    // disasm 0x7084f8-0x708504), then `*a1 = 0`. Nullability of `target` is
    // the vtable word, so clear covers all three paths.
    func.target = None;
}
// 0x7086d8 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0x7086d8(dst: &mut PairFunction, src: &BindWrapper2) {
    // IDA 0x7086d8: `function2::assign_to<bind_t>` spills the bind functor
    // and routes through the vtable `assign_to` into `assign_functor`
    // (heap `operator new(0x10)` + memberwise + `shared_count` copy, IDA
    // 0x7089c0); the retained wrapper clone is that same copy.
    dst.target = Some(src.target.clone());
}
// 0x7087d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x7087d0(src: &BindWrapper2, dst: &mut BindWrapper2, op: FunctorOp) -> bool {
    // IDA 0x7087d0: `op != 4` delegates to `manager` (disasm 0x7087d2);
    // the op-4 (get) fast path and the manager arms mirror the 1-arg
    // `0x705780`/`0x706298` discriminants: clone/move copy the bind (the
    // memberwise + `shared_count` copy), destroy drops, check/get report.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = src.clone();
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = src.clone();
            true
        }
        FunctorOp::GetType => true,
    }
}
// 0x7087ec — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSC_SC_E6invokeERNS1_15function_bufferESC_SC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x7087ec(bind: &BindWrapper2, a: &SharedPtr<Instance>, b: &SharedPtr<Instance>) {
    // IDA 0x7087ec: tail-calls
    // `bind_t::operator()<SharedPtr<Instance>, SharedPtr<Instance>>` (IDA
    // 0x708a94) with the buffer + both args; that dispatch is `execute2`.
    stub_0x708a94(bind, a, b);
}
// 0x7087f4 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x7087f4(dst: &mut PairFunction, src: &BindWrapper2) -> bool {
    // IDA 0x7087f4: `basic_vtable2::assign_to` (no tag): spills the bind
    // functor and heap-installs it via `assign_functor` (IDA 0x7089c0);
    // always fits, hence always true.
    stub_0x7086d8(dst, src);
    true
}
// 0x7088dc — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x7088dc(dst: &mut PairFunction, src: &BindWrapper2) -> bool {
    // IDA 0x7088dc: `basic_vtable2::assign_to` with `function_obj_tag`:
    // same heap-install path as 0x7087f4 (the tag only selects this
    // overload); always true.
    stub_0x7086d8(dst, src);
    true
}
// 0x7089c0 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x7089c0(src: &BindWrapper2) -> Box<BindWrapper2> {
    // IDA 0x7089c0: `operator new(0x10)`, memberwise copy of the three bind
    // words (disasm 0x7089fa-0x708a04), `shared_count` copy of the wrapper
    // (disasm 0x708a32), `*a3 = v6`. Boxing the clone is the same
    // heap-install with the same retained copy.
    Box::new(src.clone())
}
// 0x708a94 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &,rbx_core::SharedPtr<RBX::Instance> &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> &,boost::shared_ptr<RBX::Instance> &)
pub fn stub_0x708a94(bind: &BindWrapper2, a: &SharedPtr<Instance>, b: &SharedPtr<Instance>) {
    // IDA 0x708a94: loads the mf2 fn ptr + wrapper from the bind (disasm
    // 0x708a94-0x708a96), resolves the member target with the virtual-thunk
    // check (`(v1 & 1)`, disasm 0x708aa4-0x708aa8 — collapses to the direct
    // member here), then `v2(v3)` = `execute2(wrapper, a, b)` (IDA 0x708378).
    stub_0x708378(&bind.target, a, b);
}
// 0x708ab0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x708ab0(src: &BindWrapper2, dst: &mut Option<Box<BindWrapper2>>, op: FunctorOp) -> bool {
    // IDA 0x708ab0: `functor_manager::manager` with `mpl::bool_<false>`
    // (heap-only): `case 0` clones via `operator new(0x10)` + memberwise +
    // `shared_count` copy (disasm 0x708b2e-0x708b3e); move/destroy/check/get
    // mirror the 1-arg manager arms. Box/None is the heap/empty buffer.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = Some(Box::new(src.clone()));
            true
        }
        FunctorOp::Destroy => {
            *dst = None;
            false
        }
        FunctorOp::CheckType => {
            *dst = Some(Box::new(src.clone()));
            true
        }
        FunctorOp::GetType => true,
    }
}
// 0x708c08 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_0x708c08(
    sig: &Signal<(SharedPtr<Instance>, SharedPtr<Instance>)>,
    func: &PairFunction,
) -> PairConnection {
    // IDA 0x708c08: `operator new(32)` callable slot (disasm 0x708c42),
    // `callable` ctor (IDA 0x708d24) retaining the `function2` arg, slot
    // insert into the signal, connection return. The closure retains the
    // same `function2` (clone) and `Signal::connect` inserts it; the
    // returned handle owns the strong ref (see `PairConnection`).
    let retained = func.clone();
    let cb = SharedPtr::new(move |pair: (SharedPtr<Instance>, SharedPtr<Instance>)| {
        if let Some(target) = &retained.target {
            stub_0x708378(target, &pair.0, &pair.1);
        }
    });
    // `Signal::connect<F: Fn(T) + Send + Sync>` needs a sized `F`: pass the
    // concrete closure Arc (unsized-coercion to `dyn` would break the bound).
    sig.connect(cb.clone());
    PairConnection { keep: cb }
}
// 0x708d00 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_0x708d00(
    dst: *mut Option<SharedPtr<PairSlotNode>>,
    src: &Option<SharedPtr<PairSlotNode>>,
) {
    // IDA 0x708d00: `add_ref(src)` (disasm 0x708d0a-0x708d0e), store over
    // `dst`, `release(old)` (disasm 0x708d12-0x708d1a) — the 2-arg twin of
    // 0x705978. Clone-then-assign is the same order and self-assign safe.
    // SAFETY: `dst` must be writable; `src` must be readable.
    unsafe {
        *dst = src.clone();
    }
}
// 0x708d24 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_0x708d24(slot: *mut PairSlotNode, func: &PairFunction) {
    // IDA 0x708d24: vtable stores + `a1[2] = 0` (empty next link) +
    // `a1[3] = a3` (signal link, compiler-managed here) + `a1[4] = 0`, then
    // `assign_to_own(a1 + 4, func)` retaining the callback (IDA 0x709914).
    // SAFETY: `slot` must point to valid uninitialized `PairSlotNode` storage.
    unsafe {
        core::ptr::write(slot, PairSlotNode { next: None, func: func.clone() });
    }
}
// 0x708e20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_0x708e20(slot: *mut PairSlotNode) {
    // IDA 0x708e20: D1 — vtable resets (compiler-managed),
    // `function2::clear(a1 + 4)` (disasm 0x708e8c), `release(a1[2])`
    // (disasm 0x708eac-0x708eb6); storage kept.
    // SAFETY: `slot` must point to a valid `PairSlotNode`.
    unsafe {
        (*slot).func.target = None;
        (*slot).next = None;
    }
}
// 0x708f30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_0x708f30(slot: *mut PairSlotNode) {
    // IDA 0x708f30: D0 — the D1 body (`clear` + `release`, disasm
    // 0x708f9c-0x708fc4) plus `operator delete(a1)` (disasm 0x708fd0); the
    // box reclaim runs the field drops and frees together.
    // SAFETY: `slot` must be a live box pointer that is never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}
// 0x709060 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_0x709060(_conn: PairConnection) {
    // IDA 0x709060: mutex-guarded unlink of the slot from the signal list
    // (lock dance from 0x709072 on); here the connection owns the closure's
    // last strong ref, so taking it by value expires the signal's weak slot
    // — the same unlink, and `Signal`'s own lock is the same guard.
}
// 0x709170 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_0x709170(slot: *const PairSlotNode) -> bool {
    // IDA 0x709170: `*(a1 + 12) != 0` — the intrusive link word; a slot
    // with no successor is unlinked.
    // SAFETY: `slot` must point to a valid `PairSlotNode`.
    unsafe { (*slot).next.is_some() }
}
// 0x70917c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x70917c(slot: &PairSlotNode, a: &SharedPtr<Instance>, b: &SharedPtr<Instance>) {
    // IDA 0x70917c: retains both `shared_ptr` args (`shared_count` copies
    // at `[bp-9Ch]`/`[bp-94h]`), invokes the slot's `function2`, releases
    // on scope exit; clone + call + drop is the same pair.
    if let Some(target) = &slot.func.target {
        stub_0x708378(target, &a.clone(), &b.clone());
    }
}
// 0x709294 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x709294(slot: &PairSlotNode, a: &SharedPtr<Instance>, b: &SharedPtr<Instance>) {
    // IDA 0x709294: non-virtual thunk into `callable::call` (IDA 0x70917c);
    // the `this - 4` adjustment collapses — same receiver, same args.
    stub_0x70917c(slot, a, b);
}
// 0x70929c — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EclES4_S4_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0x70929c(func: &PairFunction, a: &SharedPtr<Instance>, b: &SharedPtr<Instance>) {
    // IDA 0x70929c: `function2::operator()` retains both args
    // (`shared_count` copies at `[bp-9Ch]`/`[bp-94h]`), dispatches through
    // the vtable invoker (IDA 0x7087ec) with SjLj guards, releases on exit;
    // empty-function throw collapses: an empty target is a no-op here.
    if let Some(target) = &func.target {
        stub_0x7087ec(
            &BindWrapper2 { target: target.clone() },
            &a.clone(),
            &b.clone(),
        );
    }
}
// 0x7093f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0x7093f0(slot: *mut PairSlotNode) {
    // IDA 0x7093f0: `ReleaseAssert(!intrusive_ptr_expired(item))`
    // (signal.h:261, disasm 0x709404-0x70943a), `SignalPrints` log line
    // (disasm 0x709458-0x709466), then unlink of the item from the signal
    // list. The log collapses; the expired-assert becomes a linked-assert
    // and the unlink clears the successor.
    // SAFETY: `slot` must point to a valid `PairSlotNode`.
    unsafe {
        debug_assert!((*slot).next.is_some(), "0x7093f0: intrusive_ptr_expired");
        (*slot).next = None;
    }
}
// 0x7094e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_0x7094e0() -> &'static Mutex<()> {
    // IDA 0x7094e0: thunk (`attributes: thunk`) into `safe_static_do_get_mutex`
    // (IDA 0x7094e4) — no body of its own.
    stub_0x7094e4()
}
// 0x7094e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x7094e4() -> &'static Mutex<()> {
    // IDA 0x7094e4: guard-checked once-init (`__cxa_guard_acquire`, disasm
    // 0x7094e4-0x70955a), `mutex::mutex` over the function-local `value`
    // (disasm 0x70955a), `__cxa_atexit` destructor registration (disasm
    // 0x709578), `__cxa_guard_release`. A `static` with `const` init is the
    // same once-init; the pthread object lives inside `Mutex`.
    &PAIR_SLOT_STATIC_MUTEX
}
// 0x7095d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x7095d4(slot: *mut PairSlotNode) {
    // IDA 0x7095d4: `callable` D1 — vtable resets (compiler-managed),
    // `function2::clear(a1 + 4)` (disasm 0x709640), `release(a1[2])`
    // (disasm 0x709660-0x70966a); storage kept. Same body as 0x708e20.
    // SAFETY: `slot` must point to a valid `PairSlotNode`.
    unsafe {
        (*slot).func.target = None;
        (*slot).next = None;
    }
}
// 0x7096e4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x7096e4(slot: *mut PairSlotNode) {
    // IDA 0x7096e4: `callable` D0 — the D1 body (`clear` + `release`, disasm
    // 0x709750-0x709778) plus `operator delete(a1)` (disasm 0x709784); the
    // box reclaim runs the field drops and frees together.
    // SAFETY: `slot` must be a live box pointer that is never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}
// 0x709814 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x709814(slot: *mut PairSlotNode) {
    // IDA 0x709814: `slot` D1 — vtable reset (compiler-managed) +
    // `release(a1 + 8)` (disasm 0x709832-0x709838) only; the callback word
    // belongs to the `callable` subclass and is cleared by its own D1.
    // SAFETY: `slot` must point to a valid `PairSlotNode`.
    unsafe {
        (*slot).next = None;
    }
}
// 0x709840 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x709840(slot: *mut PairSlotNode) {
    // IDA 0x709840: `slot` D0 — vtable reset + `release(a1[2])` (disasm
    // 0x709888-0x7098ae) plus `operator delete(a1)` (disasm 0x7098ba); the
    // box reclaim runs the field drops and frees together.
    // SAFETY: `slot` must be a live box pointer that is never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}
// 0x709914 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E13assign_to_ownERKS5_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_0x709914(dst: &mut PairFunction, src: &PairFunction) {
    // IDA 0x709914: `function2::assign_to_own` — self-copy of an owning
    // functor: the `shared_count` copy of the stored bind retains the same
    // wrapper; cloning the target is that retain.
    dst.target = src.target.clone();
}
// 0x709944 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEEC2ESC_PKcSF_NS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x709944(
    this: *mut EventDescPayload,
    name: &str,
    permissions: u32,
    attributes: u32,
) {
    // IDA 0x709944: `EventDesc` C2 for the 1-arg member-ref signal
    // (`signal & (Instance::*)()`): `classDescriptor()` (collapses), base
    // `EventDescriptor::EventDescriptor`, then the single `SharedPtr<Instance>`
    // signature item (`Type` lookup + `_M_insert`, same shape as 0x70633c).
    // SAFETY: `this` must point to valid uninitialized `EventDescPayload` storage.
    unsafe {
        core::ptr::write(
            this,
            EventDescPayload {
                name: name.to_string(),
                permissions,
                attributes,
                items: vec![SignatureItem { type_name: "SharedPtr<Instance>" }],
                connections: Mutex::new(Vec::new()),
                single: Signal::new(),
                triple: Signal::new(),
                triple_isi: Signal::new(),
            },
        );
    }
}
// 0x709ad4 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()
pub fn stub_0x709ad4(this: *mut EventDescPayload) {
    // IDA 0x709ad4: D0 — D1 body (`*a1` vtable reset + `_M_clear`,
    // disasm 0x709b12-0x709b38) plus `operator delete` (disasm 0x709b3e);
    // the box reclaim is both.
    // SAFETY: `this` must be a live box pointer that is never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}
// 0x709b88 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x709b88(desc: *const EventDescPayload, slot: &SharedPtr<GenericSlotWrapper>) {
    // IDA 0x709b88: 1-arg `connectGeneric` — retains the wrapper
    // `shared_ptr` (`shared_count` copy) and inserts into the source's
    // member signal; collapses to the retained clone + push, twin of
    // 0x706574/0x707dcc.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().push(slot.clone());
    }
}
// 0x709cf4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x709cf4(desc: *const EventDescPayload, args: &[Variant]) {
    // IDA 0x709cf4: assert `args.size() == 1`, `any_cast` the retained
    // `shared_ptr<Instance>` arg (the `shared_count` copy at `[bp-94h]`),
    // then invoke the member `signal<void ()(SharedPtr<Instance>)>` — the
    // direct `single` fire plus each connected wrapper's 1-arg execute
    // (`on_single`; the `execute1` frame collapses to the handler call).
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    assert!(args.len() == 1, "0x709cf4: args.size() == 1");
    let arg = match &args[0] {
        Variant::Instance(inst) => inst.clone(),
        _ => panic!("0x709cf4: any_cast<shared_ptr<Instance>> failed"),
    };
    unsafe {
        (*desc).single.fire(arg.clone());
        let slots = (*desc).connections.lock().clone();
        for slot in slots.iter() {
            if let Some(cb) = slot.on_single {
                cb(&arg);
            }
        }
    }
}
// 0x709e50 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x709e50(desc: *const EventDescPayload) {
    // IDA 0x709e50: `if (!*(a2 + 40)) return` (disasm 0x709e52) — a source
    // without the member signal disconnects nothing; else the member pointer
    // at `*(a1 + 40)` is resolved virtual-aware (`(v2 & 0x100000000)`,
    // disasm 0x709e5c-0x709e72) and `signal::disconnectAll` runs on it
    // (disasm 0x709e78). The payload always carries its member signal, so
    // the guard collapses and both the direct signal and the wrapper list
    // disconnect.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).single.disconnect_all();
        (*desc).connections.lock().clear();
    }
}
// 0x709e7c — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9getSignalEPS2_
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const
pub fn stub_0x709e7c(desc: *const EventDescPayload, source: *const Instance) -> *const Signal<SharedPtr<Instance>> {
    // IDA 0x709e7c: `ReleaseAssert(sourceActive(obj))` (`*(a2 + 76)`,
    // Event.h:272, disasm 0x709e90-0x709ec4 — the active flag has no model
    // yet, so the non-null source is the remaining precondition), then the
    // same virtual-aware `+40` member resolution as 0x709e50, returning the
    // member `signal<void ()(SharedPtr<Instance>)>` — the payload's `single`.
    // SAFETY: `desc` must point to a valid `EventDescPayload` outliving the
    // result; `source` must be non-null.
    unsafe {
        debug_assert!(!source.is_null(), "0x709e7c: sourceActive(obj)");
        &(*desc).single as *const Signal<SharedPtr<Instance>>
    }
}
// 0x709ef0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_0x709ef0(slot: *mut SingleSlotNode, func: &SingleFunction) {
    // IDA 0x709ef0: 1-arg `callable` ctor — `a1[2] = 0` (empty next link),
    // `a1[3] = a3` (signal link, compiler-managed here), `a1[4] = 0`, then
    // `function1::assign_to_own` retaining the callback (disasm 0x709f70);
    // twin of 0x708d24.
    // SAFETY: `slot` must point to valid uninitialized `SingleSlotNode` storage.
    unsafe {
        core::ptr::write(slot, SingleSlotNode { next: None, func: func.clone() });
    }
}
// 0x709ff0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0x709ff0(slot: *mut SingleSlotNode) {
    // IDA 0x709ff0: 1-arg `signal::remove` — same shape as 0x7093f0:
    // `ReleaseAssert(!intrusive_ptr_expired(item))` (signal.h:261), log
    // line, then unlink; twin on `SingleSlotNode`.
    // SAFETY: `slot` must point to a valid `SingleSlotNode`.
    unsafe {
        debug_assert!((*slot).next.is_some(), "0x709ff0: intrusive_ptr_expired");
        (*slot).next = None;
    }
}