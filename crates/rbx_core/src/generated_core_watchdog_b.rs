//! core watchdog b — 100 core stubs EA-sorted, second gap filler after watchdog_a 0x25e9ac.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_core — next 100 uncovered after 0x25e9ac (watchdog_a max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Batch: 24 IDA-grounded ports 0x25e9b0-0x25effc — the SpotLight/PointLight/Light
/// `PropDescriptor<float|Color3|bool>::GetSetImpl` cluster (predicates, member-pointer
/// getValue/setValue dispatch, descriptor ctor/dtor). Untouched carriers keep stub bodies;
/// ports live in `prop_binding` under idiomatic names, wired via `stub_25*`.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR` carrier),
/// member-function-pointer pairs -> `MemberPtr`, throws -> none (all paths total except
/// null-object misuse, matching the original). `[INFERENCE]` marks what the binary does not
/// pin down; everything else follows the IDA pseudocode + disassembly branch-for-branch.
pub mod prop_binding {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// was: `RBX::Reflection::DescribedBase` -> most-derived bias. Every getValue/setValue
    /// path computes `v = 0; if (a2) v = a2 - 36` (IDA 0x25e9b4/0x25e9d4/0x25eb40/0x25eb60/
    /// 0x25eccc/0x25ecec/0x25ee58/0x25ee80/0x25f004/0x25f028 + disasm SUBNE.W R2, R1, #0x24).
    pub const DESCRIBED_BASE_BIAS: usize = 36;

    /// was: Itanium/ARM member-function-pointer word pair (`+4/+8` getter, `+12/+16` setter
    /// inside the 0x14-byte `GetSetImpl` box). `func` = direct target, else vtable offset;
    /// `adj` = `(this_delta << 1) | virtual_bit` (IDA `v4 >> 1`, `v4 & 1`, `TST.W R3, #1`).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MemberPtr {
        pub func: usize,
        pub adj: usize,
    }

    /// was: `GetSetImpl<Getter, Setter>` heap box (`operator new(0x14u)`, IDA 0x25e9f8:
    /// `*v23 = &off_...; v23[1] = a4; v23[2] = a5; v23[3] = a6; v23[4] = a7`).
    #[derive(Debug, Clone, Copy)]
    pub struct GetSetImpl {
        pub getter: MemberPtr,
        pub setter: MemberPtr,
    }

    /// was: `GetSetImpl` vtables installed by each ctor (`*v23 = &off_...`).
    pub const POINT_LIGHT_FLOAT_GETSET_VTAB: &str = "off_122F3D8"; // IDA 0x25e9f8
    pub const LIGHT_FLOAT_GETSET_VTAB: &str = "off_122F458"; // IDA 0x25eb84
    pub const LIGHT_COLOR3_GETSET_VTAB: &str = "off_122F4E8"; // IDA 0x25ed10
    pub const LIGHT_BOOL_GETSET_VTAB: &str = "off_122F578"; // IDA 0x25eebc
    /// was: `PropDescriptor` vtables (`*a1 = &off_...` at each ctor tail).
    pub const POINT_LIGHT_FLOAT_DESC_VTAB: &str = "off_122F388"; // IDA 0x25e9f8
    pub const LIGHT_FLOAT_DESC_VTAB: &str = "off_122F408"; // IDA 0x25eb84
    pub const LIGHT_COLOR3_DESC_VTAB: &str = "off_122F488"; // IDA 0x25ed10
    pub const LIGHT_BOOL_DESC_VTAB: &str = "off_122F518"; // IDA 0x25eebc
    /// was: `TypedPropertyDescriptor<T>` vtables restored by each dtor (`*a1 = &off_...`).
    pub const TYPED_FLOAT_DESC_VTAB: &str = "off_1270A68"; // IDA 0x25eb0c/0x25ec98
    pub const TYPED_COLOR3_DESC_VTAB: &str = "off_1270988"; // IDA 0x25ee24
    pub const TYPED_BOOL_DESC_VTAB: &str = "off_1222378"; // IDA 0x25efd0

    /// IDA `v = 0; if (a2) v = a2 - 36` then `this = v + (adj >> 1)`.
    /// Null stays null-derived (original would fault on the later call too).
    pub fn resolve_this(obj: *const u8, adj: usize) -> *const u8 {
        let base = if obj.is_null() {
            0
        } else {
            (obj as usize).wrapping_sub(DESCRIBED_BASE_BIAS)
        };
        base.wrapping_add(adj >> 1) as *const u8
    }

    /// IDA virtual branch (`ITT NE; LDRNE R2, [R0]; LDRNE R1, [R2,R1]` /
    /// `LDRNE R3, [R1,R3]`): `target = *(vfunc_offset + *adjusted_this)`.
    /// Non-virtual: the stored address itself (`BX R1` / `BX R3`).
    pub unsafe fn resolve_target(mp: MemberPtr, this: *const u8) -> usize {
        if mp.adj & 1 != 0 {
            let vtable = *(this as *const usize);
            *((vtable.wrapping_add(mp.func)) as *const usize)
        } else {
            mp.func
        }
    }

    /// IDA 0x25e9b4/0x25eb40/0x25eccc/0x25f004 float `getValue`: `return v3(v5)`.
    pub unsafe fn get_f32(imp: &GetSetImpl, obj: *const u8) -> f32 {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> f32 =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(this)
    }

    /// IDA 0x25e9d4/0x25eb60/0x25ecec/0x25f028 float `setValue`: `v4(v6, *a3)`.
    /// The callee is `void (T::*)(float)`; the IDA `int` return carries no
    /// observable output, so the port returns `()`.
    pub unsafe fn set_f32(imp: &GetSetImpl, obj: *mut u8, value: f32) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, f32) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, value)
    }

    /// IDA 0x25ee58 Color3 `getValue`: `return v4(a1, v6)` — by-value Color3 via
    /// hidden out-param `a1`. The IDA `int` return is the call artifact; the
    /// observable output is the `out` write.
    pub unsafe fn get_color3(out: *mut [f32; 3], imp: &GetSetImpl, obj: *const u8) {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*mut [f32; 3], *const u8) =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(out, this)
    }

    /// IDA 0x25ee80 Color3 `setValue`: 12-byte stack copy
    /// (`v8[0] = *a3; v8[1] = a3[1]; v8[2] = a3[2]`) then `v4(v6, v8)`.
    pub unsafe fn set_color3(imp: &GetSetImpl, obj: *mut u8, value: *const [f32; 3]) {
        let tmp = *value;
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, *const [f32; 3]) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, &tmp)
    }

    /// Batch 5, IDA 0x25f004 `Light::bool getValue`: DescribedBase-36, `adj >> 1`
    /// adjust, virtual branch, tail-call getter — the `get_f32` shape over `bool`.
    pub unsafe fn get_bool(imp: &GetSetImpl, obj: *const u8) -> bool {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> bool =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(this)
    }

    /// Batch 5, IDA 0x25f028 `Light::bool setValue`: `v4(v6, *a3)` — the setter
    /// takes the dereferenced byte. The IDA `int` return carries no observable
    /// output, so the port returns `()`.
    pub unsafe fn set_bool(imp: &GetSetImpl, obj: *mut u8, value: bool) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, bool) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, value)
    }

    /// was: `RBX::Reflection::PropDescriptor<T, V>` storage. The owned `Box<GetSetImpl>`
    /// is the IDA `a1[10]` (`+0x28`) word freed by the dtor (`v2 = a1[10]; if (v2) delete`).
    /// Trailing words: `[INFERENCE]` `attributes` = ctor args a8..a10 passed through to
    /// `TypedPropertyDescriptor::init`; `permissions` = a11 (`Security::Permissions`).
    #[derive(Debug, Default)]
    pub struct PropDescriptor {
        pub vtable: &'static str,
        pub name: String,
        pub category: String,
        pub getset: Option<Box<GetSetImpl>>,
        pub attributes: (u32, u32, u32),
        pub permissions: u32,
    }

    impl PropDescriptor {
        /// IDA 0x25e9f8/0x25eb84/0x25ed10/0x25eebc ctor shape: ensure classDescriptor
        /// (lazy-static registry init, a process-global sink here), `new GetSetImpl`
        /// from the (ptr, adj) pairs, `TypedPropertyDescriptor::init` base fields,
        /// then install the `PropDescriptor` vtable. Returns `slot` (IDA `return a1`).
        pub unsafe fn construct(
            slot: *mut PropDescriptor,
            name: *const c_char,
            category: *const c_char,
            getter: MemberPtr,
            setter: MemberPtr,
            vtable: &'static str,
            attr0: u32,
            attr1: u32,
            attr2: u32,
            permissions: u32,
        ) -> *mut PropDescriptor {
            let getset = Box::new(GetSetImpl { getter, setter });
            let this = &mut *slot;
            this.name = if name.is_null() {
                String::new()
            } else {
                CStr::from_ptr(name).to_string_lossy().into_owned()
            };
            this.category = if category.is_null() {
                String::new()
            } else {
                CStr::from_ptr(category).to_string_lossy().into_owned()
            };
            this.getset = Some(getset);
            this.attributes = (attr0, attr1, attr2);
            this.permissions = permissions;
            this.vtable = vtable;
            slot
        }

        /// IDA 0x25eb0c/0x25ec98/0x25ee24/0x25efd0 deleting-dtor shape:
        /// restore the `TypedPropertyDescriptor` vtable, delete the owned box.
        /// IDA also runs `operator delete(a1)`; in Rust the slot stays caller-owned.
        /// The IDA `int` return is the delete artifact — no observable output.
        pub unsafe fn destroy(slot: *mut PropDescriptor, base_vtable: &'static str) {
            let this = &mut *slot;
            this.vtable = base_vtable;
            this.getset = None;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        extern "C" fn fake_get(this: *const u8) -> f32 {
            // Member slot at +36 in the fake derived object (mirrors FakeFloat layout).
            unsafe { *(this.add(36) as *const f32) }
        }
        extern "C" fn fake_set(this: *mut u8, v: f32) {
            unsafe { *(this.add(36) as *mut f32) = v }
        }
        extern "C" fn fake_cget(out: *mut [f32; 3], this: *const u8) {
            unsafe { *out = *(this.add(36) as *const [f32; 3]) }
        }
        extern "C" fn fake_cset(this: *mut u8, v: *const [f32; 3]) {
            unsafe { *(this.add(36) as *mut [f32; 3]) = *v }
        }

        #[repr(C)]
        struct FakeFloat {
            pad: [u8; 36],
            val: f32,
        }

        /// C++-shaped object for the virtual path: vtable slot at +0 (8-aligned),
        /// member slot at +36 like FakeFloat.
        #[repr(C)]
        struct FakeVirtual {
            vtab: usize,
            _pad: [u8; 28],
            val: f32,
        }

        fn described_of(fake: *const u8) -> *const u8 {
            fake.wrapping_add(DESCRIBED_BASE_BIAS)
        }

        fn direct(mp_fn: usize) -> MemberPtr {
            MemberPtr { func: mp_fn, adj: 0 }
        }

        #[test]
        fn float_get_set_roundtrip_direct() {
            let mut fake = FakeFloat { pad: [0; 36], val: 1.5 };
            let base = std::ptr::addr_of!(fake) as *const u8;
            let imp = GetSetImpl { getter: direct(fake_get as usize), setter: direct(fake_set as usize) };
            unsafe {
                assert_eq!(get_f32(&imp, described_of(base)), 1.5);
                set_f32(&imp, described_of(base) as *mut u8, 2.25);
            }
            assert_eq!(fake.val, 2.25);
        }

        #[test]
        fn float_dispatch_virtual_bit() {
            let mut fake = FakeVirtual { vtab: 0, _pad: [0; 28], val: 7.0 };
            let vtable: [usize; 1] = [fake_get as usize];
            fake.vtab = vtable.as_ptr() as usize;
            let base = std::ptr::addr_of!(fake) as *const u8;
            let imp = GetSetImpl {
                getter: MemberPtr { func: 0, adj: 1 },
                setter: direct(fake_set as usize),
            };
            unsafe {
                assert_eq!(get_f32(&imp, described_of(base)), 7.0);
            }
        }

        #[test]
        fn resolve_this_null_stays_derived_null() {
            assert!(resolve_this(std::ptr::null(), 0).is_null());
        }

        #[test]
        fn color3_copies_twelve_bytes() {
            #[repr(C)]
            struct FakeColor {
                pad: [u8; 36],
                col: [f32; 3],
            }
            let mut fake = FakeColor { pad: [0; 36], col: [0.1, 0.2, 0.3] };
            let obj = (std::ptr::addr_of!(fake) as *const u8).wrapping_add(DESCRIBED_BASE_BIAS);
            let imp = GetSetImpl { getter: direct(fake_cget as usize), setter: direct(fake_cset as usize) };
            let mut out = [0.0f32; 3];
            unsafe {
                get_color3(&mut out, &imp, obj);
            }
            assert_eq!(out, [0.1, 0.2, 0.3]);
            let next = [9.0f32, 8.0, 7.0];
            unsafe {
                set_color3(&imp, obj as *mut u8, &next);
            }
            assert_eq!(fake.col, next);
        }

        #[test]
        fn descriptor_construct_destroy_mirrors_dtor() {
            let mut slot = PropDescriptor::default();
            let name = c"brightness".as_ptr();
            let cat = c"light".as_ptr();
            unsafe {
                let back = PropDescriptor::construct(
                    &mut slot,
                    name,
                    cat,
                    direct(0x1000),
                    direct(0x2000),
                    LIGHT_FLOAT_DESC_VTAB,
                    1,
                    2,
                    3,
                    4,
                );
                assert!(std::ptr::eq(back, &slot));
            }
            assert_eq!(slot.vtable, LIGHT_FLOAT_DESC_VTAB);
            assert_eq!(slot.name, "brightness");
            assert!(slot.getset.is_some());
            unsafe {
                PropDescriptor::destroy(&mut slot, TYPED_FLOAT_DESC_VTAB);
            }
            assert_eq!(slot.vtable, TYPED_FLOAT_DESC_VTAB);
            assert!(slot.getset.is_none());
        }
    }
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::GetSetImpl<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>::isWriteOnly(void)const")]
// 0x25e9b0 — __ZNK3RBX10Reflection14PropDescriptorINS_9SpotLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
/// Batch 5: 22 IDA-grounded ports 0x25f54c-0x260808 — the `EventDescriptor` /
/// `FunctionDescriptor` / `YieldFunctionDescriptor` constructor shape,
/// `MemberDescriptorContainer<T>::declare` + `declareSub` sorted-registry logic,
/// the `staticData`/`allDescriptors` global registry, the boost unordered
/// `operator[]` name map, `vector<*Descriptor>::insert`, signature-list clear,
/// descriptor dtors, `sendEvent` gating, and the tiny fill/no-op/forward helpers.
/// Ports live in `member_registry` under idiomatic names, wired via the matching
/// `stub_25*`/`stub_260*`; untouched carriers keep stub bodies.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR`
/// carrier); `boost::unordered_map<const char*, T>` -> `HashMap<String, usize>`
/// (content-hashed, matching `StringHashPredicate`/`StringEqualPredicate`);
/// `RBX::Name` order words (`**(x + 12)`, `**(*(x + 20) + 12)`) -> opaque `u32`
/// keys handed out by the registry (`[INFERENCE]` — the global `Name` table is
/// outside this batch); `__cxa_throw`/`ReleaseAssert` -> `panic!` gated by the
/// `ASSERTS` flag (mirroring `FLog::Asserts`); `boost::call_once` statics ->
/// `OnceLock`/`Mutex`. `[INFERENCE]` marks the rest; everything else follows the
/// IDA pseudocode branch-for-branch.
pub mod member_registry {
    use std::collections::HashMap;
    use std::os::raw::c_char;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, OnceLock};

    /// Batch 5: was: `FLog::Asserts` — gates the `ReleaseAssert` paths in
    /// `sendEvent` (0x25f850), `declareSub` (0x25f8f0/0x25f96c) and friends.
    /// Off here matches production; tests flip it for the panic paths.
    pub static ASSERTS: AtomicBool = AtomicBool::new(false);

    /// Batch 5: was: `RBX::Reflection::MemberDescriptor` — name plus the two
    /// opaque `Name`-order words the registry sorts by (`**(x + 12)` primary,
    /// `**(*(x + 20) + 12)` tiebreak, IDA 0x25f6ce/0x25f7a2).
    #[derive(Debug, Clone)]
    pub struct MemberDescriptor {
        pub name: String,
        pub order: u32,
        pub sub_order: u32,
    }

    /// Batch 5: descriptor store backing every container + the global list.
    /// Descriptors are referenced by index (the binary uses pointers; indices are
    /// the observable-identity equivalent here).
    #[derive(Debug, Default)]
    pub struct DescriptorStore {
        pub descriptors: Vec<MemberDescriptor>,
        /// was: the global `Name::declare` order counter (`[INFERENCE]`).
        pub next_order: u32,
    }

    impl DescriptorStore {
        pub fn insert(&mut self, name: &str) -> usize {
            let order = self.next_order;
            self.next_order += 1;
            self.descriptors.push(MemberDescriptor { name: name.to_string(), order, sub_order: order });
            self.descriptors.len() - 1
        }
        pub fn order_of(&self, idx: usize) -> u32 {
            self.descriptors[idx].order
        }
        pub fn sub_order_of(&self, idx: usize) -> u32 {
            self.descriptors[idx].sub_order
        }
    }

    /// Batch 5: was: `MemberDescriptorContainer<T>` — the name-ordered member
    /// vector, the `const char* -> T*` map at +3 (`a1 + 3`, IDA 0x25f712), and the
    /// sub-collection vector at +9 (`a1[9..10]`, IDA 0x25f71e).
    /// `hiding_hook` is the process-global hook (IDA 0x25f802/0x25fa3c).
    #[derive(Debug, Default, Clone)]
    pub struct MemberContainer {
        pub members: Vec<usize>,
        pub by_name: HashMap<String, usize>,
        pub sub_collections: Vec<MemberContainer>,
        pub hiding_hook: Option<fn(new_desc: usize, old_desc: usize)>,
        /// Batch 6: was: the `[12]` parent-container link (IDA 0x2618b6/0x2625cc).
        /// Owned snapshots: `mergeMembers` only reads base members, so a snapshot
        /// chain is observably identical to the pointer chain.
        pub parent: Option<Box<MemberContainer>>,
    }

    impl MemberContainer {
        /// `lower_bound` by the primary order word (IDA 0x25f6b8/0x25f94e loops).
        pub fn lower_bound(&self, store: &DescriptorStore, desc: usize) -> usize {
            let key = store.order_of(desc);
            let mut lo = 0;
            let mut hi = self.members.len();
            while lo < hi {
                let mid = (lo + hi) / 2;
                if store.order_of(self.members[mid]) < key {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }
        /// was: `vector<T*>::insert` at the lower-bound position; returns the
        /// position. The original returns the element address (`*a1 + 4 * v6`,
        /// IDA 0x25f8ce) — the index is its position equivalent.
        pub fn insert_at(&mut self, pos: usize, desc: usize) -> usize {
            self.members.insert(pos, desc);
            pos
        }
        /// was: `table_impl::operator[]` name registration (`map[key] = desc`,
        /// IDA 0x25f716/0x25fa06). Boost bucket mechanics collapse into the entry.
        pub fn register_name(&mut self, store: &DescriptorStore, desc: usize) {
            self.by_name.insert(store.descriptors[desc].name.clone(), desc);
        }
    }

    /// Batch 5: was: the `staticData()::result` 8-byte global (IDA 0x25fa9a zeroed
    /// pair = begin/end) plus `dword_12B4858` — the process-global ordered
    /// `allDescriptors` list, lazily built under `call_once` (IDA 0x25fa6a guard).
    /// NOTE: the Event/Function/Yield/Cargo instantiations each own one in the
    /// binary (0x25fa50/0x2609c0/...); they collapse into this single list here
    /// (`[INFERENCE]` — per-type segregation is unobservable to the ported callers).
    pub static ALL_DESCRIPTORS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();

    /// Batch 5, IDA 0x25fa50 `MemberDescriptorContainer<T>::staticData` — guard-var
    /// lazy init returning the global list (the `__cxa_atexit` Collection dtor,
    /// IDA 0x25faa0, is `Drop` here).
    pub fn static_data() -> &'static Mutex<Vec<usize>> {
        ALL_DESCRIPTORS.get_or_init(|| Mutex::new(Vec::new()))
    }

    /// Batch 5: shared `declare` core behind 0x25f690/0x2604b8/0x260638 (verified
    /// identical apart from the vector-insert callee type): binary search, insert
    /// or overwrite, name-map register, `memberHidingHook` on same-name replace,
    /// sub-collection `declareSub` fan-out, then global-list registration ordered
    /// by (order, sub_order) (IDA 0x25f796 loop). Returns the registered index;
    /// the original returns element addresses / a loop residue at the already-
    /// present early exit (0x25f6da) — both discarded at every call site here.
    pub fn declare(
        container: &mut MemberContainer,
        store: &DescriptorStore,
        desc: usize,
    ) -> usize {
        let pos = container.lower_bound(store, desc);
        if pos == container.members.len() {
            container.insert_at(pos, desc);
            container.register_name(store, desc);
        } else if container.members[pos] == desc {
            return pos;
        } else if store.order_of(container.members[pos]) != store.order_of(desc) {
            container.insert_at(pos, desc);
            container.register_name(store, desc);
        } else {
            let old = container.members[pos];
            container.members[pos] = desc;
            container.register_name(store, desc);
            if let Some(hook) = container.hiding_hook {
                hook(desc, old);
            }
        }
        declare_sub_fanout(container, store, desc, desc);
        register_global(store, desc);
        pos
    }

    /// Batch 5: the LABEL_11 fan-out — `declareSub` into every sub-collection
    /// (IDA 0x25f71e loop).
    pub fn declare_sub_fanout(
        container: &mut MemberContainer,
        store: &DescriptorStore,
        desc: usize,
        replaceable: usize,
    ) {
        for sub in container.sub_collections.iter_mut() {
            declare_sub(sub, store, desc, replaceable);
        }
    }

    /// Batch 5: global `allDescriptors` scan (IDA 0x25f796): return when present,
    /// else insert ordered by (`order`, then `sub_order`, IDA 0x25f7a2) and return
    /// the position.
    pub fn register_global(store: &DescriptorStore, desc: usize) -> usize {
        let mut all = static_data().lock().unwrap();
        if let Some(pos) = all.iter().position(|&d| d == desc) {
            return pos;
        }
        let key = (store.order_of(desc), store.sub_order_of(desc));
        let pos = all.iter().position(|&d| {
            let k = (store.order_of(d), store.sub_order_of(d));
            k > key
        }).unwrap_or(all.len());
        all.insert(pos, desc);
        pos
    }

    /// Batch 5: shared `declareSub` core behind 0x25f8d0 (IDA): the member.h:216
    /// `replaceable != descriptor` assert, lower-bound insert, the member.h:227
    /// `*iter != descriptor` assert, replaceable-slot overwrite, name-keyed
    /// insert, else the hiding hook — then the sub-collection fan-out (0x25fa0c).
    /// Returns the registered index.
    pub fn declare_sub(
        container: &mut MemberContainer,
        store: &DescriptorStore,
        desc: usize,
        replaceable: usize,
    ) -> usize {
        if ASSERTS.load(Ordering::SeqCst) {
            assert!(
                replaceable != desc,
                "replaceable != descriptor file: include/reflection/member.h line: 216"
            );
        }
        let pos = container.lower_bound(store, desc);
        if pos == container.members.len() {
            container.insert_at(pos, desc);
            container.register_name(store, desc);
        } else {
            if ASSERTS.load(Ordering::SeqCst) {
                assert!(
                    container.members[pos] != desc,
                    "*iter != descriptor file: include/reflection/member.h line: 227"
                );
            }
            if container.members[pos] == replaceable {
                container.members[pos] = desc;
                container.register_name(store, desc);
            } else if store.order_of(container.members[pos]) != store.order_of(desc) {
                container.insert_at(pos, desc);
                container.register_name(store, desc);
            } else {
                let old = container.members[pos];
                if let Some(hook) = container.hiding_hook {
                    hook(desc, old);
                    return pos;
                }
                return pos;
            }
        }
        declare_sub_fanout(container, store, desc, replaceable);
        pos
    }
    /// Batch 5: was: `RemoteEventCommon::Attributes` fill (IDA 0x25f66c-0x25f674):
    /// `+8 = functionality`, `+0 = deprecated(1)`, `+4 = member`, returns the slot.
    #[derive(Debug, Default)]
    pub struct RemoteEventAttributes {
        pub deprecated: bool,
        pub member: u32,
        pub functionality: u32,
    }

    impl RemoteEventAttributes {
        /// Batch 5, IDA 0x25f66c `Attributes::deprecated(functionality, member)`.
        pub fn set_deprecated(&mut self, functionality: u32, member: u32) -> &mut Self {
            self.functionality = functionality;
            self.deprecated = true;
            self.member = member;
            self
        }
    }

    /// Batch 5: was: `EventDescriptor` / `FunctionDescriptor` /
    /// `YieldFunctionDescriptor` storage — `Descriptor::Descriptor` base (name,
    /// attributes pair), category `Name` ([4], IDA 0x25f5a6), class ([5]), member
    /// ([6]), the `SignatureDescriptor` subobject at +7, and the installed vtable.
    /// The three ctors (0x25f54c/0x260274/0x260394) are one shape modulo category,
    /// vtables, and the class-container declare offset (verified by diff).
    #[derive(Debug, Default)]
    pub struct MemberDescriptorBase {
        pub vtable: &'static str,
        pub name: String,
        pub attributes: (u32, u32),
        pub category: String,
        pub class: usize,
        pub member: u32,
        pub signatures: Vec<String>,
    }

    impl MemberDescriptorBase {
        /// Shared ctor core: base fields, `Name::declare(category)`, `*a1` install
        /// (`off_122F768`), final vtable, empty `SignatureDescriptor`, then
        /// `Container::declare` into the class container (0x25f602/0x26032a/0x26044a).
        /// `Descriptor::Descriptor` string handling and `Name::declare` interning
        /// collapse into owned `String`s.
        pub fn construct(
            &mut self,
            name: &str,
            category: &str,
            class: usize,
            member: u32,
            attr0: u32,
            attr1: u32,
            final_vtable: &'static str,
            container: &mut MemberContainer,
            store: &mut DescriptorStore,
        ) {
            self.name = name.to_string();
            self.attributes = (attr0, attr1);
            self.category = category.to_string();
            self.class = class;
            self.member = member;
            self.vtable = final_vtable;
            self.signatures.clear();
            let idx = store.insert(name);
            declare(container, store, idx);
        }
    }

    /// Batch 5: `EventDescriptor` category/vtables (IDA 0x25f57c-0x25f602).
    pub const EVENT_CATEGORY: &str = "Signals";
    pub const EVENT_MID_VTABLE: &str = "off_122F768";
    pub const EVENT_VTABLE: &str = "off_122F5A8";
    /// Batch 5: `FunctionDescriptor` (IDA 0x2602a4-0x26032a).
    pub const FUNCTION_CATEGORY: &str = "Function";
    pub const FUNCTION_VTABLE: &str = "off_1222248";
    /// Batch 5: `YieldFunctionDescriptor` (IDA 0x2603c4-0x26044a).
    pub const YIELD_FUNCTION_CATEGORY: &str = "YieldFunction";
    pub const YIELD_FUNCTION_VTABLE: &str = "off_122F5E8";

    /// Batch 5, IDA 0x25f838 `EventDescriptor::isScriptable` — returns 1.
    pub fn event_is_scriptable() -> bool {
        true
    }

    /// Batch 5, IDA 0x25f840 `EventDescriptor::sendEvent` — `ReleaseAssert(false)`
    /// at event.h:159 when asserts are on; otherwise returns the (zero) flag.
    pub fn event_send_event() -> i32 {
        let flag = i32::from(ASSERTS.load(Ordering::SeqCst));
        if ASSERTS.load(Ordering::SeqCst) {
            panic!("false file: include/reflection/event.h line: 159");
        }
        flag
    }

    /// Batch 5, IDA 0x25f688 `EventSource::raiseEventInvocation` — empty body.
    pub fn raise_event_invocation() {}

    /// Batch 5, IDA 0x25f678 `EventSource::processRemoteEvent` — tail-calls the
    /// descriptor's virtual at +20 (`(*(a2 + 20))(a2, a1)`).
    pub unsafe fn process_remote_event(desc: *const u8, source: *const u8) {
        let vtable = *(desc as *const usize);
        let target: extern "C" fn(*const u8, *const u8) =
            std::mem::transmute(*((vtable.wrapping_add(20)) as *const usize));
        target(desc, source);
    }

    /// Batch 5: was: `std::vector<T*>::insert` end fast-path shared by 0x25f898 /
    /// 0x260808 — when spare capacity exists and the position is the end, write in
    /// place and bump (0x25f8ae-0x25f8c6); otherwise the `_M_insert_aux` slow path
    /// (0x25f8b2). `Vec::insert`/`push` is exactly that split. Returns the position;
    /// the original returns the element address (`*a1 + 4 * v6`).
    pub fn descriptor_vec_insert(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
        if pos == list.len() {
            list.push(value);
        } else {
            list.insert(pos, value);
        }
        pos
    }

    /// Batch 5, IDA 0x260110 `std::_List_base<SignatureDescriptor::Item>::_M_clear` —
    /// walks the intrusive list from head, runs each node's cleanup (`*(v3 + 4)`
    /// on `node + 6`, 0x26012a — `Drop` here), and frees the node (0x26012e).
    /// `Vec::clear` drops in order; the intrusive links collapse away.
    pub fn signature_list_clear(list: &mut Vec<String>) {
        list.clear();
    }

    /// Batch 5, IDA 0x260140 `MemberDescriptor::~MemberDescriptor` — empty body.
    pub fn member_descriptor_d1() {}

    /// Batch 5: was: the `Collection` box behind 0x25fab8 (`*a1` owned, freed when
    /// non-null) and the descriptor boxes behind 0x25f810/0x2607b8/0x2607e0
    /// (vtable restore + signature-list clear at +32).
    #[derive(Debug, Default)]
    pub struct DescriptorBox {
        pub vtable: &'static str,
        pub owned: Option<Box<[u8]>>,
        pub signatures: Vec<String>,
    }

    /// Batch 5, IDA 0x25fab8 `Collection::~Collection` — free the owned slot.
    pub fn collection_d1(slot: &mut Option<Box<[u8]>>) {
        *slot = None;
    }

    /// Batch 5, IDA 0x25f810 `~EventDescriptor` — restore `off_122F5A8`, clear +32.
    pub fn event_descriptor_d1(b: &mut DescriptorBox) {
        b.vtable = EVENT_VTABLE;
        b.signatures.clear();
    }

    /// Batch 5, IDA 0x2607b8 `~FunctionDescriptor` — restore `off_1222248`, clear +32.
    pub fn function_descriptor_d1(b: &mut DescriptorBox) {
        b.vtable = FUNCTION_VTABLE;
        b.signatures.clear();
    }

    /// Batch 5, IDA 0x2607e0 `~YieldFunctionDescriptor` — `off_122F5E8`, clear +32.
    pub fn yield_function_descriptor_d1(b: &mut DescriptorBox) {
        b.vtable = YIELD_FUNCTION_VTABLE;
        b.signatures.clear();
    }

    /// Batch 5: was: `boost::unordered::detail::table_impl::operator[]` over
    /// `map<const char*, T>` (IDA 0x25fad0/0x260a40/0x260bc8) — find-or-insert by
    /// content hash (`StringHashPredicate`) and content equality. Bucket/rehash
    /// mechanics collapse into the `HashMap` entry; returns the mapped slot, like
    /// the original's mapped reference.
    pub fn unordered_index_or_insert(
        map: &mut HashMap<String, usize>,
        key: *const c_char,
    ) -> &mut usize {
        let owned = unsafe {
            assert!(!key.is_null());
            std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned()
        };
        map.entry(owned).or_insert(usize::MAX)
    }

    /// Batch 6: was: `table::reserve_for_insert(n)` (IDA 0x260d48/0x262c34) — when
    /// the table has a max-load factor (`a1[5]`), grow only if `size < n` and the
    /// bucket count disagrees (rehash, 0x260d94); with no load factor yet, create
    /// `max(buckets, min_buckets_for_size)` buckets (0x260d8a). `HashMap::reserve`
    /// is that observable contract; prime-list bucket math is boost-internal
    /// (`[INFERENCE]`). Returns the capacity, like the original size/bucket word.
    pub fn unordered_reserve(map: &mut HashMap<String, usize>, additional: usize) -> usize {
        map.reserve(additional);
        map.capacity()
    }

    /// Batch 6: was: `vector<ClassDescriptor*>::_M_insert_aux` (IDA 0x2624b4) —
    /// growth rule `1` when empty else `2 * len` (0x2624f6-0x26257e), capped at
    /// `0x3FFFFFFF` with `std::__throw_length_error("vector::_M_insert_aux")`
    /// (0x26258e), then allocate + move + insert. `Vec::insert` grows the same
    /// way; the reserve below pins the exact capacity rule. Returns the position;
    /// the original returns the element address.
    pub fn class_vec_insert_aux(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
        let new_cap = if list.is_empty() {
            1
        } else {
            if list.len() == 0x3FFFFFFF {
                panic!("vector::_M_insert_aux");
            }
            list.len() * 2
        };
        if new_cap > list.capacity() {
            list.reserve_exact(new_cap - list.len());
        }
        list.insert(pos, value);
        pos
    }

    /// Batch 6: was: `_Vector_base::_M_allocate(n)` (IDA 0x262594) —
    /// `throw_bad_alloc` at `n >= 0x40000000`, else `operator new(4 * n)`.
    /// Returns the byte count (the allocation size); the buffer itself is `Vec`
    /// business here.
    pub fn vector_allocate(n: usize) -> usize {
        if n >= 0x40000000 {
            panic!("std::bad_alloc");
        }
        4 * n
    }

    /// Batch 6: was: `vector<ClassDescriptor*>::~vector` (IDA 0x261de0) — free the
    /// buffer when non-null (0x261de6). Releasing via a fresh `Vec`.
    pub fn class_vec_destroy(list: &mut Vec<usize>) {
        *list = Vec::new();
    }

    /// Batch 6: was: container-pointer `push_back` (IDA 0x2625d4, same fast/grow
    /// split as 0xb740): write at `finish` + bump when not full, else the slow path.
    pub fn container_ptr_vec_push_back(list: &mut Vec<usize>, value: usize) {
        list.push(value);
    }

    /// Batch 7: was: `RBX::Reflection::StringHashPredicate::operator()` (IDA 0x262f6c):
    /// `for (h = 0; len; h ^= (h << 6) + (h >> 2) + c - 1640531527)` over `strlen`
    /// bytes — the content hash behind every unordered name map here. Wrapping
    /// arithmetic matches the 32-bit overflow.
    pub fn string_hash_predicate(key: &[u8]) -> u32 {
        let mut hash: u32 = 0;
        for &byte in key {
            hash ^= hash
                .wrapping_shl(6)
                .wrapping_add(hash.wrapping_shr(2))
                .wrapping_add(byte as u32)
                .wrapping_sub(1_640_531_527);
        }
        hash
    }

    /// Batch 7: was: `prime_list_template<unsigned long>::value` — 38 bucket-count
    /// primes read from the binary at 0xFA7760 (via IDA MCP `py_eval`, `get_dword`
    /// walk). Starts at 17 in this build.
    pub const UNORDERED_PRIME_LIST: [u32; 38] = [
        17, 29, 37, 53, 67, 79, 97, 131, 193, 257, 389, 521, 769, 1031, 1543, 2053,
        3079, 6151, 12289, 24593, 49157, 98317, 196613, 393241, 786433, 1572869,
        3145739, 6291469, 12582917, 25165843, 50331653, 100663319, 201326611,
        402653189, 805306457, 1610612741, 3221225473, 4294967291,
    ];

    /// Batch 7: was: `table::min_buckets_for_size(size)` (IDA 0x262db0) —
    /// `floor(size / mlf)`, clamped to `0` past `4294967300.0`, `+1`, then the
    /// prime-list `lower_bound` (0x262e10 loop) with the end-clamp: when the need
    /// exceeds the largest prime the search runs past the end (`&unk_FA77F8`) and
    /// steps back one (0x262e2a-0x262e2c), returning the largest prime —
    /// preserved here, quirk and all.
    pub fn unordered_min_buckets_for_size(size: u64, mlf: f32) -> u32 {
        let need = (size as f64 / mlf as f64).floor();
        let want: u32 = if need < 4_294_967_300.0 {
            (need as u64).wrapping_add(1) as u32
        } else {
            // Binary-search `lower_bound(0)` lands on the first prime (0x262e10).
            0
        };
        let mut lo = 0;
        let mut hi = UNORDERED_PRIME_LIST.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if UNORDERED_PRIME_LIST[mid] < want {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        // End-clamp (IDA 0x262e2a): past-the-end steps back to the largest prime.
        UNORDERED_PRIME_LIST[lo.min(UNORDERED_PRIME_LIST.len() - 1)]
    }

    /// Batch 7: was: `table` construction state — bucket count at +4, size at +16,
    /// max load factor at +12 (`1065353216` = `1.0f`, IDA 0x263100).
    #[derive(Debug, Clone)]
    pub struct UnorderedTable {
        pub bucket_count: u32,
        pub size: usize,
        /// was: `*(float*)(table + 12)`; `1.0` observed at construction.
        pub max_load_factor: f32,
    }

    /// Batch 7, IDA 0x26309c `table::table(requested)` — flag byte clear, prime
    /// `lower_bound(requested)` with the same end-clamp, size zero, mlf `1.0`.
    pub fn unordered_table_construct(requested: u32) -> UnorderedTable {
        let mut lo = 0;
        let mut hi = UNORDERED_PRIME_LIST.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if UNORDERED_PRIME_LIST[mid] < requested {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        UnorderedTable {
            bucket_count: UNORDERED_PRIME_LIST[lo.min(UNORDERED_PRIME_LIST.len() - 1)],
            size: 0,
            max_load_factor: 1.0,
        }
    }

    /// Batch 7, IDA 0x262c88 `table::create_buckets(n)` — installs
    /// `max(current, min_buckets_for_size)` buckets. `HashMap` sizes itself, so
    /// the port records the count on the table and reserves the map.
    pub fn unordered_create_buckets(
        table: &mut UnorderedTable,
        map: &mut HashMap<String, usize>,
        n: usize,
    ) -> u32 {
        let want = unordered_min_buckets_for_size(n as u64, table.max_load_factor);
        if want > table.bucket_count {
            table.bucket_count = want;
        }
        map.reserve(n);
        table.bucket_count
    }

    /// Batch 7, IDA 0x262e40 `table_impl::rehash_impl(n)` — recompute buckets for
    /// the new size and relink (`place_in_bucket` per node, 0x262e6c mechanics).
    /// Node relinking collapses; the port reserves and reports the count.
    pub fn unordered_rehash(
        table: &mut UnorderedTable,
        map: &mut HashMap<String, usize>,
        n: usize,
    ) -> u32 {
        table.bucket_count = unordered_min_buckets_for_size(n as u64, table.max_load_factor);
        map.reserve(n);
        table.bucket_count
    }

    /// Batch 7: was: `node_constructor` state — the `+4` node word and the
    /// `+8/+9` use/flag bytes (IDA 0x262ec4).
    #[derive(Debug, Default)]
    pub struct NodeConstructor {
        pub node: Option<[u8; 16]>,
        pub flag: u8,
    }

    /// Batch 7, IDA 0x262ec4 `node_constructor::construct` — when a node is held,
    /// return its flag byte (clearing a set flag, 0x262ed0-0x262ed8); otherwise
    /// zero the word, alloc `0x10`, zero the alloc tail, set the use flag and
    /// return `1` (0x262ede-0x262ef6).
    pub fn node_constructor_construct(state: &mut NodeConstructor) -> u8 {
        if state.node.is_some() {
            let flag = state.flag;
            if state.flag != 0 {
                state.flag = 0;
                return 0;
            }
            return flag;
        }
        state.node = Some([0u8; 16]);
        state.flag = 1;
        1
    }

    /// Batch 7, IDA 0x262efc `find_node_impl(hash, key)` — `hash % bucket_count`
    /// selects the chain (0x262f1a); each link compares the stored hash (0x262f44)
    /// and, on match, `strcmp` content equality (0x262f50, the grounded
    /// `StringEqualPredicate` body); a stored hash from another bucket ends the
    /// walk (0x262f5e-0x262f64). Miss yields null (`None`).
    /// The port verifies the passed hash against `string_hash_predicate` — a
    /// mismatched hash looks in the wrong bucket and misses, exactly like the
    /// binary — then does the content lookup.
    pub fn unordered_find_node(
        map: &HashMap<String, usize>,
        hash: u32,
        key: &str,
    ) -> Option<usize> {
        if hash != string_hash_predicate(key.as_bytes()) {
            return None;
        }
        map.get(key).copied()
    }

    /// Batch 7, IDA 0x262e6c `place_in_bucket(node)` — intrusive bucket-list
    /// surgery (`hash % buckets` slot, head splice, 0x262e80-0x262eb0). Linking
    /// collapses; the KEY to VALUE association is the observable effect, so the
    /// port performs the insert and returns the displaced value, like the
    /// original's node return.
    pub fn unordered_place_in_bucket(
        map: &mut HashMap<String, usize>,
        key: &str,
        value: usize,
    ) -> Option<usize> {
        map.insert(key.to_string(), value)
    }

    /// Batch 7: shared container-ctor core behind 0x261830/0x261948/0x261a60/
    /// 0x261b78/0x261c90 (verified identical modulo type): zero the member vector
    /// (0x261854), build the hash table with the ctor bucket prime (0x2618a0 —
    /// `table(11, ...)` picks the prime-list entry for 11, i.e. 17), zero the
    /// sub-collection vector, link `[12] = base`, and when based run
    /// `mergeMembers` (0x2618c4) then register self in the base's `+36` sub-list
    /// (0x2618d6 `push_back`). Returns the slot (`return a1`).
    pub fn container_construct<'a>(
        child: &'a mut MemberContainer,
        store: &DescriptorStore,
        base: Option<&mut MemberContainer>,
    ) -> &'a mut MemberContainer {
        *child = MemberContainer::default();
        if let Some(base_container) = base {
            child.parent = Some(Box::new(MemberContainer {
                members: base_container.members.clone(),
                by_name: base_container.by_name.clone(),
                sub_collections: Vec::new(),
                hiding_hook: base_container.hiding_hook,
                parent: base_container.parent.clone(),
            }));
            let snapshot = child.parent.clone();
            if let Some(parent) = snapshot {
                merge_members(child, store, &parent);
            }
            // IDA 0x2618d6 `push_back(a2 + 36, this)` — register in the base's
            // sub-list. Stored as a construction-time snapshot: with caller-owned
            // containers there is no live back-link, so post-link declares on the
            // child are invisible through the parent (`[INFERENCE]` limitation,
            // noted rather than modeled).
            base_container.sub_collections.push(child.clone());
        }
        child
    }

    /// Batch 6: was: `MemberDescriptorContainer<CallbackDescriptor>::mergeMembers`
    /// (IDA 0x2625ac; the Property/Event/Function/Yield variants share the shape) —
    /// walk the `[12]` parent chain to the root, `declare`-ing every base member
    /// into the destination (0x2625bc-0x2625ca). The return (a base end pointer,
    /// 0x2625d2) is discarded at the ctor call site, so the port returns `()`.
    pub fn merge_members(
        dest: &mut MemberContainer,
        store: &DescriptorStore,
        base: &MemberContainer,
    ) {
        let mut current = Some(base);
        while let Some(container) = current {
            let snapshot: Vec<usize> = container.members.clone();
            for member in snapshot {
                declare(dest, store, member);
            }
            current = container.parent.as_deref();
        }
    }

    /// Batch 6: was: `MemberDescriptor` delete-half of D0 (IDA 0x260f78; D1 itself
    /// is empty, 0x260140). Frees the port-side descriptor box.
    pub fn member_descriptor_d0(desc: Box<MemberDescriptor>) {
        drop(desc);
    }

    /// Batch 6: was: `RBX::Reflection::Descriptor` init (IDA 0x261798) — vtable
    /// `off_12AF558`, `+4 = flag & 1` (0x2617b6), `+8 = tag` (0x2617ba),
    /// `+12 = Name::declare(name)` (0x2617c8), the `lockedDown` RBXCRASH gate
    /// (0x2617d0), and the `!name.empty()` ReleaseAssert under asserts (Descriptor.h:58).
    #[derive(Debug, Default)]
    pub struct DescriptorInit {
        pub vtable: &'static str,
        pub flag: bool,
        pub tag: u32,
        pub name: String,
    }

    /// Batch 6: was: `Descriptor::lockedDown` (IDA 0x2617d0).
    pub static DESCRIPTOR_LOCKED_DOWN: AtomicBool = AtomicBool::new(false);

    /// Batch 6, IDA 0x261798 `Descriptor::Descriptor(name, flag, tag)` — builds the
    /// init record in place (the original constructs at `a1` and returns it).
    pub fn descriptor_construct(name: &str, flag: bool, tag: u32) -> DescriptorInit {
        if DESCRIPTOR_LOCKED_DOWN.load(Ordering::SeqCst) {
            panic!("RBXCRASH: Descriptor::lockedDown");
        }
        if ASSERTS.load(Ordering::SeqCst) {
            assert!(
                !name.is_empty(),
                "!this->name.empty() file: include/reflection/Descriptor.h line: 58"
            );
        }
        DescriptorInit { vtable: "off_12AF558", flag, tag, name: name.to_string() }
    }

    /// Batch 6: was: `RBX::Reflection::ClassDescriptor` node — `Descriptor` base
    /// (name), the five member containers (+16/+68/+120/+172/+224), vtable
    /// `off_1221E58`, the derived-class vector (+280), base link (+292, the `+73`
    /// word the `isA` walks read), and the functionality nibble (+296).
    #[derive(Debug, Default)]
    pub struct ClassNode {
        pub name: String,
        pub order: u32,
        pub property: MemberContainer,
        pub event: MemberContainer,
        pub function: MemberContainer,
        pub yield_function: MemberContainer,
        pub callback: MemberContainer,
        pub derived_classes: Vec<usize>,
        pub base: Option<usize>,
        pub functionality: u8,
    }

    /// Batch 6: was: the class graph behind `isA`/`isMemberOf`/`allClasses` plus
    /// `ClassDescriptor::count` (++/-- in C2/D2, IDA 0x261436/0x26240c).
    #[derive(Debug, Default)]
    pub struct ClassHierarchy {
        pub classes: Vec<ClassNode>,
        pub root: Option<usize>,
        pub all: Vec<usize>,
        pub count: u32,
        pub next_order: u32,
    }

    /// Batch 6: was: the `staticData2`/`dword_131E3F8` process-global class list
    /// (IDA 0x2610ac/0x261592).
    pub static CLASS_HIERARCHY: OnceLock<Mutex<ClassHierarchy>> = OnceLock::new();

    /// Batch 6, IDA 0x2610ac `ClassDescriptor::allClasses` — `call_once` init
    /// (collapsed into `OnceLock`) then the global list address.
    pub fn all_classes() -> &'static Mutex<ClassHierarchy> {
        CLASS_HIERARCHY.get_or_init(|| Mutex::new(ClassHierarchy::default()))
    }

    impl ClassHierarchy {
        /// Batch 6, IDA 0x2616c0/0x2616cc `operator==` / `operator!=` — the params
        /// are `(this, other)`, so this is pointer identity, ported as index identity.
        pub fn class_eq(a: usize, b: usize) -> bool {
            a == b
        }
        /// Batch 6, IDA 0x2616d8 `isA(base)` — walk `this + 73` (parent) while the
        /// `+3` words differ (0x2616da-0x2616ea); identity compares the interned
        /// name key here.
        pub fn is_a(&self, mut class: usize, target: usize) -> bool {
            loop {
                if class == target {
                    return true;
                }
                match self.classes[class].base {
                    Some(parent) => class = parent,
                    None => return false,
                }
            }
        }
        /// Batch 6, IDA 0x2616f0 `isA(name)` — `string::compare` walk to the root.
        pub fn is_a_name(&self, mut class: usize, name: &str) -> bool {
            loop {
                if self.classes[class].name == name {
                    return true;
                }
                match self.classes[class].base {
                    Some(parent) => class = parent,
                    None => return false,
                }
            }
        }
        /// Batch 6, IDA 0x261718 `MemberDescriptor::isMemberOf(instance)` — the
        /// `instance != NULL` ReleaseAssert (reflection_object.cpp:139), then walk
        /// `*(class + 292)` (parent) from the instance's class to the member's
        /// owning class, stopping `0` at the root (0x26177c-0x26178e). A `None`
        /// instance models the null pointer.
        pub fn is_member_of(
            &self,
            owner: usize,
            instance_class: Option<usize>,
            root: usize,
        ) -> bool {
            if ASSERTS.load(Ordering::SeqCst) {
                assert!(
                    instance_class.is_some(),
                    "instance != NULL file: reflection_object.cpp line: 139"
                );
            }
            let mut class = match instance_class {
                Some(c) => c,
                None => return false,
            };
            loop {
                if class == owner {
                    return true;
                }
                if class == root {
                    return false;
                }
                match self.classes[class].base {
                    Some(parent) => class = parent,
                    None => return false,
                }
            }
        }
        /// Batch 6: ordered insert into the global `allClasses` list by the Name
        /// order key (IDA 0x26154e-0x261592 scan); returns the position.
        pub fn register_global_class(&mut self, class: usize) -> usize {
            if let Some(pos) = self.all.iter().position(|&c| c == class) {
                return pos;
            }
            let key = self.classes[class].order;
            let pos = self.all.iter().position(|&c| self.classes[c].order > key)
                .unwrap_or(self.all.len());
            self.all.insert(pos, class);
            pos
        }
        /// Batch 6, IDA 0x26113c root ctor — `Descriptor("<<<ROOT>>>", 0, 0)`, five
        /// fresh containers (+16/+68/+120/+172/+224), vtable `off_1221E58`,
        /// `+73 = 0` (no base), empty derived vector, functionality nibble `0xD`
        /// (0x261214: `+296 = +296 & 0xF0 | 0xD`).
        pub fn construct_root(&mut self) -> usize {
            let idx = self.classes.len();
            let order = self.next_order;
            self.next_order += 1;
            self.classes.push(ClassNode {
                name: "<<<ROOT>>>".to_string(),
                order,
                functionality: 0xD,
                ..Default::default()
            });
            self.root = Some(idx);
            self.register_global_class(idx);
            idx
        }
        /// Batch 6, IDA 0x26131c named C2 ctor — `Descriptor` base, five containers
        /// linked against the base's (`base + 16/68/120/172/224`, 0x261382-0x2613d4;
        /// null when baseless), vtable `off_1221E58`, `+276 = a7`,
        /// `+280/284/288 = 0`, `+292 = base`, functionality nibble merge
        /// (`(a6 >> 1) & 0xF | old & 0xF0`, 0x26142e), `count++`, the
        /// already-derived assert (reflection_object.cpp:70), insert into the
        /// base's derived list (0x2614e0), then the ordered global insert.
        /// Container `mergeMembers` runs inside the container ctors (batch 7).
        /// Returns the node index (`return a1`).
        pub fn construct_class(
            &mut self,
            name: &str,
            base: Option<usize>,
            tag276: u32,
            functionality: u8,
        ) -> usize {
            let idx = self.classes.len();
            let order = self.next_order;
            self.next_order += 1;
            if ASSERTS.load(Ordering::SeqCst) {
                if let Some(b) = base {
                    assert!(
                        !self.classes[b].derived_classes.contains(&idx),
                        "iter == base.derivedClasses.end() || *iter != this file: reflection_object.cpp line: 70"
                    );
                }
            }
            self.classes.push(ClassNode {
                name: name.to_string(),
                order,
                base,
                functionality: (functionality >> 1) & 0x0F,
                ..Default::default()
            });
            if let Some(b) = base {
                let pos = self.classes[b].derived_classes.iter()
                    .position(|&c| self.classes[c].order > order)
                    .unwrap_or(self.classes[b].derived_classes.len());
                self.classes[b].derived_classes.insert(pos, idx);
            }
            self.count += 1;
            self.register_global_class(idx);
            idx
        }
        /// Batch 6, IDA 0x2623e8 `~ClassDescriptor` — vtable restore `off_1221E58`,
        /// `count--`, per-container buffer + bucket frees (the five
        /// `delete_buckets` + vector deletes, 0x26240e-0x2624ac — `Drop` here),
        /// then member teardown. Clears the node in place.
        pub fn destroy_class(&mut self, class: usize) {
            self.count = self.count.saturating_sub(1);
            let node = &mut self.classes[class];
            node.property = MemberContainer::default();
            node.event = MemberContainer::default();
            node.function = MemberContainer::default();
            node.yield_function = MemberContainer::default();
            node.callback = MemberContainer::default();
            node.derived_classes.clear();
        }
    }
}
pub fn stub_25e9b0() -> bool {
    // IDA 0x25e9b0: MOVS R0, #0; BX LR — read/write-open pair, never write-only.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::GetSetImpl<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x25e9b4 — __ZNK3RBX10Reflection14PropDescriptorINS_9SpotLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_25e9b4(imp: *const prop_binding::GetSetImpl, obj: *const u8) -> f32 {
    // IDA 0x25e9b4: DescribedBase-36, (adj >> 1) adjust, virtual branch, tail-call getter.
    prop_binding::get_f32(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::GetSetImpl<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// 0x25e9d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9SpotLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_25e9d4(imp: *const prop_binding::GetSetImpl, obj: *mut u8, value: f32) {
    // IDA 0x25e9d4 (disasm LDR R1, [R2]; BX R3): setter(this, value); void result.
    prop_binding::set_f32(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::PropDescriptor<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>(char const*,char const*,float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x25e9f8 — __ZN3RBX10Reflection14PropDescriptorINS_10PointLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_25e9f8(
    slot: *mut prop_binding::PropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut prop_binding::PropDescriptor {
    // IDA 0x25e9f8: classDescriptor sink, new GetSetImpl (vtable off_122F3D8),
    // TypedPropertyDescriptor<float> init, *a1 = &off_122F388, return a1.
    prop_binding::PropDescriptor::construct(
        slot,
        name,
        category,
        prop_binding::MemberPtr { func: getter_func, adj: getter_adj },
        prop_binding::MemberPtr { func: setter_func, adj: setter_adj },
        prop_binding::POINT_LIGHT_FLOAT_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::~PropDescriptor()")]
// 0x25eb0c — __ZN3RBX10Reflection14PropDescriptorINS_10PointLightEfED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_25eb0c(slot: *mut prop_binding::PropDescriptor) {
    // IDA 0x25eb0c (disasm: vtable store, LDR R0, [R4,#0x28], delete-if-nonnull, delete a1):
    // restore TypedPropertyDescriptor<float> vtable (off_1270A68), drop owned box.
    // IDA int return is the delete artifact — no observable output.
    prop_binding::PropDescriptor::destroy(slot, prop_binding::TYPED_FLOAT_DESC_VTAB)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::GetSetImpl<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>::isReadOnly(void)const")]
// 0x25eb38 — __ZNK3RBX10Reflection14PropDescriptorINS_10PointLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// type: int()
pub fn stub_25eb38() -> bool {
    // IDA 0x25eb38: return 0 — read/write-open pair, never read-only.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::GetSetImpl<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>::isWriteOnly(void)const")]
// 0x25eb3c — __ZNK3RBX10Reflection14PropDescriptorINS_10PointLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
pub fn stub_25eb3c() -> bool {
    // IDA 0x25eb3c: return 0 — read/write-open pair, never write-only.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::GetSetImpl<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x25eb40 — __ZNK3RBX10Reflection14PropDescriptorINS_10PointLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_25eb40(imp: *const prop_binding::GetSetImpl, obj: *const u8) -> f32 {
    // IDA 0x25eb40: same dispatch shape as 0x25e9b4.
    prop_binding::get_f32(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::GetSetImpl<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// 0x25eb60 — __ZNK3RBX10Reflection14PropDescriptorINS_10PointLightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_25eb60(imp: *const prop_binding::GetSetImpl, obj: *mut u8, value: f32) {
    // IDA 0x25eb60: same dispatch shape as 0x25e9d4.
    prop_binding::set_f32(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::PropDescriptor<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>(char const*,char const*,float (RBX::Light::*)(void)const,void (RBX::Light::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x25eb84 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_25eb84(
    slot: *mut prop_binding::PropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut prop_binding::PropDescriptor {
    // IDA 0x25eb84: GetSetImpl vtable off_122F458, descriptor vtable off_122F408.
    prop_binding::PropDescriptor::construct(
        slot,
        name,
        category,
        prop_binding::MemberPtr { func: getter_func, adj: getter_adj },
        prop_binding::MemberPtr { func: setter_func, adj: setter_adj },
        prop_binding::LIGHT_FLOAT_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::~PropDescriptor()")]
// 0x25ec98 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEfED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_25ec98(slot: *mut prop_binding::PropDescriptor) {
    // IDA 0x25ec98: *a1 = &off_1270A68; delete a1[10]; delete a1.
    prop_binding::PropDescriptor::destroy(slot, prop_binding::TYPED_FLOAT_DESC_VTAB)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::GetSetImpl<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>::isReadOnly(void)const")]
// 0x25ecc4 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// type: int()
pub fn stub_25ecc4() -> bool {
    // IDA 0x25ecc4: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::GetSetImpl<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>::isWriteOnly(void)const")]
// 0x25ecc8 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
pub fn stub_25ecc8() -> bool {
    // IDA 0x25ecc8: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::GetSetImpl<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x25eccc — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_25eccc(imp: *const prop_binding::GetSetImpl, obj: *const u8) -> f32 {
    // IDA 0x25eccc: same dispatch shape as 0x25e9b4.
    prop_binding::get_f32(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::GetSetImpl<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// 0x25ecec — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_25ecec(imp: *const prop_binding::GetSetImpl, obj: *mut u8, value: f32) {
    // IDA 0x25ecec: same dispatch shape as 0x25e9d4.
    prop_binding::set_f32(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x25ed10 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_25ed10(
    slot: *mut prop_binding::PropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut prop_binding::PropDescriptor {
    // IDA 0x25ed10: GetSetImpl vtable off_122F4E8, descriptor vtable off_122F488,
    // TypedPropertyDescriptor<G3D::Color3> base init.
    prop_binding::PropDescriptor::construct(
        slot,
        name,
        category,
        prop_binding::MemberPtr { func: getter_func, adj: getter_adj },
        prop_binding::MemberPtr { func: setter_func, adj: setter_adj },
        prop_binding::LIGHT_COLOR3_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
// 0x25ee24 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_25ee24(slot: *mut prop_binding::PropDescriptor) {
    // IDA 0x25ee24: *a1 = &off_1270988 (TypedPropertyDescriptor<Color3>); delete a1[10]; delete a1.
    prop_binding::PropDescriptor::destroy(slot, prop_binding::TYPED_COLOR3_DESC_VTAB)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const")]
// 0x25ee50 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
pub fn stub_25ee50() -> bool {
    // IDA 0x25ee50: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const")]
// 0x25ee54 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
pub fn stub_25ee54() -> bool {
    // IDA 0x25ee54: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x25ee58 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_25ee58(
    out: *mut [f32; 3],
    imp: *const prop_binding::GetSetImpl,
    obj: *const u8,
) {
    // IDA 0x25ee58: (out, impl, obj) arg order; v4(a1, v6) writes Color3 to out.
    prop_binding::get_color3(out, &*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
// 0x25ee80 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_25ee80(
    imp: *const prop_binding::GetSetImpl,
    obj: *mut u8,
    value: *const [f32; 3],
) {
    // IDA 0x25ee80: 12-byte stack copy then setter(this, &tmp); void result.
    prop_binding::set_color3(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::PropDescriptor<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>(char const*,char const*,bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x25eebc — __ZN3RBX10Reflection14PropDescriptorINS_5LightEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_25eebc(
    slot: *mut prop_binding::PropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut prop_binding::PropDescriptor {
    // IDA 0x25eebc: GetSetImpl vtable off_122F578, descriptor vtable off_122F518,
    // TypedPropertyDescriptor<bool> base init.
    prop_binding::PropDescriptor::construct(
        slot,
        name,
        category,
        prop_binding::MemberPtr { func: getter_func, adj: getter_adj },
        prop_binding::MemberPtr { func: setter_func, adj: setter_adj },
        prop_binding::LIGHT_BOOL_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::~PropDescriptor()")]
// 0x25efd0 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEbED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_25efd0(slot: *mut prop_binding::PropDescriptor) {
    // IDA 0x25efd0: *a1 = &off_1222378 (TypedPropertyDescriptor<bool>); delete a1[10]; delete a1.
    prop_binding::PropDescriptor::destroy(slot, prop_binding::TYPED_BOOL_DESC_VTAB)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::GetSetImpl<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>::isReadOnly(void)const")]
// 0x25effc — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
pub fn stub_25effc() -> bool {
    // IDA 0x25effc: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::GetSetImpl<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>::isWriteOnly(void)const")]
// 0x25f000 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
// IDA 0x25f000: MOVS R0, #0; BX LR — read/write-open pair, never write-only.
pub fn stub_25f000() -> bool {
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::GetSetImpl<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x25f004 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_25f004(imp: *const prop_binding::GetSetImpl, obj: *const u8) -> bool {
    // IDA 0x25f004: DescribedBase-36, (adj >> 1) adjust, virtual branch, tail-call getter.
    prop_binding::get_bool(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::GetSetImpl<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// 0x25f028 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
pub unsafe fn stub_25f028(imp: *const prop_binding::GetSetImpl, obj: *mut u8, value: bool) {
    // IDA 0x25f028 (disasm LDR R1, [R2]; BX R3): setter(this, *a3); void result.
    prop_binding::set_bool(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::EventDescriptor::EventDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x25f54c — __ZN3RBX10Reflection15EventDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
// IDA 0x25f54c: `Descriptor` base + `Name::declare("Signals")` + class/member +
// `off_122F768` -> `off_122F5A8`, empty signatures, `Container::declare(a2 + 68)`.
// Returns the slot, like the original (`return a1`).
pub fn stub_25f54c<'a>(base: &'a mut member_registry::MemberDescriptorBase, name: &str, class: usize, member: u32, attr0: u32, attr1: u32, container: &mut member_registry::MemberContainer, store: &mut member_registry::DescriptorStore) -> &'a mut member_registry::MemberDescriptorBase {
    base.construct(name, member_registry::EVENT_CATEGORY, class, member, attr0, attr1, member_registry::EVENT_VTABLE, container, store);
    base
}

#[doc(alias = "RBX::Reflection::RemoteEventCommon::Attributes::deprecated(RBX::Reflection::RemoteEventCommon::Functionality,RBX::Reflection::MemberDescriptor const*)")]
// 0x25f66c — __ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE
// type: int __fastcall(int result, int, int)
// IDA 0x25f66c: `+8 = functionality; +0 = 1; +4 = member`; returns the slot.
pub fn stub_25f66c(attrs: &mut member_registry::RemoteEventAttributes, functionality: u32, member: u32) -> &mut member_registry::RemoteEventAttributes {
    attrs.set_deprecated(functionality, member)
}

#[doc(alias = "RBX::Reflection::EventSource::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")]
// 0x25f678 — __ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE
// type: int __fastcall(int, int)
// IDA 0x25f678: tail-calls the descriptor virtual at +20 with (desc, source).
pub unsafe fn stub_25f678(desc: *const u8, source: *const u8) {
    member_registry::process_remote_event(desc, source)
}

#[doc(alias = "RBX::Reflection::EventSource::raiseEventInvocation(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const*)")]
// 0x25f688 — __ZN3RBX10Reflection11EventSource20raiseEventInvocationERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EEPKNS_13SystemAddressE
// type: void()
// IDA 0x25f688: empty body.
pub fn stub_25f688() {
    member_registry::raise_event_invocation()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declare(RBX::Reflection::EventDescriptor*)")]
// 0x25f690 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
// IDA 0x25f690: `Container<EventDescriptor>::declare` — sorted insert, name map,
// sub-collection fan-out, global registration. Returns the registered index
// (addresses / loop residue in the original are discarded at call sites).
pub fn stub_25f690(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize) -> usize {
    member_registry::declare(container, store, desc)
}

#[doc(alias = "RBX::Reflection::EventDescriptor::~EventDescriptor()")]
// 0x25f810 — __ZN3RBX10Reflection15EventDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::EventDescriptor *__hidden this)
// IDA 0x25f810: `~EventDescriptor` — restore `off_122F5A8`, clear the +32 list.
pub fn stub_25f810(b: &mut member_registry::DescriptorBox) {
    member_registry::event_descriptor_d1(b)
}

#[doc(alias = "RBX::Reflection::EventDescriptor::isScriptable(void)const")]
// 0x25f838 — __ZNK3RBX10Reflection15EventDescriptor12isScriptableEv
// type: int __fastcall(RBX::Reflection::EventDescriptor *this)
// IDA 0x25f838: returns 1 — events are scriptable.
pub fn stub_25f838() -> bool {
    member_registry::event_is_scriptable()
}

#[doc(alias = "RBX::Reflection::EventDescriptor::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x25f840 — __ZNK3RBX10Reflection15EventDescriptor9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaIS5_EE
// type: int __fastcall(int, int, int)
// IDA 0x25f840: `sendEvent` — `ReleaseAssert(false)` (event.h:159) when asserts
// are on; otherwise returns the zero flag.
pub fn stub_25f840() -> i32 {
    member_registry::event_send_event()
}

#[doc(alias = "std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")]
// 0x25f898 — __ZNSt6vectorIPN3RBX10Reflection15EventDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
// IDA 0x25f898: `vector<EventDescriptor*>::insert` — end fast-path or slow path.
// Returns the position (element address in the original).
pub fn stub_25f898(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::descriptor_vec_insert(list, pos, value)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declareSub(RBX::Reflection::EventDescriptor*,RBX::Reflection::EventDescriptor*)")]
// 0x25f8d0 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
// IDA 0x25f8d0: `Container<EventDescriptor>::declareSub` — member.h:216/227
// asserts, lower-bound insert or replaceable overwrite, name-map register,
// hiding hook, sub-collection fan-out. Returns the registered index.
pub fn stub_25f8d0(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize, replaceable: usize) -> usize {
    member_registry::declare_sub(container, store, desc, replaceable)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::staticData(void)")]
// 0x25fa50 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv
// type: double *()
// IDA 0x25fa50: guard-var lazy init of the global `allDescriptors` list
// (the `__cxa_atexit` Collection dtor is `Drop` here).
pub fn stub_25fa50() -> &'static std::sync::Mutex<Vec<usize>> {
    member_registry::static_data()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::Collection::~Collection()")]
// 0x25fab8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
// IDA 0x25fab8: `Collection::~Collection` — free the owned slot when non-null.
pub fn stub_25fab8(slot: &mut Option<Box<[u8]>>) {
    member_registry::collection_d1(slot)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x25fad0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
// IDA 0x25fad0: `table_impl::operator[]` over `map<const char*, EventDescriptor*>` —
// content-hashed find-or-insert; bucket mechanics collapse into the entry.
// Returns the mapped slot, like the original mapped reference.
pub fn stub_25fad0(map: &mut std::collections::HashMap<String, usize>, key: *const std::os::raw::c_char) -> &mut usize {
    member_registry::unordered_index_or_insert(map, key)
}

#[doc(alias = "std::_List_base<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_clear(void)")]
// 0x260110 — __ZNSt10_List_baseIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
// IDA 0x260110: `std::_List_base<SignatureDescriptor::Item>::_M_clear` — per-node
// cleanup plus free, in order (`Drop` here); intrusive links collapse away.
pub fn stub_260110(list: &mut Vec<String>) {
    member_registry::signature_list_clear(list)
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::~MemberDescriptor()")]
// 0x260140 — __ZN3RBX10Reflection16MemberDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::MemberDescriptor *__hidden this)
// IDA 0x260140: `MemberDescriptor::~MemberDescriptor` — empty body.
pub fn stub_260140() {
    member_registry::member_descriptor_d1()
}

#[doc(alias = "RBX::Reflection::FunctionDescriptor::FunctionDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x260274 — __ZN3RBX10Reflection18FunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
// IDA 0x260274: `FunctionDescriptor` ctor — `Name::declare("Function")`, `off_122F768` -> `off_1222248`, `Container::declare(a2 + 120)`.
// Returns the slot, like the original (`return a1`).
pub fn stub_260274<'a>(base: &'a mut member_registry::MemberDescriptorBase, name: &str, class: usize, member: u32, attr0: u32, attr1: u32, container: &mut member_registry::MemberContainer, store: &mut member_registry::DescriptorStore) -> &'a mut member_registry::MemberDescriptorBase {
    base.construct(name, member_registry::FUNCTION_CATEGORY, class, member, attr0, attr1, member_registry::FUNCTION_VTABLE, container, store);
    base
}

#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::YieldFunctionDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x260394 — __ZN3RBX10Reflection23YieldFunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
// IDA 0x260394: `YieldFunctionDescriptor` ctor — `Name::declare("YieldFunction")`, `off_122F768` -> `off_122F5E8`, `Container::declare(a2 + 172)`.
// Returns the slot, like the original (`return a1`).
pub fn stub_260394<'a>(base: &'a mut member_registry::MemberDescriptorBase, name: &str, class: usize, member: u32, attr0: u32, attr1: u32, container: &mut member_registry::MemberContainer, store: &mut member_registry::DescriptorStore) -> &'a mut member_registry::MemberDescriptorBase {
    base.construct(name, member_registry::YIELD_FUNCTION_CATEGORY, class, member, attr0, attr1, member_registry::YIELD_FUNCTION_VTABLE, container, store);
    base
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declare(RBX::Reflection::FunctionDescriptor*)")]
// 0x2604b8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
// IDA 0x2604b8: `Container<FunctionDescriptor>::declare` — same shared core as 0x25f690.
pub fn stub_2604b8(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize) -> usize {
    member_registry::declare(container, store, desc)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")]
// 0x260638 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
// IDA 0x260638: `Container<YieldFunctionDescriptor>::declare` — same shared core as 0x25f690.
pub fn stub_260638(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize) -> usize {
    member_registry::declare(container, store, desc)
}

#[doc(alias = "RBX::Reflection::FunctionDescriptor::~FunctionDescriptor()")]
// 0x2607b8 — __ZN3RBX10Reflection18FunctionDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::FunctionDescriptor *__hidden this)
// IDA 0x2607b8: `~FunctionDescriptor` — restore `off_1222248`, clear the +32 list.
pub fn stub_2607b8(b: &mut member_registry::DescriptorBox) {
    member_registry::function_descriptor_d1(b)
}

#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::~YieldFunctionDescriptor()")]
// 0x2607e0 — __ZN3RBX10Reflection23YieldFunctionDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::YieldFunctionDescriptor *__hidden this)
// IDA 0x2607e0: `~YieldFunctionDescriptor` — restore `off_122F5E8`, clear the +32 list.
pub fn stub_2607e0(b: &mut member_registry::DescriptorBox) {
    member_registry::yield_function_descriptor_d1(b)
}

#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")]
// 0x260808 — __ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
// IDA 0x260808: `vector<YieldFunctionDescriptor*>::insert` — same end fast-path shape as 0x25f898.
pub fn stub_260808(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::descriptor_vec_insert(list, pos, value)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")]
// 0x260840 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
// IDA 0x260840: `Container<YieldFunctionDescriptor>::declareSub` — same shared core.
pub fn stub_260840(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize, replaceable: usize) -> usize {
    member_registry::declare_sub(container, store, desc, replaceable)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")]
// 0x2609c0 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv
// type: double *()
// IDA 0x2609c0: Yield `staticData` — same collapsed global list (see 0x25fa50 note).
pub fn stub_2609c0() -> &'static std::sync::Mutex<Vec<usize>> {
    member_registry::static_data()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::Collection::~Collection()")]
// 0x260a28 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
// IDA 0x260a28: Yield `Collection::~Collection` — free the owned slot.
pub fn stub_260a28(slot: &mut Option<Box<[u8]>>) {
    member_registry::collection_d1(slot)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x260a40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
// IDA 0x260a40: `table_impl::operator[]` over `map<const char*, YieldFunctionDescriptor*>`.
pub fn stub_260a40(map: &mut std::collections::HashMap<String, usize>, key: *const std::os::raw::c_char) -> &mut usize {
    member_registry::unordered_index_or_insert(map, key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x260bc8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
// IDA 0x260bc8: `table_impl::operator[]` over `map<const char*, FunctionDescriptor*>`.
pub fn stub_260bc8(map: &mut std::collections::HashMap<String, usize>, key: *const std::os::raw::c_char) -> &mut usize {
    member_registry::unordered_index_or_insert(map, key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x260d48 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
// IDA 0x260d48: `table::reserve_for_insert(n)` — rehash or create buckets;
// returns the capacity (`HashMap::reserve` is the observable contract).
pub fn stub_260d48(map: &mut std::collections::HashMap<String, usize>, additional: usize) -> usize {
    member_registry::unordered_reserve(map, additional)
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::~MemberDescriptor()")]
// 0x260f78 — __ZN3RBX10Reflection16MemberDescriptorD0Ev
// type: void __fastcall(RBX::Reflection::MemberDescriptor *__hidden this)
// IDA 0x260f78: `MemberDescriptor D0` — D1 is empty (0x260140), so just the delete.
// Ownership moves in, mirroring the deleting-destructor contract.
pub fn stub_260f78(desc: Box<member_registry::MemberDescriptor>) {
    member_registry::member_descriptor_d0(desc)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::allClasses(void)")]
// 0x2610ac — __ZN3RBX10Reflection15ClassDescriptor10allClassesEv
// type: _DWORD __fastcall(RBX::Reflection::ClassDescriptor *__hidden this)
// IDA 0x2610ac: `ClassDescriptor::allClasses` — `call_once` init, then the global list.
pub fn stub_2610ac() -> &'static std::sync::Mutex<member_registry::ClassHierarchy> {
    member_registry::all_classes()
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(void)")]
// 0x261138 — __ZN3RBX10Reflection15ClassDescriptorC1Ev
// type: int __fastcall(RBX::Reflection::ClassDescriptor *this)
// IDA 0x261138: root `ClassDescriptor C1` — thunk forwarding to C2.
pub fn stub_261138(h: &mut member_registry::ClassHierarchy) -> usize {
    stub_26113c(h)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(void)")]
// 0x26113c — __ZN3RBX10Reflection15ClassDescriptorC2Ev
// type: RBX::Reflection::ClassDescriptor *__fastcall(RBX::Reflection::ClassDescriptor *this)
// IDA 0x26113c: root C2 — `Descriptor("<<<ROOT>>>", 0, 0)`, five fresh containers,
// vtable `off_1221E58`, no base, functionality nibble `0xD`. Returns the node.
pub fn stub_26113c(h: &mut member_registry::ClassHierarchy) -> usize {
    h.construct_root()
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(RBX::Reflection::ClassDescriptor&,char const*,RBX::Reflection::ClassDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x261300 — __ZN3RBX10Reflection15ClassDescriptorC1ERS1_PKcNS1_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0x261300: named `ClassDescriptor C1` — stack-shuffling thunk into C2.
pub fn stub_261300(h: &mut member_registry::ClassHierarchy, name: &str, base: Option<usize>, tag276: u32, functionality: u8) -> usize {
    stub_26131c(h, name, base, tag276, functionality)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(RBX::Reflection::ClassDescriptor&,char const*,RBX::Reflection::ClassDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x26131c — __ZN3RBX10Reflection15ClassDescriptorC2ERS1_PKcNS1_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int)
// IDA 0x26131c: named C2 — `Descriptor` base, five base-linked containers, vtable
// `off_1221E58`, derived-list + global ordered registration, `count++`.
// (Container `mergeMembers` runs in the container ctors, batch 7.)
// Returns the node (`return a1`).
pub fn stub_26131c(h: &mut member_registry::ClassHierarchy, name: &str, base: Option<usize>, tag276: u32, functionality: u8) -> usize {
    let _ = tag276;
    h.construct_class(name, base, tag276, functionality)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::operator==(RBX::Reflection::ClassDescriptor const&)const")]
// 0x2616c0 — __ZNK3RBX10Reflection15ClassDescriptoreqERKS1_
// type: bool __fastcall(int, int)
// IDA 0x2616c0: `operator==` compares the two (this, other) pointers — index identity.
pub fn stub_2616c0(a: usize, b: usize) -> bool {
    member_registry::ClassHierarchy::class_eq(a, b)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::operator!=(RBX::Reflection::ClassDescriptor const&)const")]
// 0x2616cc — __ZNK3RBX10Reflection15ClassDescriptorneERKS1_
// type: bool __fastcall(int, int)
// IDA 0x2616cc: `operator!=` — negated identity.
pub fn stub_2616cc(a: usize, b: usize) -> bool {
    !member_registry::ClassHierarchy::class_eq(a, b)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::isA(RBX::Reflection::ClassDescriptor const&)const")]
// 0x2616d8 — __ZNK3RBX10Reflection15ClassDescriptor3isAERKS1_
// type: int __fastcall(RBX::Reflection::ClassDescriptor *this, const ClassDescriptor *)
// IDA 0x2616d8: `isA(base)` — parent-chain walk comparing the interned keys.
pub fn stub_2616d8(h: &member_registry::ClassHierarchy, class: usize, target: usize) -> bool {
    h.is_a(class, target)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::isA(char const*)const")]
// 0x2616f0 — __ZNK3RBX10Reflection15ClassDescriptor3isAEPKc
// type: int __fastcall(RBX::Reflection::ClassDescriptor *this, const char *)
// IDA 0x2616f0: `isA(name)` — `string::compare` walk to the root.
pub fn stub_2616f0(h: &member_registry::ClassHierarchy, class: usize, name: &str) -> bool {
    h.is_a_name(class, name)
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::isMemberOf(RBX::Reflection::DescribedBase const*)const")]
// 0x261718 — __ZNK3RBX10Reflection16MemberDescriptor10isMemberOfEPKNS0_13DescribedBaseE
// type: int __fastcall(RBX::Reflection::MemberDescriptor *this, const RBX::Reflection::DescribedBase *, int)
// IDA 0x261718: `isMemberOf(instance)` — null-instance assert, then the +292
// parent walk to the owning class (`0` at the root). `None` models null.
pub fn stub_261718(h: &member_registry::ClassHierarchy, owner: usize, instance_class: Option<usize>, root: usize) -> bool {
    h.is_member_of(owner, instance_class, root)
}

#[doc(alias = "RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")]
// 0x261798 — __ZN3RBX10Reflection10DescriptorC2EPKcNS1_10AttributesE
// type: int __fastcall(int, const char *const *, char, int)
// IDA 0x261798: `Descriptor::Descriptor(name, flag, tag)` — vtable `off_12AF558`,
// `lockedDown` crash gate, non-empty assert. Returns the init record (`return a1`).
pub fn stub_261798(name: &str, flag: bool, tag: u32) -> member_registry::DescriptorInit {
    member_registry::descriptor_construct(name, flag, tag)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")]
// 0x261830 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEEC2EPS3_
// type: _DWORD *__fastcall(_DWORD *, int)
// IDA 0x261830: `Container<PropertyDescriptor>` ctor — zeroed vec, `table(11)` map, sub-vec zero, `[12] = base`, merge + sub-list link when based.
// Returns the slot (`return a1`).
pub fn stub_261830<'a>(child: &'a mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: Option<&mut member_registry::MemberContainer>) -> &'a mut member_registry::MemberContainer {
    member_registry::container_construct(child, store, base)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")]
// 0x261948 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
// IDA 0x261948: `Container<EventDescriptor>` ctor — same shared shape as 0x261830.
// Returns the slot (`return a1`).
pub fn stub_261948<'a>(child: &'a mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: Option<&mut member_registry::MemberContainer>) -> &'a mut member_registry::MemberContainer {
    member_registry::container_construct(child, store, base)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")]
// 0x261a60 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
// IDA 0x261a60: `Container<FunctionDescriptor>` ctor — same shared shape as 0x261830.
// Returns the slot (`return a1`).
pub fn stub_261a60<'a>(child: &'a mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: Option<&mut member_registry::MemberContainer>) -> &'a mut member_registry::MemberContainer {
    member_registry::container_construct(child, store, base)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")]
// 0x261b78 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
// IDA 0x261b78: `Container<YieldFunctionDescriptor>` ctor — same shared shape as 0x261830.
// Returns the slot (`return a1`).
pub fn stub_261b78<'a>(child: &'a mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: Option<&mut member_registry::MemberContainer>) -> &'a mut member_registry::MemberContainer {
    member_registry::container_construct(child, store, base)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")]
// 0x261c90 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEEC2EPS3_
// type: _DWORD *__fastcall(_DWORD *, int)
// IDA 0x261c90: `Container<CallbackDescriptor>` ctor — same shared shape as 0x261830.
// Returns the slot (`return a1`).
pub fn stub_261c90<'a>(child: &'a mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: Option<&mut member_registry::MemberContainer>) -> &'a mut member_registry::MemberContainer {
    member_registry::container_construct(child, store, base)
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor **,std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>>,RBX::Reflection::ClassDescriptor * const&)")]
// 0x261da8 — __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
// IDA 0x261da8: `vector<ClassDescriptor*>::insert` — end fast-path or slow path.
pub fn stub_261da8(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::descriptor_vec_insert(list, pos, value)
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::~vector()")]
// 0x261de0 — __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EED1Ev
// type: void **__fastcall(void **)
// IDA 0x261de0: `vector<ClassDescriptor*>::~vector` — free a non-null buffer.
pub fn stub_261de0(list: &mut Vec<usize>) {
    member_registry::class_vec_destroy(list)
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
// 0x2623e8 — __ZN3RBX10Reflection15ClassDescriptorD2Ev
// type: void __fastcall(RBX::Reflection::ClassDescriptor *__hidden this)
// IDA 0x2623e8: `~ClassDescriptor` — vtable restore `off_1221E58`, `count--`,
// per-container buffer/bucket frees plus member teardown (`Drop` here).
pub fn stub_2623e8(h: &mut member_registry::ClassHierarchy, class: usize) {
    h.destroy_class(class)
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor **,std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>>,RBX::Reflection::ClassDescriptor * const&)")]
// 0x2624b4 — __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
// IDA 0x2624b4: `vector<ClassDescriptor*>::_M_insert_aux` — 1-or-double growth
// with the `length_error` cap, then allocate + move + insert. Returns the position.
pub fn stub_2624b4(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::class_vec_insert_aux(list, pos, value)
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_allocate(unsigned long)")]
// 0x262594 — __ZNSt12_Vector_baseIPN3RBX10Reflection15ClassDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// IDA 0x262594: `_Vector_base::_M_allocate(n)` — `bad_alloc` at `n >= 0x40000000`,
// else the `4 * n` byte count.
pub fn stub_262594(n: usize) -> usize {
    member_registry::vector_allocate(n)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")]
// 0x2625ac — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int result, int *)
// IDA 0x2625ac: `mergeMembers(base)` — follow the `[12]` parent chain, `declare`-ing
// every base member into the destination. (The Property/Event/Function/Yield
// variants share the shape.)
pub fn stub_2625ac(dest: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: &member_registry::MemberContainer) {
    member_registry::merge_members(dest, store, base)
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> * const&)")]
// 0x2625d4 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int result, _DWORD *)
// IDA 0x2625d4: container-pointer `push_back` — same fast/grow split as 0xb740.
pub fn stub_2625d4(list: &mut Vec<usize>, value: usize) {
    member_registry::container_ptr_vec_push_back(list, value)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::Collection::~Collection()")]
// 0x262600 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
// IDA 0x262600: Callback `Collection::~Collection` — free the owned slot.
pub fn stub_262600(slot: &mut Option<Box<[u8]>>) {
    member_registry::collection_d1(slot)
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> * const&)")]
// 0x262618 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: char *__fastcall(int, char *__src, _DWORD *)
// IDA 0x262618: container-pointer `_M_insert_aux` — same 1-or-double growth rule.
pub fn stub_262618(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::class_vec_insert_aux(list, pos, value)
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::_M_allocate(unsigned long)")]
// 0x2626f8 — __ZNSt12_Vector_baseIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// IDA 0x2626f8: container-pointer `_M_allocate(n)` — `bad_alloc` at `n >= 0x40000000`.
pub fn stub_2626f8(n: usize) -> usize {
    member_registry::vector_allocate(n)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")]
// 0x262710 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
// IDA 0x262710: `Container<CallbackDescriptor>::declare` — same shared core.
pub fn stub_262710(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize) -> usize {
    member_registry::declare(container, store, desc)
}

#[doc(alias = "std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::CallbackDescriptor **,std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>>,RBX::Reflection::CallbackDescriptor * const&)")]
// 0x262890 — __ZNSt6vectorIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
// IDA 0x262890: `vector<CallbackDescriptor*>::insert` — end fast-path or slow path.
pub fn stub_262890(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::descriptor_vec_insert(list, pos, value)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")]
// 0x2628c8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
// IDA 0x2628c8: `Container<CallbackDescriptor>::declareSub` — same shared core.
pub fn stub_2628c8(container: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, desc: usize, replaceable: usize) -> usize {
    member_registry::declare_sub(container, store, desc, replaceable)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::initStaticData(void)")]
// 0x262a44 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE14initStaticDataEv
// IDA 0x262a44: Callback `initStaticData` — thunk into `staticData`.
pub fn stub_262a44() -> &'static std::sync::Mutex<Vec<usize>> {
    member_registry::static_data()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")]
// 0x262a48 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv
// type: double *()
// IDA 0x262a48: Callback `staticData` — same collapsed global list (see 0x25fa50 note).
pub fn stub_262a48() -> &'static std::sync::Mutex<Vec<usize>> {
    member_registry::static_data()
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x262ab0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
// IDA 0x262ab0: `table_impl::operator[]` over `map<const char*, CallbackDescriptor*>`.
pub fn stub_262ab0(map: &mut std::collections::HashMap<String, usize>, key: *const std::os::raw::c_char) -> &mut usize {
    member_registry::unordered_index_or_insert(map, key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x262c34 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
// IDA 0x262c34: Callback `table::reserve_for_insert(n)` — rehash or create buckets.
pub fn stub_262c34(map: &mut std::collections::HashMap<String, usize>, additional: usize) -> usize {
    member_registry::unordered_reserve(map, additional)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x262c88 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
// IDA 0x262c88: Callback `table::create_buckets(n)` — install `max(current, min_buckets)`.
pub fn stub_262c88(table: &mut member_registry::UnorderedTable, map: &mut std::collections::HashMap<String, usize>, n: usize) -> u32 {
    member_registry::unordered_create_buckets(table, map, n)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x262db0 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
// IDA 0x262db0: `table::min_buckets_for_size(size)` — `floor(size / mlf)`, prime
// `lower_bound` with the past-the-end largest-prime clamp (binary-exact table
// at 0xFA7760). `mlf` is `1.0` at construction (IDA 0x263100).
pub fn stub_262db0(size: u64, mlf: f32) -> u32 {
    member_registry::unordered_min_buckets_for_size(size, mlf)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x262e40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
// IDA 0x262e40: Callback `table_impl::rehash_impl(n)` — recompute buckets, relink nodes.
pub fn stub_262e40(table: &mut member_registry::UnorderedTable, map: &mut std::collections::HashMap<String, usize>, n: usize) -> u32 {
    member_registry::unordered_rehash(table, map, n)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x262e6c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
// IDA 0x262e6c: `place_in_bucket(node)` — intrusive bucket splice; the observable
// KEY to VALUE association is the insert, whose displaced value is returned.
pub fn stub_262e6c(map: &mut std::collections::HashMap<String, usize>, key: &str, value: usize) -> Option<usize> {
    member_registry::unordered_place_in_bucket(map, key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)")]
// 0x262ec4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv
// type: int __fastcall(int)
// IDA 0x262ec4: `node_constructor::construct` — held-node flag protocol, else alloc
// `0x10`, zero tail, set use flag, return `1`.
pub fn stub_262ec4(state: &mut member_registry::NodeConstructor) -> u8 {
    member_registry::node_constructor_construct(state)
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x262efc — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, const char **)
// IDA 0x262efc: `find_node_impl(hash, key)` — `hash % buckets` chain, stored-hash
// check, `strcmp` equality; a foreign-bucket hash ends the walk. Miss is null.
// A hash that disagrees with the content misses, like the wrong bucket would.
pub fn stub_262efc(map: &std::collections::HashMap<String, usize>, hash: u32, key: &str) -> Option<usize> {
    member_registry::unordered_find_node(map, hash, key)
}

#[doc(alias = "RBX::Reflection::StringHashPredicate::operator()(char const*)const")]
// 0x262f6c — __ZNK3RBX10Reflection19StringHashPredicateclEPKc
// type: unsigned int __fastcall(int, char *__s)
// IDA 0x262f6c: `StringHashPredicate::operator()` — `h ^= (h << 6) + (h >> 2) +
// c - 1640531527` per byte, wrapping. The content hash of every name map here.
pub fn stub_262f6c(key: &[u8]) -> u32 {
    member_registry::string_hash_predicate(key)
}

#[doc(alias = "std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::CallbackDescriptor **,std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>>,RBX::Reflection::CallbackDescriptor * const&)")]
// 0x262fa4 — __ZNSt6vectorIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
// IDA 0x262fa4: `vector<CallbackDescriptor*>::_M_insert_aux` — same growth rule.
pub fn stub_262fa4(list: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    member_registry::class_vec_insert_aux(list, pos, value)
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_allocate(unsigned long)")]
// 0x263084 — __ZNSt12_Vector_baseIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// IDA 0x263084: Callback `_M_allocate(n)` — same `bad_alloc` gate.
pub fn stub_263084(n: usize) -> usize {
    member_registry::vector_allocate(n)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)")]
// 0x26309c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// type: int __fastcall(int result, unsigned int)
// IDA 0x26309c: Callback `table::table(requested)` — prime `lower_bound`, size
// zero, mlf `1.0f` (`1065353216`).
pub fn stub_26309c(requested: u32) -> member_registry::UnorderedTable {
    member_registry::unordered_table_construct(requested)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")]
// 0x263108 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int result, int *)
// IDA 0x263108: `Container<YieldFunctionDescriptor>::mergeMembers` — same
// parent-chain declare walk as 0x2625ac.
pub fn stub_263108(dest: &mut member_registry::MemberContainer, store: &member_registry::DescriptorStore, base: &member_registry::MemberContainer) {
    member_registry::merge_members(dest, store, base)
}
