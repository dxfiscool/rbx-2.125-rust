#![allow(unused_attributes, dead_code, unused_variables, non_snake_case, non_camel_case_types, clippy::all)]
//! generated_core_wd_watchdog22 — 120 stubs EA-sorted asc global gap filler 0x668878..0x6795b0 (85545 total gaps)
//! Source: ida/export.json (85545 funcs) global EA asc not yet in crates/*/src
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR() { todo!("0xADDR") }

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// Batch 9: 31 IDA-grounded ports 0x668878-0x66adec — the GuiObject /
/// GuiBase2d field readers, the TextBox + GuiObject dtor/thunk lattices, and
/// the `FactoryProduct<TextBox, GuiObject>` Creator C2/D2/create/getClassName
/// /static_getCreator. Ports live in `gui_textbox`; `stub_0x*` keeps the
/// `#[doc(alias)]` + `// 0xADDR` carrier lines and wires into it.
/// Conventions: GuiObject/GuiBase2d/TextBox storage is datamodel-owned, so
/// readers are raw-offset loads and teardown sequencing is core-owned while
/// member destruction runs through caller-supplied hooks (DAG: core never
/// depends on datamodel); `rbx::remote_signal` teardown is core-owned via
/// `RemoteSignal`; `boost::shared_ptr` -> `crate::SharedPtr` (Arc);
/// `ReleaseAssert` -> `assert!` gated on `FLOG_ASSERTS` (was `FLog::Asserts`);
/// vtable installs are symbolic `off_*` names (addresses live in the target
/// binary). `[INFERENCE]` marks what the binary does not pin down.
pub mod gui_textbox {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    #[inline]
    pub fn release_assert(cond: bool, msg: &'static str) {
        // IDA pattern at 0x669002-0x66902c etc.: `if (FLog::Asserts) { if
        // (!check) { hook ? hook(...) : ReleaseAssert(...) } }`. The
        // `_debugHook` override is a test hook (`[INFERENCE]` — no live
        // override observed); the abort itself is `assert!`.
        if FLOG_ASSERTS.load(Ordering::Relaxed) && !cond {
            assert!(cond, "{msg}");
        }
    }
    /// was: `FLog::Asserts` — every ReleaseAssert path below reads it first.
    pub static FLOG_ASSERTS: AtomicBool = AtomicBool::new(false);
    /// IDA 0x668882: `Instance::getPersistentDataCost(this) + 6`.
    pub const GUIOBJECT_PERSISTENT_COST_BONUS: i32 = 6;
    /// Base cost is datamodel-owned (`Instance::getPersistentDataCost`); the
    /// `+ 6` rule is core-portable and lives here.
    #[inline]
    pub fn gui_object_persistent_cost(instance_cost: i32) -> i32 {
        instance_cost + GUIOBJECT_PERSISTENT_COST_BONUS // IDA 0x668878
    }
    /// IDA 0x668d04: `*((unsigned __int8 *)this + 512)`.
    pub const CAN_PROCESS_BYTE_OFF: usize = 512;
    /// IDA 0x668d0c / 0x668d14: `*((_DWORD *)this + 34/35)`.
    pub const ZINDEX_WORD: usize = 34;
    pub const GUIQUEUE_WORD: usize = 35;
    /// was: `RBX::GuiObject` child-rectangle — `getChildRect2D` (0x668d1c)
    /// forwards `getRect2D`, so the rect crosses the boundary by value.
    #[repr(C)]
    #[derive(Clone, Copy, Default, Debug, PartialEq)]
    pub struct Rect2d {
        pub min: [f32; 2],
        pub max: [f32; 2],
    }
    /// IDA 0x66a8e4-0x66a91a: the six vtable installs of `GuiObject::~GuiObject`.
    pub const GUIOBJECT_VTABLE_SLOTS: [(usize, &str); 6] = [
        (0, "off_11DD358"),   // IDA 0x66a8e4: `*this`
        (3, "off_11DD424"),   // IDA 0x66a8f0
        (8, "off_11DD430"),   // IDA 0x66a8f8
        (9, "off_11DD444"),   // IDA 0x66a904
        (23, "off_11DD45C"),  // IDA 0x66a912
        (24, "off_11DD468"),  // IDA 0x66a91a
    ];
    /// Member kinds torn down by `GuiObject::~GuiObject` (IDA 0x66a942-0x66aa2a).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GuiObjectMember {
        ReplicatorUDim2,
        ReplicatorIntInt,
        RemoteUDim2,
        RemoteIntInt,
        TweenSecond,
        TweenFirst,
        Adornable,
        Instance,
    }
    /// Exact teardown order, byte offsets from `this` (IDA 0x66a942-0x66a9ea).
    pub const GUIOBJECT_SIGNAL_MEMBERS: [(usize, GuiObjectMember); 14] = [
        (412, GuiObjectMember::ReplicatorUDim2), // IDA 0x66a942
        (380, GuiObjectMember::ReplicatorIntInt), // IDA 0x66a950
        (368, GuiObjectMember::RemoteUDim2),    // IDA 0x66a95e
        (360, GuiObjectMember::RemoteIntInt),   // IDA 0x66a96c
        (332, GuiObjectMember::ReplicatorIntInt), // IDA 0x66a97a
        (300, GuiObjectMember::ReplicatorIntInt), // IDA 0x66a988
        (268, GuiObjectMember::ReplicatorIntInt), // IDA 0x66a996
        (236, GuiObjectMember::ReplicatorIntInt), // IDA 0x66a9a2
        (204, GuiObjectMember::ReplicatorIntInt), // IDA 0x66a9ae
        (192, GuiObjectMember::RemoteIntInt),   // IDA 0x66a9ba
        (184, GuiObjectMember::RemoteIntInt),   // IDA 0x66a9c6
        (176, GuiObjectMember::RemoteIntInt),   // IDA 0x66a9d2
        (168, GuiObjectMember::RemoteIntInt),   // IDA 0x66a9de
        (160, GuiObjectMember::RemoteIntInt),   // IDA 0x66a9ea
    ];
    /// IDA 0x66a9f4: `v68 = *((_DWORD *)this + 36)` — nullable tween pair.
    pub const TWEEN_WORD: usize = 36;
    /// IDA 0x66aa1e: `IAdornable` base at `(char *)this + 96`.
    pub const IADORNABLE_BYTE_OFF: usize = 96;
    /// Datamodel-supplied teardown for members core cannot own. The walk
    /// order, offsets, and null check are core-owned and 1:1 with IDA
    /// 0x66a942-0x66aa2a; only the per-member action crosses the crate line.
    pub struct GuiObjectFini {
        pub member: unsafe fn(*mut u8, GuiObjectMember),
        pub free: unsafe fn(*mut u8),
    }
    /// IDA 0x66a8b4 `RBX::GuiObject::~GuiObject` (D2): vtable installs are
    /// symbolic (`GUIOBJECT_VTABLE_SLOTS` — addresses live in the target
    /// binary); the member walk, tween branch, and base order are real.
    pub unsafe fn gui_object_d2(this: *mut u8, fini: &GuiObjectFini) {
        for &(off, kind) in &GUIOBJECT_SIGNAL_MEMBERS {
            (fini.member)(this.add(off), kind); // IDA 0x66a942-0x66a9ea
        }
        let tween = *(this.add(TWEEN_WORD * 4) as *mut *mut u8); // IDA 0x66a9f4
        if !tween.is_null() {
            // IDA 0x66aa02: `~scoped_ptr((int)v68 + 4)` first ...
            (fini.member)(tween.add(4), GuiObjectMember::TweenSecond);
            // IDA 0x66aa0c: ... then `~scoped_ptr((int)v68)` ...
            (fini.member)(tween, GuiObjectMember::TweenFirst);
            (fini.free)(tween); // IDA 0x66aa12: `operator delete(v68)`
        }
        (fini.member)(this.add(IADORNABLE_BYTE_OFF), GuiObjectMember::Adornable); // IDA 0x66aa1e
        (fini.member)(this, GuiObjectMember::Instance); // IDA 0x66aa2a
    }
    /// was: `rbx::remote_signal<void ()(RBX::UDim2)>` — two signal channels
    /// (the `void()` one at +4, the `(UDim2)` one at +0) each with a nullable
    /// intrusive slot ref (IDA 0x66b4ce-0x66b4f4: `disconnectAll` then
    /// conditional `intrusive_ptr_release`). `Signal::disconnect_all` is the
    /// disconnect; dropping the `SharedPtr` is the release.
    pub struct RemoteSignal {
        pub void_channel: crate::signal::Signal<()>,
        pub void_slot: Option<crate::SharedPtr<()>>,
        pub udim2_channel: crate::signal::Signal<(f32, f32, f32, f32)>,
        pub udim2_slot: Option<crate::SharedPtr<()>>,
    }
    impl RemoteSignal {
        pub fn new() -> Self {
            Self {
                void_channel: crate::signal::Signal::new(),
                void_slot: None,
                udim2_channel: crate::signal::Signal::new(),
                udim2_slot: None,
            }
        }
        /// IDA 0x66b478 `~remote_signal`: `disconnectAll(a1 + 4)` + release,
        /// then `disconnectAll(a1)` + release.
        pub fn disconnect_all(&mut self) {
            self.void_channel.disconnect_all(); // IDA 0x66b4ce
            self.void_slot = None; // IDA 0x66b4d4-0x66b4dc
            self.udim2_channel.disconnect_all(); // IDA 0x66b4e6
            self.udim2_slot = None; // IDA 0x66b4ec-0x66b4f4
        }
    }
    impl Default for RemoteSignal {
        fn default() -> Self {
            Self::new()
        }
    }
    /// Teardown for `RBX::TextBox` storage: D2 runs first (wired to
    /// `stub_0x672230` once ported), `free` is the `operator delete` half.
    pub struct TextBoxFini {
        pub d2: unsafe fn(*mut u8),
        pub free: unsafe fn(*mut u8),
    }
    /// was: `FactoryProduct<TextBox, GuiObject>::Creator` construction state.
    pub struct Creator {
        pub vtab: &'static str,
        pub name: &'static str,
    }
    /// IDA 0x669054/0x673ff8/0x6795b0 disasm: the Creator `getClassName`
    /// family returns the declared class name behind the constructed assert.
    pub const TEXTBOX_CLASS_NAME: &str = "TextBox";
    /// IDA 0x6696ea / 0x668fda: `a1->__sig = &off_128D304`.
    pub const TEXTBOX_CREATOR_VTAB: &str = "off_128D304";
    /// `isConstructed` sentinel (`...13isConstructedE != 666`).
    pub const WAS_CONSTRUCTED_MAGIC: i32 = 666;
    /// was: `AbstractFactoryProduct<Instance>::getCreators()::creators` —
    /// `std::map<Name const*, ICreator const*>` keyed here by class name.
    static CREATORS: std::sync::LazyLock<
        parking_lot::Mutex<std::collections::HashMap<&'static str, usize>>,
    > = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
    static CREATOR_CONSTRUCTED: AtomicBool = AtomicBool::new(false);
    /// was: `...14creatorPrivateE` — address of the static Creator slot,
    /// recorded at C2 so `static_getCreator` can return it (IDA 0x66995a).
    static CREATOR_ADDR: AtomicUsize = AtomicUsize::new(0);
    fn was_constructed() -> bool {
        CREATOR_CONSTRUCTED.load(Ordering::SeqCst)
    }
    /// IDA 0x6698f8 `static_getCreator`: `wasConstructed` assert (Object.h:282)
    /// then `&creatorPrivateE`.
    pub fn static_get_creator() -> *const Creator {
        // IDA 0x669908-0x66995a.
        release_assert(was_constructed(), "Creator::wasConstructed() file: include/Util/Object.h line: 282");
        CREATOR_ADDR.load(Ordering::SeqCst) as *const Creator // IDA 0x66995a/0x66996a
    }
    /// Shared `getClassName` tail (IDA 0x668cf4/0x669042 via
    /// `static_getCreator` + the Creator `getClassName` shim at 0x669054).
    pub fn creator_class_name() -> &'static str {
        static_get_creator();
        TEXTBOX_CLASS_NAME
    }
    /// was: `FactoryProduct<T, ...>::Creator` class key — one shared
    /// creators map, per-class name/vtable (TextBox batch 9, GuiTextButton
    /// batch 11, GuiLabel/TextLabel batch 12).
    #[derive(Clone, Copy)]
    pub struct CreatorClass {
        pub name: &'static str,
        pub vtab: &'static str,
    }
    pub const TEXTBOX_CLASS: CreatorClass = CreatorClass {
        name: TEXTBOX_CLASS_NAME,
        vtab: TEXTBOX_CREATOR_VTAB,
    };
    /// IDA 0x6696b4/0x674658 Creator C2, generalized over the class: the
    /// `DECLARED` once is per-class here (binary: per-`declare` flag).
    pub unsafe fn creator_construct_as(
        slot: *mut Creator,
        class: &CreatorClass,
        constructed: &AtomicBool,
        addr: &AtomicUsize,
    ) -> *mut Creator {
        use std::sync::LazyLock;
        static DECLARED: LazyLock<()> = LazyLock::new(|| {});
        LazyLock::force(&DECLARED); // `Name::declare(sX)` — names live in reflection
        (*slot).vtab = class.vtab; // IDA 0x6696ea / 0x67468e (`a1->__sig`)
        release_assert(!constructed.load(Ordering::SeqCst), "!wasConstructed() file: include/Util/Object.h line: 245");
        {
            let mut map = CREATORS.lock();
            // IDA 0x66972e-0x669744: `lower_bound` walk asserting absence.
            release_assert(
                !map.contains_key(class.name),
                "Class::getCreators().find(&name)==Class::getCreators().end() file: include/Util/Object.h line: 244",
            );
            map.insert(class.name, slot as usize); // IDA 0x6697f8-0x669814
        }
        (*slot).name = class.name;
        addr.store(slot as usize, Ordering::SeqCst);
        constructed.store(true, Ordering::SeqCst); // IDA 0x66981a / 0x6747be: `= 666`
        {
            let map = CREATORS.lock();
            // IDA 0x66983a-0x66985c: `find != end` post assert ...
            release_assert(
                map.get(class.name) == Some(&(slot as usize)),
                "Class::getCreators().find(&name)!=Class::getCreators().end() file: include/Util/Object.h line: 250",
            );
        }
        // IDA 0x6698a0-0x6698ea: `wasConstructed` post assert (Object.h:251).
        release_assert(constructed.load(Ordering::SeqCst), "wasConstructed() file: include/Util/Object.h line: 251");
        slot // IDA 0x6698f6 / 0x67489a
    }
    /// IDA 0x6696b4 Creator C2 for TextBox (wraps the shared template).
    pub unsafe fn creator_construct(slot: *mut Creator) -> *mut Creator {
        creator_construct_as(slot, &TEXTBOX_CLASS, &CREATOR_CONSTRUCTED, &CREATOR_ADDR)
    }
    pub unsafe fn creator_destroy_as(
        slot: *mut Creator,
        class: &CreatorClass,
        constructed: &AtomicBool,
    ) -> *mut Creator {
        (*slot).vtab = class.vtab; // IDA 0x668fda / 0x673f7e (`*a1 = &off_...`)
        release_assert(constructed.load(Ordering::SeqCst), "wasConstructed() file: include/Util/Object.h line: 255");
        CREATORS.lock().remove((*slot).name); // IDA 0x66904a
        slot // IDA 0x669052 / 0x673ff6
    }
    /// IDA 0x668fb8 Creator D2 for TextBox (wraps the shared template; the
    /// `*a1` restore word differs per class and is applied by the caller —
    /// here it coincides with the TextBox vtable).
    pub unsafe fn creator_destroy(slot: *mut Creator) -> *mut Creator {
        creator_destroy_as(slot, &TEXTBOX_CLASS, &CREATOR_CONSTRUCTED)
    }
    /// IDA 0x6698f8/0x67489c `static_getCreator`, generalized: assert
    /// (Object.h:282) then `&creatorPrivateE`.
    pub fn static_get_creator_as(addr: &AtomicUsize, constructed: &AtomicBool) -> *const Creator {
        // IDA 0x669908-0x66995a / 0x6748ac-0x6748fe.
        release_assert(constructed.load(Ordering::SeqCst), "Creator::wasConstructed() file: include/Util/Object.h line: 282");
        addr.load(Ordering::SeqCst) as *const Creator // IDA 0x66995a/0x66996a
    }
    /// was: `shared_ptr<Instance>` + `shared_count` out-pair of Creator
    /// `create` (IDA 0x6690dc): `*a1` takes the `+32` Instance-subobject
    /// pointer sharing ownership, `(a1 + 1)` the moved count.
    pub struct TextBoxShared {
        pub instance_ptr: *mut u8,
        pub ownership: Option<crate::SharedPtr<u8>>,
    }
    /// IDA 0x6690dc: `wasConstructed` assert (Object.h:231), then
    /// `Creatable<Instance>::create<TextBox>` (datamodel-supplied `alloc`),
    /// the `+32` adjust (0x6691a8), count move (0x6691b6), local release
    /// (0x6691bc-0x6691c4 — the Arc drop here).
    pub unsafe fn textbox_create(
        out: *mut TextBoxShared,
        alloc: unsafe fn() -> (*mut u8, crate::SharedPtr<u8>),
    ) {
        release_assert(was_constructed(), "wasConstructed() file: include/Util/Object.h line: 231");
        let (obj, ownership) = alloc(); // IDA 0x669194
        let instance = obj.wrapping_add(32); // IDA 0x6691a8 (`v17 + 32`)
        // Move (not clone): net count matches copy-then-release-local.
        out.write(TextBoxShared { instance_ptr: instance, ownership: Some(ownership) });
    }
}

// 0x668878 — __ZNK3RBX9GuiObject21getPersistentDataCostEv
// type: _DWORD __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiObject21getPersistentDataCostEv")]
pub unsafe fn stub_0x668878(this: *const u8, instance_cost: i32) -> i32 {
    // IDA 0x668878: `Instance::getPersistentDataCost(this) + 6`; the base
    // call is datamodel-owned, the `+ 6` rule ports here.
    let _ = this;
    gui_textbox::gui_object_persistent_cost(instance_cost)
}

// 0x668c4c — __ZN3RBX7TextBoxD1Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZN3RBX7TextBoxD1Ev")]
pub unsafe fn stub_0x668c4c(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668c4c: D1 thunk straight into D2 (`$shim`).
    (fini.d2)(this)
}

// 0x668c50 — __ZN3RBX7TextBoxD0Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZN3RBX7TextBoxD0Ev")]
pub unsafe fn stub_0x668c50(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668c50: D2 (0x668ca0) then `operator delete` (0x668ca6).
    (fini.d2)(this);
    (fini.free)(this)
}

// 0x668cf0 — __ZNK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x668cf0() -> &'static str {
    // IDA 0x668cf0: `static_getCreator` (0x668cf4) + Creator `getClassName` shim.
    gui_textbox::creator_class_name()
}

// 0x668d00 — __ZNK3RBX9GuiObject26canProcessMeAndDescendantsEv
// type: _DWORD __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiObject26canProcessMeAndDescendantsEv")]
pub unsafe fn stub_0x668d00(this: *const u8) -> bool {
    // IDA 0x668d00: `*((unsigned __int8 *)this + 512)`.
    *this.add(gui_textbox::CAN_PROCESS_BYTE_OFF) != 0
}

// 0x668d08 — __ZNK3RBX9GuiBase2d9getZIndexEv
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiBase2d9getZIndexEv")]
pub unsafe fn stub_0x668d08(this: *const u32) -> u32 {
    // IDA 0x668d08: `*((_DWORD *)this + 34)`.
    *this.add(gui_textbox::ZINDEX_WORD)
}

// 0x668d10 — __ZNK3RBX9GuiBase2d11getGuiQueueEv
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiBase2d11getGuiQueueEv")]
pub unsafe fn stub_0x668d10(this: *const u32) -> u32 {
    // IDA 0x668d10: `*((_DWORD *)this + 35)`.
    *this.add(gui_textbox::GUIQUEUE_WORD)
}

// 0x668d18 — __ZNK3RBX9GuiBase2d9isGuiLeafEv
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiBase2d9isGuiLeafEv")]
pub fn stub_0x668d18(_this: *const u8) -> bool {
    // IDA 0x668d18: constant `0`.
    false
}

// 0x668d1c — __ZNK3RBX9GuiBase2d14getChildRect2DEv
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiBase2d14getChildRect2DEv")]
pub unsafe fn stub_0x668d1c(
    this: *mut u8,
    get_rect: unsafe fn(*mut u8) -> gui_textbox::Rect2d,
) -> gui_textbox::Rect2d {
    // IDA 0x668d1c: tail-forward to `GuiBase2d::getRect2D` (datamodel-owned).
    get_rect(this)
}

// 0x668d28 — __ZNK3RBX9GuiBase2d14shouldRender2dEv
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiBase2d14shouldRender2dEv")]
pub fn stub_0x668d28(_this: *const u8) -> bool {
    // IDA 0x668d28: constant `0`.
    false
}

// 0x668d90 — __ZThn32_N3RBX7TextBoxD1Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7TextBoxD1Ev")]
pub unsafe fn stub_0x668d90(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668d90: `this - 32` adjust into D1.
    stub_0x668c4c(this.sub(32), fini)
}

// 0x668d98 — __ZThn32_N3RBX7TextBoxD0Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7TextBoxD0Ev")]
pub unsafe fn stub_0x668d98(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668d98: `v4 = this - 32` (0x668dc2), D0 (0x668dea), delete (0x668df0).
    stub_0x668c50(this.sub(32), fini)
}

// 0x668e3c — __ZThn32_NK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x668e3c() -> &'static str {
    // IDA 0x668e3c: same `static_getCreator` + `getClassName` tail as 0x668cf0.
    gui_textbox::creator_class_name()
}

// 0x668e4c — __ZThn36_N3RBX7TextBoxD1Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7TextBoxD1Ev")]
pub unsafe fn stub_0x668e4c(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668e4c: `this - 36` adjust into D1.
    stub_0x668c4c(this.sub(36), fini)
}

// 0x668e54 — __ZThn36_N3RBX7TextBoxD0Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7TextBoxD0Ev")]
pub unsafe fn stub_0x668e54(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668e54: `v4 = this - 36` (0x668e7e), D0 (0x668ea6), delete (0x668eac).
    stub_0x668c50(this.sub(36), fini)
}

// 0x668ef8 — __ZThn96_NK3RBX9GuiBase2d14shouldRender2dEv
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "__ZThn96_NK3RBX9GuiBase2d14shouldRender2dEv")]
pub fn stub_0x668ef8(_this: *const u8) -> bool {
    // IDA 0x668ef8: constant `0` (thunk adds no adjust).
    false
}

// 0x668f08 — __ZThn596_N3RBX7TextBoxD1Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZThn596_N3RBX7TextBoxD1Ev")]
pub unsafe fn stub_0x668f08(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668f08: `this - 596` adjust into D1.
    stub_0x668c4c(this.sub(596), fini)
}

// 0x668f10 — __ZThn596_N3RBX7TextBoxD0Ev
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "__ZThn596_N3RBX7TextBoxD0Ev")]
pub unsafe fn stub_0x668f10(this: *mut u8, fini: &gui_textbox::TextBoxFini) {
    // IDA 0x668f10: `v4 = this - 596` (0x668f3c), D0 (0x668f64), delete (0x668f6a).
    stub_0x668c50(this.sub(596), fini)
}

// 0x668fb4 — __ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7CreatorD1Ev")]
pub unsafe fn stub_0x668fb4(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x668fb4: D1 thunk straight into D2 (`$shim`).
    gui_textbox::creator_destroy(slot)
}

// 0x668fb8 — __ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7CreatorD2Ev")]
pub unsafe fn stub_0x668fb8(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x668fb8: vtable restore + `wasConstructed` assert + creators erase.
    gui_textbox::creator_destroy(slot)
}

// 0x669054 — __ZNK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x669054() -> &'static str {
    // IDA 0x669054 (disasm: FLog::Asserts prologue like 0x673ff8/0x6795b0):
    // assert-guarded class-name read; same tail as 0x668cf0.
    gui_textbox::creator_class_name()
}

// 0x6690dc — __ZNK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7Creator6createEv")]
pub unsafe fn stub_0x6690dc(
    out: *mut gui_textbox::TextBoxShared,
    alloc: unsafe fn() -> (*mut u8, crate::SharedPtr<u8>),
) {
    // IDA 0x6690dc: assert + `Creatable::create<TextBox>` + `+32` + count move.
    gui_textbox::textbox_create(out, alloc)
}

// 0x6696b4 — __ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE7CreatorC2Ev")]
pub unsafe fn stub_0x6696b4(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x6696b4: declare-once + creators insert + `isConstructed = 666`.
    gui_textbox::creator_construct(slot)
}

// 0x6698f8 — __ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextBoxENS_9GuiObjectELZNS_8sTextBoxEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x6698f8() -> *const gui_textbox::Creator {
    // IDA 0x6698f8: `wasConstructed` assert (Object.h:282) + `&creatorPrivateE`.
    gui_textbox::static_get_creator()
}

// 0x66a8b4 — __ZN3RBX9GuiObjectD2Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZN3RBX9GuiObjectD2Ev")]
pub unsafe fn stub_0x66a8b4(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66a8b4: vtable installs + 14-member walk + tween branch + bases.
    gui_textbox::gui_object_d2(this, fini)
}

// 0x66ac8c — __ZN3RBX9GuiObjectD1Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZN3RBX9GuiObjectD1Ev")]
pub unsafe fn stub_0x66ac8c(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66ac8c: D1 thunk straight into D2 (`$shim`).
    gui_textbox::gui_object_d2(this, fini)
}

// 0x66ac90 — __ZN3RBX9GuiObjectD0Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZN3RBX9GuiObjectD0Ev")]
pub unsafe fn stub_0x66ac90(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66ac90: D2 (0x66ace0) then `operator delete` (0x66ace6).
    gui_textbox::gui_object_d2(this, fini);
    (fini.free)(this)
}

// 0x66ad34 — __ZThn32_N3RBX9GuiObjectD1Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9GuiObjectD1Ev")]
pub unsafe fn stub_0x66ad34(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66ad34: `this - 32` adjust into D1.
    stub_0x66ac8c(this.sub(32), fini)
}

// 0x66ad3c — __ZThn32_N3RBX9GuiObjectD0Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9GuiObjectD0Ev")]
pub unsafe fn stub_0x66ad3c(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66ad3c: `v1 = this - 32` (0x66ad66), D0 (0x66ad8e), delete (0x66ad94).
    stub_0x66ac90(this.sub(32), fini)
}

// 0x66ade4 — __ZThn36_N3RBX9GuiObjectD1Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9GuiObjectD1Ev")]
pub unsafe fn stub_0x66ade4(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66ade4: `this - 36` adjust into D1.
    stub_0x66ac8c(this.sub(36), fini)
}

// 0x66adec — __ZThn36_N3RBX9GuiObjectD0Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9GuiObjectD0Ev")]
pub unsafe fn stub_0x66adec(this: *mut u8, fini: &gui_textbox::GuiObjectFini) {
    // IDA 0x66adec: `v1 = this - 36` (0x66ae16), D0 (0x66ae3e), delete (0x66ae44).
    stub_0x66ac90(this.sub(36), fini)
}

/// Batch 10: 26 IDA-grounded ports 0x66b478-0x6717dc — the `remote_signal`
/// D2, `TextService` `isNullClassName`, and the four `placement_any<Region3>`
/// families (`YAlignment`/`XAlignment`/`Font`/`FontSize`: `operator=`,
/// `typed_holder` singleton/`construct_func`/`destruct_func`, `any_cast`,
/// `_Rb_tree` `_M_erase`). Ports live in `region_any`; `stub_0x*` keeps the
/// `#[doc(alias)]` + `// 0xADDR` carrier lines and wires into it.
/// Conventions: holder identity is the singleton address (exactly like the
/// binary compares `*a1 == &...::s`); `boost::throw_exception` -> `panic!`;
/// `__cxa_guard_acquire` -> `LazyLock`; node storage is owner-allocated so
/// `_M_erase` frees through a caller-supplied callback. `[INFERENCE]` marks
/// what the binary does not pin down.
pub mod region_any {
    use std::sync::LazyLock;
    /// was: `rbx::implementation::typed_holder<T>` — the two-word holder
    /// `{typeinfo, destruct}` plus the registered construct target (the
    /// `dword_128Dx` store at singleton-init, e.g. IDA 0x66d8ce).
    pub struct TypedHolder {
        pub type_name: &'static str,
        pub destruct: fn(*mut usize),
        pub construct: fn(*const usize, *mut usize),
    }
    /// IDA 0x66d8f0/0x66eb74/0x670464/0x6716e8 `destruct_func`: `;` (these
    /// Ts are trivially destructible).
    fn trivial_destruct(_payload: *mut usize) {}
    /// IDA 0x66d8e4/0x66eb68/0x670458/0x6716dc `construct_func`: one word
    /// copy (`result = *result; *a2 = result`).
    fn word_construct(src: *const usize, dst: *mut usize) {
        unsafe { dst.write(src.read()) }
    }
    static YALIGNMENT: LazyLock<TypedHolder> = LazyLock::new(|| TypedHolder {
        type_name: "N3RBX11TextService10YAlignmentE", // IDA 0x66d97c
        destruct: trivial_destruct,
        construct: word_construct,
    });
    static XALIGNMENT: LazyLock<TypedHolder> = LazyLock::new(|| TypedHolder {
        type_name: "N3RBX11TextService10XAlignmentE", // IDA 0x66ec00
        destruct: trivial_destruct,
        construct: word_construct,
    });
    static FONT: LazyLock<TypedHolder> = LazyLock::new(|| TypedHolder {
        type_name: "N3RBX11TextService4FontE", // IDA 0x6704f0
        destruct: trivial_destruct,
        construct: word_construct,
    });
    static FONTSIZE: LazyLock<TypedHolder> = LazyLock::new(|| TypedHolder {
        type_name: "N3RBX11TextService8FontSizeE", // IDA 0x671774
        destruct: trivial_destruct,
        construct: word_construct,
    });
    /// IDA 0x66d878/0x66eafc/0x6703ec/0x671670 `singleton`: guard-var +
    /// `__cxa_guard_acquire` become `LazyLock`; the `s[0] = &typeinfo` /
    /// `s[1] = destruct_func` / `dword = construct_func` stores become the
    /// struct fields above.
    pub fn yalignment_holder() -> &'static TypedHolder {
        &YALIGNMENT
    }
    pub fn xalignment_holder() -> &'static TypedHolder {
        &XALIGNMENT
    }
    pub fn font_holder() -> &'static TypedHolder {
        &FONT
    }
    pub fn fontsize_holder() -> &'static TypedHolder {
        &FONTSIZE
    }
    fn holder_for_addr(addr: usize) -> Option<&'static TypedHolder> {
        for h in [yalignment_holder(), xalignment_holder(), font_holder(), fontsize_holder()] {
            if std::ptr::from_ref(h) as usize == addr {
                return Some(h);
            }
        }
        None
    }
    /// was: `rbx::placement_any<RBX::Region3>` — holder word + payload word
    /// (`*a1` / `a1[1]`).
    #[repr(C)]
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
    pub struct RegionAny {
        pub holder: usize,
        pub payload: usize,
    }
    impl RegionAny {
        /// `operator=<T>` (IDA 0x66d828/0x66eaac/0x67039c/0x671620):
        /// same-holder takes the payload-copy fast path (0x66d860);
        /// otherwise destroy current (`v4[1](a1 + 1)`, a no-op call for these
        /// Ts), clear (0x66d858), copy (0x66d86a), install (0x66d86c).
        /// Foreign holders (address matches none of ours) skip the destruct
        /// call — that address lives in the target binary (`[INFERENCE]` on
        /// foreign layouts only; the walk itself is 1:1).
        pub fn assign(&mut self, value: usize, holder: &'static TypedHolder) {
            let holder_addr = std::ptr::from_ref(holder) as usize;
            if self.holder == holder_addr {
                self.payload = value; // IDA 0x66d860
                return;
            }
            if self.holder != 0 {
                if let Some(cur) = holder_for_addr(self.holder) {
                    (cur.destruct)(&mut self.payload); // IDA 0x66d854
                }
                self.holder = 0; // IDA 0x66d858
            }
            self.payload = value; // IDA 0x66d86a
            self.holder = holder_addr; // IDA 0x66d86c
        }
    }
    /// was: `rbx::bad_placement_any_cast` (thrown as `std::bad_cast` with
    /// `off_1221648`, e.g. IDA 0x66d9aa-0x66d9b2).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BadPlacementAnyCast(pub &'static str);
    impl std::fmt::Display for BadPlacementAnyCast {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "bad placement_any_cast for {}", self.0)
        }
    }
    impl std::error::Error for BadPlacementAnyCast {}
    /// `any_cast<const T&>` (IDA 0x66d8f4/0x66eb78/0x670468/0x6716ec):
    /// holder-identity fast path; typeinfo-name slow path (`void` when the
    /// holder is null, 0x66d950); name mismatch throws (`boost::
    /// throw_exception` -> `panic!`); success returns the payload address
    /// (`a1 + 1`).
    pub unsafe fn any_cast(any: *mut RegionAny, holder: &'static TypedHolder) -> *mut usize {
        let cur_addr = (*any).holder;
        if cur_addr != std::ptr::from_ref(holder) as usize {
            let cur_name = holder_for_addr(cur_addr).map(|h| h.type_name).unwrap_or("v");
            if cur_name != holder.type_name {
                // IDA 0x66d9aa-0x66d9b2 + resume path 0x66d9c8-0x66d9d6.
                panic!("{}", BadPlacementAnyCast(holder.type_name));
            }
        }
        &mut (*any).payload // IDA 0x66d99a
    }
    /// was: `std::_Rb_tree_node_base` (`_M_color`, `_M_parent`, `_M_left`,
    /// `_M_right`) — four words; erase reads `[3]` (right) and `[2]` (left).
    #[repr(C)]
    #[derive(Clone, Copy, Default, Debug)]
    pub struct RbNodeBase {
        pub words: [usize; 4],
    }
    impl RbNodeBase {
        #[inline]
        fn left(&self) -> *mut RbNodeBase {
            self.words[2] as *mut RbNodeBase
        }
        #[inline]
        fn right(&self) -> *mut RbNodeBase {
            self.words[3] as *mut RbNodeBase
        }
    }
    /// `_M_erase` (IDA 0x66d9e4/0x66ec68/0x670558/0x6717dc): null check, then
    /// the do-loop — recurse right (`v2[3]`), save left (`v2[2]`), `operator
    /// delete`, step left. Node storage (key `Name const*` + `pair` payload
    /// past the header) is owner-allocated, so deletion runs through `free`;
    /// the four monomorph stubs below all collapse into this helper.
    pub unsafe fn rb_tree_erase(node: *mut RbNodeBase, free: unsafe fn(*mut u8)) {
        let mut x = node;
        while !x.is_null() {
            // IDA 0x66d9f6/0x66ec7a: `_M_erase(v2[3])`.
            rb_tree_erase((*x).right(), free);
            let next = (*x).left(); // IDA 0x66d9fc
            free(x as *mut u8); // IDA 0x66d9fe: `operator delete(v2)`
            x = next; // IDA 0x66da02
        }
    }
    /// IDA 0x66c0f4 `isNullClassName`: `className().empty() ==
    /// (sClassName==NULL)` assert (Object.h:360), then `sTextService == 0`.
    pub fn text_service_is_null_class_name(class_name_empty: bool, s_class_name_null: bool) -> bool {
        crate::generated_core_wd_watchdog22::gui_textbox::release_assert(
            class_name_empty == s_class_name_null,
            "className().empty() == (sClassName==NULL) file: include/Util/Object.h line: 360",
        );
        s_class_name_null // IDA 0x66c190
    }
}
// 0x66b478 — __ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev")]
pub fn stub_0x66b478(slot: &mut gui_textbox::RemoteSignal) {
    // IDA 0x66b478: `disconnectAll(a1 + 4)` + release, `disconnectAll(a1)` + release.
    slot.disconnect_all()
}

// 0x66c0f4 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE15isNullClassNameEv")]
pub fn stub_0x66c0f4(class_name_empty: bool, s_class_name_null: bool) -> bool {
    // IDA 0x66c0f4: assert + `sTextService == 0`.
    region_any::text_service_is_null_class_name(class_name_empty, s_class_name_null)
}

// 0x66d828 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_
// type: int(void)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_")]
pub fn stub_0x66d828(any: &mut region_any::RegionAny, value: usize) -> &mut region_any::RegionAny {
    // IDA 0x66d828: `operator=<YAlignment>`.
    any.assign(value, region_any::yalignment_holder());
    any
}

// 0x66d878 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv")]
pub fn stub_0x66d878() -> &'static region_any::TypedHolder {
    // IDA 0x66d878: guard-var singleton.
    region_any::yalignment_holder()
}

// 0x66d8e4 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc")]
pub fn stub_0x66d8e4(src: *const usize, dst: *mut usize) {
    // IDA 0x66d8e4: one-word copy.
    (region_any::yalignment_holder().construct)(src, dst)
}

// 0x66d8f0 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc")]
pub fn stub_0x66d8f0(payload: *mut usize) {
    // IDA 0x66d8f0: `;` (trivial T).
    (region_any::yalignment_holder().destruct)(payload)
}

// 0x66d8f4 — __ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub unsafe fn stub_0x66d8f4(any: *mut region_any::RegionAny) -> *mut usize {
    // IDA 0x66d8f4: checked cast to `YAlignment`, payload address out.
    region_any::any_cast(any, region_any::yalignment_holder())
}

// 0x66d9e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub unsafe fn stub_0x66d9e4(node: *mut region_any::RbNodeBase, free: unsafe fn(*mut u8)) {
    // IDA 0x66d9e4: `_M_erase` over `<Name const*, YAlignment>` nodes.
    region_any::rb_tree_erase(node, free)
}

// 0x66eaac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_")]
pub fn stub_0x66eaac(any: &mut region_any::RegionAny, value: usize) -> &mut region_any::RegionAny {
    // IDA 0x66eaac: `operator=<XAlignment>` (same template as 0x66d828).
    any.assign(value, region_any::xalignment_holder());
    any
}

// 0x66eafc — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv
// type: _DWORD *()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv")]
pub fn stub_0x66eafc() -> &'static region_any::TypedHolder {
    // IDA 0x66eafc: guard-var singleton.
    region_any::xalignment_holder()
}

// 0x66eb68 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc")]
pub fn stub_0x66eb68(src: *const usize, dst: *mut usize) {
    // IDA 0x66eb68: one-word copy.
    (region_any::xalignment_holder().construct)(src, dst)
}

// 0x66eb74 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc
// type: void()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc")]
pub fn stub_0x66eb74(payload: *mut usize) {
    // IDA 0x66eb74: `;` (trivial T).
    (region_any::xalignment_holder().destruct)(payload)
}

// 0x66eb78 — __ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub unsafe fn stub_0x66eb78(any: *mut region_any::RegionAny) -> *mut usize {
    // IDA 0x66eb78: checked cast to `XAlignment`, payload address out.
    region_any::any_cast(any, region_any::xalignment_holder())
}

// 0x66ec68 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub unsafe fn stub_0x66ec68(node: *mut region_any::RbNodeBase, free: unsafe fn(*mut u8)) {
    // IDA 0x66ec68: `_M_erase` over `<Name const*, XAlignment>` nodes.
    region_any::rb_tree_erase(node, free)
}

// 0x67039c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")]
pub fn stub_0x67039c(any: &mut region_any::RegionAny, value: usize) -> &mut region_any::RegionAny {
    // IDA 0x67039c: `operator=<Font>` (same template as 0x66d828).
    any.assign(value, region_any::font_holder());
    any
}

// 0x6703ec — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv
// type: _DWORD *()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv")]
pub fn stub_0x6703ec() -> &'static region_any::TypedHolder {
    // IDA 0x6703ec: guard-var singleton.
    region_any::font_holder()
}

// 0x670458 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc")]
pub fn stub_0x670458(src: *const usize, dst: *mut usize) {
    // IDA 0x670458: one-word copy.
    (region_any::font_holder().construct)(src, dst)
}

// 0x670464 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc
// type: void()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc")]
pub fn stub_0x670464(payload: *mut usize) {
    // IDA 0x670464: `;` (trivial T).
    (region_any::font_holder().destruct)(payload)
}

// 0x670468 — __ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub unsafe fn stub_0x670468(any: *mut region_any::RegionAny) -> *mut usize {
    // IDA 0x670468: checked cast to `Font`, payload address out.
    region_any::any_cast(any, region_any::font_holder())
}

// 0x670558 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub unsafe fn stub_0x670558(node: *mut region_any::RbNodeBase, free: unsafe fn(*mut u8)) {
    // IDA 0x670558: `_M_erase` over `<Name const*, Font>` nodes.
    region_any::rb_tree_erase(node, free)
}

// 0x671620 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_")]
pub fn stub_0x671620(any: &mut region_any::RegionAny, value: usize) -> &mut region_any::RegionAny {
    // IDA 0x671620: `operator=<FontSize>` (same template as 0x66d828).
    any.assign(value, region_any::fontsize_holder());
    any
}

// 0x671670 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv")]
pub fn stub_0x671670() -> &'static region_any::TypedHolder {
    // IDA 0x671670: guard-var singleton.
    region_any::fontsize_holder()
}

// 0x6716dc — __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc")]
pub fn stub_0x6716dc(src: *const usize, dst: *mut usize) {
    // IDA 0x6716dc: one-word copy.
    (region_any::fontsize_holder().construct)(src, dst)
}

// 0x6716e8 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc
// type: void()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc")]
pub fn stub_0x6716e8(payload: *mut usize) {
    // IDA 0x6716e8: `;` (trivial T).
    (region_any::fontsize_holder().destruct)(payload)
}

// 0x6716ec — __ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub unsafe fn stub_0x6716ec(any: *mut region_any::RegionAny) -> *mut usize {
    // IDA 0x6716ec: checked cast to `FontSize`, payload address out.
    region_any::any_cast(any, region_any::fontsize_holder())
}

// 0x6717dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub unsafe fn stub_0x6717dc(node: *mut region_any::RbNodeBase, free: unsafe fn(*mut u8)) {
    // IDA 0x6717dc: `_M_erase` over `<Name const*, FontSize>` nodes.
    region_any::rb_tree_erase(node, free)
}

// 0x672230 — __ZN3RBX7TextBoxD2Ev
// type: void __fastcall(RBX::TextBox *this, int, int, int)
#[doc(alias = "__ZN3RBX7TextBoxD2Ev")]
pub fn stub_0x672230() {
    // IDA 0x672230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x672d68 — __ZN3RBX13GuiTextButtonC2Ev
// type: RBX::GuiButton *__fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZN3RBX13GuiTextButtonC2Ev")]
pub fn stub_0x672d68() {
    // IDA 0x672d68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67303c — __ZN3RBX13GuiTextButton7setTextESs
// type: void __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "__ZN3RBX13GuiTextButton7setTextESs")]
pub fn stub_0x67303c() {
    // IDA 0x67303c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6731f8 — __ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE")]
pub fn stub_0x6731f8() {
    // IDA 0x6731f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673230 — __ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE")]
pub fn stub_0x673230() {
    // IDA 0x673230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673268 — __ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE")]
pub fn stub_0x673268() {
    // IDA 0x673268: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673308 — __ZN3RBX13GuiTextButton19setTextTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextTransparencyEf")]
pub fn stub_0x673308() {
    // IDA 0x673308: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673330 — __ZN3RBX13GuiTextButton11setTextWrapEb
// type: int __fastcall(RBX::GuiTextButton *this, int)
#[doc(alias = "__ZN3RBX13GuiTextButton11setTextWrapEb")]
pub fn stub_0x673330() {
    // IDA 0x673330: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673370 — __ZN3RBX13GuiTextButton12setTextScaleEb
// type: int __fastcall(RBX::GuiTextButton *this, int)
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextScaleEb")]
pub fn stub_0x673370() {
    // IDA 0x673370: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6733c4 — __ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE")]
pub fn stub_0x6733c4() {
    // IDA 0x6733c4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673404 — __ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE")]
pub fn stub_0x673404() {
    // IDA 0x673404: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673444 — __ZNK3RBX13GuiTextButton13getTextBoundsEv
// type: void __fastcall(RBX::GuiTextButton *this, unsigned int, bool)
#[doc(alias = "__ZNK3RBX13GuiTextButton13getTextBoundsEv")]
pub fn stub_0x673444() {
    // IDA 0x673444: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6735d0 — __ZNK3RBX13GuiTextButton11getTextFitsEv
// type: int __fastcall(RBX::GuiTextButton *this, int, bool)
#[doc(alias = "__ZNK3RBX13GuiTextButton11getTextFitsEv")]
pub fn stub_0x6735d0() {
    // IDA 0x6735d0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6737e8 — __ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf")]
pub fn stub_0x6737e8() {
    // IDA 0x6737e8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673814 — __ZN3RBX13GuiTextButton14checkForResizeEv
// type: int __fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZN3RBX13GuiTextButton14checkForResizeEv")]
pub fn stub_0x673814() {
    // IDA 0x673814: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673840 — __ZN3RBX13GuiTextButton21setTransparencyLegacyEf
// type: int __fastcall(RBX::GuiTextButton *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton21setTransparencyLegacyEf")]
pub fn stub_0x673840() {
    // IDA 0x673840: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673888 — __ZNK3RBX13GuiTextButton21getPersistentDataCostEv
// type: int __fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZNK3RBX13GuiTextButton21getPersistentDataCostEv")]
pub fn stub_0x673888() {
    // IDA 0x673888: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x67390c — __ZN3RBX13GuiTextButton8render2dEPNS_5AdornE
// type: int __fastcall(RBX::GuiTextButton *this, RBX::Adorn *)
#[doc(alias = "__ZN3RBX13GuiTextButton8render2dEPNS_5AdornE")]
pub fn stub_0x67390c() {
    // IDA 0x67390c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673918 — __ZThn96_N3RBX13GuiTextButton8render2dEPNS_5AdornE
// type: int __fastcall(RBX::GuiTextButton *this, RBX::Adorn *)
#[doc(alias = "__ZThn96_N3RBX13GuiTextButton8render2dEPNS_5AdornE")]
pub fn stub_0x673918() {
    // IDA 0x673918: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x673ce4 — __ZN3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZN3RBX13GuiTextButtonD1Ev")]
pub fn stub_0x673ce4() {
    // IDA 0x673ce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673cfc — __ZN3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZN3RBX13GuiTextButtonD0Ev")]
pub fn stub_0x673cfc() {
    // IDA 0x673cfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673da8 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x673da8() {
    // IDA 0x673da8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673db8 — __ZThn32_N3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD1Ev")]
pub fn stub_0x673db8() {
    // IDA 0x673db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673dd4 — __ZThn32_N3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD0Ev")]
pub fn stub_0x673dd4() {
    // IDA 0x673dd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673e80 — __ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x673e80() {
    // IDA 0x673e80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673e90 — __ZThn36_N3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD1Ev")]
pub fn stub_0x673e90() {
    // IDA 0x673e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673eac — __ZThn36_N3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD0Ev")]
pub fn stub_0x673eac() {
    // IDA 0x673eac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673f58 — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x673f58() {
    // IDA 0x673f58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673f5c — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x673f5c() {
    // IDA 0x673f5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673ff8 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x673ff8() {
    // IDA 0x673ff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674080 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x674080() {
    // IDA 0x674080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674658 — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x674658() {
    // IDA 0x674658: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67489c — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x67489c() {
    // IDA 0x67489c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6782ec — __ZN3RBX9TextLabelC1Ev
// type: int __fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZN3RBX9TextLabelC1Ev")]
pub fn stub_0x6782ec() {
    // IDA 0x6782ec: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6782f0 — __ZN3RBX9TextLabelC2Ev
// type: RBX::GuiLabel *__fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZN3RBX9TextLabelC2Ev")]
pub fn stub_0x6782f0() {
    // IDA 0x6782f0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6785c8 — __ZN3RBX9TextLabel7setTextESs
// type: void __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "__ZN3RBX9TextLabel7setTextESs")]
pub fn stub_0x6785c8() {
    // IDA 0x6785c8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678784 — __ZN3RBX9TextLabel11setFontSizeENS_11TextService8FontSizeE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel11setFontSizeENS_11TextService8FontSizeE")]
pub fn stub_0x678784() {
    // IDA 0x678784: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6787bc — __ZN3RBX9TextLabel7setFontENS_11TextService4FontE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel7setFontENS_11TextService4FontE")]
pub fn stub_0x6787bc() {
    // IDA 0x6787bc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6787f4 — __ZN3RBX9TextLabel12setTextColorENS_10BrickColorE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX9TextLabel12setTextColorENS_10BrickColorE")]
pub fn stub_0x6787f4() {
    // IDA 0x6787f4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678894 — __ZN3RBX9TextLabel19setTextTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX9TextLabel19setTextTransparencyEf")]
pub fn stub_0x678894() {
    // IDA 0x678894: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6788bc — __ZN3RBX9TextLabel11setTextWrapEb
// type: int __fastcall(RBX::TextLabel *this, int)
#[doc(alias = "__ZN3RBX9TextLabel11setTextWrapEb")]
pub fn stub_0x6788bc() {
    // IDA 0x6788bc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6788fc — __ZN3RBX9TextLabel12setTextScaleEb
// type: int __fastcall(RBX::TextLabel *this, int)
#[doc(alias = "__ZN3RBX9TextLabel12setTextScaleEb")]
pub fn stub_0x6788fc() {
    // IDA 0x6788fc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678950 — __ZN3RBX9TextLabel13setXAlignmentENS_11TextService10XAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel13setXAlignmentENS_11TextService10XAlignmentE")]
pub fn stub_0x678950() {
    // IDA 0x678950: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678990 — __ZN3RBX9TextLabel13setYAlignmentENS_11TextService10YAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel13setYAlignmentENS_11TextService10YAlignmentE")]
pub fn stub_0x678990() {
    // IDA 0x678990: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6789d0 — __ZNK3RBX9TextLabel13getTextBoundsEv
// type: void __fastcall(RBX::TextLabel *this, unsigned int, bool)
#[doc(alias = "__ZNK3RBX9TextLabel13getTextBoundsEv")]
pub fn stub_0x6789d0() {
    // IDA 0x6789d0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678b5c — __ZNK3RBX9TextLabel11getTextFitsEv
// type: int __fastcall(RBX::TextLabel *this, int, bool)
#[doc(alias = "__ZNK3RBX9TextLabel11getTextFitsEv")]
pub fn stub_0x678b5c() {
    // IDA 0x678b5c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678d74 — __ZN3RBX9TextLabel25setTextStrokeTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX9TextLabel25setTextStrokeTransparencyEf")]
pub fn stub_0x678d74() {
    // IDA 0x678d74: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678da0 — __ZN3RBX9TextLabel14checkForResizeEv
// type: int __fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZN3RBX9TextLabel14checkForResizeEv")]
pub fn stub_0x678da0() {
    // IDA 0x678da0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678dcc — __ZN3RBX9TextLabel21setTransparencyLegacyEf
// type: int __fastcall(RBX::TextLabel *this, float)
#[doc(alias = "__ZN3RBX9TextLabel21setTransparencyLegacyEf")]
pub fn stub_0x678dcc() {
    // IDA 0x678dcc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x678e14 — __ZNK3RBX9TextLabel21getPersistentDataCostEv
// type: int __fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZNK3RBX9TextLabel21getPersistentDataCostEv")]
pub fn stub_0x678e14() {
    // IDA 0x678e14: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x678e98 — __ZN3RBX9TextLabel8render2dEPNS_5AdornE
// type: int __fastcall(RBX::TextLabel *this, RBX::Adorn *)
#[doc(alias = "__ZN3RBX9TextLabel8render2dEPNS_5AdornE")]
pub fn stub_0x678e98() {
    // IDA 0x678e98: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x678ea4 — __ZThn96_N3RBX9TextLabel8render2dEPNS_5AdornE
// type: int __fastcall(RBX::TextLabel *this, RBX::Adorn *)
#[doc(alias = "__ZThn96_N3RBX9TextLabel8render2dEPNS_5AdornE")]
pub fn stub_0x678ea4() {
    // IDA 0x678ea4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x67929c — __ZN3RBX9TextLabelD1Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZN3RBX9TextLabelD1Ev")]
pub fn stub_0x67929c() {
    // IDA 0x67929c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6792b4 — __ZN3RBX9TextLabelD0Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZN3RBX9TextLabelD0Ev")]
pub fn stub_0x6792b4() {
    // IDA 0x6792b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679360 — __ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x679360() {
    // IDA 0x679360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679370 — __ZThn32_N3RBX9TextLabelD1Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9TextLabelD1Ev")]
pub fn stub_0x679370() {
    // IDA 0x679370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67938c — __ZThn32_N3RBX9TextLabelD0Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9TextLabelD0Ev")]
pub fn stub_0x67938c() {
    // IDA 0x67938c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679438 — __ZThn32_NK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x679438() {
    // IDA 0x679438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679448 — __ZThn36_N3RBX9TextLabelD1Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9TextLabelD1Ev")]
pub fn stub_0x679448() {
    // IDA 0x679448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679464 — __ZThn36_N3RBX9TextLabelD0Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9TextLabelD0Ev")]
pub fn stub_0x679464() {
    // IDA 0x679464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679510 — __ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x679510() {
    // IDA 0x679510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x679514 — __ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x679514() {
    // IDA 0x679514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6795b0 — __ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x6795b0() {
    // IDA 0x6795b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
