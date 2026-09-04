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

/// Batch 11: 33 IDA-grounded ports 0x672230-0x67489c — `TextBox` D2, the
/// `GuiTextButton` C2 member table, text/font/alignment/transparency setters,
/// `getTextBounds`/`getTextFits`, `checkForResize`, `setTransparencyLegacy`,
/// `getPersistentDataCost`, `render2d` + thunk, the D1/D0/thunk lattice, and
/// the `FactoryProduct<GuiTextButton, GuiButton>` Creator family. Ports live
/// in `gui_textbutton`; `stub_0x*` keeps the `#[doc(alias)]` + `// 0xADDR`
/// carrier lines and wires into it. Creator sequencing reuses
/// `gui_textbox::creator_*_as` with `GUITEXTBUTTON_CLASS`.
/// Conventions: foreign storage (std::string members, service objects,
/// vtables) is touched through caller-supplied hooks; scalar field
/// compare-stores, descriptor raises (symbolic `unk_*` names — addresses live
/// in the target binary), and control flow are core-owned and 1:1.
/// `[INFERENCE]` marks what the binary does not pin down.
pub mod gui_textbutton {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use super::gui_textbox;
    /// was: `FactoryProduct<GuiTextButton, GuiButton>::Creator` class key.
    pub const GUITEXTBUTTON_CLASS: gui_textbox::CreatorClass = gui_textbox::CreatorClass {
        name: "GuiTextButton", // `sGuiTextButton`
        vtab: "off_128E43C",  // IDA 0x67468e / 0x673f7e (`a1->__sig`)
    };
    static CONSTRUCTED: AtomicBool = AtomicBool::new(false);
    static CREATOR_ADDR: AtomicUsize = AtomicUsize::new(0);
    /// IDA 0x674658 Creator C2 (same template as TextBox 0x6696b4).
    pub unsafe fn creator_construct(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
        gui_textbox::creator_construct_as(slot, &GUITEXTBUTTON_CLASS, &CONSTRUCTED, &CREATOR_ADDR)
    }
    /// IDA 0x673f5c Creator D2 (same template as TextBox 0x668fb8).
    pub unsafe fn creator_destroy(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
        gui_textbox::creator_destroy_as(slot, &GUITEXTBUTTON_CLASS, &CONSTRUCTED)
    }
    /// IDA 0x67489c `static_getCreator` (same template as TextBox 0x6698f8).
    pub fn static_get_creator() -> *const gui_textbox::Creator {
        gui_textbox::static_get_creator_as(&CREATOR_ADDR, &CONSTRUCTED)
    }
    /// Shared `getClassName` tail (IDA 0x673dac + shim, like 0x668cf4).
    pub fn creator_class_name() -> &'static str {
        static_get_creator();
        GUITEXTBUTTON_CLASS.name
    }
    /// `create` out-pair shares `TextBoxShared`'s shape (`+32`
    /// Instance-subobject + moved count, IDA 0x674138-0x674168).
    pub unsafe fn guitextbutton_create(
        out: *mut gui_textbox::TextBoxShared,
        alloc: unsafe fn() -> (*mut u8, crate::SharedPtr<u8>),
    ) {
        gui_textbox::release_assert(
            CONSTRUCTED.load(Ordering::SeqCst),
            "wasConstructed() file: include/Util/Object.h line: 231",
        );
        let (obj, ownership) = alloc(); // IDA 0x674138
        let instance = obj.wrapping_add(32); // IDA 0x67414c (`v16 + 32`)
        out.write(gui_textbox::TextBoxShared { instance_ptr: instance, ownership: Some(ownership) });
    }
    /// IDA 0x67225e-0x672298: the seven vtable installs of `TextBox::~TextBox`.
    pub const TEXTBOX_VTABLE_SLOTS: [(usize, &str); 7] = [
        (0, "off_11EA788"),   // IDA 0x67225e: `*this`
        (3, "off_11EA858"),   // IDA 0x672266
        (8, "off_11EA864"),   // IDA 0x67226e
        (9, "off_11EA878"),   // IDA 0x672276
        (23, "off_11EA890"),  // IDA 0x67227e
        (24, "off_11EA89C"),  // IDA 0x67228a
        (149, "off_11EA8D4"), // IDA 0x672298
    ];
    /// Member teardown of `TextBox::~TextBox` (IDA 0x6722cc-0x672328).
    /// `v7` in the decompile is `this` in word units (`v7 + 149` = +596,
    /// `v7 + 152` = +608, `v7 + 135` = +540, `v7 + 164` = +656).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TextBoxMember {
        BoolSignal,
        Connection,
        TextA,
        Heartbeat,
        TextB,
        GuiObject,
    }
    /// Datamodel-supplied teardown for `TextBox` members. `BoolSignal` covers
    /// `disconnectAll(this + 660)` + the conditional `intrusive_ptr_release`
    /// (IDA 0x6722cc-0x6722da); `Connection` covers
    /// `connection::disconnect(this + 656)` + the conditional weak release
    /// (IDA 0x6722ec-0x6722fa).
    pub struct TextBoxD2Fini {
        pub member: unsafe fn(*mut u8, TextBoxMember),
    }
    /// IDA 0x672230 `RBX::TextBox::~TextBox` (D2): vtable installs are
    /// symbolic (`TEXTBOX_VTABLE_SLOTS`); the member walk and base order are
    /// real: bool-signal (660), connection (656), string (608), heartbeat
    /// (596), string (540), `GuiObject` base.
    pub unsafe fn textbox_d2(this: *mut u8, fini: &TextBoxD2Fini) {
        (fini.member)(this.add(660), TextBoxMember::BoolSignal); // IDA 0x6722cc
        (fini.member)(this.add(656), TextBoxMember::Connection); // IDA 0x6722ec
        (fini.member)(this.add(608), TextBoxMember::TextA); // IDA 0x672304
        (fini.member)(this.add(596), TextBoxMember::Heartbeat); // IDA 0x672312
        (fini.member)(this.add(540), TextBoxMember::TextB); // IDA 0x67231c
        (fini.member)(this, TextBoxMember::GuiObject); // IDA 0x672328
    }
    /// `GuiTextButton` field layout from C2 (0x672d68) + setters.
    pub const TEXT_WORD: usize = 201; // std::string at +804 (`setText`)
    pub const LEGACY0_WORD: usize = 200; // cleared on text commit (0x67311e)
    pub const FONTSIZE_WORD: usize = 202; // `setFontSize`
    pub const COLOR3_WORDS: (usize, usize) = (203, 204); // `Color3` bits
    pub const TEXT_TRANSP_FLOAT: usize = 206; // `setTextTransparency` (float-indexed)
    pub const STROKE_TRANSP_FLOAT: usize = 210; // `setTextStrokeTransparency`
    pub const TEXTWRAP_BYTE: usize = 844; // `setTextWrap`
    pub const TEXTSCALE_BYTE: usize = 845; // `setTextScale`
    pub const XA_WORD: usize = 212; // `setXAlignment`
    pub const YA_WORD: usize = 213; // `setYAlignment`
    pub const FONT_WORD: usize = 214; // `setFont`
    /// IDA 0x67390c: `(*(this + 196))(this, a2, 0)` — word 49 of the vtable.
    pub const RENDER2D_VTAB_WORD: usize = 49;
    /// Property descriptors raised by the setters (symbolic: addresses live
    /// in the target binary).
    pub const DESC_TEXT: &str = "unk_13275F0"; // IDA 0x67312e (`setText`)
    pub const DESC_TEXT2: &str = "unk_1327724"; // IDA 0x67313c + most setters
    pub const DESC_TEXT3: &str = "unk_1327750"; // IDA 0x67314a + most setters
    pub const DESC_FONTSIZE: &str = "unk_13277D4"; // IDA 0x67321c
    pub const DESC_FONT: &str = "unk_1327808"; // IDA 0x673254
    pub const DESC_TRANSP: &str = "unk_1327674"; // IDA 0x67332c / 0x673870
    pub const DESC_WRAP: &str = "unk_13276A0"; // IDA 0x673354
    pub const DESC_SCALE: &str = "unk_13276F8"; // IDA 0x673396
    pub const DESC_XA: &str = "unk_132783C"; // IDA 0x6733e8
    pub const DESC_YA: &str = "unk_1327870"; // IDA 0x673428
    pub const DESC_STROKE: &str = "unk_13277A8"; // IDA 0x67380e
    /// IDA 0x672dc4/0x672e44/0x672f32: the three six-word vtable rounds of
    /// `GuiTextButton::GuiTextButton` (post-base, post-describe, final).
    pub const CONSTRUCT_VTABLE_ROUNDS: [[(usize, &str); 6]; 3] = [
        [
            (0, "off_128EC28"),
            (3, "off_128ECF4"),
            (8, "off_128ED00"),
            (9, "off_128ED14"),
            (23, "off_128ED2C"),
            (24, "off_128ED38"),
        ],
        [
            (0, "off_128EAD8"),
            (3, "off_128EBA4"),
            (8, "off_128EBB0"),
            (9, "off_128EBC4"),
            (23, "off_128EBDC"),
            (24, "off_128EBE8"),
        ],
        [
            (0, "off_11EA928"),
            (3, "off_11EA9F8"),
            (8, "off_11EAA04"),
            (9, "off_11EAA18"),
            (23, "off_11EAA30"),
            (24, "off_11EAA3C"),
        ],
    ];
    /// Datamodel-supplied construction hooks shared by the `GuiTextButton`
    /// (0x672d68) and `TextLabel` (0x6782f0) C2s.
    pub struct GuiTextButtonCtor {
        /// `GuiButton::GuiButton` / `GuiLabel::GuiLabel` base with the class
        /// tag (`"TextButton"` 0x672d92 / `"TextLabel"` 0x67831a).
        pub base: unsafe fn(*mut u8),
        /// `classDescriptor` + word-12 install + registrar++ (IDA
        /// 0x672e0c-0x672e3e / 0x678394-0x6783c6).
        pub describe: unsafe fn(*mut u8),
        /// std::string assign at the text offset (owns the temp refcount
        /// dance, 0x672f1a-0x672f9e / 0x6784a2-0x67852c).
        pub set_text: unsafe fn(*mut u8, &str),
        /// `BrickColor::color3` default feeding the color words (IDA
        /// 0x672e90 / 0x678418).
        pub default_text_color: fn() -> [f32; 3],
    }
    /// Per-class member layout of the text-widget C2s (`GuiTextButton` from
    /// 0x672d68, `TextLabel` from 0x6782f0).
    pub struct GuiTextLayout {
        pub label: &'static str,
        pub text_byte: usize,
        pub legacy_word: usize,
        pub fontsize_word: usize,
        pub color_words: (usize, usize),
        pub transp_float: usize,
        pub zero_qword_byte: usize,
        pub stroke_float: usize,
        pub wrap_byte: usize,
        pub scale_byte: usize,
        pub xa_word: usize,
        pub ya_word: usize,
        pub font_word: usize,
        /// Extra trailing store (`TextLabel` word-35 `= 1`, IDA 0x6784e6).
        pub trailing_word: Option<(usize, u32)>,
    }
    pub const GUITEXTBUTTON_LAYOUT: GuiTextLayout = GuiTextLayout {
        label: "Button", // IDA 0x672e80
        text_byte: 804,
        legacy_word: LEGACY0_WORD,
        fontsize_word: FONTSIZE_WORD,
        color_words: COLOR3_WORDS,
        transp_float: 205,
        zero_qword_byte: 824,
        stroke_float: STROKE_TRANSP_FLOAT,
        wrap_byte: TEXTWRAP_BYTE,
        scale_byte: TEXTSCALE_BYTE,
        xa_word: XA_WORD,
        ya_word: YA_WORD,
        font_word: FONT_WORD,
        trailing_word: None,
    };
    /// IDA 0x6782f0 `TextLabel::TextLabel` layout: text at +540 (word 135),
    /// legacy word 134, fontsize 136, color 137-138, transp float 139,
    /// zero box at 560, stroke float 144, wrap/scale 580-581, XA/YA/Font
    /// 146-148, plus word-35 `= 1` (the `GuiQueue` default — IDA 0x6784e6;
    /// `GuiBase2d::getGuiQueue` reads word 35).
    pub const GUITEXTLABEL_LAYOUT: GuiTextLayout = GuiTextLayout {
        label: "Label", // IDA 0x678408
        text_byte: 540,
        legacy_word: 134,
        fontsize_word: 136,
        color_words: (137, 138),
        transp_float: 139,
        zero_qword_byte: 560,
        stroke_float: 144,
        wrap_byte: 580,
        scale_byte: 581,
        xa_word: 146,
        ya_word: 147,
        font_word: 148,
        trailing_word: Some((35, 1)),
    };
    /// Shared C2 body (IDA 0x672d68 / 0x6782f0): base, vtable rounds
    /// (symbolic), describe, the label text, and the scalar member table —
    /// all stores real.
    pub unsafe fn guitext_construct(this: *mut u8, layout: &GuiTextLayout, ctor: &GuiTextButtonCtor) {
        (ctor.base)(this);
        (ctor.describe)(this);
        let words = this as *mut u32;
        words.add(layout.legacy_word).write(0);
        (ctor.set_text)(this.add(layout.text_byte), layout.label);
        words.add(layout.fontsize_word).write(0);
        let c = (ctor.default_text_color)();
        words.add(layout.color_words.0).write(c[0].to_bits());
        words.add(layout.color_words.1).write(c[1].to_bits());
        (this as *mut f32).add(layout.transp_float).write(c[2]);
        (this.add(layout.zero_qword_byte) as *mut u64).write_bytes(0, 2);
        (this as *mut f32).add(layout.stroke_float).write(1.0);
        this.add(layout.wrap_byte).write(0u8);
        this.add(layout.scale_byte).write(0u8);
        words.add(layout.xa_word).write(2);
        words.add(layout.ya_word).write(1);
        words.add(layout.font_word).write(0);
        if let Some((w, v)) = layout.trailing_word {
            words.add(w).write(v);
        }
    }
    /// IDA 0x672d68 C2 for `GuiTextButton` (wraps the shared body).
    pub unsafe fn guitextbutton_construct(this: *mut u8, ctor: &GuiTextButtonCtor) {
        guitext_construct(this, &GUITEXTBUTTON_LAYOUT, ctor)
    }
    /// IDA 0x6782f0 C2 for `TextLabel` (wraps the shared body).
    pub unsafe fn guitextlabel_construct(this: *mut u8, ctor: &GuiTextButtonCtor) {
        guitext_construct(this, &GUITEXTLABEL_LAYOUT, ctor)
    }
    /// `raisePropertyChanged` hook: descriptor crosses as its symbolic name.
    pub type RaiseHook = unsafe fn(*mut u8, &'static str) -> i32;
    /// Word compare-store with descriptor raises (IDA 0x6731f8/0x673230/
    /// 0x6733c4/0x673404 shape): returns the last raise on change, the old
    /// value otherwise.
    pub unsafe fn set_word(
        this: *mut u8,
        word: usize,
        value: u32,
        descs: &[&'static str],
        raise: RaiseHook,
    ) -> u32 {
        let slot = (this as *mut u32).add(word);
        let cur = slot.read();
        if cur != value {
            slot.write(value);
            let mut out = 0;
            for d in descs {
                out = raise(this, d);
            }
            return out as u32;
        }
        cur
    }
    /// Float compare-store with one raise (IDA 0x673308/0x6737e8 shape).
    pub unsafe fn set_float(
        this: *mut f32,
        idx: usize,
        value: f32,
        desc: &'static str,
        raise: unsafe fn(*mut u8, &'static str) -> *mut u8,
    ) -> *mut f32 {
        if this.add(idx).read() != value {
            this.add(idx).write(value);
            return raise(this as *mut u8, desc) as *mut f32;
        }
        this
    }
    /// Byte compare-store with descriptor raises (IDA 0x673330/0x673370 shape).
    pub unsafe fn set_byte(
        this: *mut u8,
        byte: usize,
        value: i32,
        descs: &[&'static str],
        raise: RaiseHook,
    ) -> i32 {
        let cur = this.add(byte).read() as i32;
        if cur != value {
            this.add(byte).write(value as u8);
            let mut out = 0;
            for d in descs {
                out = raise(this, d);
            }
            return out;
        }
        cur
    }
    /// Datamodel-supplied text pipeline for `setText` (IDA 0x67303c).
    pub struct GuiTextSvc {
        /// `ProfanityFilter::ContainsProfanity` (IDA 0x6730f2).
        pub contains_profanity: fn(&str) -> bool,
        /// `*(_BYTE *)(fw(this) + 22)` fast path (IDA 0x6730f2).
        pub forwarded: unsafe fn(*mut u8) -> bool,
        /// Current text at +804 for the compare (IDA 0x6730fe).
        pub current_text: unsafe fn(*mut u8) -> String,
        /// Commit: assign at the text offset + legacy-word clear (IDA
        /// 0x673110-0x67311e / 0x67869c-0x6786aa).
        pub commit_text: unsafe fn(*mut u8, &str),
        pub raise: RaiseHook,
    }
    /// Descriptor triple raised by the text commit (`setText` 0x67303c /
    /// 0x6785c8: text, text2, text3).
    pub struct TextDescs {
        pub text: &'static str,
        pub text2: &'static str,
        pub text3: &'static str,
    }
    pub const BUTTON_DESCS: TextDescs = TextDescs {
        text: DESC_TEXT,
        text2: DESC_TEXT2,
        text3: DESC_TEXT3,
    };
    pub const LABEL_DESCS: TextDescs = TextDescs {
        text: LDESC_TEXT,
        text2: LDESC_TEXT2,
        text3: LDESC_TEXT3,
    };
    /// Shared `setText` body (IDA 0x67303c / 0x6785c8): over-long inputs
    /// (`length > 0x400`) pass through a normalize copy with no observable
    /// change; the profanity/forwarded gate, the compare, the commit, and
    /// the three raises are real.
    pub unsafe fn guitext_set_text(
        this: *mut u8,
        text: &str,
        svc: &GuiTextSvc,
        descs: &TextDescs,
    ) {
        let _normalize_copy = text.len() > 0x400; // IDA 0x67309c / 0x678628
        if (svc.contains_profanity)(text) && !(svc.forwarded)(this) {
            return; // IDA 0x6730f2 / 0x67867e
        }
        if (svc.current_text)(this) != text {
            // IDA 0x6730fe / 0x67868a
            (svc.commit_text)(this, text); // IDA 0x673110-0x67311e
            (svc.raise)(this, descs.text);
            (svc.raise)(this, descs.text2);
            (svc.raise)(this, descs.text3);
        }
    }
    /// IDA 0x67303c `setText` for `GuiTextButton` (wraps the shared body).
    pub unsafe fn guitextbutton_set_text(this: *mut u8, text: &str, svc: &GuiTextSvc) {
        guitext_set_text(this, text, svc, &BUTTON_DESCS)
    }
    /// IDA 0x6785c8 `setText` for `TextLabel` (wraps the shared body).
    pub unsafe fn guitextlabel_set_text(this: *mut u8, text: &str, svc: &GuiTextSvc) {
        guitext_set_text(this, text, svc, &LABEL_DESCS)
    }
    /// (`v3 = len/0x64 > 1 ? len/0x64 : 1`, 0x6738ae-0x6738cc; the string
    /// copy/destroy dance has no observable effect).
    #[inline]
    pub fn textbutton_persistent_cost(base: i32, text_len: usize) -> i32 {
        base + (text_len / 100).max(1) as i32 + gui_textbox::GUIOBJECT_PERSISTENT_COST_BONUS
    }
    /// Datamodel-supplied text measurement for `getTextBounds`/`getTextFits`.
    pub struct TextMeasureSvc {
        /// `Players::frontendProcessing` gate (IDA 0x6734ac/0x673632).
        pub frontend_processing: fn() -> bool,
        /// `ServiceProvider::create<TextService> != null` (IDA 0x6734ac/0x673632).
        pub text_service: fn() -> bool,
        /// `TextService::getTypesetter(font)` — `None` is the null typesetter.
        pub get_typesetter: fn(font: u32) -> bool,
        /// `GuiObject::convertFontSize` (IDA 0x6734d2/0x67365e).
        pub convert_font_size: fn(u32) -> u32,
        /// `GuiBase2d::getRect2D` for the wrap-avail box (IDA 0x6734ee/0x67367a).
        pub get_rect: unsafe fn(*mut u8) -> gui_textbox::Rect2d,
        /// Typesetter layout: `(text, converted size, avail) -> bounds`
        /// (IDA 0x67357c).
        pub layout: fn(&str, u32, [f32; 2]) -> [f32; 2],
        /// Typesetter measure with fits flag (IDA 0x6736d0): `None` is
        /// `v32[0] == 0`, `Some(w)` carries the candidate width `v31[0]`.
        pub measure_fits: fn(&str, u32, [f32; 2]) -> Option<f32>,
        /// `sp_counted_base::release` pairing the typesetter (IDA 0x673522/0x67358a).
        pub release: fn(),
    }
    /// IDA 0x673444 `getTextBounds` (the `this` slot is the hidden
    /// by-value return slot; the button arrives as the second word):
    /// service gates with zero-vector fallback (LABEL_8), wrap-gated avail
    /// box from `getRect2D`, typesetter layout, release pairing.
    pub unsafe fn guitextbutton_text_bounds(
        button: *mut u8,
        text: &str,
        wrap: bool,
        font_size: u32,
        font: u32,
        svc: &TextMeasureSvc,
    ) -> [f32; 2] {
        if !(svc.frontend_processing)() || !(svc.text_service)() {
            return [0.0, 0.0]; // IDA 0x673526-0x673536 (LABEL_8)
        }
        if !(svc.get_typesetter)(font) {
            // IDA 0x6734bc-0x673522
            (svc.release)();
            return [0.0, 0.0];
        }
        let avail = if wrap {
            // IDA 0x6734e2-0x67350e: rect dims.
            let r = (svc.get_rect)(button);
            [r.max[0] - r.min[0], r.max[1] - r.min[1]]
        } else {
            [0.0, 0.0] // IDA 0x673558-0x673562
        };
        let out = (svc.layout)(text, (svc.convert_font_size)(font_size), avail); // IDA 0x67357c
        (svc.release)(); // IDA 0x673582-0x67358a
        out
    }
    /// IDA 0x6735d0 `getTextFits`: same gates (failure is `0`, 0x6736a4);
    /// without the fits flag the answer is `0` (0x673706); otherwise the
    /// measured width must beat the rect width (0x6736d8-0x673702).
    pub unsafe fn guitextbutton_text_fits(
        button: *mut u8,
        text: &str,
        wrap: bool,
        font_size: u32,
        font: u32,
        svc: &TextMeasureSvc,
    ) -> bool {
        if !(svc.frontend_processing)() || !(svc.text_service)() {
            return false; // IDA 0x6736a4
        }
        if !(svc.get_typesetter)(font) {
            (svc.release)();
            return false; // `v18 == 0` path (0x67371c)
        }
        let avail = if wrap {
            // IDA 0x67366e-0x67369a: rect dims.
            let r = (svc.get_rect)(button);
            [r.max[0] - r.min[0], r.max[1] - r.min[1]]
        } else {
            [0.0, 0.0] // IDA 0x6736ac-0x6736b6
        };
        let fits = match (svc.measure_fits)(text, (svc.convert_font_size)(font_size), avail) {
            // IDA 0x6736d0
            Some(w) => {
                let r = (svc.get_rect)(button); // IDA 0x6736e6
                w < r.max[0] - r.min[0] // IDA 0x6736ea-0x673702
            }
            None => false, // IDA 0x673706
        };
        (svc.release)(); // IDA 0x67370e-0x673716
        fits
    }

    /// Teardown for `RBX::GuiTextButton` storage (IDA 0x673ce4-0x673d56):
    /// string at +804, then the `GuiButton` base; `free` is the `operator
    /// delete` half of D0.
    pub struct GuiButtonFini {
        pub drop_text: unsafe fn(*mut u8),
        pub base: unsafe fn(*mut u8),
        pub free: unsafe fn(*mut u8),
    }
    /// IDA 0x673ce4 D1: `~string(this + 804)` then `~GuiButton(this)`.
    pub unsafe fn guibutton_d1(this: *mut u8, fini: &GuiButtonFini) {
        (fini.drop_text)(this.add(804)); // IDA 0x673cee
        (fini.base)(this); // IDA 0x673cf8
    }
    /// IDA 0x673cfc D0: D1 then `operator delete`.
    pub unsafe fn guibutton_d0(this: *mut u8, fini: &GuiButtonFini) {
        guibutton_d1(this, fini); // IDA 0x673d20-0x673d56
        (fini.free)(this);
    }
    /// IDA 0x67390c `render2d`: virtual dispatch through vtable word 49
    /// (`(*(this + 196))(this, a2, 0)`), tail arg `0`.
    pub unsafe fn guibutton_render2d(this: *mut u8, adorn: *mut u8) -> i32 {
        let vtab = *(this as *mut *mut usize);
        let slot = *vtab.add(RENDER2D_VTAB_WORD);
        let f: unsafe fn(*mut u8, *mut u8, u32) -> i32 = std::mem::transmute::<usize, unsafe fn(*mut u8, *mut u8, u32) -> i32>(slot);
        f(this, adorn, 0)
    }
    /// IDA 0x673918 thunk: `this - 96` adjust, vtable read off the adjusted
    /// base (`*((this - 96) + 196)`), same `(adj, a2, 0)` call shape.
    pub unsafe fn guibutton_render2d_thunk(this: *mut u8, adorn: *mut u8) -> i32 {
        let adj = this.sub(96); // IDA `(char *)this - 96`
        let vtab = *(adj as *mut *mut usize);
        let slot = *vtab.add(RENDER2D_VTAB_WORD);
        let f: unsafe fn(*mut u8, *mut u8, u32) -> i32 = std::mem::transmute::<usize, unsafe fn(*mut u8, *mut u8, u32) -> i32>(slot);
        f(adj, adorn, 0)
    }
    /// IDA 0x67834c/0x6783cc/0x6784ba: the three six-word vtable rounds of
    /// `TextLabel::TextLabel` (post-base, post-describe, final).
    pub const TEXTLABEL_VTABLE_ROUNDS: [[(usize, &str); 6]; 3] = [
        [
            (0, "off_128F558"),
            (3, "off_128F624"),
            (8, "off_128F630"),
            (9, "off_128F644"),
            (23, "off_128F65C"),
            (24, "off_128F668"),
        ],
        [
            (0, "off_128F408"),
            (3, "off_128F4D4"),
            (8, "off_128F4E0"),
            (9, "off_128F4F4"),
            (23, "off_128F50C"),
            (24, "off_128F518"),
        ],
        [
            (0, "off_11EAAA8"),
            (3, "off_11EAB78"),
            (8, "off_11EAB84"),
            (9, "off_11EAB98"),
            (23, "off_11EABB0"),
            (24, "off_11EABBC"),
        ],
    ];
    /// Property descriptors raised by the `TextLabel` setters (symbolic).
    pub const LDESC_TEXT: &str = "unk_13278C0"; // IDA 0x6786ba (`setText`)
    pub const LDESC_TEXT2: &str = "unk_13279F4"; // IDA 0x6786c8 + most setters
    pub const LDESC_TEXT3: &str = "unk_1327A20"; // IDA 0x6786d6 + most setters
    pub const LDESC_FONTSIZE: &str = "unk_1327AA4"; // IDA 0x6787a8
    pub const LDESC_FONT: &str = "unk_1327AD8"; // IDA 0x6787e0
    pub const LDESC_TRANSP: &str = "unk_1327944"; // IDA 0x6788b8 / 0x678dfc
    pub const LDESC_WRAP: &str = "unk_1327970"; // IDA 0x6788e0
    pub const LDESC_SCALE: &str = "unk_13279C8"; // IDA 0x678922
    pub const LDESC_XA: &str = "unk_1327B0C"; // IDA 0x678974
    pub const LDESC_YA: &str = "unk_1327B40"; // IDA 0x6789b4
    pub const LDESC_STROKE: &str = "unk_1327A78"; // IDA 0x678d9a
    /// `TextLabel` field layout (mirrors `GUITEXTLABEL_LAYOUT` for the
    /// setter helpers).
    pub const LTEXT_WORD: usize = 135; // std::string at +540
    pub const LLEGACY_WORD: usize = 134;
    pub const LFONTSIZE_WORD: usize = 136;
    pub const LTRANSP_FLOAT: usize = 140;
    pub const LSTROKE_FLOAT: usize = 144;
    pub const LWRAP_BYTE: usize = 580;
    pub const LSCALE_BYTE: usize = 581;
    pub const LXA_WORD: usize = 146;
    pub const LYA_WORD: usize = 147;
    pub const LFONT_WORD: usize = 148;
    /// was: `FactoryProduct<TextLabel, GuiLabel>::Creator` class key.
    pub const GUITEXTLABEL_CLASS: gui_textbox::CreatorClass = gui_textbox::CreatorClass {
        name: "TextLabel", // `sTextLabel`
        vtab: "off_128EDAC", // IDA 0x679536 (`*a1 = &off_...`)
    };
    static LCONSTRUCTED: AtomicBool = AtomicBool::new(false);
    static LCREATOR_ADDR: AtomicUsize = AtomicUsize::new(0);
    /// IDA 0x679514 Creator D2 (same template as 0x673f5c).
    pub unsafe fn label_creator_destroy(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
        gui_textbox::creator_destroy_as(slot, &GUITEXTLABEL_CLASS, &LCONSTRUCTED)
    }
    /// Shared `getClassName` tail (IDA 0x679360 + shim, like 0x673da8).
    pub fn label_creator_class_name() -> &'static str {
        gui_textbox::static_get_creator_as(&LCREATOR_ADDR, &LCONSTRUCTED);
        GUITEXTLABEL_CLASS.name
    }
    /// D1 never frees; placeholder for the `GuiObjectFini.free` slot.
    unsafe fn no_free(_: *mut u8) {}
    /// IDA 0x67929c D1: `~string(this + 540)` then `~GuiObject(this)` —
    /// note the decompile skips any `GuiLabel` middle (no members there).
    pub unsafe fn textlabel_d1(
        this: *mut u8,
        drop_text: unsafe fn(*mut u8),
        member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
    ) {
        drop_text(this.add(540)); // IDA 0x6792a6
        gui_textbox::gui_object_d2(this, &gui_textbox::GuiObjectFini { member, free: no_free });
    }
    /// IDA 0x6792b4 D0: D1 then `operator delete`.
    pub unsafe fn textlabel_d0(
        this: *mut u8,
        drop_text: unsafe fn(*mut u8),
        member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
        free: unsafe fn(*mut u8),
    ) {
        textlabel_d1(this, drop_text, member); // IDA 0x6792d8-0x67930e
        free(this);
    }
}
// 0x672230 — __ZN3RBX7TextBoxD2Ev
// type: void __fastcall(RBX::TextBox *this, int, int, int)
#[doc(alias = "__ZN3RBX7TextBoxD2Ev")]
pub unsafe fn stub_0x672230(this: *mut u8, fini: &gui_textbutton::TextBoxD2Fini) {
    // IDA 0x672230: seven vtable installs + member walk + `GuiObject` base.
    gui_textbutton::textbox_d2(this, fini)
}

// 0x672d68 — __ZN3RBX13GuiTextButtonC2Ev
// type: RBX::GuiButton *__fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZN3RBX13GuiTextButtonC2Ev")]
pub unsafe fn stub_0x672d68(this: *mut u8, ctor: &gui_textbutton::GuiTextButtonCtor) -> *mut u8 {
    // IDA 0x672d68: base + vtable rounds + describe + `"Button"` + member table.
    gui_textbutton::guitextbutton_construct(this, ctor);
    this // IDA 0x672f7a
}

// 0x67303c — __ZN3RBX13GuiTextButton7setTextESs
// type: void __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "__ZN3RBX13GuiTextButton7setTextESs")]
pub unsafe fn stub_0x67303c(this: *mut u8, text: &str, svc: &gui_textbutton::GuiTextSvc) {
    // IDA 0x67303c: normalize + profanity gate + compare + commit + 3 raises.
    gui_textbutton::guitextbutton_set_text(this, text, svc)
}

// 0x6731f8 — __ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE")]
pub unsafe fn stub_0x6731f8(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x6731f8: word-202 compare-store + raises (unk_13277D4, unk_1327724).
    gui_textbutton::set_word(
        this,
        gui_textbutton::FONTSIZE_WORD,
        value,
        &[gui_textbutton::DESC_FONTSIZE, gui_textbutton::DESC_TEXT2],
        raise,
    )
}

// 0x673230 — __ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE")]
pub unsafe fn stub_0x673230(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x673230: word-214 compare-store + raises (unk_1327808, unk_1327724).
    gui_textbutton::set_word(
        this,
        gui_textbutton::FONT_WORD,
        value,
        &[gui_textbutton::DESC_FONT, gui_textbutton::DESC_TEXT2],
        raise,
    )
}

// 0x673268 — __ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE")]
pub unsafe fn stub_0x673268(
    this: *mut u8,
    packed: u32,
    color3: fn(u32) -> [f32; 3],
    set_color3: unsafe fn(*mut u8, [f32; 3]) -> i32,
) -> i32 {
    // IDA 0x673268: `v4[3] = a2` pack (0x67326e), `BrickColor::color3`
    // (0x673278), forward to `setTextColor3` (0x673286).
    set_color3(this, color3(packed))
}

// 0x673308 — __ZN3RBX13GuiTextButton19setTextTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextTransparencyEf")]
pub unsafe fn stub_0x673308(
    this: *mut f32,
    value: f32,
    raise: unsafe fn(*mut u8, &'static str) -> *mut u8,
) -> *mut f32 {
    // IDA 0x673308: float-206 compare-store + raise (unk_1327674).
    gui_textbutton::set_float(this, gui_textbutton::TEXT_TRANSP_FLOAT, value, gui_textbutton::DESC_TRANSP, raise)
}

// 0x673330 — __ZN3RBX13GuiTextButton11setTextWrapEb
// type: int __fastcall(RBX::GuiTextButton *this, int)
#[doc(alias = "__ZN3RBX13GuiTextButton11setTextWrapEb")]
pub unsafe fn stub_0x673330(
    this: *mut u8,
    value: i32,
    raise: gui_textbutton::RaiseHook,
) -> i32 {
    // IDA 0x673330: byte-844 compare-store + 3 raises.
    gui_textbutton::set_byte(
        this,
        gui_textbutton::TEXTWRAP_BYTE,
        value,
        &[gui_textbutton::DESC_WRAP, gui_textbutton::DESC_TEXT2, gui_textbutton::DESC_TEXT3],
        raise,
    )
}

// 0x673370 — __ZN3RBX13GuiTextButton12setTextScaleEb
// type: int __fastcall(RBX::GuiTextButton *this, int)
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextScaleEb")]
pub unsafe fn stub_0x673370(
    this: *mut u8,
    value: i32,
    raise: gui_textbutton::RaiseHook,
) -> i32 {
    // IDA 0x673370: byte-845 compare (0x67337e); on change store (0x67338c) +
    // raise (0x673396), then `setTextWrap(this, 1)` when enabling (0x67339c)
    // else the two text raises (0x6733b0-0x6733be).
    let slot = this.add(gui_textbutton::TEXTSCALE_BYTE);
    if slot.read() as i32 != value {
        slot.write(value as u8);
        raise(this, gui_textbutton::DESC_SCALE);
        if value == 1 {
            return stub_0x673330(this, 1, raise);
        }
        raise(this, gui_textbutton::DESC_TEXT2);
        return raise(this, gui_textbutton::DESC_TEXT3);
    }
    slot.read() as i32
}

// 0x6733c4 — __ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE")]
pub unsafe fn stub_0x6733c4(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x6733c4: word-212 compare-store + 3 raises.
    gui_textbutton::set_word(
        this,
        gui_textbutton::XA_WORD,
        value,
        &[gui_textbutton::DESC_XA, gui_textbutton::DESC_TEXT2, gui_textbutton::DESC_TEXT3],
        raise,
    )
}

// 0x673404 — __ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE")]
pub unsafe fn stub_0x673404(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x673404: word-213 compare-store + 3 raises.
    gui_textbutton::set_word(
        this,
        gui_textbutton::YA_WORD,
        value,
        &[gui_textbutton::DESC_YA, gui_textbutton::DESC_TEXT2, gui_textbutton::DESC_TEXT3],
        raise,
    )
}

// 0x673444 — __ZNK3RBX13GuiTextButton13getTextBoundsEv
// type: void __fastcall(RBX::GuiTextButton *this, unsigned int, bool)
#[doc(alias = "__ZNK3RBX13GuiTextButton13getTextBoundsEv")]
pub unsafe fn stub_0x673444(
    button: *mut u8,
    text: &str,
    wrap: bool,
    font_size: u32,
    font: u32,
    svc: &gui_textbutton::TextMeasureSvc,
) -> [f32; 2] {
    // IDA 0x673444: gates + wrap avail + layout + release. (`this` is the
    // hidden by-value return slot; the button arrives as the second word.)
    gui_textbutton::guitextbutton_text_bounds(button, text, wrap, font_size, font, svc)
}

// 0x6735d0 — __ZNK3RBX13GuiTextButton11getTextFitsEv
// type: int __fastcall(RBX::GuiTextButton *this, int, bool)
#[doc(alias = "__ZNK3RBX13GuiTextButton11getTextFitsEv")]
pub unsafe fn stub_0x6735d0(
    button: *mut u8,
    text: &str,
    wrap: bool,
    font_size: u32,
    font: u32,
    svc: &gui_textbutton::TextMeasureSvc,
) -> bool {
    // IDA 0x6735d0: gates + avail + fits flag + width-vs-rect compare.
    gui_textbutton::guitextbutton_text_fits(button, text, wrap, font_size, font, svc)
}

// 0x6737e8 — __ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf")]
pub unsafe fn stub_0x6737e8(
    this: *mut f32,
    value: f32,
    raise: unsafe fn(*mut u8, &'static str) -> *mut u8,
) -> *mut f32 {
    // IDA 0x6737e8: float-210 compare-store + raise (unk_13277A8).
    gui_textbutton::set_float(this, gui_textbutton::STROKE_TRANSP_FLOAT, value, gui_textbutton::DESC_STROKE, raise)
}

// 0x673814 — __ZN3RBX13GuiTextButton14checkForResizeEv
// type: int __fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZN3RBX13GuiTextButton14checkForResizeEv")]
pub unsafe fn stub_0x673814(
    this: *mut u8,
    check: unsafe fn(*mut u8),
    raise: gui_textbutton::RaiseHook,
) -> i32 {
    // IDA 0x673814: `GuiObject::checkForResize` (0x67381a) then raises
    // unk_1327724 (0x67382e) + unk_1327750, returning the last.
    check(this);
    raise(this, gui_textbutton::DESC_TEXT2);
    raise(this, gui_textbutton::DESC_TEXT3)
}

// 0x673840 — __ZN3RBX13GuiTextButton21setTransparencyLegacyEf
// type: int __fastcall(RBX::GuiTextButton *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton21setTransparencyLegacyEf")]
pub unsafe fn stub_0x673840(
    this: *mut u8,
    value: f32,
    raise: gui_textbutton::RaiseHook,
    set_background: unsafe fn(*mut u8, f32) -> i32,
) -> i32 {
    // IDA 0x673840: float-206 compare-store + raise (unk_1327674), then the
    // `GuiObject::setBackgroundTransparency` tail call.
    if (this as *mut f32).add(gui_textbutton::TEXT_TRANSP_FLOAT).read() != value {
        // IDA 0x67385a-0x673864
        (this as *mut f32).add(gui_textbutton::TEXT_TRANSP_FLOAT).write(value);
        raise(this, gui_textbutton::DESC_TRANSP); // IDA 0x673870
    }
    set_background(this, value)
}

// 0x673888 — __ZNK3RBX13GuiTextButton21getPersistentDataCostEv
// type: int __fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZNK3RBX13GuiTextButton21getPersistentDataCostEv")]
pub fn stub_0x673888(base: i32, text_len: usize) -> i32 {
    // IDA 0x673888: `Instance::getPersistentDataCost + max(len/100, 1) + 6`.
    gui_textbutton::textbutton_persistent_cost(base, text_len)
}

// 0x67390c — __ZN3RBX13GuiTextButton8render2dEPNS_5AdornE
// type: int __fastcall(RBX::GuiTextButton *this, RBX::Adorn *)
#[doc(alias = "__ZN3RBX13GuiTextButton8render2dEPNS_5AdornE")]
pub unsafe fn stub_0x67390c(this: *mut u8, adorn: *mut u8) -> i32 {
    // IDA 0x67390c: vtable-word-49 dispatch `(this, a2, 0)`.
    gui_textbutton::guibutton_render2d(this, adorn)
}

// 0x673918 — __ZThn96_N3RBX13GuiTextButton8render2dEPNS_5AdornE
// type: int __fastcall(RBX::GuiTextButton *this, RBX::Adorn *)
#[doc(alias = "__ZThn96_N3RBX13GuiTextButton8render2dEPNS_5AdornE")]
pub unsafe fn stub_0x673918(this: *mut u8, adorn: *mut u8) -> i32 {
    // IDA 0x673918: `this - 96` adjust + same dispatch shape.
    gui_textbutton::guibutton_render2d_thunk(this, adorn)
}

// 0x673ce4 — __ZN3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZN3RBX13GuiTextButtonD1Ev")]
pub unsafe fn stub_0x673ce4(this: *mut u8, fini: &gui_textbutton::GuiButtonFini) {
    // IDA 0x673ce4: `~string(+804)` + `~GuiButton`.
    gui_textbutton::guibutton_d1(this, fini)
}

// 0x673cfc — __ZN3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZN3RBX13GuiTextButtonD0Ev")]
pub unsafe fn stub_0x673cfc(this: *mut u8, fini: &gui_textbutton::GuiButtonFini) {
    // IDA 0x673cfc: D1 (0x673d20-0x673d56) then `operator delete`.
    gui_textbutton::guibutton_d0(this, fini)
}

// 0x673da8 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x673da8() -> &'static str {
    // IDA 0x673da8: `static_getCreator` (0x673dac) + Creator `getClassName` shim.
    gui_textbutton::creator_class_name()
}

// 0x673db8 — __ZThn32_N3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD1Ev")]
pub unsafe fn stub_0x673db8(this: *mut u8, fini: &gui_textbutton::GuiButtonFini) {
    // IDA 0x673db8: `this - 32` adjust into D1 (string at +772, base at -32).
    stub_0x673ce4(this.sub(32), fini)
}

// 0x673dd4 — __ZThn32_N3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD0Ev")]
pub unsafe fn stub_0x673dd4(this: *mut u8, fini: &gui_textbutton::GuiButtonFini) {
    // IDA 0x673dd4: `v1 = this - 32` (0x673df2), D0, delete.
    stub_0x673cfc(this.sub(32), fini)
}

// 0x673e80 — __ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x673e80() -> &'static str {
    // IDA 0x673e80: same `static_getCreator` + `getClassName` tail as 0x673da8.
    gui_textbutton::creator_class_name()
}

// 0x673e90 — __ZThn36_N3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD1Ev")]
pub unsafe fn stub_0x673e90(this: *mut u8, fini: &gui_textbutton::GuiButtonFini) {
    // IDA 0x673e90: `this - 36` adjust into D1 (string at +768, base at -36).
    stub_0x673ce4(this.sub(36), fini)
}

// 0x673eac — __ZThn36_N3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD0Ev")]
pub unsafe fn stub_0x673eac(this: *mut u8, fini: &gui_textbutton::GuiButtonFini) {
    // IDA 0x673eac: `v1 = this - 36` (0x673eca), D0, delete.
    stub_0x673cfc(this.sub(36), fini)
}

// 0x673f58 — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev")]
pub unsafe fn stub_0x673f58(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x673f58: D1 thunk straight into D2 (`$shim`, same as TextBox 0x668fb4).
    gui_textbutton::creator_destroy(slot)
}

// 0x673f5c — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev")]
pub unsafe fn stub_0x673f5c(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x673f5c: vtable restore (`*a1 = &off_128E43C`) + `wasConstructed`
    // assert + creators erase (same template as TextBox 0x668fb8).
    gui_textbutton::creator_destroy(slot)
}

// 0x673ff8 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x673ff8() -> &'static str {
    // IDA 0x673ff8 (disasm: FLog::Asserts prologue): assert-guarded
    // class-name read; same tail as TextBox 0x669054.
    gui_textbutton::creator_class_name()
}

// 0x674080 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv")]
pub unsafe fn stub_0x674080(
    out: *mut gui_textbox::TextBoxShared,
    alloc: unsafe fn() -> (*mut u8, crate::SharedPtr<u8>),
) {
    // IDA 0x674080: assert + `Creatable::create<GuiTextButton>` + `+32` +
    // count move (same template as TextBox 0x6690dc).
    gui_textbutton::guitextbutton_create(out, alloc)
}

// 0x674658 — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev")]
pub unsafe fn stub_0x674658(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x674658: declare-once + creators insert + `isConstructed = 666`
    // (same template as TextBox 0x6696b4, `sGuiTextButton` name).
    gui_textbutton::creator_construct(slot)
}

// 0x67489c — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x67489c() -> *const gui_textbox::Creator {
    // IDA 0x67489c: `wasConstructed` assert (Object.h:282) + `&creatorPrivateE`.
    gui_textbutton::static_get_creator()
}

// 0x6782ec — __ZN3RBX9TextLabelC1Ev
// type: int __fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZN3RBX9TextLabelC1Ev")]
pub unsafe fn stub_0x6782ec(this: *mut u8, ctor: &gui_textbutton::GuiTextButtonCtor) -> *mut u8 {
    // IDA 0x6782ec: C1 thunk straight into C2.
    stub_0x6782f0(this, ctor)
}

// 0x6782f0 — __ZN3RBX9TextLabelC2Ev
// type: RBX::GuiLabel *__fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZN3RBX9TextLabelC2Ev")]
pub unsafe fn stub_0x6782f0(this: *mut u8, ctor: &gui_textbutton::GuiTextButtonCtor) -> *mut u8 {
    // IDA 0x6782f0: base + vtable rounds + describe + `"Label"` + member table.
    gui_textbutton::guitextlabel_construct(this, ctor);
    this // IDA 0x678508
}

// 0x6785c8 — __ZN3RBX9TextLabel7setTextESs
// type: void __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "__ZN3RBX9TextLabel7setTextESs")]
pub unsafe fn stub_0x6785c8(this: *mut u8, text: &str, svc: &gui_textbutton::GuiTextSvc) {
    // IDA 0x6785c8: normalize + profanity gate + compare + commit + 3 raises
    // (unk_13278C0/F4/20).
    gui_textbutton::guitextlabel_set_text(this, text, svc)
}

// 0x678784 — __ZN3RBX9TextLabel11setFontSizeENS_11TextService8FontSizeE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel11setFontSizeENS_11TextService8FontSizeE")]
pub unsafe fn stub_0x678784(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x678784: word-136 compare-store + raises (unk_1327AA4, unk_13279F4).
    gui_textbutton::set_word(
        this,
        gui_textbutton::LFONTSIZE_WORD,
        value,
        &[gui_textbutton::LDESC_FONTSIZE, gui_textbutton::LDESC_TEXT2],
        raise,
    )
}

// 0x6787bc — __ZN3RBX9TextLabel7setFontENS_11TextService4FontE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel7setFontENS_11TextService4FontE")]
pub unsafe fn stub_0x6787bc(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x6787bc: word-148 compare-store + raises (unk_1327AD8, unk_13279F4).
    gui_textbutton::set_word(
        this,
        gui_textbutton::LFONT_WORD,
        value,
        &[gui_textbutton::LDESC_FONT, gui_textbutton::LDESC_TEXT2],
        raise,
    )
}

// 0x6787f4 — __ZN3RBX9TextLabel12setTextColorENS_10BrickColorE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX9TextLabel12setTextColorENS_10BrickColorE")]
pub unsafe fn stub_0x6787f4(
    this: *mut u8,
    packed: u32,
    color3: fn(u32) -> [f32; 3],
    set_color3: unsafe fn(*mut u8, [f32; 3]) -> i32,
) -> i32 {
    // IDA 0x6787f4: pack (0x6787fa), `BrickColor::color3` (0x678804),
    // forward to `setTextColor3` (0x678812).
    set_color3(this, color3(packed))
}

// 0x678894 — __ZN3RBX9TextLabel19setTextTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX9TextLabel19setTextTransparencyEf")]
pub unsafe fn stub_0x678894(
    this: *mut f32,
    value: f32,
    raise: unsafe fn(*mut u8, &'static str) -> *mut u8,
) -> *mut f32 {
    // IDA 0x678894: float-140 compare-store + raise (unk_1327944).
    gui_textbutton::set_float(this, gui_textbutton::LTRANSP_FLOAT, value, gui_textbutton::LDESC_TRANSP, raise)
}

// 0x6788bc — __ZN3RBX9TextLabel11setTextWrapEb
// type: int __fastcall(RBX::TextLabel *this, int)
#[doc(alias = "__ZN3RBX9TextLabel11setTextWrapEb")]
pub unsafe fn stub_0x6788bc(
    this: *mut u8,
    value: i32,
    raise: gui_textbutton::RaiseHook,
) -> i32 {
    // IDA 0x6788bc: byte-580 compare-store + 3 raises.
    gui_textbutton::set_byte(
        this,
        gui_textbutton::LWRAP_BYTE,
        value,
        &[gui_textbutton::LDESC_WRAP, gui_textbutton::LDESC_TEXT2, gui_textbutton::LDESC_TEXT3],
        raise,
    )
}

// 0x6788fc — __ZN3RBX9TextLabel12setTextScaleEb
// type: int __fastcall(RBX::TextLabel *this, int)
#[doc(alias = "__ZN3RBX9TextLabel12setTextScaleEb")]
pub unsafe fn stub_0x6788fc(
    this: *mut u8,
    value: i32,
    raise: gui_textbutton::RaiseHook,
) -> i32 {
    // IDA 0x6788fc: byte-581 compare (0x67890a); on change store +
    // raise (0x678922), then `setTextWrap(this, 1)` when enabling (0x678928)
    // else the two text raises (0x67893c-0x67894a).
    let slot = this.add(gui_textbutton::LSCALE_BYTE);
    if slot.read() as i32 != value {
        slot.write(value as u8);
        raise(this, gui_textbutton::LDESC_SCALE);
        if value == 1 {
            return stub_0x6788bc(this, 1, raise);
        }
        raise(this, gui_textbutton::LDESC_TEXT2);
        return raise(this, gui_textbutton::LDESC_TEXT3);
    }
    slot.read() as i32
}

// 0x678950 — __ZN3RBX9TextLabel13setXAlignmentENS_11TextService10XAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel13setXAlignmentENS_11TextService10XAlignmentE")]
pub unsafe fn stub_0x678950(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x678950: word-146 compare-store + 3 raises.
    gui_textbutton::set_word(
        this,
        gui_textbutton::LXA_WORD,
        value,
        &[gui_textbutton::LDESC_XA, gui_textbutton::LDESC_TEXT2, gui_textbutton::LDESC_TEXT3],
        raise,
    )
}

// 0x678990 — __ZN3RBX9TextLabel13setYAlignmentENS_11TextService10YAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX9TextLabel13setYAlignmentENS_11TextService10YAlignmentE")]
pub unsafe fn stub_0x678990(
    this: *mut u8,
    value: u32,
    raise: gui_textbutton::RaiseHook,
) -> u32 {
    // IDA 0x678990: word-147 compare-store + 3 raises.
    gui_textbutton::set_word(
        this,
        gui_textbutton::LYA_WORD,
        value,
        &[gui_textbutton::LDESC_YA, gui_textbutton::LDESC_TEXT2, gui_textbutton::LDESC_TEXT3],
        raise,
    )
}

// 0x6789d0 — __ZNK3RBX9TextLabel13getTextBoundsEv
// type: void __fastcall(RBX::TextLabel *this, unsigned int, bool)
#[doc(alias = "__ZNK3RBX9TextLabel13getTextBoundsEv")]
pub unsafe fn stub_0x6789d0(
    button: *mut u8,
    text: &str,
    wrap: bool,
    font_size: u32,
    font: u32,
    svc: &gui_textbutton::TextMeasureSvc,
) -> [f32; 2] {
    // IDA 0x6789d0: gates + wrap avail (rect at +540 text, +580 wrap) +
    // layout + release (same shape as 0x673444; font word 148 at +592,
    // fontsize word 136 at +544).
    gui_textbutton::guitextbutton_text_bounds(button, text, wrap, font_size, font, svc)
}

// 0x678b5c — __ZNK3RBX9TextLabel11getTextFitsEv
// type: int __fastcall(RBX::TextLabel *this, int, bool)
#[doc(alias = "__ZNK3RBX9TextLabel11getTextFitsEv")]
pub unsafe fn stub_0x678b5c(
    button: *mut u8,
    text: &str,
    wrap: bool,
    font_size: u32,
    font: u32,
    svc: &gui_textbutton::TextMeasureSvc,
) -> bool {
    // IDA 0x678b5c: gates + avail + fits flag + width-vs-rect compare
    // (same shape as 0x6735d0).
    gui_textbutton::guitextbutton_text_fits(button, text, wrap, font_size, font, svc)
}

// 0x678d74 — __ZN3RBX9TextLabel25setTextStrokeTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX9TextLabel25setTextStrokeTransparencyEf")]
pub unsafe fn stub_0x678d74(
    this: *mut f32,
    value: f32,
    raise: unsafe fn(*mut u8, &'static str) -> *mut u8,
) -> *mut f32 {
    // IDA 0x678d74: float-144 compare-store + raise (unk_1327A78).
    gui_textbutton::set_float(this, gui_textbutton::LSTROKE_FLOAT, value, gui_textbutton::LDESC_STROKE, raise)
}

// 0x678da0 — __ZN3RBX9TextLabel14checkForResizeEv
// type: int __fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZN3RBX9TextLabel14checkForResizeEv")]
pub unsafe fn stub_0x678da0(
    this: *mut u8,
    check: unsafe fn(*mut u8),
    raise: gui_textbutton::RaiseHook,
) -> i32 {
    // IDA 0x678da0: `GuiObject::checkForResize` (0x678da6) then raises
    // unk_13279F4 (0x678dba) + unk_1327A20, returning the last.
    check(this);
    raise(this, gui_textbutton::LDESC_TEXT2);
    raise(this, gui_textbutton::LDESC_TEXT3)
}

// 0x678dcc — __ZN3RBX9TextLabel21setTransparencyLegacyEf
// type: int __fastcall(RBX::TextLabel *this, float)
#[doc(alias = "__ZN3RBX9TextLabel21setTransparencyLegacyEf")]
pub unsafe fn stub_0x678dcc(
    this: *mut u8,
    value: f32,
    raise: gui_textbutton::RaiseHook,
    set_background: unsafe fn(*mut u8, f32) -> i32,
) -> i32 {
    // IDA 0x678dcc: float-140 compare-store + raise (unk_1327944), then the
    // `GuiObject::setBackgroundTransparency` tail call.
    if (this as *mut f32).add(gui_textbutton::LTRANSP_FLOAT).read() != value {
        // IDA 0x678de6-0x678df0
        (this as *mut f32).add(gui_textbutton::LTRANSP_FLOAT).write(value);
        raise(this, gui_textbutton::LDESC_TRANSP); // IDA 0x678dfc
    }
    set_background(this, value)
}

// 0x678e14 — __ZNK3RBX9TextLabel21getPersistentDataCostEv
// type: int __fastcall(RBX::TextLabel *this)
#[doc(alias = "__ZNK3RBX9TextLabel21getPersistentDataCostEv")]
pub fn stub_0x678e14(base: i32, text_len: usize) -> i32 {
    // IDA 0x678e14: `Instance::getPersistentDataCost + max(len/100, 1) + 6`
    // over the +540 string (same shape as 0x673888).
    gui_textbutton::textbutton_persistent_cost(base, text_len)
}

// 0x678e98 — __ZN3RBX9TextLabel8render2dEPNS_5AdornE
// type: int __fastcall(RBX::TextLabel *this, RBX::Adorn *)
#[doc(alias = "__ZN3RBX9TextLabel8render2dEPNS_5AdornE")]
pub unsafe fn stub_0x678e98(this: *mut u8, adorn: *mut u8) -> i32 {
    // IDA 0x678e98: vtable-word-49 dispatch `(this, a2, 0)` (same as 0x67390c).
    gui_textbutton::guibutton_render2d(this, adorn)
}

// 0x678ea4 — __ZThn96_N3RBX9TextLabel8render2dEPNS_5AdornE
// type: int __fastcall(RBX::TextLabel *this, RBX::Adorn *)
#[doc(alias = "__ZThn96_N3RBX9TextLabel8render2dEPNS_5AdornE")]
pub unsafe fn stub_0x678ea4(this: *mut u8, adorn: *mut u8) -> i32 {
    // IDA 0x678ea4: `this - 96` adjust + same dispatch shape (like 0x673918).
    gui_textbutton::guibutton_render2d_thunk(this, adorn)
}

// 0x67929c — __ZN3RBX9TextLabelD1Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZN3RBX9TextLabelD1Ev")]
pub unsafe fn stub_0x67929c(
    this: *mut u8,
    drop_text: unsafe fn(*mut u8),
    member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
) {
    // IDA 0x67929c: `~string(+540)` + `~GuiObject` (no `GuiLabel` middle).
    gui_textbutton::textlabel_d1(this, drop_text, member)
}

// 0x6792b4 — __ZN3RBX9TextLabelD0Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZN3RBX9TextLabelD0Ev")]
pub unsafe fn stub_0x6792b4(
    this: *mut u8,
    drop_text: unsafe fn(*mut u8),
    member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
    free: unsafe fn(*mut u8),
) {
    // IDA 0x6792b4: D1 (0x6792d8-0x67930e) then `operator delete`.
    gui_textbutton::textlabel_d0(this, drop_text, member, free)
}

// 0x679360 — __ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x679360() -> &'static str {
    // IDA 0x679360: `static_getCreator` + Creator `getClassName` shim.
    gui_textbutton::label_creator_class_name()
}

// 0x679370 — __ZThn32_N3RBX9TextLabelD1Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9TextLabelD1Ev")]
pub unsafe fn stub_0x679370(
    this: *mut u8,
    drop_text: unsafe fn(*mut u8),
    member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
) {
    // IDA 0x679370: `this - 32` adjust into D1 (string at +508).
    stub_0x67929c(this.sub(32), drop_text, member)
}

// 0x67938c — __ZThn32_N3RBX9TextLabelD0Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9TextLabelD0Ev")]
pub unsafe fn stub_0x67938c(
    this: *mut u8,
    drop_text: unsafe fn(*mut u8),
    member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
    free: unsafe fn(*mut u8),
) {
    // IDA 0x67938c: `v1 = this - 32` (0x6793aa), D0, delete.
    stub_0x6792b4(this.sub(32), drop_text, member, free)
}

// 0x679438 — __ZThn32_NK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x679438() -> &'static str {
    // IDA 0x679438: same `static_getCreator` + `getClassName` tail as 0x679360.
    gui_textbutton::label_creator_class_name()
}

// 0x679448 — __ZThn36_N3RBX9TextLabelD1Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9TextLabelD1Ev")]
pub unsafe fn stub_0x679448(
    this: *mut u8,
    drop_text: unsafe fn(*mut u8),
    member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
) {
    // IDA 0x679448: `this - 36` adjust into D1 (string at +504).
    stub_0x67929c(this.sub(36), drop_text, member)
}

// 0x679464 — __ZThn36_N3RBX9TextLabelD0Ev
// type: void __fastcall(RBX::TextLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9TextLabelD0Ev")]
pub unsafe fn stub_0x679464(
    this: *mut u8,
    drop_text: unsafe fn(*mut u8),
    member: unsafe fn(*mut u8, gui_textbox::GuiObjectMember),
    free: unsafe fn(*mut u8),
) {
    // IDA 0x679464: `v1 = this - 36` (0x679482), D0, delete.
    stub_0x6792b4(this.sub(36), drop_text, member, free)
}

// 0x679510 — __ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD1Ev")]
pub unsafe fn stub_0x679510(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x679510: D1 thunk straight into D2 (`$shim`).
    gui_textbutton::label_creator_destroy(slot)
}

// 0x679514 — __ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7CreatorD2Ev")]
pub unsafe fn stub_0x679514(slot: *mut gui_textbox::Creator) -> *mut gui_textbox::Creator {
    // IDA 0x679514: vtable restore (`*a1 = &off_128EDAC`) + `wasConstructed`
    // assert + creators erase.
    gui_textbutton::label_creator_destroy(slot)
}

// 0x6795b0 — __ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9TextLabelENS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x6795b0() -> &'static str {
    // IDA 0x6795b0 (disasm: FLog::Asserts prologue): assert-guarded
    // class-name read; same tail as 0x669054.
    gui_textbutton::label_creator_class_name()
}
