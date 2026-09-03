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
    /// Lazily allocated block from `RBX::Instance::onDemandWrite` (IDA `0x7010ac`).
    pub write: Option<Box<InstanceWrite>>,
    /// Embedded `boost::enable_shared_from_this` weak owner at `this + 40`
    /// (IDA `0x7039e4` reads px at `+40`, pi at `+44`).
    pub weak_owner: WeakPtr<Instance>,
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

// 0x703444 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED1Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()
pub fn stub_0x703444() -> ! {
    todo!("0x703444 RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")
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
pub fn stub_0x703490() -> ! {
    todo!("0x703490 RBX::Instance::setParent(RBX::Instance*)")
}
// 0x703498 — __ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_0x703498() -> ! {
    todo!("0x703498 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::~RefPropDescriptor()")
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
pub fn stub_0x7034d8() -> ! {
    todo!("0x7034d8 RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")
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
pub fn stub_0x703520() -> ! {
    todo!("0x703520 RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")
}
// 0x703544 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")]
pub fn stub_0x703544() -> ! {
    todo!("0x703544 RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")
}
// 0x703568 — __ZN3RBX22AbstractFactoryProductINS_8InstanceEE6createERKNS_4NameENS_11CreatorRoleE
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::create(RBX::Name const&,RBX::CreatorRole)")]
pub fn stub_0x703568() -> ! {
    todo!("0x703568 RBX::AbstractFactoryProduct<RBX::Instance>::create(RBX::Name const&,RBX::CreatorRole)")
}
// 0x703748 — __ZN10Serializer13canWriteChildEN5boost10shared_ptrIN3RBX8InstanceEEENS3_10SaveFilterE
#[doc(alias = "Serializer::canWriteChild(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance::SaveFilter)")]
// was: Serializer::canWriteChild(boost::shared_ptr<RBX::Instance>,RBX::Instance::SaveFilter)
pub fn stub_0x703748() -> ! {
    todo!("0x703748 Serializer::canWriteChild(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance::SaveFilter)")
}
// 0x7039cc — __ZN3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(RBX::Instance const*)")]
pub fn stub_0x7039cc() -> ! {
    todo!("0x7039cc RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(RBX::Instance const*)")
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
pub fn stub_0x70430c() -> ! {
    todo!("0x70430c void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>> const&)const")
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
pub fn stub_0x704748() -> ! {
    todo!("0x704748 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>)")
}
// 0x704794 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKPN3RBX10Reflection15EventDescriptorESt6vectorIS5_SaIS5_EEEEN5boost3_bi6bind_tIvNSC_4_mfi4cmf1IvS4_PNS3_11EventSourceEEENSD_5list2INSC_3argILi1EEENSD_5valueIPNS2_8InstanceEEEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)
pub fn stub_0x704794() -> ! {
    todo!("0x704794 boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)")
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
pub fn stub_0x70633c() -> ! {
    todo!("0x70633c RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0x7064c0 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")]
pub fn stub_0x7064c0() -> ! {
    todo!("0x7064c0 RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")
}
// 0x706574 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x706574() -> ! {
    todo!("0x706574 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}
// 0x7066c8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x7066c8() -> ! {
    todo!("0x7066c8 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}
// 0x706754 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x706754() -> ! {
    todo!("0x706754 RBX::Reflection::EventDescBase<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}
// 0x707b28 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x707b28() -> ! {
    todo!("0x707b28 RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0x707d18 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()
pub fn stub_0x707d18() -> ! {
    todo!("0x707d18 RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")
}
// 0x707dcc — __ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x707dcc() -> ! {
    todo!("0x707dcc RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}
// 0x707f20 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x707f20() -> ! {
    todo!("0x707f20 RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}
// 0x7080d0 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x7080d0() -> ! {
    todo!("0x7080d0 RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}
// 0x7080e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_0x7080e4() -> ! {
    todo!("0x7080e4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")
}
// 0x70825c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEES8_NS4_IS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_0x70825c() -> ! {
    todo!("0x70825c boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}
// 0x708378 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEES6_EEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x708378() -> ! {
    todo!("0x708378 void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")
}
// 0x7084e0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E5clearEv
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::clear(void)
pub fn stub_0x7084e0() -> ! {
    todo!("0x7084e0 boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")
}
// 0x7086d8 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0x7086d8() -> ! {
    todo!("0x7086d8 void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}
// 0x7087d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x7087d0() -> ! {
    todo!("0x7087d0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}
// 0x7087ec — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSC_SC_E6invokeERNS1_15function_bufferESC_SC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x7087ec() -> ! {
    todo!("0x7087ec boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}
// 0x7087f4 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x7087f4() -> ! {
    todo!("0x7087f4 bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}
// 0x7088dc — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x7088dc() -> ! {
    todo!("0x7088dc bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
// 0x7089c0 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x7089c0() -> ! {
    todo!("0x7089c0 void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}
// 0x708a94 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &,rbx_core::SharedPtr<RBX::Instance> &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> &,boost::shared_ptr<RBX::Instance> &)
pub fn stub_0x708a94() -> ! {
    todo!("0x708a94 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &,rbx_core::SharedPtr<RBX::Instance> &)")
}
// 0x708ab0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x708ab0() -> ! {
    todo!("0x708ab0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}
// 0x708c08 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_0x708c08() -> ! {
    todo!("0x708c08 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)")
}
// 0x708d00 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_0x708d00() -> ! {
    todo!("0x708d00 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")
}
// 0x708d24 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_0x708d24() -> ! {
    todo!("0x708d24 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*)")
}
// 0x708e20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_0x708e20() -> ! {
    todo!("0x708e20 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")
}
// 0x708f30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_0x708f30() -> ! {
    todo!("0x708f30 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")
}
// 0x709060 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_0x709060() -> ! {
    todo!("0x709060 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")
}
// 0x709170 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_0x709170() -> ! {
    todo!("0x709170 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")
}
// 0x70917c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x70917c() -> ! {
    todo!("0x70917c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}
// 0x709294 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x709294() -> ! {
    todo!("0x709294 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}
// 0x70929c — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EclES4_S4_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0x70929c() -> ! {
    todo!("0x70929c boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")
}
// 0x7093f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0x7093f0() -> ! {
    todo!("0x7093f0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}
// 0x7094e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_0x7094e0() -> ! {
    todo!("0x7094e0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")
}
// 0x7094e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x7094e4() -> ! {
    todo!("0x7094e4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")
}
// 0x7095d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x7095d4() -> ! {
    todo!("0x7095d4 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}
// 0x7096e4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x7096e4() -> ! {
    todo!("0x7096e4 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}
// 0x709814 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x709814() -> ! {
    todo!("0x709814 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")
}
// 0x709840 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x709840() -> ! {
    todo!("0x709840 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")
}
// 0x709914 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E13assign_to_ownERKS5_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_0x709914() -> ! {
    todo!("0x709914 boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>> const&)")
}
// 0x709944 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEEC2ESC_PKcSF_NS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x709944() -> ! {
    todo!("0x709944 RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")
}
// 0x709ad4 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()
pub fn stub_0x709ad4() -> ! {
    todo!("0x709ad4 RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")
}
// 0x709b88 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x709b88() -> ! {
    todo!("0x709b88 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}
// 0x709cf4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x709cf4() -> ! {
    todo!("0x709cf4 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}
// 0x709e50 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x709e50() -> ! {
    todo!("0x709e50 RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")
}
// 0x709e7c — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9getSignalEPS2_
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const
pub fn stub_0x709e7c() -> ! {
    todo!("0x709e7c RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const")
}
// 0x709ef0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_0x709ef0() -> ! {
    todo!("0x709ef0 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")
}
// 0x709ff0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0x709ff0() -> ! {
    todo!("0x709ff0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}