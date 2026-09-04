// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x70086c..0xa2fd44 | total filtered 10215, remaining 7755 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Batch 2 (0x70086c..0x7029f4): 24 ports grounded on IDA decompile+disasm.
// 0x700acc/0x700bf8 (ancestor-chain signallers), 0x701470 (luaClone),
// 0x701600/0x701a24 (ctors) remain stubs: bodies unrecovered.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_05::{
    ATTR_CLASS, TAG_ITEM, CombinedEvent, CreatorRole, Instance, InstanceHooks, InstanceName,
    InstanceWrite, PropertyDescriptor, XmlAttr, XmlElement, borrow_shared, stub_0x704228,
};
use crate::generated_86::stub_6ffc98;

/// Rust model of `RBX::AncestorChanged` (IDA `0x700d28`): the decompile
/// retains `*a2` as the child (`shared_from`, `0x700dd6`) and `a2[2]` as the
/// other end (`0x700de2`).
/// // BUG: member roles beyond "child + scope pair" are inferred; the
/// // original layout is wider than these two words.
pub struct AncestorChanged {
    pub child: Option<SharedPtr<Instance>>,
    pub scope: Option<SharedPtr<Instance>>,
}
/// Combined-signal kind for property changes (IDA `0x7029cc` passes `2`).
pub const COMBINED_PROPERTY_CHANGED: u32 = 2;
/// Combined-signal kind for ancestry changes (IDA `0x700e18` passes `5`).
pub const COMBINED_ANCESTRY_CHANGED: u32 = 5;
/// `RBX::Instance::propArchivable` descriptor identity (IDA `0x702a20`).
pub static PROP_ARCHIVABLE: PropertyDescriptor = PropertyDescriptor { name: "Archivable" };
/// `RBX::Instance::desc_Name` descriptor identity (IDA `0x70286a`).
pub static PROP_NAME: PropertyDescriptor = PropertyDescriptor { name: "Name" };
/// `FFlag::NoThrowOnReparenting` (IDA `0x702614` reads it in
/// `setAndLockParent`): default `false`, so failures throw.
/// Mirrors the private `NO_THROW_ON_REPARENTING` in `generated_86`.
const NO_THROW_ON_REPARENTING: bool = false;
/// `FFlag::FastClone` (IDA `0x70129e` in `clone`): unmodeled here, so `clone`
/// always takes the deep-copy (`Cloner`-equivalent) path.
/// // BUG: when the flag is set the original runs `RBX::Cloner`, whose
/// // referent/property fixups are not reproduced.
const FAST_CLONE: bool = false;
/// Name-length gate in `setName` (IDA `0x7027de` compares the rep length
/// against `0x65`, truncating via `substr` above it).
/// // BUG: the exact truncation bound is unconfirmed from the recovered
/// // body; over-long names are cut to 100 chars here.
const MAX_NAME_LEN: usize = 100;

// 0x70086c — __ZNK3RBX8Instance11getFullNameEv
#[doc(alias = "RBX::Instance::getFullName(void)const")]
pub fn stub_0x70086c(this: *const Instance) -> String {
    // IDA 0x70086c: when the instance is rooted under a `ServiceProvider`
    // (class-descriptor `isA` gate, 0x7008e0-0x700908), the result is
    // `parent.getFullName() + "." + name` (0x70090a-0x700940); otherwise it
    // is just `name` (0x7008f4, `*(a2 + 68) + 24` is the name string).
    // The `ServiceProvider` class-descriptor check collapses into
    // parent-presence: a detached instance has no dotted prefix.
    // // BUG: recursion depth equals ancestry depth; the original builds the
    // // string iteratively.
    // SAFETY: `this` must point to a valid `Instance` whose ancestry
    // outlives the call.
    unsafe {
        let parent = (*this).parent;
        if parent.is_null() {
            (*this).name.text.clone()
        } else {
            format!("{}.{}", stub_0x70086c(parent), (*this).name.text)
        }
    }
}

// 0x700ab8 — __ZNK3RBX8Instance8containsEPKS0_
#[doc(alias = "RBX::Instance::contains(RBX::Instance const*)const")]
pub fn stub_0x700ab8(this: *const Instance, other: *const Instance) -> bool {
    // IDA 0x700ab8, disasm `LDR R1,[R1,#0x34]` (0x700aba): walk `other`'s
    // parent chain (`+13` words); true iff `this` is met, false on null.
    // SAFETY: both must be null or point into valid `Instance` trees.
    unsafe {
        let mut cursor = other;
        while !cursor.is_null() {
            if cursor == this {
                return true;
            }
            cursor = (*cursor).parent;
        }
        false
    }
}

// 0x700acc — __ZN3RBX8Instance24signalDescendantRemovingERKN5boost10shared_ptrIS0_EEPS0_S6_
#[doc(alias = "RBX::Instance::signalDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&,RBX::Instance*,RBX::Instance*)")]
// was: RBX::Instance::signalDescendantRemoving(boost::shared_ptr<RBX::Instance> const&,RBX::Instance*,RBX::Instance*)
pub fn stub_700acc() -> ! {
    todo!("0x700acc RBX::Instance::signalDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&,RBX::Instance*,RBX::Instance*)")
}

// 0x700bf8 — __ZN3RBX8Instance21signalDescendantAddedEPS0_S1_S1_
#[doc(alias = "RBX::Instance::signalDescendantAdded(RBX::Instance*,RBX::Instance*,RBX::Instance*)")]
pub fn stub_700bf8() -> ! {
    todo!("0x700bf8 RBX::Instance::signalDescendantAdded(RBX::Instance*,RBX::Instance*,RBX::Instance*)")
}

// 0x700d28 — __ZN3RBX8Instance17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::Instance::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x700d28(this: *mut Instance, ev: *const AncestorChanged) {
    // IDA 0x700d28: observer vector at `+56` (word `+14`, vtable slot `+88`
    // per entry, 0x700d8c-0x700dae) is unmodeled (see BUG below). When the
    // combined signal at `+80` (word `+20`) is live, retain `*a2` (child,
    // 0x700dd6) and `a2[2]` (scope, 0x700de2) and fire kind `5` (0x700e18);
    // when the 2-arg signal at `+84` (word `+21`) is live, fire it with the
    // same pair (0x700e74). Empty `Signal`s are no-ops, so liveness checks
    // collapse into `fire`; retains are clones.
    // // BUG: the `+56` observer-vector walk has no model and is skipped.
    // SAFETY: `this` must point to a valid `Instance`; `ev` must be null or
    // point to a valid `AncestorChanged` whose members stay alive.
    unsafe {
        if ev.is_null() {
            return;
        }
        let (Some(child), Some(scope)) = ((*ev).child.clone(), (*ev).scope.clone()) else {
            return;
        };
        (*this).combined.slots.fire(CombinedEvent {
            kind: COMBINED_ANCESTRY_CHANGED,
            child: child.clone(),
        });
        (*this).ancestry_changed.fire((child, scope));
    }
}

// 0x700fcc — __ZN3RBX8Instance17onDescendantAddedEPS0_
#[doc(alias = "RBX::Instance::onDescendantAdded(RBX::Instance*)")]
pub fn stub_0x700fcc(this: *mut Instance, child: *const Instance) {
    // IDA 0x700fcc: if the on-demand block exists (`*(this + 19)`, 0x700ff8),
    // fire `onDemandWrite(this) + 12` (descendant-added) with
    // `shared_from(child)` (0x701030-0x70103e); the retain/release pair is a
    // clone/drop via `borrow_shared`.
    // SAFETY: `this` must point to a valid `Instance`; `child` must be null
    // or point to a shared `Instance` that outlives the fire.
    unsafe {
        if (*this).write.is_some() && !child.is_null() {
            let block = stub_0x7010ac(this);
            (*block).descendant_added.fire(borrow_shared(child));
        }
    }
}

// 0x7010a8 — __ZNK3RBX8Instance12onDemandReadEv
#[doc(alias = "RBX::Instance::onDemandRead(void)const")]
pub fn stub_0x7010a8(this: *const Instance) -> *const InstanceWrite {
    // IDA 0x7010a8, disasm `LDR R0,[R0,#0x4C]; BX LR`: return the on-demand
    // block word (`+19`); null when never allocated.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe {
        match (*this).write.as_deref() {
            Some(block) => block as *const InstanceWrite,
            None => core::ptr::null(),
        }
    }
}

// 0x7010ac — __ZN3RBX8Instance13onDemandWriteEv
#[doc(alias = "RBX::Instance::onDemandWrite(void)")]
pub fn stub_0x7010ac(this: *mut Instance) -> *mut InstanceWrite {
    // IDA 0x7010ac: return `*(this + 19)` (0x7010b2); when null, build it
    // through vtable slot `+12` (0x7010b8-0x7010be), store it (0x7010c2),
    // delete the loser on a race (0x7010c4-0x7010ce), then `ReleaseAssert`
    // non-null via `FLog::Asserts` (0x7010dc). The vtable factory
    // collapses into default allocation (which cannot fail, so the assert
    // path is unreachable); single-threaded use collapses the race delete.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe {
        let inst = &mut *this;
        if inst.write.is_none() {
            inst.write = Some(Box::default());
        }
        inst.write.as_deref_mut().unwrap() as *mut InstanceWrite
    }
}

// 0x70112c — __ZN3RBX8Instance20onDescendantRemovingERKN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Instance::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x70112c(this: *mut Instance, child: &SharedPtr<Instance>) {
    // IDA 0x70112c, disasm `B.W descendantRemovingSignal$shim`: thunk to
    // `descendantRemovingSignal` — if `*(this + 19)` is set, fire
    // `onDemandWrite(this) + 16` with a clone (see `stub_0x704228`).
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        stub_0x704228(this, child);
    }
}

// 0x701130 — __ZN3RBX8Instance12toNewXmlRootEPS0_NS_11CreatorRoleE
#[doc(alias = "RBX::Instance::toNewXmlRoot(RBX::Instance*,RBX::CreatorRole)")]
pub fn stub_0x701130(this: *const Instance, _role: CreatorRole) -> XmlElement {
    // IDA 0x701130: build a `bool(Instance*)` filter binding `isInScope`
    // (0x701180-0x701186) and call the `SerializerV2` virtual at slot `+48`
    // (0x70116c-0x7011b0), returning the written root element.
    // `SerializerV2` is unmodeled here, so the virtual dispatch collapses
    // into the root `Item` element; `isInScope` (0x701228) stays the
    // documented filter predicate for the future serializer port.
    // // BUG: subtree serialization is missing — children do not become
    // // nested `Item` elements yet.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        XmlElement {
            tag: TAG_ITEM.to_string(),
            attrs: vec![
                XmlAttr { name: ATTR_CLASS.to_string(), value: (*this).class_name.to_string() },
                XmlAttr { name: PROP_NAME.name.to_string(), value: (*this).name.text.clone() },
            ],
            children: Vec::new(),
        }
    }
}

// 0x701228 — __ZN3RBXL9isInScopeEPNS_8InstanceES1_
#[doc(alias = "RBX::isInScope(RBX::Instance *,RBX::Instance *)")]
pub fn stub_0x701228(scope: *const Instance, node: *const Instance) -> bool {
    // IDA 0x701228, disasm: `R2 = 1`; if `scope == node` done (0x70122a);
    // else walk `node`'s parents (`[R1,#0x34]`, 0x701234) for `scope`,
    // hitting null fails (0x70122e-0x70123a).
    // SAFETY: both must be null or point into valid `Instance` trees.
    unsafe {
        if scope == node {
            return true;
        }
        let mut cursor = node;
        while !cursor.is_null() {
            cursor = (*cursor).parent;
            if cursor == scope {
                return true;
            }
        }
        false
    }
}

// 0x701240 — __ZN3RBX8Instance5cloneENS_11CreatorRoleE
#[doc(alias = "RBX::Instance::clone(RBX::CreatorRole)")]
pub fn stub_0x701240(this: *const Instance, _role: CreatorRole) -> Option<SharedPtr<Instance>> {
    // IDA 0x701240: under `FFlag::FastClone` (0x70129e) run `RBX::Cloner`
    // (0x7012aa); otherwise `toNewXmlRoot` + `loadInstancesFromMemory`
    // (0x7012f2-0x70130c) and return the first loaded instance, or null
    // when the loaded vector is empty (0x701318-0x701338). `FAST_CLONE` is
    // false here, and the XML round trip collapses into a direct deep copy:
    // name, class, flags, hooks plus recursively cloned children; the copy
    // is detached (null parent), like a freshly deserialized root.
    // // BUG: `Cloner` referent fixups and per-property serialization are
    // // not reproduced; property values beyond name/class/flags are lost.
    // SAFETY: `this` must point to a valid `Instance` subtree outliving
    // the call.
    unsafe {
        if this.is_null() || FAST_CLONE {
            return None;
        }
        let src = &*this;
        let mut children = Vec::with_capacity(src.children.len());
        for child in src.children.iter() {
            if let Some(copy) = stub_0x701240(SharedPtr::as_ptr(child), _role) {
                children.push(copy);
            }
        }
        // `InstanceHooks` is not `Clone`, but every slot is a `Copy` fn
        // pointer, so the override table copies memberwise (a `Cloner`
        // keeps the class behavior; signals and the on-demand block start
        // empty, like freshly deserialized instances).
        let h = &src.hooks;
        let hooks = InstanceHooks {
            changing: h.changing,
            ancestry_changing: h.ancestry_changing,
            child_added: h.child_added,
            descendant_added: h.descendant_added,
            added: h.added,
            removing: h.removing,
            child_removed: h.child_removed,
            ancestry_changed: h.ancestry_changed,
            property_changed: h.property_changed,
            on_property_changed: h.on_property_changed,
            on_child_changed: h.on_child_changed,
            data_cost: h.data_cost,
            read_node: h.read_node,
            primitive_filter: h.primitive_filter,
        };
        Some(SharedPtr::new(Instance {
            parent: core::ptr::null(),
            name: InstanceName { text: src.name.text.clone() },
            roblox_locked: src.roblox_locked,
            parent_locked: src.parent_locked,
            class_name: src.class_name,
            children,
            in_set_parent: false,
            combined: Default::default(),
            hooks,
            write: None,
            weak_owner: std::sync::Weak::new(),
            archivable: src.archivable,
            fw_cookie: src.fw_cookie,
            ancestry_changed: Default::default(),
            property_changed: Default::default(),
            notify_child_changed: src.notify_child_changed,
        }))
    }
}

// 0x701468 — __ZN3RBX14countInstancesEN5boost10shared_ptrINS_8InstanceEEEPi
#[doc(alias = "RBX::countInstances(rbx_core::SharedPtr<RBX::Instance>,int *)")]
// was: RBX::countInstances(boost::shared_ptr<RBX::Instance>,int *)
pub fn stub_0x701468(_inst: &SharedPtr<Instance>, counter: *mut i32) -> i32 {
    // IDA 0x701468, disasm `LDR R0,[R1]; ADDS R0,#1; STR R0,[R1]; BX LR`:
    // pre-increment `*counter` (the `visitDescendants` visitor) and return it.
    // SAFETY: `counter` must point to a writable `i32` outliving the call.
    unsafe {
        *counter += 1;
        *counter
    }
}

// 0x701470 — __ZN3RBX8Instance8luaCloneEv
#[doc(alias = "RBX::Instance::luaClone(void)")]
pub fn stub_701470() -> ! {
    todo!("0x701470 RBX::Instance::luaClone(void)")
}

// 0x701600 — __ZN3RBX8InstanceC2EPNS_10FWInstanceE
#[doc(alias = "RBX::Instance::Instance(RBX::FWInstance *)")]
pub fn stub_701600() -> ! {
    todo!("0x701600 RBX::Instance::Instance(RBX::FWInstance *)")
}

// 0x701a24 — __ZN3RBX8InstanceC2EPKcPNS_10FWInstanceE
#[doc(alias = "RBX::Instance::Instance(char const*,RBX::FWInstance *)")]
pub fn stub_701a24() -> ! {
    todo!("0x701a24 RBX::Instance::Instance(char const*,RBX::FWInstance *)")
}

// 0x701ef4 — __ZNK3RBX8Instance2fwEv
#[doc(alias = "RBX::Instance::fw(void)const")]
pub fn stub_0x701ef4(this: *const Instance) -> *const () {
    // IDA 0x701ef4, disasm `LDR R0,[R0,#0x44]; BX LR`: return the
    // name-store holder word (`*(this + 17)`), whose `+21`/`+23`/`+24`
    // members (locks, name) are modelled inline; the cookie's address is
    // its stable identity. See `Instance::fw_cookie`.
    // SAFETY: `this` must point to a valid `Instance` outliving the result.
    unsafe { &(*this).fw_cookie as *const u32 as *const () }
}

// 0x701ef8 — __ZN3RBX8InstanceD0Ev
#[doc(alias = "RBX::Instance::~Instance()")]
pub fn stub_0x701ef8(this: *mut Instance) {
    // IDA 0x701ef8: `~Instance()` (0x701f48, i.e. D2) then `operator
    // delete(this)` (0x701f4e). Dropping the box runs the field drops and
    // frees the storage.
    // SAFETY: `this` must come from a live `Box<Instance>` and never be
    // used again.
    unsafe {
        stub_0x701fac(this);
        drop(Box::from_raw(this));
    }
}

// 0x701f98 — __ZN3RBX8InstanceD1Ev
#[doc(alias = "RBX::Instance::~Instance()")]
pub fn stub_0x701f98(this: *mut Instance) {
    // IDA 0x701f98, disasm `B.W __ZN3RBX8InstanceD2Ev`: D1 tail-jumps to D2
    // (the `operator delete` lives only in D0).
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        stub_0x701fac(this);
    }
}

// 0x701f9c — __ZThn32_N3RBX8InstanceD0Ev
#[doc(alias = "non-virtual thunk to RBX::Instance::~Instance()")]
// was: non-virtual thunk to RBX::Instance::~Instance()
pub fn stub_0x701f9c(this: *mut Instance) {
    // IDA 0x701f9c, disasm `SUBS R0,#0x20; B.W D0`: adjust `this - 32`
    // (secondary-base to `Instance` base) then deleting destructor.
    // SAFETY: `this` must point 32 bytes into a live `Box<Instance>`.
    unsafe {
        stub_0x701ef8((this as *mut u8).offset(-32) as *mut Instance);
    }
}

// 0x701fa4 — __ZThn36_N3RBX8InstanceD0Ev
#[doc(alias = "non-virtual thunk to RBX::Instance::~Instance()")]
// was: non-virtual thunk to RBX::Instance::~Instance()
pub fn stub_0x701fa4(this: *mut Instance) {
    // IDA 0x701fa4, disasm `SUBS R0,#0x24; B.W D0`: adjust `this - 36`
    // then deleting destructor.
    // SAFETY: `this` must point 36 bytes into a live `Box<Instance>`.
    unsafe {
        stub_0x701ef8((this as *mut u8).offset(-36) as *mut Instance);
    }
}

// 0x701fac — __ZN3RBX8InstanceD2Ev
#[doc(alias = "RBX::Instance::~Instance()")]
pub fn stub_0x701fac(this: *mut Instance) {
    // IDA 0x701fac (D2): vtable resets (0x701fda-0x70200a,
    // compiler-managed) and `ReleaseAssert(parent == NULL)`
    // (Instance.cpp:846, 0x702014-0x702066); `FWLifetime` log (0x70208e);
    // `disconnectAll` on the property (`+88`), 2-arg ancestry (`+84`) and
    // combined (`+80`) signals (0x7020aa-0x7020fa); delete the on-demand
    // block (`+19`, 0x702100); release the shared counts (`+18`, `+15`,
    // 0x702110-0x702126) and the weak owner (`+11`, 0x70212c); `GuidItem`,
    // `Diagnostics::Countable` and `Limits::Countable` teardown
    // (0x702140-0x702162). Member drops (children vector, signals) and the
    // `Arc`/`Weak` releases collapse into field drops at box free; the
    // guid/diagnostics counters are unmodeled.
    // // BUG: `GuidItem` registry unlink and the diagnostics/countable
    // // counters have no model.
    // SAFETY: `this` must point to a valid `Instance` with no parent.
    unsafe {
        debug_assert!(
            (*this).parent.is_null(),
            "0x701fac: parent == NULL (Instance.cpp:846)"
        );
        (*this).property_changed.disconnect_all();
        (*this).ancestry_changed.disconnect_all();
        (*this).combined.slots.disconnect_all();
        drop((*this).write.take());
        (*this).children.clear();
    }
}

// 0x7023a8 — __ZThn32_N3RBX8InstanceD1Ev
#[doc(alias = "non-virtual thunk to RBX::Instance::~Instance()")]
// was: non-virtual thunk to RBX::Instance::~Instance()
pub fn stub_0x7023a8(this: *mut Instance) {
    // IDA 0x7023a8, disasm `SUBS R0,#0x20; B.W D2`: adjust `this - 32`
    // then complete destructor.
    // SAFETY: `this` must point 32 bytes into a valid `Instance`.
    unsafe {
        stub_0x701fac((this as *mut u8).offset(-32) as *mut Instance);
    }
}

// 0x7023b0 — __ZThn36_N3RBX8InstanceD1Ev
#[doc(alias = "non-virtual thunk to RBX::Instance::~Instance()")]
// was: non-virtual thunk to RBX::Instance::~Instance()
pub fn stub_0x7023b0(this: *mut Instance) {
    // IDA 0x7023b0, disasm `SUBS R0,#0x24; B.W D2`: adjust `this - 36`
    // then complete destructor.
    // SAFETY: `this` must point 36 bytes into a valid `Instance`.
    unsafe {
        stub_0x701fac((this as *mut u8).offset(-36) as *mut Instance);
    }
}

// 0x7023b8 — __ZN3RBX8Instance7destroyEv
#[doc(alias = "RBX::Instance::destroy(void)")]
pub fn stub_0x7023b8(this: *mut Instance) {
    // IDA 0x7023b8: when the `Parent` property is locked (holder `+ 21`,
    // 0x7023e8) while parented (`+13`, 0x70240a), throw `runtime_error`
    // ("The Parent property of %s is locked", 0x702414-0x7025ae) —
    // a panic here. Otherwise retain self via `shared_from` (0x702478,
    // collapses into caller-held ownership), `setAndLockParent(this, 0)`
    // (0x702492), `destroy()` over every child (bound `mf0` for_each,
    // 0x70249a-0x7024aa), and an `EventDescriptor` event-source sweep
    // (0x7024b8-0x7024c4, unmodeled, see BUG).
    // // BUG: the `EventDescriptor`/`EventSource` sweep has no model.
    // SAFETY: `this` must point to a valid, parent-locked-consistent
    // `Instance` whose subtree outlives the call with caller-held
    // ownership.
    unsafe {
        if (*this).parent_locked && !(*this).parent.is_null() {
            panic!(
                "The Parent property of {} is locked",
                stub_0x70086c(this)
            );
        }
        stub_0x7025bc(this, core::ptr::null());
        for child in (*this).children.clone() {
            stub_0x7023b8(SharedPtr::as_ptr(&child) as *mut Instance);
        }
    }
}

// 0x7025bc — __ZN3RBX8Instance16setAndLockParentEPS0_
#[doc(alias = "RBX::Instance::setAndLockParent(RBX::Instance*)")]
pub fn stub_0x7025bc(this: *mut Instance, new_parent: *const Instance) {
    // IDA 0x7025bc: set the holder `+ 21` lock via `FWValue<bool>::set`
    // (0x70261a-0x702638, inlined into `parent_locked`), then
    // `setParentInternal(this, a2, 1)` (0x702644). Under
    // `FFlag::NoThrowOnReparenting` (0x702614) a `false` return resets the
    // lock (0x702654-0x70266c); otherwise failures throw out of
    // `setParentInternal` itself.
    // SAFETY: `this` must point to a valid `Instance`; both trees must
    // outlive the call; `new_parent` must be null or valid.
    unsafe {
        (*this).parent_locked = true;
        let ok = stub_6ffc98(this, new_parent, true);
        if NO_THROW_ON_REPARENTING && !ok {
            (*this).parent_locked = false;
        }
    }
}

// 0x702778 — __ZN3RBX8Instance7setNameERKSs
#[doc(alias = "RBX::Instance::setName(std::string const&)")]
pub fn stub_0x702778(this: *mut Instance, new_name: &str) {
    // IDA 0x702778: compare against the holder `+ 24` string (0x70279a);
    // when different, `FWValue<string>::set` it (0x702854, truncating long
    // inputs via `substr`, 0x7027de-0x7027f0) and
    // `raisePropertyChanged(desc_Name)` (0x70286a).
    // SAFETY: `this` must point to a valid `Instance` outliving the call.
    unsafe {
        if (*this).name.text != new_name {
            let mut text = new_name.to_string();
            if text.len() > MAX_NAME_LEN {
                text.truncate(MAX_NAME_LEN);
            }
            (*this).name.text = text;
            stub_0x7028ec(this, core::ptr::addr_of!(PROP_NAME));
        }
    }
}

// 0x7028ec — __ZN3RBX8Instance20raisePropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Instance::raisePropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x7028ec(this: *mut Instance, desc: *const PropertyDescriptor) -> i32 {
    // IDA 0x7028ec: `ReleaseAssert(isMemberOf(desc, this + 36))` under
    // `FLog::Asserts` (0x702900-0x702978, unmodeled — see BUG); the vtable
    // `+116` override (0x7029b0, `hooks.on_property_changed`); the combined
    // emit at `+80` with kind `2` (0x7029c4-0x7029cc); the descriptor emit
    // on the `+88` signal (0x7029d6); then the parent's vtable `+112`
    // `onChildChanged(child, &PropertyChanged{desc})` (0x7029da-0x7029e8),
    // whose value is returned (`0` with no parent).
    // // BUG: the `isMemberOf` assert and the `PropertyChanged` event
    // // wrapper have no model; the kind-`2` combined payload carries the
    // // instance (retained) where the original carries the descriptor, and
    // // `on_property_changed` does not receive `desc` (arity of the
    // // existing hook).
    // SAFETY: `this` must point to a valid `Instance`; `desc` must point
    // to a valid `PropertyDescriptor` outliving the fires.
    unsafe {
        if let Some(hook) = (*this).hooks.on_property_changed {
            hook(this);
        }
        (*this).combined.slots.fire(CombinedEvent {
            kind: COMBINED_PROPERTY_CHANGED,
            child: borrow_shared(this),
        });
        (*this).property_changed.fire(desc);
        let parent = (*this).parent as *mut Instance;
        if parent.is_null() {
            0
        } else if let Some(notify) = (*parent).notify_child_changed {
            notify(parent, this, desc)
        } else {
            0
        }
    }
}

// 0x7029f4 — __ZN3RBX8Instance15setIsArchivableEb
#[doc(alias = "RBX::Instance::setIsArchivable(bool)")]
pub fn stub_0x7029f4(this: *mut Instance, archivable: bool) -> i32 {
    // IDA 0x7029f4: when the holder `+ 23` byte differs (`LDRB [R0,#0x17]`,
    // 0x702a02-0x702a06), `FWValue<bool>::set(holder + 23, val, this + 68)`
    // (0x702a08-0x702a10, inlined into `archivable`) and
    // `raisePropertyChanged(propArchivable)` (0x702a14-0x702a20), whose
    // value is returned; otherwise unchanged (`0` here — the original
    // returns the holder word, meaningless in the model).
    // SAFETY: `this` must point to a valid `Instance` outliving the call.
    unsafe {
        if (*this).archivable == archivable {
            return 0;
        }
        (*this).archivable = archivable;
        stub_0x7028ec(this, core::ptr::addr_of!(PROP_ARCHIVABLE))
    }
}

// 0x9fb45c — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EED0Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~BoundCallbackDesc()")]
// was: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~BoundCallbackDesc()
pub fn stub_9fb45c() -> ! {
    todo!("0x9fb45c RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~BoundCallbackDesc()")
}

// 0x9fb5b8 — __ZNK3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_ELi2EE18setGenericCallbackEPNS0_13DescribedBaseENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSD_EEEEEEE
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
// was: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const
pub fn stub_9fb5b8() -> ! {
    todo!("0x9fb5b8 RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")
}

// 0x9fba3c — __ZNK3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE13clearCallbackEPNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::clearCallback(RBX::Reflection::DescribedBase *)const
pub fn stub_9fba3c() -> ! {
    todo!("0x9fba3c RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::clearCallback(RBX::Reflection::DescribedBase *)const")
}

// 0x9fbb30 — __ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESF_SD_NS_3argILi1EEENSG_ILi2EEEEENS_3_bi6bind_tIT_PFSL_T0_T1_T2_ENSJ_9list_av_3IT3_T4_T5_E4typeEEESQ_SS_ST_SU_
#[doc(alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_3<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_3<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)
pub fn stub_9fbb30() -> ! {
    todo!("0x9fbb30 boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_3<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)")
}

// 0x9fbf98 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_ELi2EE11callGenericENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSB_EEEEEEES7_S7_
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_9fbf98() -> ! {
    todo!("0x9fbf98 RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9fc950 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESD_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSI_EEEEEEESJ_
#[doc(alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
// was: boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)
pub fn stub_9fc950() -> ! {
    todo!("0x9fc950 boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")
}

// 0x9fcf30 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESG_E4typeENS5_ISD_EE
#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
// was: boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)
pub fn stub_9fcf30() -> ! {
    todo!("0x9fcf30 boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")
}

// 0x9fd4e0 — __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_E9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_S6_ENS9_5list3INS9_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_9fd4e0() -> ! {
    todo!("0x9fd4e0 void boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x9fd950 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESJ_ENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_9fd950() -> ! {
    todo!("0x9fd950 boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x9fd974 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESJ_ENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEES7_SJ_SJ_E6invokeERNS1_15function_bufferESJ_SJ_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_9fd974() -> ! {
    todo!("0x9fd974 boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9fd990 — __ZNK5boost6detail8function13basic_vtable2IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEES8_E9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS3_10Reflection5TupleEEENS6_IKSF_EEEEEEES8_S8_ENSB_5list3INSB_5valueISL_EENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_9fd990() -> ! {
    todo!("0x9fd990 bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x9fdc30 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEENSF_ILi2EEEEclINS5_7Network12FilterResultEPFSL_SD_NS3_INS5_8InstanceEEESN_ENS0_5list2IRSN_SR_EEEET_NS0_4typeIST_EERT0_RT1_l
#[doc(alias = "RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,rbx_core::SharedPtr<RBX::Instance>&> &,long)")]
// was: RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,boost::shared_ptr<RBX::Instance>&> &,long)
pub fn stub_9fdc30() -> ! {
    todo!("0x9fdc30 RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,rbx_core::SharedPtr<RBX::Instance>&> &,long)")
}

// 0x9fe290 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESJ_ENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_9fe290() -> ! {
    todo!("0x9fe290 boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x9fe368 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EED1Ev
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~CallbackDesc()")]
// was: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~CallbackDesc()
pub fn stub_9fe368() -> ! {
    todo!("0x9fe368 RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~CallbackDesc()")
}

// 0x9fe4a8 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EED0Ev
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~CallbackDesc()")]
// was: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~CallbackDesc()
pub fn stub_9fe4a8() -> ! {
    todo!("0x9fe4a8 RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~CallbackDesc()")
}

// 0x9fe604 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE6SetterINS2_16ServerReplicatorEED1Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::~Setter()")]
// was: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::~Setter()
pub fn stub_9fe604() -> ! {
    todo!("0x9fe604 RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::~Setter()")
}

// 0x9fe608 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE6SetterINS2_16ServerReplicatorEED0Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::~Setter()")]
// was: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::~Setter()
pub fn stub_9fe608() -> ! {
    todo!("0x9fe608 RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::~Setter()")
}

// 0x9fe614 — __ZNK3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE6SetterINS2_16ServerReplicatorEE11setCallbackEPNS0_13DescribedBaseERKNS4_8functionIS8_EE
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)const
pub fn stub_9fe614() -> ! {
    todo!("0x9fe614 RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)const")
}

// 0x9fe650 — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EEaSERKS8_
#[doc(alias = "boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::operator=(boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_9fe650() -> ! {
    todo!("0x9fe650 boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::operator=(boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)")
}

// 0x9fe948 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_ELi2EED1Ev
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~CallbackDescImpl()")]
// was: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::~CallbackDescImpl()
pub fn stub_9fe948() -> ! {
    todo!("0x9fe948 RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~CallbackDescImpl()")
}

// 0x9fea88 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_ELi2EED0Ev
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~CallbackDescImpl()")]
// was: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::~CallbackDescImpl()
pub fn stub_9fea88() -> ! {
    todo!("0x9fea88 RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~CallbackDescImpl()")
}

// 0x9ffbc0 — __ZN3RBX7Network10Replicator15NewInstanceItemC2EPS1_N5boost10shared_ptrIKNS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::NewInstanceItem::NewInstanceItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const>)")]
// was: RBX::Network::Replicator::NewInstanceItem::NewInstanceItem(RBX::Network::Replicator*,boost::shared_ptr<RBX::Instance const>)
pub fn stub_9ffbc0() -> ! {
    todo!("0x9ffbc0 RBX::Network::Replicator::NewInstanceItem::NewInstanceItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const>)")
}

// 0xa03878 — __ZN3RBX7Network7Players11whisperChatESsN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Players::whisperChat(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Players::whisperChat(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a03878() -> ! {
    todo!("0xa03878 RBX::Network::Players::whisperChat(std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa04c10 — __ZN3RBX7Network7Players14reportAbuseLuaEN5boost10shared_ptrINS_8InstanceEEESsSs
#[doc(alias = "RBX::Network::Players::reportAbuseLua(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string)")]
// was: RBX::Network::Players::reportAbuseLua(boost::shared_ptr<RBX::Instance>,std::string,std::string)
pub fn stub_a04c10() -> ! {
    todo!("0xa04c10 RBX::Network::Players::reportAbuseLua(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string)")
}

// 0xa06598 — __ZN3RBX7Network7Players19playerFromCharacterEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Players::playerFromCharacter(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Players::playerFromCharacter(boost::shared_ptr<RBX::Instance>)
pub fn stub_a06598() -> ! {
    todo!("0xa06598 RBX::Network::Players::playerFromCharacter(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa06b38 — __ZN3RBX7Network7Players15isNetworkClientEPNS_8InstanceE
#[doc(alias = "RBX::Network::Players::isNetworkClient(RBX::Instance *)")]
pub fn stub_a06b38() -> ! {
    todo!("0xa06b38 RBX::Network::Players::isNetworkClient(RBX::Instance *)")
}

// 0xa07ea0 — __ZN3RBX7Network7Players15clientIsPresentEPKNS_8InstanceEb
#[doc(alias = "RBX::Network::Players::clientIsPresent(RBX::Instance const*,bool)")]
pub fn stub_a07ea0() -> ! {
    todo!("0xa07ea0 RBX::Network::Players::clientIsPresent(RBX::Instance const*,bool)")
}

// 0xa07eac — __ZN3RBX7Network7Players15serverIsPresentEPKNS_8InstanceEb
#[doc(alias = "RBX::Network::Players::serverIsPresent(RBX::Instance const*,bool)")]
pub fn stub_a07eac() -> ! {
    todo!("0xa07eac RBX::Network::Players::serverIsPresent(RBX::Instance const*,bool)")
}

// 0xa07ec8 — __ZN3RBX7Network7Players18frontendProcessingEPKNS_8InstanceEb
#[doc(alias = "RBX::Network::Players::frontendProcessing(RBX::Instance const*,bool)")]
pub fn stub_a07ec8() -> ! {
    todo!("0xa07ec8 RBX::Network::Players::frontendProcessing(RBX::Instance const*,bool)")
}

// 0xa07f44 — __ZN3RBX7Network7Players17backendProcessingEPKNS_8InstanceEb
#[doc(alias = "RBX::Network::Players::backendProcessing(RBX::Instance const*,bool)")]
pub fn stub_a07f44() -> ! {
    todo!("0xa07f44 RBX::Network::Players::backendProcessing(RBX::Instance const*,bool)")
}

// 0xa07fc0 — __ZN3RBX7Network7Players25findLocalSimulatorAddressEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Players::findLocalSimulatorAddress(RBX::Instance const*)")]
pub fn stub_a07fc0() -> ! {
    todo!("0xa07fc0 RBX::Network::Players::findLocalSimulatorAddress(RBX::Instance const*)")
}

// 0xa0803c — __ZN3RBX7Network7Players14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
#[doc(alias = "RBX::Network::Players::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
pub fn stub_a0803c() -> ! {
    todo!("0xa0803c RBX::Network::Players::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")
}

// 0xa14aa0 — __ZNK3RBX7Network7Players11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Players::askAddChild(RBX::Instance const*)const")]
pub fn stub_a14aa0() -> ! {
    todo!("0xa14aa0 RBX::Network::Players::askAddChild(RBX::Instance const*)const")
}

// 0xa14bec — __ZN3RBX7Network7Players18findLocalCharacterEPNS_8InstanceE
#[doc(alias = "RBX::Network::Players::findLocalCharacter(RBX::Instance *)")]
pub fn stub_a14bec() -> ! {
    todo!("0xa14bec RBX::Network::Players::findLocalCharacter(RBX::Instance *)")
}

// 0xa14c18 — __ZN3RBX7Network7Players15findLocalPlayerEPNS_8InstanceE
#[doc(alias = "RBX::Network::Players::findLocalPlayer(RBX::Instance *)")]
pub fn stub_a14c18() -> ! {
    todo!("0xa14c18 RBX::Network::Players::findLocalPlayer(RBX::Instance *)")
}

// 0xa14c40 — __ZN3RBX7Network7Players23findConstLocalCharacterEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)")]
pub fn stub_a14c40() -> ! {
    todo!("0xa14c40 RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)")
}

// 0xa14c6c — __ZN3RBX7Network7Players20findConstLocalPlayerEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)")]
pub fn stub_a14c6c() -> ! {
    todo!("0xa14c6c RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)")
}

// 0xa14c94 — __ZN3RBX7Network7Players18findAncestorPlayerEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)")]
pub fn stub_a14c94() -> ! {
    todo!("0xa14c94 RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)")
}

// 0xa1526c — __ZN3RBX7Network7Players22getPlayerFromCharacterEPNS_8InstanceE
#[doc(alias = "RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)")]
pub fn stub_a1526c() -> ! {
    todo!("0xa1526c RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)")
}

// 0xa15560 — __ZN3RBX7Network7Players20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Players::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Network::Players::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_a15560() -> ! {
    todo!("0xa15560 RBX::Network::Players::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa15700 — __ZN3RBX7Network7Players15onChildRemovingEPNS_8InstanceE
#[doc(alias = "RBX::Network::Players::onChildRemoving(RBX::Instance *)")]
pub fn stub_a15700() -> ! {
    todo!("0xa15700 RBX::Network::Players::onChildRemoving(RBX::Instance *)")
}

// 0xa1624c — __ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
#[doc(alias = "RBX::Network::Players::remoteInsertResultHelper(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
// was: RBX::Network::Players::remoteInsertResultHelper(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3)
pub fn stub_a1624c() -> ! {
    todo!("0xa1624c RBX::Network::Players::remoteInsertResultHelper(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")
}

// 0xa16648 — __ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
#[doc(alias = "RBX::Network::Players::remoteInsertResult(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
// was: RBX::Network::Players::remoteInsertResult(boost::shared_ptr<RBX::Instance>,G3D::Vector3)
pub fn stub_a16648() -> ! {
    todo!("0xa16648 RBX::Network::Players::remoteInsertResult(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")
}

// 0xa16fa4 — __ZN3RBX7Network7Players16disconnectPlayerERNS_8InstanceEi
#[doc(alias = "RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)")]
pub fn stub_a16fa4() -> ! {
    todo!("0xa16fa4 RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)")
}

// 0xa18bc4 — __ZN3RBX7Network7Players12onChildAddedEPNS_8InstanceE
#[doc(alias = "RBX::Network::Players::onChildAdded(RBX::Instance *)")]
pub fn stub_a18bc4() -> ! {
    todo!("0xa18bc4 RBX::Network::Players::onChildAdded(RBX::Instance *)")
}

// 0xa1ae48 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_a1ae48() -> ! {
    todo!("0xa1ae48 RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")
}

// 0xa1ae74 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()
pub fn stub_a1ae74() -> ! {
    todo!("0xa1ae74 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")
}

// 0xa1afb0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_a1afb0() -> ! {
    todo!("0xa1afb0 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0xa1afbc — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_a1afbc() -> ! {
    todo!("0xa1afbc RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b04c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
pub fn stub_a1b04c() -> ! {
    todo!("0xa1b04c RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")
}

// 0xa1b058 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_a1b058() -> ! {
    todo!("0xa1b058 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa1b26c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_a1b26c() -> ! {
    todo!("0xa1b26c RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0xa1b2b4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_a1b2b4() -> ! {
    todo!("0xa1b2b4 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xa1b2c0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_a1b2c0() -> ! {
    todo!("0xa1b2c0 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b308 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_a1b308() -> ! {
    todo!("0xa1b308 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b548 — __ZN3RBX9DataModel13getGuiBuilderEv
#[doc(alias = "RBX::DataModel::getGuiBuilder(void)")]
pub fn stub_a1b548() -> ! {
    todo!("0xa1b548 RBX::DataModel::getGuiBuilder(void)")
}

// 0xa1c8e8 — __ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_a1c8e8() -> ! {
    todo!("0xa1c8e8 rbx_core::SharedPtr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa1ca84 — __ZN3rbx7signals16signal_with_argsILi2EFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEEclESsS6_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::operator()(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a1ca84() -> ! {
    todo!("0xa1ca84 rbx::signals::signal_with_args<2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa1ced4 — __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a1ced4() -> ! {
    todo!("0xa1ced4 rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa1da0c — __ZN5boost10shared_ptrIN3RBX8GuidItemINS1_8InstanceEE8RegistryEEaSERKS6_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>::operator=(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry> const&)")]
// was: boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>::operator=(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry> const&)
pub fn stub_a1da0c() -> ! {
    todo!("0xa1da0c rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>::operator=(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry> const&)")
}

// 0xa1ef88 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEEEclES6_S6_S8_
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")]
// was: rbx::signals::signal_with_args<3,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)
pub fn stub_a1ef88() -> ! {
    todo!("0xa1ef88 rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")
}

// 0xa1f6a0 — __ZN3RBX11shared_fromINS_8InstanceEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> RBX::shared_from<RBX::Instance>(RBX::Instance*)")]
// was: boost::shared_ptr<RBX::Instance> RBX::shared_from<RBX::Instance>(RBX::Instance*)
pub fn stub_a1f6a0() -> ! {
    todo!("0xa1f6a0 rbx_core::SharedPtr<RBX::Instance> RBX::shared_from<RBX::Instance>(RBX::Instance*)")
}

// 0xa1f934 — __ZN3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEE5writeEv
#[doc(alias = "RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::write(void)")]
// was: RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::write(void)
pub fn stub_a1f934() -> ! {
    todo!("0xa1f934 RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::write(void)")
}

// 0xa1ff60 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)
pub fn stub_a1ff60() -> ! {
    todo!("0xa1ff60 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")
}

// 0xa20ee4 — __ZNK3RBX8Instance13visitChildrenI21AppendOtherCharactersEEvRKT_
#[doc(alias = "void RBX::Instance::visitChildren<AppendOtherCharacters>(AppendOtherCharacters const&)const")]
pub fn stub_a20ee4() -> ! {
    todo!("0xa20ee4 void RBX::Instance::visitChildren<AppendOtherCharacters>(AppendOtherCharacters const&)const")
}

// 0xa23aa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_a23aa8() -> ! {
    todo!("0xa23aa8 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa23ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_a23ab0() -> ! {
    todo!("0xa23ab0 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa23ad0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_a23ad0() -> ! {
    todo!("0xa23ad0 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa23ae8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_a23ae8() -> ! {
    todo!("0xa23ae8 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa242b8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")]
// was: boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)
pub fn stub_a242b8() -> ! {
    todo!("0xa242b8 rbx_core::SharedPtr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")
}

// 0xa246b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_a246b0() -> ! {
    todo!("0xa246b0 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa25aa8 — __ZN21AppendOtherCharactersclEN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "AppendOtherCharacters::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: AppendOtherCharacters::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_a25aa8() -> ! {
    todo!("0xa25aa8 AppendOtherCharacters::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa26548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_a26548() -> ! {
    todo!("0xa26548 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa26550 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_a26550() -> ! {
    todo!("0xa26550 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa2656c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_a2656c() -> ! {
    todo!("0xa2656c boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa26584 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_a26584() -> ! {
    todo!("0xa26584 boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa2b3d0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance>*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_a2b3d0() -> ! {
    todo!("0xa2b3d0 std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xa2ccc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_a2ccc8() -> ! {
    todo!("0xa2ccc8 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa2ccd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_a2ccd0() -> ! {
    todo!("0xa2ccd0 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa2e2e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_a2e2e4() -> ! {
    todo!("0xa2e2e4 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa2e2e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_a2e2e8() -> ! {
    todo!("0xa2e2e8 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa2e2f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_a2e2f4() -> ! {
    todo!("0xa2e2f4 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa2e310 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_a2e310() -> ! {
    todo!("0xa2e310 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa2e328 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_a2e328() -> ! {
    todo!("0xa2e328 boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa2f1b0 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_8HumanoidEEEPKT_v
#[doc(alias = "RBX::Humanoid const* RBX::Instance::findConstFirstChildOfType<RBX::Humanoid>(void)const")]
pub fn stub_a2f1b0() -> ! {
    todo!("0xa2f1b0 RBX::Humanoid const* RBX::Instance::findConstFirstChildOfType<RBX::Humanoid>(void)const")
}

// 0xa2f6ec — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)
pub fn stub_a2f6ec() -> ! {
    todo!("0xa2f6ec void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")
}

// 0xa2f8dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_a2f8dc() -> ! {
    todo!("0xa2f8dc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xa2f900 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_a2f900() -> ! {
    todo!("0xa2f900 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xa2f918 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const
pub fn stub_a2f918() -> ! {
    todo!("0xa2f918 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")
}

// 0xa2faf4 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_a2faf4() -> ! {
    todo!("0xa2faf4 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa2fd44 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEclIPFvS7_NS_10shared_ptrINS4_8InstanceEEESC_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_a2fd44() -> ! {
    todo!("0xa2fd44 void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}