//! core watchdog u — 150 core stubs EA-sorted, twenty-second gap filler — 150 core stubs EA-sorted, gap filler after 0x3c2000.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in core — next 150 uncovered after 0x3c2000 (watchdog_t max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// Batch 1: 22 IDA-grounded ports 0x3c202c-0x3c253c — the BillboardGui
/// `PropDescriptor<bool|UDim2|Vector2|Vector3>::GetSetImpl` cluster (predicates,
/// member-pointer getValue/setValue dispatch, descriptor ctor/dtor). Untouched
/// carriers keep stub bodies; ports live in `billboard_prop` under idiomatic
/// names, wired via `stub_3c20*`-`stub_3c25*`.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR`
/// carrier); member-function-pointer pairs -> `MemberPtr`; throws -> none (all
/// ported paths total except null-object misuse, matching the original).
/// `[INFERENCE]` marks what the binary does not pin down; everything else follows
/// the IDA pseudocode + disassembly branch-for-branch.
pub mod billboard_prop {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// was: `RBX::Reflection::DescribedBase` -> most-derived bias. Every getValue/setValue
    /// path computes `v = 0; if (a) v = a - 36` (IDA 0x3c2034/0x3c2058/0x3c21c4/0x3c21ec/
    /// 0x3c236c/0x3c239c/0x3c2508/0x3c253c/0x3c2b18/0x3c2b38 + disasm SUBNE.W R2, R1, #0x24).
    pub const DESCRIBED_BASE_BIAS: usize = 36;

    /// was: Itanium/ARM member-function-pointer word pair (`+4/+8` getter, `+12/+16` setter
    /// inside the 0x14-byte `GetSetImpl` box). `func` = direct target, else vtable offset;
    /// `adj` = `(this_delta << 1) | virtual_bit` (IDA `v4 >> 1`, `v4 & 1`, `TST.W R3, #1`).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MemberPtr {
        pub func: usize,
        pub adj: usize,
    }

    /// was: `GetSetImpl<Getter, Setter>` heap box (`operator new(0x14u)`, IDA 0x3c207c:
    /// `*v23 = &off_123FDF8; v23[1] = a4; v23[2] = a5; v23[3] = a6; v23[4] = a7`).
    #[derive(Debug, Clone, Copy)]
    pub struct GetSetImpl {
        pub getter: MemberPtr,
        pub setter: MemberPtr,
    }

    /// was: `GetSetImpl` vtables installed by each ctor (`*v23 = &off_...`).
    pub const UDIM2_GETSET_VTAB: &str = "off_123FDF8"; // IDA 0x3c207c
    pub const VECTOR2_GETSET_VTAB: &str = "off_123FE88"; // IDA 0x3c2224
    pub const VECTOR3_GETSET_VTAB: &str = "off_123FF18"; // IDA 0x3c23c0
    /// was: `PropDescriptor` vtables (`*a1 = &off_...` at each ctor tail).
    pub const UDIM2_DESC_VTAB: &str = "off_123FD98"; // IDA 0x3c207c
    pub const VECTOR2_DESC_VTAB: &str = "off_123FE28"; // IDA 0x3c2224
    pub const VECTOR3_DESC_VTAB: &str = "off_123FEB8"; // IDA 0x3c23c0
    /// was: `TypedPropertyDescriptor<T>` vtables restored by each dtor (`*a1 = &off_...`).
    pub const TYPED_UDIM2_DESC_VTAB: &str = "off_12603F8"; // IDA 0x3c2190
    pub const TYPED_VECTOR2_DESC_VTAB: &str = "off_128D9E8"; // IDA 0x3c2338
    pub const TYPED_VECTOR3_DESC_VTAB: &str = "off_1270B58"; // IDA 0x3c24d4

    /// was: `RBX::UDim2` (ScaleX/OffsetX/ScaleY/OffsetY, 16 bytes; IDA 0x3c21ec
    /// forwards `*a3, a3[1], a3[2], a3[3]`).
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    #[repr(C)]
    pub struct UDim2 {
        pub scale_x: f32,
        pub offset_x: i32,
        pub scale_y: f32,
        pub offset_y: i32,
    }

    /// was: `G3D::Vector2` (8 bytes; IDA 0x3c236c copies `*v8, v8[1]`).
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    #[repr(C)]
    pub struct Vector2 {
        pub x: f32,
        pub y: f32,
    }

    /// was: `G3D::Vector3` (12 bytes; IDA 0x3c2508 copies `*v8, v8[4], v8[8]`).
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    #[repr(C)]
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

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

    /// IDA virtual branch (`TST.W R3, #1` / `ITT NE` + indirect load):
    /// `target = *(vfunc_offset + *adjusted_this)`. Non-virtual: stored address (`BX R1`).
    pub unsafe fn resolve_target(mp: MemberPtr, this: *const u8) -> usize {
        if mp.adj & 1 != 0 {
            let vtable = *(this as *const usize);
            *((vtable.wrapping_add(mp.func)) as *const usize)
        } else {
            mp.func
        }
    }

    /// IDA 0x3c2034 bool `getValue`: `return v3(v5)`.
    pub unsafe fn get_bool(imp: &GetSetImpl, obj: *const u8) -> bool {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> bool =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(this)
    }

    /// IDA 0x3c2058 bool `setValue`: `v4(v6, *a3)`. The IDA `int` return carries no
    /// observable output, so the port returns `()`.
    pub unsafe fn set_bool(imp: &GetSetImpl, obj: *mut u8, value: bool) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, bool) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, value)
    }

    /// IDA 0x3c21c4 UDim2 `getValue`: `return v4(a1, v6)` — by-value UDim2 via
    /// hidden out-param `a1`. The IDA `int` return is the call artifact; the
    /// observable output is the `out` write.
    pub unsafe fn get_udim2(out: *mut UDim2, imp: &GetSetImpl, obj: *const u8) {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*mut UDim2, *const u8) =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(out, this)
    }

    /// IDA 0x3c21ec UDim2 `setValue`: 16-byte stack copy
    /// (`*a3, a3[1], a3[2], a3[3]`) then `v4(v6, ...)`. The copy is preserved via
    /// `tmp`; the IDA `int` return carries no observable output.
    pub unsafe fn set_udim2(imp: &GetSetImpl, obj: *mut u8, value: *const UDim2) {
        let tmp = *value;
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, *const UDim2) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, &tmp)
    }

    /// IDA 0x3c236c Vector2 `getValue`: getter returns `const Vector2&`
    /// (`v8 = v6(v7)`), then `*a1 = *v8; a1[1] = v8[1]; return v8[1]`. The port
    /// copies 8 bytes to `out` and returns the trailing word like the original.
    pub unsafe fn get_vector2(out: *mut Vector2, imp: &GetSetImpl, obj: *const u8) -> u32 {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> *const Vector2 =
            std::mem::transmute(resolve_target(imp.getter, this));
        let src = target(this);
        *out = *src;
        (*src).y.to_bits()
    }

    /// IDA 0x3c239c Vector2 `setValue`: `v4(v6, a3)` — the const-ref address is
    /// forwarded directly. The IDA `int` return carries no observable output.
    pub unsafe fn set_vector2(imp: &GetSetImpl, obj: *mut u8, value: *const Vector2) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, *const Vector2) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, value)
    }

    /// IDA 0x3c2508 Vector3 `getValue`: getter returns `const Vector3&`, then 12
    /// bytes are copied to the hidden out-param; trailing word returned.
    pub unsafe fn get_vector3(out: *mut Vector3, imp: &GetSetImpl, obj: *const u8) -> u32 {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> *const Vector3 =
            std::mem::transmute(resolve_target(imp.getter, this));
        let src = target(this);
        *out = *src;
        (*src).z.to_bits()
    }

    /// IDA 0x3c253c Vector3 `setValue`: `v4(v6, a3)` — const-ref forwarded directly.
    pub unsafe fn set_vector3(imp: &GetSetImpl, obj: *mut u8, value: *const Vector3) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, *const Vector3) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, value)
    }
    /// IDA 0x3d0b80 float `getValue`: `return v3(v5)` — same member dispatch
    /// as the Instance* `getValue` (0x3d09f4), float result.
    pub unsafe fn get_float(imp: &GetSetImpl, obj: *const u8) -> f32 {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> f32 =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(this)
    }

    /// IDA 0x3d0ba0 float `setValue`: `v4(v6, *a3)` — the const-ref is
    /// dereferenced by the caller glue (`LDR R1, [R2]`), forwarded by value.
    pub unsafe fn set_float(imp: &GetSetImpl, obj: *mut u8, value: f32) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, f32) =
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
        /// IDA 0x3c207c/0x3c2224/0x3c23c0 ctor shape: ensure classDescriptor
        /// (lazy-static registry init, a process-global sink here), `new GetSetImpl`
        /// from the (ptr, adj) pairs, `TypedPropertyDescriptor::init` base fields,
        /// then install the `PropDescriptor` vtable. Returns `slot` (IDA `return a1`).
        /// The transient `v31` box (`if (v31) operator delete(v31)`) collapses into
        /// the owned `Box` move.
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

        /// IDA 0x3c2190/0x3c2338/0x3c24d4 deleting-dtor shape:
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
        extern "C" fn fake_bget(this: *const u8) -> bool {
            unsafe { *(this.add(36) as *const bool) }
        }
        extern "C" fn fake_bset(this: *mut u8, v: bool) {
            unsafe { *(this.add(36) as *mut bool) = v }
        }
        extern "C" fn fake_fget(this: *const u8) -> f32 {
            unsafe { *(this.add(36) as *const f32) }
        }
        extern "C" fn fake_fset(this: *mut u8, v: f32) {
            unsafe { *(this.add(36) as *mut f32) = v }
        }
        extern "C" fn fake_uget(out: *mut UDim2, this: *const u8) {
            unsafe { *out = *(this.add(36) as *const UDim2) }
        }
        extern "C" fn fake_uset(this: *mut u8, v: *const UDim2) {
            unsafe { *(this.add(36) as *mut UDim2) = *v }
        }
        extern "C" fn fake_v2get(this: *const u8) -> *const Vector2 {
            unsafe { (this.add(36) as *const Vector2) as *const Vector2 }
        }
        extern "C" fn fake_v2set(this: *mut u8, v: *const Vector2) {
            unsafe { *(this.add(36) as *mut Vector2) = *v }
        }
        extern "C" fn fake_v3get(this: *const u8) -> *const Vector3 {
            unsafe { (this.add(36) as *const Vector3) as *const Vector3 }
        }
        extern "C" fn fake_v3set(this: *mut u8, v: *const Vector3) {
            unsafe { *(this.add(36) as *mut Vector3) = *v }
        }

        #[repr(C)]
        struct FakeBool {
            pad: [u8; 36],
            b: bool,
        }

        #[repr(C)]
        struct FakeUDim2 {
            pad: [u8; 36],
            u: UDim2,
        }

        #[repr(C)]
        struct FakeV2 {
            pad: [u8; 36],
            v: Vector2,
        }

        #[repr(C)]
        struct FakeV3 {
            pad: [u8; 36],
            v: Vector3,
        }

        #[repr(C)]
        struct FakeFloat {
            pad: [u8; 36],
            f: f32,
        }

        fn described_of(fake: *const u8) -> *const u8 {
            fake.wrapping_add(DESCRIBED_BASE_BIAS)
        }

        fn direct(f: usize) -> MemberPtr {
            MemberPtr { func: f, adj: 0 }
        }

        #[test]
        fn bool_get_set_roundtrip_direct() {
            let mut fake = FakeBool { pad: [0; 36], b: true };
            let base = std::ptr::addr_of!(fake) as *const u8;
            let imp = GetSetImpl { getter: direct(fake_bget as usize), setter: direct(fake_bset as usize) };
            unsafe {
                assert_eq!(get_bool(&imp, described_of(base)), true);
                set_bool(&imp, described_of(base) as *mut u8, false);
            }
            assert_eq!(fake.b, false);
        }
        #[test]
        fn float_get_set_roundtrip_direct() {
            let mut fake = FakeFloat { pad: [0; 36], f: 1.5 };
            let base = std::ptr::addr_of!(fake) as *const u8;
            let imp = GetSetImpl { getter: direct(fake_fget as usize), setter: direct(fake_fset as usize) };
            unsafe {
                assert_eq!(get_float(&imp, described_of(base)), 1.5);
                set_float(&imp, described_of(base) as *mut u8, 2.5);
            }
            assert_eq!(fake.f, 2.5);
        }

        #[test]
        fn udim2_roundtrip_direct() {
            let mut fake = FakeUDim2 {
                pad: [0; 36],
                u: UDim2 { scale_x: 1.0, offset_x: 2, scale_y: 3.0, offset_y: 4 },
            };
            let base = std::ptr::addr_of!(fake) as *const u8;
            let uimp = GetSetImpl { getter: direct(fake_uget as usize), setter: direct(fake_uset as usize) };
            unsafe {
                let mut u = UDim2::default();
                get_udim2(&mut u, &uimp, described_of(base));
                assert_eq!(u, fake.u);
                let nv = UDim2 { scale_x: 9.0, offset_x: 8, scale_y: 7.0, offset_y: 6 };
                set_udim2(&uimp, described_of(base) as *mut u8, &nv);
                assert_eq!(fake.u, nv);
            }
        }

        #[test]
        fn vector2_roundtrip_direct() {
            let mut fake = FakeV2 { pad: [0; 36], v: Vector2 { x: 5.0, y: 6.0 } };
            let base = std::ptr::addr_of!(fake) as *const u8;
            let v2imp = GetSetImpl { getter: direct(fake_v2get as usize), setter: direct(fake_v2set as usize) };
            unsafe {
                let mut v2 = Vector2::default();
                let tail = get_vector2(&mut v2, &v2imp, described_of(base));
                assert_eq!(v2, fake.v);
                assert_eq!(tail, 6.0f32.to_bits());
                let nv2 = Vector2 { x: 1.5, y: 2.5 };
                set_vector2(&v2imp, described_of(base) as *mut u8, &nv2);
                assert_eq!(fake.v, nv2);
            }
        }

        #[test]
        fn vector3_roundtrip_direct() {
            let mut fake = FakeV3 { pad: [0; 36], v: Vector3 { x: 7.0, y: 8.0, z: 9.0 } };
            let base = std::ptr::addr_of!(fake) as *const u8;
            let v3imp = GetSetImpl { getter: direct(fake_v3get as usize), setter: direct(fake_v3set as usize) };
            unsafe {
                let mut v3 = Vector3::default();
                let tail = get_vector3(&mut v3, &v3imp, described_of(base));
                assert_eq!(v3, fake.v);
                assert_eq!(tail, 9.0f32.to_bits());
                let nv3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
                set_vector3(&v3imp, described_of(base) as *mut u8, &nv3);
                assert_eq!(fake.v, nv3);
            }
        }

        #[test]
        fn virtual_bit_dispatches_through_vtable() {
            #[repr(C)]
            struct FakeVirtual {
                vtab: usize,
                _pad: [u8; 28],
                val: bool,
            }
            let mut fake = FakeVirtual { vtab: 0, _pad: [0; 28], val: true };
            let vtable: [usize; 1] = [fake_bget as usize];
            fake.vtab = vtable.as_ptr() as usize;
            let base = std::ptr::addr_of!(fake) as *const u8;
            let imp = GetSetImpl {
                getter: MemberPtr { func: 0, adj: 1 },
                setter: direct(fake_bset as usize),
            };
            unsafe {
                assert_eq!(get_bool(&imp, described_of(base)), true);
            }
        }
    }
}
/// Batch 2: 21 IDA-grounded ports 0x3c2560-0x3c39b4 — the BillboardGui
/// `RefPropDescriptor<BillboardGui, Instance>` cluster (ctor/dtor, box-delegated
/// predicates, variant/copy/xml/binder/idref paths, Instance* `GetSetImpl`) plus
/// `Camera::getCameraSubjectInstanceDangerous/setCameraSubject`. Ports live in
/// `billboard_ref` and `camera_subject`, wired via `stub_3c25*`-`stub_3c39b4`.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR`
/// carrier); `boost::detail::shared_count` copies/releases -> no-ops (host has no
/// intrusive control block); `__dynamic_cast` -> hooks/predicates; throws
/// (`std::bad_cast`) -> `BadCast` error return. `[INFERENCE]` marks what the
/// binary does not pin down; everything else follows the IDA pseudocode
/// branch-for-branch.
pub mod billboard_ref {
    use super::billboard_prop::{GetSetImpl, MemberPtr, resolve_target, resolve_this};
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// was: `RefPropDescriptor` vtable (`*a1 = &off_123FF48`, IDA 0x3c2560/0x3c2604).
    pub const REF_DESC_VTAB: &str = "off_123FF48"; // IDA 0x3c2560
    /// was: `RefPropDescriptor` sub-table (`a1[10] = &off_123FF9C`, IDA 0x3c2560/0x3c2604).
    pub const REF_SUB_VTAB: &str = "off_123FF9C"; // IDA 0x3c2560
    /// was: `GetSetImpl` vtable inside the `a1[11]` box (`*v16 = &off_123FFC8`).
    pub const REF_GETSET_VTAB: &str = "off_123FFC8"; // IDA 0x3c2560
    /// was: `RBX::Reflection::RefType<RBX::Instance *>::singleton` result `type`
    /// passed to `PropertyDescriptor::PropertyDescriptor` (IDA 0x3c2560).
    pub const REF_TYPE_SINGLETON: &str = "RBX::Reflection::RefType<RBX::Instance *>::singleton";
    /// was: `RBX::Reflection::Type::getSingleton<boost::shared_ptr<DescribedBase>>`
    /// stored into the out-Variant (IDA 0x3c267c).
    pub const SHARED_PTR_DESCRIBED_TYPE: &str =
        "Type::getSingleton<boost::shared_ptr<RBX::Reflection::DescribedBase>>";

    /// Pointer direction: the `DescribedBase` subobject sits 36 bytes past the
    /// most-derived `Instance` (`derived = base - 36`, `base = derived + 36`).
    /// Every conversion below matches an IDA `+36`/`-36` step.
    pub const BASE_TO_DERIVED: usize = 36;

    /// was: `RBX::Reflection::RefPropDescriptor<T, U>` storage. Words mirror the
    /// ctor: `a1[10]` sub-table, `a1[11]` owned `GetSetImpl` box (freed by the dtor).
    /// Trailing words: `[INFERENCE]` same `attributes`/`permissions` passthrough
    /// as `billboard_prop::PropDescriptor` (ctor args a8..a11).
    #[derive(Debug, Default)]
    pub struct RefPropDescriptor {
        pub vtable: &'static str,
        pub sub_vtable: &'static str,
        pub name: String,
        pub category: String,
        pub getset: Option<Box<GetSetImpl>>,
        pub attributes: (u32, u32, u32),
        pub permissions: u32,
    }

    impl RefPropDescriptor {
        /// IDA 0x3c2560 ctor shape: ensure classDescriptor, `RefType<Instance*>`
        /// singleton, `PropertyDescriptor` base init, install `off_123FF48` +
        /// `off_123FF9C`, `new GetSetImpl(off_123FFC8, ...)` into `a1[11]`.
        /// Returns `slot` (IDA `return a1`).
        pub unsafe fn construct(
            slot: *mut RefPropDescriptor,
            name: *const c_char,
            category: *const c_char,
            getter: MemberPtr,
            setter: MemberPtr,
            attr0: u32,
            attr1: u32,
            attr2: u32,
            permissions: u32,
        ) -> *mut RefPropDescriptor {
            let getset = Box::new(GetSetImpl { getter, setter });
            let this = &mut *slot;
            this.vtable = REF_DESC_VTAB;
            this.sub_vtable = REF_SUB_VTAB;
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
            slot
        }
        /// Same ctor shape with caller vtables (IDA 0x3d043c Camera
        /// `RefPropDescriptor`: `*a1 = &off_1240708`, `a1[10] = &off_124075C`,
        /// `new GetSetImpl(off_12407C8, ...)` into `a1[11]`).
        /// Returns `slot` (IDA `return a1`).
        pub unsafe fn construct_with(
            slot: *mut RefPropDescriptor,
            vtable: &'static str,
            sub_vtable: &'static str,
            name: *const c_char,
            category: *const c_char,
            getter: MemberPtr,
            setter: MemberPtr,
            attr0: u32,
            attr1: u32,
            attr2: u32,
            permissions: u32,
        ) -> *mut RefPropDescriptor {
            let out = Self::construct(
                slot, name, category, getter, setter, attr0, attr1, attr2, permissions,
            );
            let this = &mut *slot;
            this.vtable = vtable;
            this.sub_vtable = sub_vtable;
            out
        }

        /// IDA 0x3c2604 deleting-dtor shape: restore `off_123FF48` + `off_123FF9C`,
        /// delete the `a1[11]` box. Slot stays caller-owned (IDA `delete a1` artifact).
        pub unsafe fn destroy(slot: *mut RefPropDescriptor) {
            Self::destroy_with(slot, REF_DESC_VTAB, REF_SUB_VTAB)
        }

        /// Same shape with caller vtables (e.g. IDA 0x3c8c84 Camera `RefPropDescriptor`:
        /// `*a1 = &off_1240708`, `a1[10] = &off_124075C`, delete `a1[11]`).
        pub unsafe fn destroy_with(
            slot: *mut RefPropDescriptor,
            vtable: &'static str,
            sub_vtable: &'static str,
        ) {
            let this = &mut *slot;
            this.vtable = vtable;
            this.sub_vtable = sub_vtable;
            this.getset = None;
        }

        unsafe fn imp(&self) -> &GetSetImpl {
            self.getset.as_ref().expect("RefPropDescriptor box deleted")
        }
    }

    /// IDA box `+8` slot (`getValue`): DescribedBase-36, `(adj >> 1)` adjust,
    /// virtual branch, tail-call getter. Returns the derived `Instance*`
    /// (whatever the member getter yields).
    pub unsafe fn get_raw(imp: &GetSetImpl, obj: *const u8) -> *mut u8 {
        let this = resolve_this(obj, imp.getter.adj);
        let target: extern "C" fn(*const u8) -> *mut u8 =
            std::mem::transmute(resolve_target(imp.getter, this));
        target(this)
    }

    /// IDA box `+12` slot (`setValue`): `setter(this, value)`; void result.
    pub unsafe fn set_raw(imp: &GetSetImpl, obj: *mut u8, value: *mut u8) {
        let this = resolve_this(obj as *const u8, imp.setter.adj);
        let target: extern "C" fn(*mut u8, *mut u8) =
            std::mem::transmute(resolve_target(imp.setter, this));
        target(this as *mut u8, value)
    }

    /// IDA 0x3c2634: delegates to the box `+0` (`isReadOnly`), which for the
    /// Instance* `GetSetImpl` is 0x3c2b10 (`return 0`).
    pub unsafe fn is_read_only(desc: &RefPropDescriptor) -> bool {
        let _ = desc.imp();
        false
    }

    /// IDA 0x3c2644: delegates to the box `+4` (`isWriteOnly`), which for the
    /// Instance* `GetSetImpl` is 0x3c2b14 (`return 0`).
    pub unsafe fn is_write_only(desc: &RefPropDescriptor) -> bool {
        let _ = desc.imp();
        false
    }

    /// IDA 0x3c2654: `getValue(box, a2) == getValue(box, a3)`. (The first call's
    /// `a2` is elided in the IDA display; the second call shows the shape.)
    pub unsafe fn equal_values(
        desc: &RefPropDescriptor,
        a: *const u8,
        b: *const u8,
    ) -> bool {
        get_raw(desc.imp(), a) == get_raw(desc.imp(), b)
    }

    /// was: `RBX::Reflection::Variant` holding `shared_ptr<DescribedBase>`.
    /// `ptr` is the base (`derived + 36`), null when empty; `type_tag` is the
    /// `Type::getSingleton` identity. `[INFERENCE]` the host keeps the raw base
    /// instead of an intrusive shared count (refcount traffic is a no-op here).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct RefVariant {
        pub type_tag: &'static str,
        pub ptr: *const u8,
    }

    /// IDA 0x3c267c: `v18 = getValue(box, a2)` (derived); `shared_from` (refcount
    /// traffic, no-op here); `v19 = v21 ? v21 + 36 : 0` (derived -> base);
    /// out-Variant = (DescribedBase-shared-ptr singleton, shared value).
    pub unsafe fn get_variant(desc: &RefPropDescriptor, obj: *const u8) -> RefVariant {
        let derived = get_raw(desc.imp(), obj);
        let base = if derived.is_null() {
            derived as *const u8
        } else {
            derived.add(BASE_TO_DERIVED) as *const u8
        };
        RefVariant { type_tag: SHARED_PTR_DESCRIBED_TYPE, ptr: base }
    }

    /// IDA 0x3c2794: `Variant::get<shared_ptr<DescribedBase>>(&v13, a3)` then the
    /// descriptor `+64` setter `(a1, a2, v13)`; release traffic no-op here. The
    /// stored base converts back to derived (`- 36`) for the box setter.
    pub unsafe fn set_variant(desc: &RefPropDescriptor, obj: *mut u8, variant: &RefVariant) {
        let base = variant.ptr;
        let derived = if base.is_null() {
            base as *mut u8
        } else {
            base.sub(BASE_TO_DERIVED) as *mut u8
        };
        set_raw(desc.imp(), obj, derived)
    }

    /// IDA 0x3c285c: `v6 = getValue(box, a2)` (elided arg as in 0x3c2654);
    /// `setValue(box, a3, &v6)`. The IDA `int` return is the call artifact.
    pub unsafe fn copy_value(desc: &RefPropDescriptor, src: *const u8, dst: *mut u8) {
        let v = get_raw(desc.imp(), src);
        set_raw(desc.imp(), dst, v)
    }

    /// was: `XmlElement` — only the `+12` name/value slot is observed
    /// (`XmlNameValuePair::setValue(a3 + 12, ...)`).
    #[repr(C)]
    #[derive(Debug, Default)]
    pub struct XmlElement {
        pub _pad: [u8; 12],
        pub value: *const u8,
    }

    /// IDA 0x3c2880: `v12 = getValue(box, a2)` (derived); `v13 = v12 ? v12 + 36`
    /// (derived -> base); `InstanceHandle(base)`; `setValue(a3 + 12, handle)`.
    pub unsafe fn write_value(
        desc: &RefPropDescriptor,
        obj: *const u8,
        xml: *mut XmlElement,
    ) {
        let derived = get_raw(desc.imp(), obj);
        let base = if derived.is_null() {
            derived as *const u8
        } else {
            derived.add(BASE_TO_DERIVED) as *const u8
        };
        (*xml).value = base;
    }

    /// was: `RBX::IReferenceBinder` — the single observed slot `(*a4 + 4)` is
    /// `(binder, value_slot_or_null, obj, desc + 40)`.
    pub trait ReferenceBinder {
        unsafe fn bind(
            &self,
            value_slot: *const u8,
            obj: *mut u8,
            desc_cookie: *const u8,
        ) -> i32;
    }

    /// IDA 0x3c2954: `if (a3) a3 += 12; return binder[4](a4, a3, a2, a1 + 40)`.
    pub unsafe fn read_value(
        desc: &RefPropDescriptor,
        obj: *mut u8,
        xml: *const XmlElement,
        binder: &dyn ReferenceBinder,
    ) -> i32 {
        let slot = if xml.is_null() {
            std::ptr::null()
        } else {
            &(*xml).value as *const *const u8 as *const u8
        };
        let cookie = (desc as *const RefPropDescriptor as *const u8).add(40);
        binder.bind(slot, obj, cookie)
    }

    /// IDA 0x3c2978: `result = getValue(box)` (elided obj as above);
    /// `if (result) result += 36` (derived -> base).
    pub unsafe fn get_ref_value(desc: &RefPropDescriptor, obj: *const u8) -> *const u8 {
        let derived = get_raw(desc.imp(), obj);
        if derived.is_null() {
            derived as *const u8
        } else {
            derived.add(BASE_TO_DERIVED) as *const u8
        }
    }

    /// was: `std::bad_cast` thrown when a non-null `DescribedBase*` fails
    /// `__dynamic_cast` to `RBX::Instance*` (IDA 0x3c298c).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BadCast;

    /// IDA 0x3c298c: null passes through; non-null `__dynamic_cast`s to
    /// `Instance*` (`[INFERENCE]` the host cannot RTTI the raw pointer, so a
    /// caller-provided `is_instance` predicate stands in for the cast);
    /// `setValue(box, a2, &derived)`. The IDA `int` return is the call artifact.
    pub unsafe fn set_ref_value(
        desc: &RefPropDescriptor,
        obj: *mut u8,
        src: *const u8,
        is_instance: fn(*const u8) -> bool,
    ) -> Result<(), BadCast> {
        let derived = if src.is_null() {
            src as *mut u8
        } else if is_instance(src) {
            src.sub(BASE_TO_DERIVED) as *mut u8
        } else {
            return Err(BadCast);
        };
        set_raw(desc.imp(), obj, derived);
        Ok(())
    }

    /// IDA 0x3c2a08: `v3 = a3 ? a3 - 36 : 0` (base -> derived, no cast);
    /// `setValue(box, a2, &v5)`.
    pub unsafe fn set_ref_value_unsafe(
        desc: &RefPropDescriptor,
        obj: *mut u8,
        src: *const u8,
    ) {
        let derived = if src.is_null() {
            src as *mut u8
        } else {
            src.sub(BASE_TO_DERIVED) as *mut u8
        };
        set_raw(desc.imp(), obj, derived)
    }

    /// was: `RBX::InstanceHandle` — only the first word (`a3->pi_`) and the
    /// `a3 + 1` count copy are observed (IDA 0x3c2a28). `[INFERENCE]` host layout
    /// keeps just those two words.
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct InstanceHandle {
        pub pi: *const u8,
        pub _count: usize,
    }

    /// IDA 0x3c2a28: copy the `a3 + 1` count (no-op here), `v22 = pi ? pi - 36`;
    /// `setValue(box, a2, &v24)`; release traffic no-op here.
    pub unsafe fn assign_idref(
        desc: &RefPropDescriptor,
        obj: *mut u8,
        handle: *const InstanceHandle,
    ) {
        let pi = (*handle).pi;
        let v = if pi.is_null() {
            pi as *mut u8
        } else {
            pi.sub(BASE_TO_DERIVED) as *mut u8
        };
        set_raw(desc.imp(), obj, v)
    }

    /// IDA 0x3c2b08 non-virtual thunk: `return assignIDREF(a1 - 40, ...)`.
    pub unsafe fn assign_idref_thunk(
        this_adj: *const u8,
        obj: *mut u8,
        handle: *const InstanceHandle,
    ) {
        let desc = this_adj.sub(40) as *const RefPropDescriptor;
        assign_idref(&*desc, obj, handle)
    }

    /// IDA 0x3c2b18 Instance* `getValue`: DescribedBase-36, `(adj >> 1)` adjust,
    /// virtual branch, tail-call getter (returns derived `Instance*`).
    pub unsafe fn get_instance(imp: &GetSetImpl, obj: *const u8) -> *mut u8 {
        get_raw(imp, obj)
    }

    /// IDA 0x3c2b38 Instance* `setValue`: `setter(this, *a3)`; void result.
    pub unsafe fn set_instance(imp: &GetSetImpl, obj: *mut u8, value: *mut u8) {
        set_raw(imp, obj, value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::generated_core_watchdog_c::billboard_prop::{
            DESCRIBED_BASE_BIAS, MemberPtr,
        };

        extern "C" fn fake_rget(this: *const u8) -> *mut u8 {
            // Member slot 36 past `this` (ARM 4-aligned); the x86_64 host needs
            // unaligned access to emulate that layout exactly.
            unsafe { std::ptr::read_unaligned(this.add(36) as *const *mut u8) }
        }
        extern "C" fn fake_rset(this: *mut u8, v: *mut u8) {
            unsafe { std::ptr::write_unaligned(this.add(36) as *mut *mut u8, v) }
        }

        /// Most-derived fake: member slot 36 past `this`; the `DescribedBase`
        /// view is `this + 36`, so `described = derived + 36`.
        #[repr(C)]
        struct FakeDerived {
            pad: [u8; 36],
            slot: *mut u8,
        }

        fn described_of(derived: *const u8) -> *const u8 {
            derived.wrapping_add(DESCRIBED_BASE_BIAS)
        }

        fn described_mut_of<T>(p: *mut T) -> *mut u8 {
            (p as *mut u8).wrapping_add(DESCRIBED_BASE_BIAS)
        }

        fn direct(f: usize) -> MemberPtr {
            MemberPtr { func: f, adj: 0 }
        }

        fn desc() -> RefPropDescriptor {
            RefPropDescriptor {
                vtable: REF_DESC_VTAB,
                sub_vtable: REF_SUB_VTAB,
                name: String::from("Adornee"),
                category: String::from("Gui"),
                getset: Some(Box::new(GetSetImpl {
                    getter: direct(fake_rget as usize),
                    setter: direct(fake_rset as usize),
                })),
                attributes: (0, 0, 0),
                permissions: 0,
            }
        }

        #[test]
        fn ref_get_set_roundtrip_with_bias() {
            let target = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let tderived = std::ptr::addr_of!(target) as *const u8;
            let mut holder = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let hderived = std::ptr::addr_of_mut!(holder) as *mut u8;
            let d = desc();
            unsafe {
                set_ref_value_unsafe(&d, described_mut_of(hderived), described_of(tderived));
                let raw = get_raw(d.imp(), described_of(hderived));
                assert_eq!(raw, tderived as *mut u8);
                let base = get_ref_value(&d, described_of(hderived));
                assert_eq!(base, tderived.wrapping_add(36));
            }
        }

        #[test]
        fn equal_and_copy_follow_box() {
            let t = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let tderived = std::ptr::addr_of!(t) as *const u8;
            let mut a = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let mut b = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let d = desc();
            unsafe {
                set_ref_value_unsafe(&d, described_mut_of(std::ptr::addr_of_mut!(a)), described_of(tderived));
                set_ref_value_unsafe(&d, described_mut_of(std::ptr::addr_of_mut!(b)), described_of(tderived));
                assert!(equal_values(
                    &d,
                    described_of(std::ptr::addr_of!(a) as *const u8),
                    described_of(std::ptr::addr_of!(b) as *const u8)
                ));
                set_ref_value_unsafe(&d, described_mut_of(std::ptr::addr_of_mut!(b)), std::ptr::null());
                assert!(!equal_values(
                    &d,
                    described_of(std::ptr::addr_of!(a) as *const u8),
                    described_of(std::ptr::addr_of!(b) as *const u8)
                ));
                copy_value(
                    &d,
                    described_of(std::ptr::addr_of!(a) as *const u8),
                    described_mut_of(std::ptr::addr_of_mut!(b)),
                );
                assert!(equal_values(
                    &d,
                    described_of(std::ptr::addr_of!(a) as *const u8),
                    described_of(std::ptr::addr_of!(b) as *const u8)
                ));
            }
        }

        #[test]
        fn variant_xml_idref_paths() {
            let t = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let tderived = std::ptr::addr_of!(t) as *const u8;
            let mut h = FakeDerived { pad: [0; 36], slot: std::ptr::null_mut() };
            let d = desc();
            unsafe {
                set_ref_value_unsafe(&d, described_mut_of(std::ptr::addr_of_mut!(h)), described_of(tderived));
                let v = get_variant(&d, described_of(std::ptr::addr_of!(h) as *const u8));
                assert_eq!(v.type_tag, SHARED_PTR_DESCRIBED_TYPE);
                assert_eq!(v.ptr, described_of(tderived));
                set_ref_value_unsafe(&d, described_mut_of(std::ptr::addr_of_mut!(h)), std::ptr::null());
                assert!(get_raw(d.imp(), described_of(std::ptr::addr_of!(h) as *const u8)).is_null());
                set_variant(&d, described_mut_of(std::ptr::addr_of_mut!(h)), &v);
                assert_eq!(
                    get_raw(d.imp(), described_of(std::ptr::addr_of!(h) as *const u8)),
                    tderived as *mut u8
                );
                let mut xml = XmlElement::default();
                write_value(&d, described_of(std::ptr::addr_of!(h) as *const u8), &mut xml);
                assert_eq!(xml.value, described_of(tderived));
                let handle = InstanceHandle { pi: described_of(tderived), _count: 0 };
                set_ref_value_unsafe(&d, described_mut_of(std::ptr::addr_of_mut!(h)), std::ptr::null());
                assign_idref(&d, described_mut_of(std::ptr::addr_of_mut!(h)), &handle);
                assert_eq!(
                    get_raw(d.imp(), described_of(std::ptr::addr_of!(h) as *const u8)),
                    tderived as *mut u8
                );
                assert!(set_ref_value(&d, described_mut_of(std::ptr::addr_of_mut!(h)), std::ptr::null(), |_| true).is_ok());
                assert!(set_ref_value(&d, described_mut_of(std::ptr::addr_of_mut!(h)), described_of(tderived), |_| false).is_err());
            }
        }
    }
}

/// Batch 2 (cont.): `Camera` subject slot (IDA 0x3c39ac/0x3c39b4).
pub mod camera_subject {
    /// was: `RBX::Camera` `shared_ptr<Instance>` at word `+103` (count at `+104`);
    /// IDA 0x3c39ac `return *(this + 103)`, 0x3c39b4 `v29 = this + 103`.
    pub const SUBJECT_WORD: usize = 103;
    /// was: `unk_1320C2C` property descriptor passed to `raisePropertyChanged`.
    pub const SUBJECT_PROPERTY: &str = "unk_1320C2C";
    /// was: `"Humanoid"` child name probed by `setCameraSubject`.
    pub const HUMANOID_CHILD: &str = "Humanoid";

    /// Host-side hooks standing in for `__dynamic_cast`, the `ICharacterSubject`
    /// vtable slot, the `Instance` child lookup, `ClassDescriptor::isA`, and
    /// `raisePropertyChanged`. `[INFERENCE]` the trait boundary itself; the
    /// selection logic below is IDA 0x3c39b4 branch-for-branch.
    pub trait SubjectHooks {
        fn is_camera_subject(&self, ptr: *const u8) -> bool;
        fn is_character_subject(&self, ptr: *const u8) -> bool;
        fn notify_subject_changed(&self, old: *const u8, new: *const u8);
        fn find_child_by_name(&self, ptr: *const u8, name: &str) -> *const u8;
        fn is_humanoid(&self, ptr: *const u8) -> bool;
        fn raise_property_changed(&self);
    }

    /// was: `RBX::Camera` subject words. The real object is far larger; only the
    /// `+103` px is modeled (`[INFERENCE]` the count word traffic is a no-op).
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Camera {
        pub subject: *const u8,
    }

    impl Camera {
        /// IDA 0x3c39ac: `return *((DWORD *)this + 103)` — raw px, no addref.
        pub unsafe fn subject_dangerous(&self) -> *const u8 {
            self.subject
        }

        /// IDA 0x3c39b4: proceed only if `old != new`, `new != 0`, and `new`
        /// casts to `CameraSubject` (note the `v12` quirk: clearing with null is
        /// a no-op — kept as-is, not "fixed"); notify the old `ICharacterSubject`;
        /// take `new`, preferring its `Humanoid` child when `isA(Humanoid)`;
        /// `raisePropertyChanged(unk_1320C2C)`.
        pub unsafe fn set_subject(&mut self, new: *const u8, hooks: &dyn SubjectHooks) {
            let old = self.subject;
            if old == new || new.is_null() {
                return;
            }
            if !hooks.is_camera_subject(new) {
                return;
            }
            if !old.is_null() && hooks.is_character_subject(old) {
                hooks.notify_subject_changed(old, new);
            }
            let mut selected = new;
            let child = hooks.find_child_by_name(new, HUMANOID_CHILD);
            if !child.is_null() && hooks.is_humanoid(child) {
                selected = child;
            }
            self.subject = selected;
            hooks.raise_property_changed();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct FakeHooks {
            notified: AtomicUsize,
            raised: AtomicUsize,
            humanoid: *const u8,
        }
        impl SubjectHooks for FakeHooks {
            fn is_camera_subject(&self, _: *const u8) -> bool {
                true
            }
            fn is_character_subject(&self, ptr: *const u8) -> bool {
                !ptr.is_null()
            }
            fn notify_subject_changed(&self, _: *const u8, _: *const u8) {
                self.notified.fetch_add(1, Ordering::SeqCst);
            }
            fn find_child_by_name(&self, _: *const u8, name: &str) -> *const u8 {
                assert_eq!(name, HUMANOID_CHILD);
                self.humanoid
            }
            fn is_humanoid(&self, ptr: *const u8) -> bool {
                ptr == self.humanoid && !ptr.is_null()
            }
            fn raise_property_changed(&self) {
                self.raised.fetch_add(1, Ordering::SeqCst);
            }
        }

        #[test]
        fn null_and_same_are_noops() {
            let mut cam = Camera { subject: 0x1000 as *const u8 };
            let h = FakeHooks {
                notified: AtomicUsize::new(0),
                raised: AtomicUsize::new(0),
                humanoid: std::ptr::null(),
            };
            unsafe {
                cam.set_subject(std::ptr::null(), &h);
                assert_eq!(cam.subject, 0x1000 as *const u8);
                cam.set_subject(0x1000 as *const u8, &h);
                assert_eq!(cam.subject, 0x1000 as *const u8);
            }
            assert_eq!(h.raised.load(Ordering::SeqCst), 0);
        }

        #[test]
        fn prefers_humanoid_child_and_notifies() {
            let mut cam = Camera { subject: 0x1000 as *const u8 };
            let h = FakeHooks {
                notified: AtomicUsize::new(0),
                raised: AtomicUsize::new(0),
                humanoid: 0x3000 as *const u8,
            };
            unsafe {
                cam.set_subject(0x2000 as *const u8, &h);
                assert_eq!(cam.subject, 0x3000 as *const u8);
                assert_eq!(cam.subject_dangerous(), 0x3000 as *const u8);
            }
            assert_eq!(h.notified.load(Ordering::SeqCst), 1);
            assert_eq!(h.raised.load(Ordering::SeqCst), 1);
        }
    }
}
/// Batch 3: 20 IDA-grounded ports 0x3c437c-0x3c9580 — the `Camera`
/// `EnumDesc<CameraType|CameraMode|CameraPanMode>` tables (ctor pair lists +
/// shared `addPair` core), the `EnumProp/Prop/RefProp/BoundFunc/EventDesc`
/// deleting dtors, and the `CameraType/Mode/PanMode::addPair` instantiations.
/// Ports live in `camera_enum`, wired via `stub_3c43*`-`stub_3c95*`.
/// Conventions: the `addPair` body reuses the canonical
/// `generated_core_watchdog_k::render_settings::EnumDescData` port (same
/// template — same locals, same `off_1270CA8` item vtable, same
/// enumconverter.h:210-211 asserts, verified per instantiation);
/// `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR` carrier).
/// `[INFERENCE]` marks what the binary does not pin down; everything else
/// follows the IDA pseudocode branch-for-branch.
pub mod camera_enum {
    use crate::generated_core_watchdog_k::render_settings::EnumDescData;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// was: `RBX::Camera::CameraType` — IDA 0x3c437c pairs in declaration order.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum CameraType {
        Fixed = 0,
        Attach = 1,
        Watch = 2,
        Track = 3,
        Follow = 4,
        Custom = 5,
        #[default]
        Scriptable = 6,
    }

    /// was: `RBX::Camera::CameraMode` — IDA 0x3c45b4 pairs in declaration order.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum CameraMode {
        #[default]
        Classic = 0,
        LockFirstPerson = 1,
    }

    /// was: `RBX::Camera::CameraPanMode` — IDA 0x3c4778 pairs in declaration order.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum CameraPanMode {
        #[default]
        Classic = 0,
        EdgeBump = 1,
    }

    /// was: `EnumDescriptor` vtables installed by each ctor (`*a1 = &off_...`).
    pub const CAMERA_TYPE_VTAB: &str = "off_1240018"; // IDA 0x3c437c
    pub const CAMERA_MODE_VTAB: &str = "off_1240048"; // IDA 0x3c45b4
    pub const CAMERA_PAN_MODE_VTAB: &str = "off_1240078"; // IDA 0x3c4778
    /// was: `EnumDescriptor::Item` box vtable (`*v40 = &off_1270CA8` in all three
    /// `addPair` instantiations: IDA 0x3c8ec0/0x3c9220/0x3c9580).
    pub const ENUM_ITEM_VTAB: &str = "off_1270CA8";
    /// was: `FunctionDescriptor` base vtable restored by every `BoundFuncDesc`
    /// dtor (`*a1 = &off_1222248`, IDA 0x3c8cb0/0x3c8cf0/0x3c8d24/0x3c8d64/
    /// 0x3c8da4/0x3c8de4/0x3c8e24).
    pub const FUNCTION_DESC_VTAB: &str = "off_1222248";
    /// was: `EventDescriptor` base vtable restored by every `EventDesc` dtor
    /// (`*a1 = &off_122F5A8`, IDA 0x3c8e78/0x3c8e9c).
    pub const EVENT_DESC_VTAB: &str = "off_122F5A8";
    /// was: `EnumPropDescriptor<Camera, CameraType>` vtable restored by its dtor
    /// (`*a1 = &off_1240978`, IDA 0x3c8bec; `a1[11]` box deleted).
    pub const ENUM_PROP_DESC_VTAB: &str = "off_1240978"; // IDA 0x3c8bec
    /// was: `TypedPropertyDescriptor<CoordinateFrame>` vtable (IDA 0x3c8c14).
    pub const TYPED_CF_DESC_VTAB: &str = "off_1270DA8"; // IDA 0x3c8c14
    /// was: `TypedPropertyDescriptor<float>` vtable (IDA 0x3c8c60).
    pub const TYPED_FLOAT_DESC_VTAB: &str = "off_1270A68"; // IDA 0x3c8c60
    /// was: Camera `RefPropDescriptor` + sub-table vtables (IDA 0x3c8c84).
    pub const CAMERA_REF_DESC_VTAB: &str = "off_1240708"; // IDA 0x3c8c84
    pub const CAMERA_REF_SUB_VTAB: &str = "off_124075C"; // IDA 0x3c8c84
    /// was: `BoundFuncDesc` vtables restored before the base restore + list clear
    /// (IDA 0x3c8cb0/0x3c8d24/0x3c8d64/0x3c8da4/0x3c8de4/0x3c8e24; the arity-0
    /// `float ()(void)` C2 at IDA 0x3cff78 installs `off_12406B8` = vtable
    /// `__ZTVN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EEE` (0x12406b0) + 8).
    pub const BOUND_VOID_FLOAT_VTAB: &str = "off_12406E8"; // IDA 0x3c8cb0
    pub const BOUND_VOID_PANMODE_VTAB: &str = "off_1240688"; // IDA 0x3c8d24
    pub const BOUND_BOOL_FLOAT_VTAB: &str = "off_1240668"; // IDA 0x3c8d64
    pub const BOUND_VOID_INT_VTAB: &str = "off_1240648"; // IDA 0x3c8da4
    pub const BOUND_BOOL_INT_VTAB: &str = "off_1240628"; // IDA 0x3c8de4
    pub const BOUND_VOID_CF3_VTAB: &str = "off_12405E8"; // IDA 0x3c8e24
    pub const BOUND_FLOAT_VOID_VTAB: &str = "off_12406B8"; // IDA 0x3cff78

    /// IDA 0x3c437c `EnumDesc<CameraType>::EnumDesc` — base
    /// `EnumDescriptor("CameraType")`, empty-table init, then the seven
    /// `addPair` calls in declaration order.
    pub fn camera_type_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "CameraType",
            vtable: CAMERA_TYPE_VTAB,
            ..Default::default()
        };
        d.add_pair(0, "Fixed");
        d.add_pair(2, "Watch");
        d.add_pair(1, "Attach");
        d.add_pair(3, "Track");
        d.add_pair(4, "Follow");
        d.add_pair(5, "Custom");
        d.add_pair(6, "Scriptable");
        d
    }

    /// IDA 0x3c45b4 `EnumDesc<CameraMode>::EnumDesc` — base
    /// `EnumDescriptor("CameraMode")`, empty-table init, then the two `addPair` calls.
    pub fn camera_mode_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "CameraMode",
            vtable: CAMERA_MODE_VTAB,
            ..Default::default()
        };
        d.add_pair(0, "Classic");
        d.add_pair(1, "LockFirstPerson");
        d
    }

    /// IDA 0x3c45b0 `EnumDesc<CameraMode>::EnumDesc` (C1) — thunk tail-calling C2.
    pub fn camera_mode_desc_c1() -> EnumDescData {
        camera_mode_desc()
    }

    /// IDA 0x3c4778 `EnumDesc<CameraPanMode>::EnumDesc` — base
    /// `EnumDescriptor("CameraPanMode")`, empty-table init, then the two `addPair` calls.
    pub fn camera_pan_mode_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "CameraPanMode",
            vtable: CAMERA_PAN_MODE_VTAB,
            ..Default::default()
        };
        d.add_pair(0, "Classic");
        d.add_pair(1, "EdgeBump");
        d
    }
    /// was: `Singleton<EnumDesc<CameraPanMode> const>::doGetSingleton::s` —
    /// the function-local static built once under the `__cxa_guard_*` protocol
    /// with a `__cxa_atexit`-registered `~EnumDesc` (IDA 0x3cfe88: guard check,
    /// `EnumDesc<CameraPanMode>::EnumDesc` (0x3c4778) into the slot, atexit the
    /// D1). `LazyLock` is the host equivalent of that guard + atexit pair.
    static PAN_MODE_SINGLETON: std::sync::LazyLock<EnumDescData> =
        std::sync::LazyLock::new(camera_pan_mode_desc);

    /// IDA 0x3cfe88 `Singleton<EnumDesc<CameraPanMode> const>::doGetSingleton`:
    /// guard-var lazy init of the `CameraPanMode` enum descriptor; returns `&s`.
    pub fn pan_mode_singleton() -> &'static EnumDescData {
        &PAN_MODE_SINGLETON
    }

    /// IDA 0x3cfe84 `Singleton<EnumDesc<CameraPanMode> const>::initSingleton`:
    /// tail-calls `doGetSingleton` (a `B.W` shim in the binary).
    pub fn init_pan_mode_singleton() -> &'static EnumDescData {
        pan_mode_singleton()
    }

    /// Shared `EnumDesc<T>::addPair(value, name)` core (IDA 0x3c8ec0/0x3c9220/
    /// 0x3c9580 — same template, verified by diff: same locals, same item vtable
    /// `off_1270CA8`, same enumconverter.h:210-211 asserts). Forwards to the
    /// canonical `EnumDescData::add_pair` port.
    pub unsafe fn add_pair(desc: *mut EnumDescData, value: i32, name: *const c_char) {
        let text = if name.is_null() {
            String::new()
        } else {
            CStr::from_ptr(name).to_string_lossy().into_owned()
        };
        (&mut *desc).add_pair(value, &text)
    }

    /// was: `EnumPropDescriptor<Camera, CameraType>` storage — the owned box is
    /// the IDA `a1[11]` word freed by the dtor.
    #[derive(Debug, Default)]
    pub struct EnumPropDescriptor {
        pub vtable: &'static str,
        pub name: String,
        pub value: i32,
    }

    impl EnumPropDescriptor {
        /// IDA 0x3c8bec deleting-dtor shape: restore `off_1240978`, delete the
        /// `a1[11]` box. Slot stays caller-owned.
        pub fn destroy(slot: &mut EnumPropDescriptor) {
            slot.vtable = ENUM_PROP_DESC_VTAB;
        }
    }

    /// was: `BoundFuncDesc<Camera, ...>` storage. `arg_boxes` are the opaque
    /// per-argument descriptor boxes (`a1[12]` for arity 1, `a1[12..14]` for
    /// arity 3 — `[INFERENCE]` contents); `signatures` is the
    /// `SignatureDescriptor::Item` list at `+8` cleared by every dtor.
    /// `member_*` is the bound member-function-pointer pair at `+40`
    /// (`__PAIR64__` in the C2); `arg_names`/`arg_types` are the declared
    /// signature arguments (`SignatureDescriptor::addArgument` entries).
    #[derive(Debug, Default)]
    pub struct BoundFuncDescriptor {
        pub vtable: &'static str,
        pub arg_boxes: Vec<Option<Box<[u8]>>>,
        pub signatures: Vec<String>,
        pub member_func: usize,
        pub member_adj: usize,
        pub arg_names: Vec<String>,
        pub arg_types: Vec<&'static str>,
    }

    impl BoundFuncDescriptor {
        /// BoundFunc deleting-dtor shape: restore the most-derived vtable,
        /// delete the `a1[12..]` arg boxes, restore `off_1222248`, clear the
        /// `+8` signature list. IDA 0x3c8cb0/0x3c8d24/0x3c8d64/0x3c8da4/0x3c8de4
        /// (arity 1, `vtab` + one box), 0x3c8cf0 (arity 0, `None` + no boxes),
        /// 0x3c8e24 (arity 3, `vtab` + three boxes).
        /// `[INFERENCE]` the host drops arg boxes by count, not by word index.
        pub fn destroy(slot: &mut BoundFuncDescriptor, vtable: Option<&'static str>, arity: usize) {
            if let Some(v) = vtable {
                slot.vtable = v;
            }
            slot.arg_boxes.truncate(0);
            debug_assert!(arity <= 3);
            let _ = arity;
            slot.vtable = FUNCTION_DESC_VTAB;
            slot.signatures.clear();
        }
    }

    /// was: `EventDesc<Camera, ...>` storage — only the vtable + `+8` signature
    /// list are observed by the dtors.
    #[derive(Debug, Default)]
    pub struct EventDescriptor {
        pub vtable: &'static str,
        pub signatures: Vec<String>,
    }

    impl EventDescriptor {
        /// IDA 0x3c8e78/0x3c8e9c deleting-dtor shape: restore `off_122F5A8`,
        /// clear the `+8` signature list.
        pub fn destroy(slot: &mut EventDescriptor) {
            slot.vtable = EVENT_DESC_VTAB;
            slot.signatures.clear();
        }
    }

    /// was: `Variant::genericConvert<CameraPanMode>` input — either the payload
    /// (`any_cast` hit) or text for the `StringConverter` path.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GenericInput {
        Value(i32),
        Text(String),
    }

    /// IDA 0x3c98e0 `Variant::genericConvert<CameraPanMode>`: `any_cast` hit
    /// returns the payload; otherwise the string path runs
    /// `StringConverter<CameraPanMode>::convertToValue` (name-table lookup) and,
    /// on success, writes the value back (assign + singleton hop, no-ops here);
    /// total miss throws `runtime_error("Unable to cast %s to %s")`, surfaced as
    /// `Err` so the port stays total.
    pub fn generic_convert(
        desc: &EnumDescData,
        input: &GenericInput,
    ) -> Result<i32, String> {
        match input {
            GenericInput::Value(v) => Ok(*v),
            GenericInput::Text(s) => desc.lookup_value(s).ok_or_else(|| {
                format!("Unable to cast {} to CameraPanMode", s)
            }),
        }
    }

    /// IDA 0x3c9c4c/0x3c9c64 `ServiceProvider::create<T>(instance)` shared shape:
    /// `if (findServiceProvider(a1, a2)) return create<T>(); else return 0`.
    /// `[INFERENCE]` the provider registry lives outside core; caller hooks
    /// stand in for both calls.
    pub unsafe fn service_create(
        provider: *const u8,
        instance: *const u8,
        has_provider: fn(*const u8, *const u8) -> bool,
        make_service: fn() -> *mut u8,
    ) -> *mut u8 {
        if has_provider(provider, instance) {
            make_service()
        } else {
            std::ptr::null_mut()
        }
    }

    /// was: `FactoryProduct<Camera, Instance, sCamera>::Creator::getClassName()`
    /// reached via `static_getCreator` (IDA 0x3c9d7c C++ + 0x3c9d9c `Thn32` thunk —
    /// identical bodies since the static path ignores the adjusted `this`).
    pub const CAMERA_CLASS_NAME: &[u8] = b"Camera\0";

    /// IDA 0x3c9d7c/0x3c9d9c: return the `Camera` creator class name.
    pub fn camera_class_name() -> *const c_char {
        CAMERA_CLASS_NAME.as_ptr() as *const c_char
    }

    /// IDA 0x3ca510 `EnumDesc<CameraPanMode>::convertToString(value)` (the
    /// CameraType/CameraMode instantiations are later files): `value>=0`
    /// (enumconverter.h:262) and `value<enumToItem.size()` (:263) ReleaseAsserts,
    /// then `out = value < 0 || value >= size ? "" : table[value]` (the +108
    /// name-string vector).
    pub fn enum_to_string(desc: &EnumDescData, value: i32, out: &mut String) {
        assert!(value >= 0, "value>=0 file: include/reflection/enumconverter.h line: 262");
        assert!(
            (value as usize) < desc.name_strings.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
        *out = desc.name_strings[value as usize].clone();
    }

    /// IDA 0x3ca77c/0x3cadf4/0x3cb46c `EnumDesc<T>::convertToItem(value)` (same
    /// template — enumconverter.h:273/:274 asserts verified per instantiation):
    /// `value>=0` (:273) and `value<enumToItem.size()` (:274) ReleaseAsserts,
    /// then `value < 0 ? 0 : (value < size ? table[value] : 0)` (the +120
    /// item-pointer vector). Miss/empty becomes `None`.
    pub fn enum_to_item(desc: &EnumDescData, value: i32) -> Option<usize> {
        assert!(value >= 0, "value>=0 file: include/reflection/enumconverter.h line: 273");
        assert!(
            (value as usize) < desc.item_index.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274"
        );
        desc.convert_to_item(value as usize)
    }

    /// IDA 0x3ca938/0x3cafb0/0x3cb628 `EnumDesc<T>::convertToValue(name, out)`
    /// (same template): two ordered tree walks (main name map, then the alias
    /// map) with upper-bound-then-equality semantics; hit writes `*out` and
    /// returns 1, miss returns 0.
    pub fn enum_to_value_by_name(desc: &EnumDescData, name: &str, out: &mut i32) -> bool {
        if let Some(&v) = desc.by_name.get(name) {
            *out = v;
            return true;
        }
        if let Some((_, v)) = desc.aliases.iter().find(|(n, _)| n == name) {
            *out = *v;
            return true;
        }
        false
    }

    /// was: `FactoryProduct<Camera, ...>::Creator` singleton
    /// (`creatorPrivate`, IDA 0x3cb878).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Creator {
        pub constructed: bool,
        pub name: &'static str,
    }

    static CAMERA_CREATOR: Creator = Creator { constructed: true, name: "Camera" };

    /// IDA 0x3cb878 `static_getCreator`: `Creator::wasConstructed()` ReleaseAssert
    /// (Util/Object.h:282), then return `&creatorPrivate`.
    pub fn static_get_creator() -> &'static Creator {
        assert!(
            CAMERA_CREATOR.constructed,
            "Creator::wasConstructed() file: include/Util/Object.h line: 282"
        );
        &CAMERA_CREATOR
    }

    /// was: `RBX::Reflection::ClassDescriptor` function-local singleton
    /// (IDA 0x3cb8ec `describedClassDescriptor` + guard variable).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ClassDescriptorInfo {
        pub name: &'static str,
        pub base: &'static str,
    }

    static INSTANCE_DESC: std::sync::LazyLock<ClassDescriptorInfo> =
        std::sync::LazyLock::new(|| ClassDescriptorInfo { name: "Instance", base: "DescribedBase" });
    static HUMANOID_DESC: std::sync::LazyLock<ClassDescriptorInfo> = std::sync::LazyLock::new(|| {
        // Base descriptor ensured first, mirroring the IDA call order.
        let _ = *INSTANCE_DESC;
        ClassDescriptorInfo { name: "Humanoid", base: "Instance" }
    });

    /// Base descriptor ensured first, mirroring the IDA call order.
    pub fn instance_class_descriptor() -> &'static ClassDescriptorInfo {
        &INSTANCE_DESC
    }

    /// IDA 0x3cb8ec `Described<Humanoid>::classDescriptor()`: guard-var lazy init
    /// (ensure `Instance` base, construct `ClassDescriptor(sHumanoid)`,
    /// `__cxa_atexit` registered dtor — `LazyLock` drop-glue here); returns the
    /// singleton.
    pub fn humanoid_class_descriptor() -> &'static ClassDescriptorInfo {
        &HUMANOID_DESC
    }

    /// was: `RBX::Instance::~Instance` teardown reached through the
    /// `Described<Camera>` D1/D0 + `Thn32`/`Thn36` thunks (IDA 0x3cbf50/0x3cbf54/
    /// 0x3cbff4/0x3cbffc/0x3cc0a0/0x3cc0a8). Owned by the datamodel crate;
    /// the hook stands in for it. `[INFERENCE]` the trait boundary only.
    pub trait InstanceTeardown {
        fn teardown(&self, instance: *mut u8);
    }

    /// IDA 0x3cbf50/0x3cbf54 D1/D0: `Instance::~Instance(this)` (D0 adds the
    /// delete artifact; slot stays caller-owned here).
    pub unsafe fn camera_described_teardown(instance: *mut u8, t: &dyn InstanceTeardown) {
        t.teardown(instance)
    }

    /// IDA 0x3cbff4/0x3cbffc `Thn32`: `Instance::~Instance(a1 - 32)`.
    pub unsafe fn camera_described_teardown_thn32(adj: *const u8, t: &dyn InstanceTeardown) {
        t.teardown(adj.sub(32) as *mut u8)
    }

    /// IDA 0x3cc0a0/0x3cc0a8 `Thn36`: `Instance::~Instance(a1 - 36)`.
    pub unsafe fn camera_described_teardown_thn36(adj: *const u8, t: &dyn InstanceTeardown) {
        t.teardown(adj.sub(36) as *mut u8)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn camera_type_table_matches_ida_order() {
            let d = camera_type_desc();
            assert_eq!(d.vtable, CAMERA_TYPE_VTAB);
            let pairs: Vec<(i32, &str)> =
                d.pairs.iter().map(|(v, n)| (*v, n.as_str())).collect();
            assert_eq!(
                pairs,
                vec![
                    (0, "Fixed"),
                    (2, "Watch"),
                    (1, "Attach"),
                    (3, "Track"),
                    (4, "Follow"),
                    (5, "Custom"),
                    (6, "Scriptable"),
                ]
            );
            assert_eq!(d.lookup_value("Track"), Some(3));
            assert_eq!(d.lookup_name(6), Some("Scriptable"));
        }

        #[test]
        fn mode_tables_and_c1_thunk() {
            let m = camera_mode_desc();
            assert_eq!(m.vtable, CAMERA_MODE_VTAB);
            assert_eq!(m.lookup_value("LockFirstPerson"), Some(1));
            let c1 = camera_mode_desc_c1();
            assert_eq!(c1.pairs, m.pairs);
            let p = camera_pan_mode_desc();
            assert_eq!(p.vtable, CAMERA_PAN_MODE_VTAB);
            assert_eq!(p.lookup_value("EdgeBump"), Some(1));
            assert_eq!(p.lookup_name(0), Some("Classic"));
        }

        #[test]
        #[should_panic(expected = "value>=0")]
        fn add_pair_rejects_negative() {
            camera_type_desc().add_pair(-1, "Bogus");
        }

        #[test]
        fn dtors_restore_vtables_and_drop() {
            let mut e = EnumPropDescriptor {
                vtable: "off_dead",
                name: String::from("CameraType"),
                value: 3,
            };
            EnumPropDescriptor::destroy(&mut e);
            assert_eq!(e.vtable, ENUM_PROP_DESC_VTAB);
            let mut b = BoundFuncDescriptor {
                vtable: BOUND_VOID_FLOAT_VTAB,
                arg_boxes: vec![Some(vec![1u8].into_boxed_slice())],
                signatures: vec![String::from("float")],
                ..Default::default()
            };
            BoundFuncDescriptor::destroy(&mut b, Some(BOUND_VOID_FLOAT_VTAB), 1);
            assert_eq!(b.vtable, FUNCTION_DESC_VTAB);
            assert!(b.arg_boxes.is_empty() && b.signatures.is_empty());
            let mut v = EventDescriptor {
                vtable: "off_dead",
                signatures: vec![String::from("sig")],
            };
            EventDescriptor::destroy(&mut v);
            assert_eq!(v.vtable, EVENT_DESC_VTAB);
            assert!(v.signatures.is_empty());
        }
        #[test]
        fn generic_convert_hit_string_and_miss() {
            let d = camera_pan_mode_desc();
            assert_eq!(generic_convert(&d, &GenericInput::Value(1)), Ok(1));
            assert_eq!(
                generic_convert(&d, &GenericInput::Text(String::from("EdgeBump"))),
                Ok(1)
            );
            assert!(generic_convert(&d, &GenericInput::Text(String::from("Bogus"))).is_err());
        }

        #[test]
        fn service_create_gates_on_provider() {
            fn has(p: *const u8, _: *const u8) -> bool {
                !p.is_null()
            }
            fn make() -> *mut u8 {
                0x777 as *mut u8
            }
            unsafe {
                assert_eq!(
                    service_create(0x1 as *const u8, std::ptr::null(), has, make),
                    0x777 as *mut u8
                );
                assert!(service_create(std::ptr::null(), std::ptr::null(), has, make).is_null());
            }
        }

        #[test]
        fn class_name_and_enum_to_string() {
            use std::ffi::CStr;
            unsafe {
                assert_eq!(
                    CStr::from_ptr(camera_class_name()).to_string_lossy(),
                    "Camera"
                );
            }
            let d = camera_pan_mode_desc();
            let mut s = String::new();
            enum_to_string(&d, 1, &mut s);
            assert_eq!(s, "EdgeBump");
            let t = camera_type_desc();
            enum_to_string(&t, 2, &mut s);
            assert_eq!(s, "Watch");
        }

        #[test]
        #[should_panic(expected = "value>=0")]
        fn enum_to_string_rejects_negative() {
            let d = camera_pan_mode_desc();
            enum_to_string(&d, -1, &mut String::new());
        }
        #[test]
        fn enum_to_item_and_value_by_name() {
            let d = camera_pan_mode_desc();
            assert_eq!(enum_to_item(&d, 1), d.convert_to_item(1));
            assert_eq!(enum_to_item(&d, 0), Some(0));
            let mut v = -1;
            assert!(enum_to_value_by_name(&d, "EdgeBump", &mut v));
            assert_eq!(v, 1);
            assert!(!enum_to_value_by_name(&d, "Bogus", &mut v));
            let t = camera_type_desc();
            assert_eq!(enum_to_item(&t, 2), t.convert_to_item(2));
        }

        #[test]
        #[should_panic(expected = "value>=0")]
        fn enum_to_item_rejects_negative() {
            let d = camera_pan_mode_desc();
            enum_to_item(&d, -1);
        }

        #[test]
        fn creator_and_descriptors() {
            assert!(static_get_creator().constructed);
            assert_eq!(static_get_creator().name, "Camera");
            let h = humanoid_class_descriptor();
            assert_eq!((h.name, h.base), ("Humanoid", "Instance"));
            assert_eq!(instance_class_descriptor().name, "Instance");
        }

        #[test]
        fn teardown_thunks_adjust_this() {
            use std::sync::atomic::{AtomicUsize, Ordering};
            struct Rec {
                seen: AtomicUsize,
            }
            impl InstanceTeardown for Rec {
                fn teardown(&self, instance: *mut u8) {
                    self.seen.store(instance as usize, Ordering::SeqCst);
                }
            }
            let r = Rec { seen: AtomicUsize::new(0) };
            unsafe {
                camera_described_teardown(0x100 as *mut u8, &r);
                assert_eq!(r.seen.load(Ordering::SeqCst), 0x100);
                camera_described_teardown_thn32(0x100 as *const u8, &r);
                assert_eq!(r.seen.load(Ordering::SeqCst), 0x100 - 32);
                camera_described_teardown_thn36(0x100 as *const u8, &r);
                assert_eq!(r.seen.load(Ordering::SeqCst), 0x100 - 36);
            }
        }
    }
}

/// Batch 6: 8 IDA-grounded ports 0x3cd164-0x3ce4c8 — the `Camera`
/// `EventDesc<void(bool)>` constructor, both `EventDesc` deleting dtors,
/// `EventDescImpl<1/0>::fireEvent`, `EventDescBase::disconnectAll` ×2, and
/// `GenericSlotWrapper::execute1<bool>`. Ports live in `camera_event`.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR`
/// carrier); `rbx::signals::signal` emit/disconnect -> hook traits (the signal
/// object lives at `derived + desc.signal_offset`; the host receives it
/// directly); `std::vector<Variant>` / placement-any traffic collapses into
/// slices; `__stack_chk_guard` returns carry no observable output.
/// `[INFERENCE]` marks what the binary does not pin down; everything else
/// follows the IDA pseudocode branch-for-branch.
pub mod camera_event {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// was: `EventDesc<Camera, void(bool)>` vtable (`*a1 = &off_1240418`, IDA 0x3cd164).
    pub const EVENT_BOOL_VTAB: &str = "off_1240418"; // IDA 0x3cd164

    /// was: `EventDesc<Camera, void(bool)>` storage — the `a1[10]` member-signal
    /// offset plus the `+8` signature list (one `bool` item, IDA 0x3cd164).
    #[derive(Debug, Default)]
    pub struct EventDescBool {
        pub vtable: &'static str,
        pub signal_offset: usize,
        pub name: String,
        pub signatures: Vec<String>,
    }

    impl EventDescBool {
        /// IDA 0x3cd164 ctor shape: classDescriptor ensure (registry sink here),
        /// `EventDescriptor` base init, `a1[10] = member_offset`,
        /// `*a1 = &off_1240418`, `Name::declare` + one `bool` signature item.
        /// Returns `slot` (IDA `return a1`).
        pub unsafe fn construct(
            slot: *mut EventDescBool,
            member_offset: usize,
            name: *const c_char,
        ) -> *mut EventDescBool {
            let this = &mut *slot;
            this.vtable = EVENT_BOOL_VTAB;
            this.signal_offset = member_offset;
            this.name = if name.is_null() {
                String::new()
            } else {
                CStr::from_ptr(name).to_string_lossy().into_owned()
            };
            this.signatures = vec![String::from("bool")];
            slot
        }
    }

    /// was: `rbx::signals::signal<void()(bool)>` emit + disconnectAll slots.
    /// The object is located at `derived + signal_offset` in the binary; the
    /// host passes it directly.
    pub trait BoolSignal {
        fn emit(&self, value: bool);
        fn disconnect_all(&self);
    }

    /// was: `rbx::signals::signal<void()(void)>` emit + disconnectAll slots.
    pub trait VoidSignal {
        fn emit(&self);
        fn disconnect_all(&self);
    }

    /// was: `GenericSlotWrapper` callable — the `(*a1 + 8)(a1, variants)` slot.
    pub trait BoolSlot {
        fn call(&self, value: bool);
    }

    /// IDA 0x3cd4f0 `EventDescImpl<1, Camera, void(bool)>::fireEvent`:
    /// `args.size() == 1` ReleaseAssert (Reflection/Event.h:320), derived `a2 - 36`,
    /// `any_cast<bool>(args[0])`, `signal(derived + [40], value)`.
    pub fn fire_event_bool(
        desc: &EventDescBool,
        source: *const u8,
        args: &[bool],
        sig: &dyn BoolSignal,
    ) {
        assert!(args.len() == 1, "args.size() == 1 file: include/Reflection/Event.h line: 320");
        let _derived = if source.is_null() {
            source
        } else {
            source.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let _ = (desc.signal_offset, _derived);
        sig.emit(args[0])
    }

    /// IDA 0x3ce454 `EventDescImpl<0, Camera, void(void)>::fireEvent`:
    /// `args.size() == 0` ReleaseAssert (Reflection/Event.h:295), derived `a2 - 36`,
    /// `signal(derived + [40])`.
    pub fn fire_event_void(source: *const u8, args: &[()], sig: &dyn VoidSignal) {
        assert!(args.is_empty(), "args.size() == 0 file: include/Reflection/Event.h line: 295");
        let _derived = if source.is_null() {
            source
        } else {
            source.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let _ = _derived;
        sig.emit()
    }

    /// IDA 0x3cd57c `EventDescBase::disconnectAll` (bool):
    /// `disconnectAll(*(a1 + 40) + (a2 ? a2 - 36 : 0))`.
    pub fn disconnect_all_bool(source: *const u8, sig: &dyn BoolSignal) {
        let _derived = if source.is_null() {
            source
        } else {
            source.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let _ = _derived;
        sig.disconnect_all()
    }

    /// IDA 0x3ce4c8 `EventDescBase::disconnectAll` (void): same shape.
    pub fn disconnect_all_void(source: *const u8, sig: &dyn VoidSignal) {
        let _derived = if source.is_null() {
            source
        } else {
            source.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let _ = _derived;
        sig.disconnect_all()
    }

    /// IDA 0x3cd590 `GenericSlotWrapper::execute1<bool>`: builds the one-element
    /// `vector<Variant>` (`void` tag + `bool` payload via placement-any), calls
    /// the `+8` slot, destroys the vector, returns the canary (no observable
    /// output — the port returns `()`).
    pub fn execute1_bool(slot: &dyn BoolSlot, value: bool) {
        slot.call(value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

        struct FakeSig {
            emitted: AtomicBool,
            value: AtomicBool,
            disconnected: AtomicUsize,
        }
        impl BoolSignal for FakeSig {
            fn emit(&self, value: bool) {
                self.value.store(value, Ordering::SeqCst);
                self.emitted.store(true, Ordering::SeqCst);
            }
            fn disconnect_all(&self) {
                self.disconnected.fetch_add(1, Ordering::SeqCst);
            }
        }
        struct FakeVoid {
            emitted: AtomicBool,
            disconnected: AtomicUsize,
        }
        impl VoidSignal for FakeVoid {
            fn emit(&self) {
                self.emitted.store(true, Ordering::SeqCst);
            }
            fn disconnect_all(&self) {
                self.disconnected.fetch_add(1, Ordering::SeqCst);
            }
        }
        struct FakeSlot {
            seen: AtomicBool,
        }
        impl BoolSlot for FakeSlot {
            fn call(&self, value: bool) {
                self.seen.store(value, Ordering::SeqCst);
            }
        }

        #[test]
        fn bool_event_construct_fire_disconnect() {
            let mut slot = EventDescBool::default();
            let name = c"Changed".as_ptr();
            unsafe {
                EventDescBool::construct(&mut slot, 40, name);
            }
            assert_eq!(slot.vtable, EVENT_BOOL_VTAB);
            assert_eq!((slot.signal_offset, slot.signatures.len()), (40, 1));
            let sig = FakeSig {
                emitted: AtomicBool::new(false),
                value: AtomicBool::new(false),
                disconnected: AtomicUsize::new(0),
            };
            fire_event_bool(&slot, 0x100 as *const u8, &[true], &sig);
            assert!(sig.emitted.load(Ordering::SeqCst) && sig.value.load(Ordering::SeqCst));
            disconnect_all_bool(0x100 as *const u8, &sig);
            assert_eq!(sig.disconnected.load(Ordering::SeqCst), 1);
        }

        #[test]
        #[should_panic(expected = "args.size() == 1")]
        fn fire_bool_rejects_wrong_arity() {
            let d = EventDescBool::default();
            let sig = FakeSig {
                emitted: AtomicBool::new(false),
                value: AtomicBool::new(false),
                disconnected: AtomicUsize::new(0),
            };
            fire_event_bool(&d, std::ptr::null(), &[], &sig);
        }

        #[test]
        fn void_event_and_slot_wrapper() {
            let sig = FakeVoid {
                emitted: AtomicBool::new(false),
                disconnected: AtomicUsize::new(0),
            };
            fire_event_void(0x100 as *const u8, &[], &sig);
            assert!(sig.emitted.load(Ordering::SeqCst));
            disconnect_all_void(std::ptr::null(), &sig);
            assert_eq!(sig.disconnected.load(Ordering::SeqCst), 1);
            let slot = FakeSlot { seen: AtomicBool::new(false) };
            execute1_bool(&slot, true);
            assert!(slot.seen.load(Ordering::SeqCst));
        }
    }
}

/// Batch 7: 10 IDA-grounded ports 0x3ce4dc-0x3cf240 — the `Camera`
/// `BoundFuncDesc` constructors (member pair + default variants +
/// `declareSignature`), `declareSignature` bodies, deleting D0s, `execute`
/// dispatch, and `Call3/Call1Helper::call`. Ports live in `camera_boundfunc`;
/// descriptor storage reuses `camera_enum::BoundFuncDescriptor` (extended with
/// the `+40` member pair + declared args).
/// Conventions: `G3D::CoordinateFrame` = 12xf32 (`Matrix3` + translation);
/// member-function dispatch reuses the canonical `(adj >> 1)` + virtual-bit
/// rule; `ArgHelper::getArg` traffic collapses into typed parameters; callee
/// bodies are hook traits. `[INFERENCE]` marks what the binary does not pin
/// down; everything else follows the IDA pseudocode branch-for-branch.
pub mod camera_boundfunc {
    use super::billboard_prop::resolve_target;
    use super::camera_enum::BoundFuncDescriptor;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// was: `G3D::CoordinateFrame` — 48 bytes: `Matrix3` (9xf32) + translation
    /// (3xf32). IDA 0x3ce94c copies both halves (`Matrix3::Matrix3` + 12-byte tail).
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    #[repr(C)]
    pub struct CoordinateFrame {
        pub rot: [f32; 9],
        pub trans: [f32; 3],
    }

    fn cstr_name(ptr: *const c_char) -> String {
        if ptr.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
        }
    }

    /// Shared `BoundFuncDesc<Camera, ...>::BoundFuncDesc` core (IDA 0x3ce4dc
    /// arity 3 + 0x3cef84 arity 1 — same template: classDescriptor ensure,
    /// `FunctionDescriptor` base init, vtable install, `__PAIR64__` member pair
    /// at `+40`, zeroed arg boxes, per-arg `void`-singleton defaults, then
    /// `declareSignature` with the caller names).
    pub unsafe fn construct(
        slot: *mut BoundFuncDescriptor,
        vtable: &'static str,
        member_func: usize,
        member_adj: usize,
        ret_type: &'static str,
        args: &[(&str, &'static str)],
    ) -> *mut BoundFuncDescriptor {
        let this = &mut *slot;
        this.vtable = vtable;
        this.member_func = member_func;
        this.member_adj = member_adj;
        this.arg_boxes = args.iter().map(|_| None).collect();
        this.arg_names = args.iter().map(|(n, _)| n.to_string()).collect();
        this.arg_types = std::iter::once(ret_type).chain(args.iter().map(|(_, t)| *t)).collect();
        this.signatures = args.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
        slot
    }

    /// Shared `declareSignature` core (IDA 0x3ce6f4 arity 3; the 1-arg
    /// instantiations are the same template with one `addArgument`): sets the
    /// return type and appends one `name: type` entry per argument.
    pub unsafe fn declare_signature(
        desc: *mut BoundFuncDescriptor,
        ret_type: &'static str,
        args: &[(*const c_char, &'static str)],
    ) {
        let this = &mut *desc;
        if this.arg_types.is_empty() {
            this.arg_types.push(ret_type);
        } else {
            this.arg_types[0] = ret_type;
        }
        for (name_ptr, ty) in args {
            let name = cstr_name(*name_ptr);
            this.arg_names.push(name.clone());
            this.arg_types.push(*ty);
            this.signatures.push(format!("{}: {}", name, ty));
        }
    }

    /// was: bound `void (Camera::*)(CoordinateFrame, CoordinateFrame, float)` —
    /// the `Call3Helper` callee (IDA 0x3ce94c): member dispatch
    /// (`this + (adj >> 1)`, virtual branch), 48-byte CF copies, then the call.
    pub trait FnCameraCf3 {
        fn call(&self, this: *const u8, a: &CoordinateFrame, b: &CoordinateFrame, c: f32);
    }

    /// was: bound `bool (Camera::*)(int)` — the `Call1Helper` callee
    /// (IDA 0x3cf240): member dispatch, `call(this, arg)`, bool out.
    pub trait FnCameraIntBool {
        fn call(&self, this: *const u8, arg: i32) -> bool;
    }

    /// IDA 0x3ce848 `BoundFuncDesc::execute` (arity 3): derived `a2 - 36`,
    /// member pair from `+40`, three `ArgHelper::getArg`s, `Call3Helper::call`.
    pub unsafe fn execute_cf3(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        a: &CoordinateFrame,
        b: &CoordinateFrame,
        c: f32,
        callee: &dyn FnCameraCf3,
    ) {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        callee.call(this, a, b, c)
    }

    /// IDA 0x3cf200 `BoundFuncDesc::execute` (arity 1): derived `a2 - 36`,
    /// member pair from `+40`, `ArgHelper::getArg<int, 1>`, `Call1Helper::call`.
    pub unsafe fn execute_int_bool(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        arg: i32,
        callee: &dyn FnCameraIntBool,
    ) -> bool {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        callee.call(this, arg)
    }

    /// IDA 0x3ce94c `Call3Helper::call`: `(adj >> 1)` adjust + virtual branch
    /// (same member rule), 48-byte CF copies (borrowed here), then the call.
    /// `this` is already the derived `Camera*` (no DescribedBase bias).
    pub unsafe fn call_cf3(
        this: *const u8,
        member_func: usize,
        member_adj: usize,
        a: &CoordinateFrame,
        b: &CoordinateFrame,
        c: f32,
        callee: &dyn FnCameraCf3,
    ) {
        let adjusted = (this as usize).wrapping_add(member_adj >> 1) as *const u8;
        let _ = resolve_target(
            super::billboard_prop::MemberPtr { func: member_func, adj: member_adj },
            adjusted,
        );
        callee.call(adjusted, a, b, c)
    }
    /// was: bound `void (Camera::*)(int)` — the IDA 0x3cf4f4 `execute` tail-calls
    /// the member directly (no `Call1Helper` is emitted for this instantiation):
    /// member dispatch (`derived + (adj >> 1)`, odd-`adj` virtual branch), then
    /// the call with the `getArg<int, 1>` value.
    pub trait FnCameraIntVoid {
        fn call(&self, this: *const u8, arg: i32);
    }

    /// was: bound `bool (Camera::*)(float)` — the `Call1Helper` callee
    /// (IDA 0x3cf7e4): member dispatch, `call(this, arg)`, `bool` out-param
    /// (`Type::getSingleton<bool>` + `placement_any<bool>`).
    pub trait FnCameraFloatBool {
        fn call(&self, this: *const u8, arg: f32) -> bool;
    }

    /// was: bound `void (Camera::*)(CameraPanMode)` — the IDA 0x3cfc6c `execute`
    /// tail-calls the member directly (same shape as 0x3cf4f4, with
    /// `getArg<CameraPanMode, 1>`).
    pub trait FnCameraPanModeVoid {
        fn call(&self, this: *const u8, arg: super::camera_enum::CameraPanMode);
    }

    /// was: bound `float (Camera::*)()` — the `Call0Helper` callee
    /// (IDA 0x3d0154): member dispatch, `call(this)`, `float` out-param
    /// (`Type::getSingleton<float>` + `placement_any<float>`).
    pub trait FnCameraVoidFloat {
        fn call(&self, this: *const u8) -> f32;
    }

    /// was: bound `void (Camera::*)(float)` — the IDA 0x3d02fc-family `execute`
    /// callee (same 1-arg tail-call template as 0x3cf4f4/0x3cfc6c, with
    /// `getArg<float, 1>`; the `Call1Helper` lives past this file's range).
    pub trait FnCameraFloatVoid {
        fn call(&self, this: *const u8, arg: f32);
    }

    /// IDA 0x3cf4f4 `BoundFuncDesc<void ()(int), 1>::execute`: derived `a2 - 36`
    /// (null stays null), member pair from `+40`, `ArgHelper::getArg<int, 1>`,
    /// then the member tail-call (`(adj >> 1)` adjust, odd-`adj` virtual hop).
    pub unsafe fn execute_int_void(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        arg: i32,
        callee: &dyn FnCameraIntVoid,
    ) {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        callee.call(this, arg)
    }

    /// IDA 0x3cf7a4 `BoundFuncDesc<bool ()(float), 1>::execute`: derived
    /// `a2 - 36`, member pair from `+40`, `ArgHelper::getArg<float, 1>`, then
    /// `Call1Helper::call` (0x3cf7e4).
    pub unsafe fn execute_float_bool(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        arg: f32,
        callee: &dyn FnCameraFloatBool,
    ) -> bool {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        call_float_bool(this, desc.member_func, desc.member_adj, arg, callee)
    }

    /// IDA 0x3cf7e4 `Call1Helper::call`: `(adj >> 1)` adjust + odd-`adj`
    /// virtual branch, `call(this, arg)`, then the `bool` out-param hop
    /// (collapsed here into the return value).
    /// `this` is already the derived `Camera*` (no DescribedBase bias).
    pub unsafe fn call_float_bool(
        this: *const u8,
        member_func: usize,
        member_adj: usize,
        arg: f32,
        callee: &dyn FnCameraFloatBool,
    ) -> bool {
        let adjusted = (this as usize).wrapping_add(member_adj >> 1) as *const u8;
        let _ = resolve_target(
            super::billboard_prop::MemberPtr { func: member_func, adj: member_adj },
            adjusted,
        );
        callee.call(adjusted, arg)
    }

    /// IDA 0x3cfc6c `BoundFuncDesc<void ()(CameraPanMode), 1>::execute`: derived
    /// `a2 - 36`, member pair from `+40`, `ArgHelper::getArg<CameraPanMode, 1>`,
    /// then the member tail-call (same dispatch as 0x3cf4f4).
    pub unsafe fn execute_panmode_void(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        arg: super::camera_enum::CameraPanMode,
        callee: &dyn FnCameraPanModeVoid,
    ) {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        callee.call(this, arg)
    }

    /// Records the `operator new`'d `CameraPanMode` default box at `+0x30`
    /// (IDA 0x3cf9bc: `new(4)`, `*box = arg_4`). The void-default siblings keep
    /// a zeroed box, which `construct` already models as `None`.
    pub fn set_default_panmode(desc: &mut BoundFuncDescriptor, default: super::camera_enum::CameraPanMode) {
        let bytes = (default as i32).to_le_bytes();
        if desc.arg_boxes.is_empty() {
            desc.arg_boxes.push(Some(bytes.to_vec().into_boxed_slice()));
        } else {
            desc.arg_boxes[0] = Some(bytes.to_vec().into_boxed_slice());
        }
    }

    /// IDA 0x3d0130 `BoundFuncDesc<float ()(void), 0>::execute`: derived
    /// `a2 - 36` (null stays null), member pair from `+40`, then
    /// `Call0Helper::call` (0x3d0154, tail-jumped via its `$shim`).
    pub unsafe fn execute_float_0(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        callee: &dyn FnCameraVoidFloat,
    ) -> f32 {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        call_float_0(this, desc.member_func, desc.member_adj, callee)
    }

    /// IDA 0x3d0154 `Call0Helper::call`: `(adj >> 1)` adjust + odd-`adj`
    /// virtual branch, `call(this)`, then the `float` out-param hop
    /// (collapsed here into the return value).
    /// `this` is already the derived `Camera*` (no DescribedBase bias).
    pub unsafe fn call_float_0(
        this: *const u8,
        member_func: usize,
        member_adj: usize,
        callee: &dyn FnCameraVoidFloat,
    ) -> f32 {
        let adjusted = (this as usize).wrapping_add(member_adj >> 1) as *const u8;
        let _ = resolve_target(
            super::billboard_prop::MemberPtr { func: member_func, adj: member_adj },
            adjusted,
        );
        callee.call(adjusted)
    }

    /// IDA 0x3d02fc-family `BoundFuncDesc<void ()(float), 1>::execute`
    /// (same 1-arg tail-call template as 0x3cf4f4/0x3cfc6c, `getArg<float, 1>`).
    pub unsafe fn execute_float_void(
        desc: &BoundFuncDescriptor,
        obj: *const u8,
        arg: f32,
        callee: &dyn FnCameraFloatVoid,
    ) {
        let derived = if obj.is_null() {
            obj
        } else {
            obj.wrapping_sub(super::billboard_ref::BASE_TO_DERIVED)
        };
        let this = (derived as usize).wrapping_add(desc.member_adj >> 1) as *const u8;
        let _ = unsafe { resolve_target(
            super::billboard_prop::MemberPtr { func: desc.member_func, adj: desc.member_adj },
            this,
        ) };
        callee.call(this, arg)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

        struct FakeCf3 {
            calls: AtomicUsize,
            last_c: AtomicBool,
        }
        impl FnCameraCf3 for FakeCf3 {
            fn call(&self, _: *const u8, a: &CoordinateFrame, b: &CoordinateFrame, c: f32) {
                assert_eq!(a.rot[0], 1.0);
                assert_eq!(b.trans[2], 3.0);
                self.calls.fetch_add(1, Ordering::SeqCst);
                self.last_c.store(c > 0.0, Ordering::SeqCst);
            }
        }
        struct FakeIntBool;
        impl FnCameraIntBool for FakeIntBool {
            fn call(&self, _: *const u8, arg: i32) -> bool {
                arg != 0
            }
        }

        fn cf3_desc() -> BoundFuncDescriptor {
            BoundFuncDescriptor {
                vtable: super::super::camera_enum::BOUND_VOID_CF3_VTAB,
                member_func: 0,
                member_adj: 0,
                ..Default::default()
            }
        }

        #[test]
        fn construct_declare_execute_cf3() {
            let mut slot = BoundFuncDescriptor::default();
            unsafe {
                construct(
                    &mut slot,
                    super::super::camera_enum::BOUND_VOID_CF3_VTAB,
                    0x40,
                    0,
                    "void",
                    &[
                        ("a", "CoordinateFrame"),
                        ("b", "CoordinateFrame"),
                        ("c", "float"),
                    ],
                );
            }
            assert_eq!(slot.vtable, super::super::camera_enum::BOUND_VOID_CF3_VTAB);
            assert_eq!((slot.member_func, slot.member_adj), (0x40, 0));
            assert_eq!(slot.signatures.len(), 3);
            let f = FakeCf3 { calls: AtomicUsize::new(0), last_c: AtomicBool::new(false) };
            let mut a = CoordinateFrame::default();
            let mut b = CoordinateFrame::default();
            a.rot[0] = 1.0;
            b.trans[2] = 3.0;
            let mut d = cf3_desc();
            d.member_func = 0;
            unsafe {
                execute_cf3(&d, 0x100 as *const u8, &a, &b, 1.5, &f);
            }
            assert_eq!(f.calls.load(Ordering::SeqCst), 1);
            assert!(f.last_c.load(Ordering::SeqCst));
        }

        #[test]
        fn execute_int_bool_roundtrip() {
            let d = BoundFuncDescriptor {
                vtable: super::super::camera_enum::BOUND_BOOL_INT_VTAB,
                ..Default::default()
            };
            let f = FakeIntBool;
            unsafe {
                assert!(execute_int_bool(&d, 0x100 as *const u8, 7, &f));
                assert!(!execute_int_bool(&d, std::ptr::null(), 0, &f));
            }
        }

        struct FakeIntVoid {
            last: AtomicUsize,
        }
        impl FnCameraIntVoid for FakeIntVoid {
            fn call(&self, _: *const u8, arg: i32) {
                self.last.store(arg as usize, Ordering::SeqCst);
            }
        }
        struct FakeFloatBool;
        impl FnCameraFloatBool for FakeFloatBool {
            fn call(&self, _: *const u8, arg: f32) -> bool {
                arg > 0.0
            }
        }
        struct FakePanVoid {
            last: AtomicUsize,
        }
        impl FnCameraPanModeVoid for FakePanVoid {
            fn call(&self, _: *const u8, arg: super::super::camera_enum::CameraPanMode) {
                self.last.store(arg as usize, Ordering::SeqCst);
            }
        }
        struct FakeVoidFloat(f32);
        impl FnCameraVoidFloat for FakeVoidFloat {
            fn call(&self, _: *const u8) -> f32 {
                self.0
            }
        }
        struct FakeFloatVoid {
            last: AtomicBool,
        }
        impl FnCameraFloatVoid for FakeFloatVoid {
            fn call(&self, _: *const u8, arg: f32) {
                self.last.store(arg > 0.0, Ordering::SeqCst);
            }
        }

        #[test]
        fn int_void_float_bool_panmode_roundtrip() {
            let d = BoundFuncDescriptor::default();
            let f = FakeIntVoid { last: AtomicUsize::new(0) };
            unsafe {
                execute_int_void(&d, 0x100 as *const u8, 9, &f);
            }
            assert_eq!(f.last.load(Ordering::SeqCst), 9);
            let g = FakeFloatBool;
            unsafe {
                assert!(execute_float_bool(&d, 0x100 as *const u8, 1.5, &g));
                assert!(!call_float_bool(0x100 as *const u8, 0, 0, -1.0, &g));
            }
            let mut p = BoundFuncDescriptor::default();
            set_default_panmode(&mut p, super::super::camera_enum::CameraPanMode::EdgeBump);
            assert_eq!(p.arg_boxes.len(), 1);
            assert_eq!(
                p.arg_boxes[0].as_deref(),
                Some((1i32).to_le_bytes().as_slice())
            );
            let h = FakePanVoid { last: AtomicUsize::new(99) };
            unsafe {
                execute_panmode_void(
                    &p,
                    0x100 as *const u8,
                    super::super::camera_enum::CameraPanMode::Classic,
                    &h,
                );
            }
            assert_eq!(h.last.load(Ordering::SeqCst), 0);
        }

        #[test]
        fn float_0_and_float_void_roundtrip() {
            let d = BoundFuncDescriptor::default();
            let f = FakeVoidFloat(2.5);
            unsafe {
                assert_eq!(execute_float_0(&d, 0x100 as *const u8, &f), 2.5);
                assert_eq!(call_float_0(0x100 as *const u8, 0, 0, &f), 2.5);
            }
            let g = FakeFloatVoid { last: AtomicBool::new(false) };
            unsafe {
                execute_float_void(&d, std::ptr::null(), 3.0, &g);
            }
            assert!(g.last.load(Ordering::SeqCst));
        }
    }
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::isReadOnly(void)const")]
// 0x3c202c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
pub fn stub_3c202c() -> bool {
    // IDA 0x3c202c: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::isWriteOnly(void)const")]
// 0x3c2030 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
pub fn stub_3c2030() -> bool {
    // IDA 0x3c2030: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c2034 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_3c2034(imp: *const billboard_prop::GetSetImpl, obj: *const u8) -> bool {
    // IDA 0x3c2034: DescribedBase-36, (adj >> 1) adjust, virtual branch, tail-call getter.
    billboard_prop::get_bool(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// 0x3c2058 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
pub unsafe fn stub_3c2058(imp: *const billboard_prop::GetSetImpl, obj: *mut u8, value: bool) {
    // IDA 0x3c2058: setter(this, *a3); void result.
    billboard_prop::set_bool(&*imp, obj, value)
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::PropDescriptor<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>(char const*,char const*,RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3c207c — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_3c207c(
    slot: *mut billboard_prop::PropDescriptor,
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
) -> *mut billboard_prop::PropDescriptor {
    // IDA 0x3c207c: GetSetImpl vtable off_123FDF8, descriptor vtable off_123FD98,
    // TypedPropertyDescriptor<UDim2> base init.
    billboard_prop::PropDescriptor::construct(
        slot,
        name,
        category,
        billboard_prop::MemberPtr { func: getter_func, adj: getter_adj },
        billboard_prop::MemberPtr { func: setter_func, adj: setter_adj },
        billboard_prop::UDIM2_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::~PropDescriptor()")]
// 0x3c2190 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3c2190(slot: *mut billboard_prop::PropDescriptor) {
    // IDA 0x3c2190: *a1 = &off_12603F8 (TypedPropertyDescriptor<UDim2>); delete a1[10]; delete a1.
    billboard_prop::PropDescriptor::destroy(slot, billboard_prop::TYPED_UDIM2_DESC_VTAB)
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::isReadOnly(void)const")]
// 0x3c21bc — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
pub fn stub_3c21bc() -> bool {
    // IDA 0x3c21bc: return 0.
    false
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::isWriteOnly(void)const")]
// 0x3c21c0 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
pub fn stub_3c21c0() -> bool {
    // IDA 0x3c21c0: return 0.
    false
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c21c4 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3c21c4(
    out: *mut billboard_prop::UDim2,
    imp: *const billboard_prop::GetSetImpl,
    obj: *const u8,
) {
    // IDA 0x3c21c4: DescribedBase-36, (adj >> 1) adjust, virtual branch, getter(out, this).
    billboard_prop::get_udim2(out, &*imp, obj)
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::setValue(RBX::Reflection::DescribedBase *,RBX::UDim2 const&)const")]
// 0x3c21ec — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_3c21ec(
    imp: *const billboard_prop::GetSetImpl,
    obj: *mut u8,
    value: *const billboard_prop::UDim2,
) {
    // IDA 0x3c21ec: 16-byte copy (*a3, a3[1], a3[2], a3[3]) then setter(this, ...).
    billboard_prop::set_udim2(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>(char const*,char const*,G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3c2224 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_3c2224(
    slot: *mut billboard_prop::PropDescriptor,
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
) -> *mut billboard_prop::PropDescriptor {
    // IDA 0x3c2224: GetSetImpl vtable off_123FE88, descriptor vtable off_123FE28,
    // TypedPropertyDescriptor<Vector2> base init.
    billboard_prop::PropDescriptor::construct(
        slot,
        name,
        category,
        billboard_prop::MemberPtr { func: getter_func, adj: getter_adj },
        billboard_prop::MemberPtr { func: setter_func, adj: setter_adj },
        billboard_prop::VECTOR2_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()")]
// 0x3c2338 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3c2338(slot: *mut billboard_prop::PropDescriptor) {
    // IDA 0x3c2338: *a1 = &off_128D9E8 (TypedPropertyDescriptor<Vector2>); delete a1[10]; delete a1.
    billboard_prop::PropDescriptor::destroy(slot, billboard_prop::TYPED_VECTOR2_DESC_VTAB)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isReadOnly(void)const")]
// 0x3c2364 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// type: int()
pub fn stub_3c2364() -> bool {
    // IDA 0x3c2364: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isWriteOnly(void)const")]
// 0x3c2368 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// type: int()
pub fn stub_3c2368() -> bool {
    // IDA 0x3c2368: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c236c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(_DWORD *, int, int)
pub unsafe fn stub_3c236c(
    out: *mut billboard_prop::Vector2,
    imp: *const billboard_prop::GetSetImpl,
    obj: *const u8,
) -> u32 {
    // IDA 0x3c236c: DescribedBase-36, (adj >> 1) adjust, virtual branch, ref-getter,
    // 8-byte copy to hidden out-param, trailing word returned.
    billboard_prop::get_vector2(out, &*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// 0x3c239c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3c239c(
    imp: *const billboard_prop::GetSetImpl,
    obj: *mut u8,
    value: *const billboard_prop::Vector2,
) {
    // IDA 0x3c239c: setter(this, a3); void result.
    billboard_prop::set_vector2(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3c23c0 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_3c23c0(
    slot: *mut billboard_prop::PropDescriptor,
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
) -> *mut billboard_prop::PropDescriptor {
    // IDA 0x3c23c0: GetSetImpl vtable off_123FF18, descriptor vtable off_123FEB8,
    // TypedPropertyDescriptor<Vector3> base init.
    billboard_prop::PropDescriptor::construct(
        slot,
        name,
        category,
        billboard_prop::MemberPtr { func: getter_func, adj: getter_adj },
        billboard_prop::MemberPtr { func: setter_func, adj: setter_adj },
        billboard_prop::VECTOR3_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()")]
// 0x3c24d4 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3c24d4(slot: *mut billboard_prop::PropDescriptor) {
    // IDA 0x3c24d4: *a1 = &off_1270B58 (TypedPropertyDescriptor<Vector3>); delete a1[10]; delete a1.
    billboard_prop::PropDescriptor::destroy(slot, billboard_prop::TYPED_VECTOR3_DESC_VTAB)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
// 0x3c2500 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// type: int()
pub fn stub_3c2500() -> bool {
    // IDA 0x3c2500: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
// 0x3c2504 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// type: int()
pub fn stub_3c2504() -> bool {
    // IDA 0x3c2504: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c2508 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3c2508(
    out: *mut billboard_prop::Vector3,
    imp: *const billboard_prop::GetSetImpl,
    obj: *const u8,
) -> u32 {
    // IDA 0x3c2508: DescribedBase-36, (adj >> 1) adjust, virtual branch, ref-getter,
    // 12-byte copy to hidden out-param, trailing word returned.
    billboard_prop::get_vector3(out, &*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
// 0x3c253c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3c253c(
    imp: *const billboard_prop::GetSetImpl,
    obj: *mut u8,
    value: *const billboard_prop::Vector3,
) {
    // IDA 0x3c253c: setter(this, a3); void result.
    billboard_prop::set_vector3(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3c2560 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int, int)
pub unsafe fn stub_3c2560(
    slot: *mut billboard_ref::RefPropDescriptor,
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
) -> *mut billboard_ref::RefPropDescriptor {
    // IDA 0x3c2560: RefType<Instance*> singleton + PropertyDescriptor base init,
    // *a1 = &off_123FF48, a1[10] = &off_123FF9C, new GetSetImpl(&off_123FFC8) at a1[11].
    billboard_ref::RefPropDescriptor::construct(
        slot,
        name,
        category,
        billboard_prop::MemberPtr { func: getter_func, adj: getter_adj },
        billboard_prop::MemberPtr { func: setter_func, adj: setter_adj },
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
// 0x3c2604 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3c2604(slot: *mut billboard_ref::RefPropDescriptor) {
    // IDA 0x3c2604: *a1 = &off_123FF48, a1[10] = &off_123FF9C, delete a1[11], delete a1.
    billboard_ref::RefPropDescriptor::destroy(slot)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isReadOnly(void)const")]
// 0x3c2634 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10isReadOnlyEv
// type: int __fastcall(int)
pub unsafe fn stub_3c2634(desc: *const billboard_ref::RefPropDescriptor) -> bool {
    // IDA 0x3c2634: delegates to the box +0 slot (Instance* GetSetImpl 0x3c2b10 = 0).
    billboard_ref::is_read_only(&*desc)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isWriteOnly(void)const")]
// 0x3c2644 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
pub unsafe fn stub_3c2644(desc: *const billboard_ref::RefPropDescriptor) -> bool {
    // IDA 0x3c2644: delegates to the box +4 slot (Instance* GetSetImpl 0x3c2b14 = 0).
    billboard_ref::is_write_only(&*desc)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x3c2654 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
pub unsafe fn stub_3c2654(
    desc: *const billboard_ref::RefPropDescriptor,
    a: *const u8,
    b: *const u8,
) -> bool {
    // IDA 0x3c2654: getValue(box, a2) == getValue(box, a3).
    billboard_ref::equal_values(&*desc, a, b)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x3c267c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub unsafe fn stub_3c267c(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *const u8,
) -> billboard_ref::RefVariant {
    // IDA 0x3c267c: getValue + shared_from, base (+36) into a DescribedBase-shared-ptr Variant.
    billboard_ref::get_variant(&*desc, obj)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x3c2794 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
pub unsafe fn stub_3c2794(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    variant: *const billboard_ref::RefVariant,
) {
    // IDA 0x3c2794: Variant::get<shared_ptr<DescribedBase>> then descriptor +64 setter.
    billboard_ref::set_variant(&*desc, obj, &*variant)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x3c285c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3c285c(
    desc: *const billboard_ref::RefPropDescriptor,
    src: *const u8,
    dst: *mut u8,
) {
    // IDA 0x3c285c: v6 = getValue(box, src); setValue(box, dst, &v6).
    billboard_ref::copy_value(&*desc, src, dst)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x3c2880 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
pub unsafe fn stub_3c2880(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *const u8,
    xml: *mut billboard_ref::XmlElement,
) {
    // IDA 0x3c2880: InstanceHandle(base) into XmlNameValuePair slot a3 + 12.
    billboard_ref::write_value(&*desc, obj, xml)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x3c2954 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
pub unsafe fn stub_3c2954(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    xml: *const billboard_ref::XmlElement,
    binder: &dyn billboard_ref::ReferenceBinder,
) -> i32 {
    // IDA 0x3c2954: binder[4](binder, xml ? xml + 12 : 0, obj, desc + 40).
    billboard_ref::read_value(&*desc, obj, xml, binder)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c2978 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub unsafe fn stub_3c2978(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *const u8,
) -> *const u8 {
    // IDA 0x3c2978: result = getValue(box, obj); if (result) result += 36 (derived -> base).
    billboard_ref::get_ref_value(&*desc, obj)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3c298c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub unsafe fn stub_3c298c(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    src: *const u8,
    is_instance: fn(*const u8) -> bool,
) -> Result<(), billboard_ref::BadCast> {
    // IDA 0x3c298c: null passes; non-null __dynamic_casts DescribedBase* -> Instance*
    // (bad_cast throw otherwise); setValue(box, obj, &derived).
    billboard_ref::set_ref_value(&*desc, obj, src, is_instance)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3c2a08 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3c2a08(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    src: *const u8,
) {
    // IDA 0x3c2a08: v3 = src ? src - 36 : 0 (base -> derived, no cast); setValue(box, obj, &v3).
    billboard_ref::set_ref_value_unsafe(&*desc, obj, src)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3c2a28 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub unsafe fn stub_3c2a28(
    desc: *const billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    handle: *const billboard_ref::InstanceHandle,
) {
    // IDA 0x3c2a28: count copy (no-op) + pi ? pi - 36 into setValue(box, obj, ...).
    billboard_ref::assign_idref(&*desc, obj, handle)
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3c2b08 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
pub unsafe fn stub_3c2b08(
    this_adj: *const u8,
    obj: *mut u8,
    handle: *const billboard_ref::InstanceHandle,
) {
    // IDA 0x3c2b08 non-virtual thunk: return assignIDREF(a1 - 40, obj, handle).
    billboard_ref::assign_idref_thunk(this_adj, obj, handle)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isReadOnly(void)const")]
// 0x3c2b10 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
pub fn stub_3c2b10() -> bool {
    // IDA 0x3c2b10: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isWriteOnly(void)const")]
// 0x3c2b14 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
pub fn stub_3c2b14() -> bool {
    // IDA 0x3c2b14: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c2b18 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_3c2b18(imp: *const billboard_prop::GetSetImpl, obj: *const u8) -> *mut u8 {
    // IDA 0x3c2b18: DescribedBase-36, (adj >> 1) adjust, virtual branch, tail-call getter.
    billboard_ref::get_instance(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// 0x3c2b38 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_3c2b38(
    imp: *const billboard_prop::GetSetImpl,
    obj: *mut u8,
    value: *const *mut u8,
) {
    // IDA 0x3c2b38: setter(this, *a3); void result.
    billboard_ref::set_instance(&*imp, obj, *value)
}

#[doc(alias = "RBX::Camera::getCameraSubjectInstanceDangerous(void)const")]
// 0x3c39ac — __ZNK3RBX6Camera33getCameraSubjectInstanceDangerousEv
// type: int __fastcall(RBX::Camera *this)
pub unsafe fn stub_3c39ac(this: *const camera_subject::Camera) -> *const u8 {
    // IDA 0x3c39ac: return *((DWORD *)this + 103) — raw subject px, no addref.
    (&*this).subject_dangerous()
}

#[doc(alias = "RBX::Camera::setCameraSubject(RBX::Instance *)")]
// 0x3c39b4 — __ZN3RBX6Camera16setCameraSubjectEPNS_8InstanceE
// type: void __fastcall(shared_count *this, RBX::Instance *)
pub unsafe fn stub_3c39b4(
    this: *mut camera_subject::Camera,
    new: *const u8,
    hooks: &dyn camera_subject::SubjectHooks,
) {
    // IDA 0x3c39b4: CameraSubject-gated select (Humanoid-child preference) + property notify.
    (&mut *this).set_subject(new, hooks)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::EnumDesc(void)")]
// 0x3c437c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEEC2Ev
// type: int __fastcall(int)
pub fn stub_3c437c() -> crate::generated_core_watchdog_k::render_settings::EnumDescData {
    // IDA 0x3c437c: EnumDescriptor("CameraType") + empty-table init + 7 addPair calls.
    camera_enum::camera_type_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::EnumDesc(void)")]
// 0x3c45b0 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEEC1Ev
// type: int()
pub fn stub_3c45b0() -> crate::generated_core_watchdog_k::render_settings::EnumDescData {
    // IDA 0x3c45b0: C1 thunk tail-calling C2 (0x3c45b4).
    camera_enum::camera_mode_desc_c1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::EnumDesc(void)")]
// 0x3c45b4 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_3c45b4() -> crate::generated_core_watchdog_k::render_settings::EnumDescData {
    // IDA 0x3c45b4: EnumDescriptor("CameraMode") + empty-table init + 2 addPair calls.
    camera_enum::camera_mode_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::EnumDesc(void)")]
// 0x3c4778 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_3c4778() -> crate::generated_core_watchdog_k::render_settings::EnumDescData {
    // IDA 0x3c4778: EnumDescriptor("CameraPanMode") + empty-table init + 2 addPair calls.
    camera_enum::camera_pan_mode_desc()
}

/// Batch 7: 4 IDA-grounded `Camera` view ports (disasm-first — the decompiler
/// yields nothing for all four): `askSetParent` (0x3c4e90), `isPartVisibleFast`
/// (0x3c4f8c), `isPartInFrustum` (0x3c50fc), `zoomExtents(ModelInstance*,
/// ZoomType)` (0x3c7590). Conventions: raw `Instance`/`PartInstance` graph
/// access collapses into hook traits (the datamodel/rendering owners live
/// outside `rbx_core`); `[INFERENCE]` marks what the binary does not pin down;
/// everything else follows the disassembly branch-for-branch.
pub mod camera_view {
    /// was: `RBX::Camera::ZoomType` — forwarded opaquely by IDA 0x3c7590 into
    /// `zoomExtents(Extents, ZoomType)` (0x3c75e4, owned by the datamodel
    /// crate). `[INFERENCE]` the enumerators; the width is observed (`MOV R4, R2`).
    pub type ZoomType = i32;

    /// IDA 0x3c4e90 `Camera::askSetParent(parent)`: null parent returns 0;
    /// otherwise the `DescribedBase` view (`parent + 0x24`) lends its
    /// `ClassDescriptor` (`[adjusted + 0xC]`) to
    /// `Workspace::classDescriptor()->isA(desc)`. A non-`Workspace` parent
    /// returns 0; a `Workspace` parent returns `adjusted != 0x24` (i.e. 1 —
    /// null was already excluded). `[INFERENCE]` the descriptor registry
    /// lives outside core; caller hooks stand in for both lookups.
    pub fn ask_set_parent(
        parent: *const u8,
        parent_desc: *const u8,
        workspace_desc: *const u8,
        is_a: fn(*const u8, *const u8) -> bool,
    ) -> u32 {
        if parent.is_null() {
            return 0;
        }
        // `R4 = parent + 0x24` (IT NE guard: null adds nothing, returns above).
        let adjusted = (parent as usize).wrapping_add(0x24);
        if !is_a(parent_desc, workspace_desc) {
            return 0;
        }
        // `CMP R4, #0x24; MOVNE R0, #1`.
        u32::from(adjusted != 0x24)
    }

    /// was: `isPartVisibleFast` scene query — the camera eye (`[cam + 0x94]`,
    /// `+0x98`, `+0x9C`), the part center (`PartInstance::getCoordinateFrame`
    /// `+0x24/+0x28/+0x2C`), and `ContactManager::getHit` over the
    /// `ignoreBool`/`dummyCellID` filter set. `[INFERENCE]` the trait boundary
    /// only; the arithmetic below mirrors the binary exactly.
    pub trait VisibleFastHooks {
        fn camera_pos(&self, cam: *const u8) -> [f32; 3];
        fn part_center(&self, part: *const u8) -> [f32; 3];
        fn get_hit(&self, origin: &[f32; 3], dir: &[f32; 3]) -> *const u8;
        fn part_primitive(&self, part: *const u8) -> *const u8;
    }

    /// IDA 0x3c4f8c `Camera::isPartVisibleFast(part, rect, contacts)`: builds
    /// the `RbxRay` (eye at the camera position, direction `2 * (center -
    /// eye)` — the binary doubles each component: `VADD D5, D20, D20` etc.),
    /// runs `ContactManager::getHit`, and returns 1 iff the first hit is the
    /// part's own primitive (`[part + 0xAC]`); a null hit returns 0 (`CBZ`).
    pub fn is_part_visible_fast(
        cam: *const u8,
        part: *const u8,
        hooks: &dyn VisibleFastHooks,
    ) -> bool {
        let eye = hooks.camera_pos(cam);
        let center = hooks.part_center(part);
        // `VSUB` + `VADD x, x, x` per lane.
        let dir = [
            (center[0] - eye[0]) + (center[0] - eye[0]),
            (center[1] - eye[1]) + (center[1] - eye[1]),
            (center[2] - eye[2]) + (center[2] - eye[2]),
        ];
        let hit = hooks.get_hit(&eye, &dir);
        // `CBZ R0, miss`.
        if hit.is_null() {
            return false;
        }
        // `CMP R0, [part + 0xAC]; MOVEQ R4, #1`.
        hit == hooks.part_primitive(part)
    }

    /// was: `Camera::frustum(rect)` + `PartInstance::containedByFrustum` —
    /// the `G3D::Array<G3D::Plane, 10, 32>` scratch lives on the IDA frame and
    /// is destroyed before return. `[INFERENCE]` the trait boundary only.
    pub trait FrustumHooks {
        fn frustum_planes(&self, cam: *const u8, rect: &[f32; 4]) -> Vec<[f32; 4]>;
        fn contained_by_frustum(&self, part: *const u8, planes: &[[f32; 4]]) -> bool;
    }

    /// IDA 0x3c50fc `Camera::isPartInFrustum(part, rect)`: `frustum = frustum(rect)`,
    /// return `part->containedByFrustum(frustum)`.
    pub fn is_part_in_frustum(
        cam: *const u8,
        part: *const u8,
        rect: &[f32; 4],
        hooks: &dyn FrustumHooks,
    ) -> bool {
        let planes = hooks.frustum_planes(cam, rect);
        hooks.contained_by_frustum(part, &planes)
    }

    /// was: `ModelInstance` extents source (`[model]->vtab[0x88]`, IDA 0x3c7598)
    /// and the `Tolerance::millionCube` clamp bound, forwarded into
    /// `zoomExtents(Extents, ZoomType)` (0x3c75e4). `[INFERENCE]` the trait
    /// boundary only.
    pub trait ZoomHooks {
        fn model_extents(&self, model: *const u8) -> ([f32; 3], [f32; 3]);
        fn million_cube(&self) -> ([f32; 3], [f32; 3]);
        fn zoom_extents(&self, cam: *mut u8, extents: &([f32; 3], [f32; 3]), zoom: ZoomType);
    }

    /// IDA 0x3c75b8-0x3c75ba `Extents::clampInsideOf(millionCube)` —
    /// component-wise clamp of both corners into the bound corners.
    pub fn clamp_inside_of(
        extents: &([f32; 3], [f32; 3]),
        bounds: &([f32; 3], [f32; 3]),
    ) -> ([f32; 3], [f32; 3]) {
        let mut out = ([0.0f32; 3], [0.0f32; 3]);
        for i in 0..3 {
            out.0[i] = extents.0[i].clamp(bounds.0[i], bounds.1[i]);
            out.1[i] = extents.1[i].clamp(bounds.0[i], bounds.1[i]);
        }
        out
    }

    /// IDA 0x3c7590 `Camera::zoomExtents(model, zoom)`: fetch the model
    /// extents, clamp inside `millionCube`, call `zoomExtents(extents, zoom)`.
    pub fn zoom_extents_model(
        cam: *mut u8,
        model: *const u8,
        zoom: ZoomType,
        hooks: &dyn ZoomHooks,
    ) {
        let extents = hooks.model_extents(model);
        let clamped = clamp_inside_of(&extents, &hooks.million_cube());
        hooks.zoom_extents(cam, &clamped, zoom)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn is_a(a: *const u8, b: *const u8) -> bool {
            !a.is_null() && a == b
        }

        #[test]
        fn ask_set_parent_branches() {
            let ws = 0x700 as *const u8;
            assert_eq!(ask_set_parent(std::ptr::null(), ws, ws, is_a), 0);
            assert_eq!(ask_set_parent(0x100 as *const u8, std::ptr::null(), ws, is_a), 0);
            assert_eq!(ask_set_parent(0x100 as *const u8, ws, ws, is_a), 1);
        }

        struct Vis {
            hit: *const u8,
            prim: *const u8,
        }
        impl VisibleFastHooks for Vis {
            fn camera_pos(&self, _: *const u8) -> [f32; 3] {
                [0.0, 0.0, 0.0]
            }
            fn part_center(&self, _: *const u8) -> [f32; 3] {
                [1.0, 2.0, 4.0]
            }
            fn get_hit(&self, origin: &[f32; 3], dir: &[f32; 3]) -> *const u8 {
                assert_eq!(origin, &[0.0, 0.0, 0.0]);
                assert_eq!(dir, &[2.0, 4.0, 8.0]);
                self.hit
            }
            fn part_primitive(&self, _: *const u8) -> *const u8 {
                self.prim
            }
        }

        #[test]
        fn visible_fast_hit_miss_null() {
            let p = 0x200 as *const u8;
            assert!(is_part_visible_fast(
                0x100 as *const u8,
                p,
                &Vis { hit: p, prim: p }
            ));
            assert!(!is_part_visible_fast(
                0x100 as *const u8,
                p,
                &Vis { hit: 0x300 as *const u8, prim: p }
            ));
            assert!(!is_part_visible_fast(
                0x100 as *const u8,
                p,
                &Vis { hit: std::ptr::null(), prim: p }
            ));
        }

        struct Fr(bool);
        impl FrustumHooks for Fr {
            fn frustum_planes(&self, _: *const u8, rect: &[f32; 4]) -> Vec<[f32; 4]> {
                assert_eq!(rect.len(), 4);
                vec![[1.0, 0.0, 0.0, 5.0]]
            }
            fn contained_by_frustum(&self, _: *const u8, planes: &[[f32; 4]]) -> bool {
                assert_eq!(planes.len(), 1);
                self.0
            }
        }

        #[test]
        fn frustum_forwards() {
            assert!(is_part_in_frustum(
                0x100 as *const u8,
                0x200 as *const u8,
                &[0.0, 0.0, 64.0, 64.0],
                &Fr(true)
            ));
            assert!(!is_part_in_frustum(
                0x100 as *const u8,
                0x200 as *const u8,
                &[0.0, 0.0, 64.0, 64.0],
                &Fr(false)
            ));
        }

        struct Zm {
            seen: std::cell::Cell<([f32; 3], [f32; 3])>,
        }
        impl ZoomHooks for Zm {
            fn model_extents(&self, _: *const u8) -> ([f32; 3], [f32; 3]) {
                ([-100.0, -100.0, -100.0], [100.0, 5.0, 5.0])
            }
            fn million_cube(&self) -> ([f32; 3], [f32; 3]) {
                ([-10.0, -10.0, -10.0], [10.0, 10.0, 10.0])
            }
            fn zoom_extents(&self, _: *mut u8, extents: &([f32; 3], [f32; 3]), zoom: ZoomType) {
                assert_eq!(zoom, 2);
                self.seen.set(*extents);
            }
        }

        #[test]
        fn zoom_clamps_then_calls() {
            let z = Zm { seen: std::cell::Cell::new(([0.0; 3], [0.0; 3])) };
            zoom_extents_model(0x100 as *mut u8, 0x200 as *const u8, 2, &z);
            assert_eq!(z.seen.get(), ([-10.0, -10.0, -10.0], [10.0, 5.0, 5.0]));
            assert_eq!(
                clamp_inside_of(&([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), &z.million_cube()),
                ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])
            );
        }
    }
}

#[doc(alias = "RBX::Camera::askSetParent(RBX::Instance const*)const")]
// 0x3c4e90 — __ZNK3RBX6Camera12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Camera *__hidden this, const RBX::Instance *)
pub fn stub_3c4e90(
    parent: *const u8,
    parent_desc: *const u8,
    workspace_desc: *const u8,
    is_a: fn(*const u8, *const u8) -> bool,
) -> u32 {
    // IDA 0x3c4e90: null -> 0; +0x24 adjust; Workspace::classDescriptor isA;
    // non-Workspace -> 0; else adjusted != 0x24.
    camera_view::ask_set_parent(parent, parent_desc, workspace_desc, is_a)
}

#[doc(alias = "RBX::Camera::isPartVisibleFast(RBX::PartInstance const&,G3D::Rect2D const&,RBX::ContactManager const&)const")]
// 0x3c4f8c — __ZNK3RBX6Camera17isPartVisibleFastERKNS_12PartInstanceERKN3G3D6Rect2DERKNS_14ContactManagerE
// type: bool __fastcall(_DWORD *, RBX::PartInstance *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int)
pub fn stub_3c4f8c(
    cam: *const u8,
    part: *const u8,
    hooks: &dyn camera_view::VisibleFastHooks,
) -> bool {
    // IDA 0x3c4f8c: RbxRay eye->2*(center-eye), ContactManager::getHit,
    // 1 iff the first hit is the part's own primitive ([part + 0xAC]).
    camera_view::is_part_visible_fast(cam, part, hooks)
}

#[doc(alias = "RBX::Camera::isPartInFrustum(RBX::PartInstance const&,G3D::Rect2D const&)const")]
// 0x3c50fc — __ZNK3RBX6Camera15isPartInFrustumERKNS_12PartInstanceERKN3G3D6Rect2DE
// type: int __fastcall(int, RBX::PartInstance *)
pub fn stub_3c50fc(
    cam: *const u8,
    part: *const u8,
    rect: &[f32; 4],
    hooks: &dyn camera_view::FrustumHooks,
) -> bool {
    // IDA 0x3c50fc: frustum = Camera::frustum(rect);
    // return PartInstance::containedByFrustum(frustum).
    camera_view::is_part_in_frustum(cam, part, rect, hooks)
}

#[doc(alias = "RBX::Camera::zoomExtents(RBX::ModelInstance const*,RBX::Camera::ZoomType)")]
// 0x3c7590 — __ZN3RBX6Camera11zoomExtentsEPKNS_13ModelInstanceENS0_8ZoomTypeE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_3c7590(
    cam: *mut u8,
    model: *const u8,
    zoom: camera_view::ZoomType,
    hooks: &dyn camera_view::ZoomHooks,
) {
    // IDA 0x3c7590: model extents via vtab[0x88], clampInsideOf(millionCube),
    // zoomExtents(extents, zoom) (0x3c75e4).
    camera_view::zoom_extents_model(cam, model, zoom, hooks)
}
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::~EnumPropDescriptor()")]
// 0x3c8bec — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8bec(slot: &mut camera_enum::EnumPropDescriptor) {
    // IDA 0x3c8bec: *a1 = &off_1240978; delete a1[11].
    camera_enum::EnumPropDescriptor::destroy(slot)
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()")]
// 0x3c8c14 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub unsafe fn stub_3c8c14(slot: *mut billboard_prop::PropDescriptor) {
    // IDA 0x3c8c14: *a1 = &off_1270DA8 (TypedPropertyDescriptor<CoordinateFrame>); delete a1[10].
    billboard_prop::PropDescriptor::destroy(slot, camera_enum::TYPED_CF_DESC_VTAB)
}
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::~PropDescriptor()")]
// 0x3c8c60 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub unsafe fn stub_3c8c60(slot: *mut billboard_prop::PropDescriptor) {
    // IDA 0x3c8c60: *a1 = &off_1270A68 (TypedPropertyDescriptor<float>); delete a1[10].
    billboard_prop::PropDescriptor::destroy(slot, camera_enum::TYPED_FLOAT_DESC_VTAB)
}
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::~RefPropDescriptor()")]
// 0x3c8c84 — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub unsafe fn stub_3c8c84(slot: *mut billboard_ref::RefPropDescriptor) {
    // IDA 0x3c8c84: *a1 = &off_1240708, a1[10] = &off_124075C, delete a1[11].
    billboard_ref::RefPropDescriptor::destroy_with(
        slot,
        camera_enum::CAMERA_REF_DESC_VTAB,
        camera_enum::CAMERA_REF_SUB_VTAB,
    )
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::~BoundFuncDesc()")]
// 0x3c8cb0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8cb0(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8cb0: *a1 = &off_12406E8; delete a1[12]; *a1 = &off_1222248; clear +8 list.
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_FLOAT_VTAB), 1)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::~BoundFuncDesc()")]
// 0x3c8cf0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8cf0(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8cf0: *a1 = &off_1222248; clear +8 list (arity 0, no boxes).
    camera_enum::BoundFuncDescriptor::destroy(slot, None, 0)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::~BoundFuncDesc()")]
// 0x3c8d24 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8d24(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8d24: *a1 = &off_1240688; delete a1[12]; *a1 = &off_1222248; clear +8 list.
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_PANMODE_VTAB), 1)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::~BoundFuncDesc()")]
// 0x3c8d64 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8d64(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8d64: *a1 = &off_1240668; delete a1[12]; *a1 = &off_1222248; clear +8 list.
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_BOOL_FLOAT_VTAB), 1)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::~BoundFuncDesc()")]
// 0x3c8da4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8da4(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8da4: *a1 = &off_1240648; delete a1[12]; *a1 = &off_1222248; clear +8 list.
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_INT_VTAB), 1)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::~BoundFuncDesc()")]
// 0x3c8de4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8de4(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8de4: *a1 = &off_1240628; delete a1[12]; *a1 = &off_1222248; clear +8 list.
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_BOOL_INT_VTAB), 1)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()")]
// 0x3c8e24 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8e24(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3c8e24: *a1 = &off_12405E8; delete a1[14], a1[13], a1[12];
    // *a1 = &off_1222248; clear +8 list.
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_CF3_VTAB), 3)
}
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::~EventDesc()")]
// 0x3c8e78 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8e78(slot: &mut camera_enum::EventDescriptor) {
    // IDA 0x3c8e78: *a1 = &off_122F5A8; clear +8 signature list.
    camera_enum::EventDescriptor::destroy(slot)
}
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::~EventDesc()")]
// 0x3c8e9c — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8e9c(slot: &mut camera_enum::EventDescriptor) {
    // IDA 0x3c8e9c: *a1 = &off_122F5A8; clear +8 signature list.
    camera_enum::EventDescriptor::destroy(slot)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::addPair(RBX::Camera::CameraType,char const*)")]
// 0x3c8ec0 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub unsafe fn stub_3c8ec0(
    desc: *mut crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
    name: *const std::os::raw::c_char,
) {
    // IDA 0x3c8ec0 CameraType::addPair — same template as the canonical port.
    camera_enum::add_pair(desc, value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::addPair(RBX::Camera::CameraMode,char const*)")]
// 0x3c9220 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub unsafe fn stub_3c9220(
    desc: *mut crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
    name: *const std::os::raw::c_char,
) {
    // IDA 0x3c9220 CameraMode::addPair — same template as the canonical port.
    camera_enum::add_pair(desc, value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::addPair(RBX::Camera::CameraPanMode,char const*)")]
// 0x3c9580 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub unsafe fn stub_3c9580(
    desc: *mut crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
    name: *const std::os::raw::c_char,
) {
    // IDA 0x3c9580 CameraPanMode::addPair — same template as the canonical port.
    camera_enum::add_pair(desc, value, name)
}

#[doc(alias = "RBX::Camera::CameraPanMode & RBX::Reflection::Variant::genericConvert<RBX::Camera::CameraPanMode>(void)")]
// 0x3c98e0 — __ZN3RBX10Reflection7Variant14genericConvertINS_6Camera13CameraPanModeEEERT_v
// type: int __fastcall(int)
pub fn stub_3c98e0(input: &camera_enum::GenericInput) -> Result<i32, String> {
    // IDA 0x3c98e0: any_cast hit returns payload; string path converts by name;
    // miss throws runtime_error("Unable to cast %s to %s").
    camera_enum::generic_convert(&camera_enum::camera_pan_mode_desc(), input)
}

#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
// 0x3c9c4c — __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub unsafe fn stub_3c9c4c(
    provider: *const u8,
    instance: *const u8,
    has_provider: fn(*const u8, *const u8) -> bool,
    make_players: fn() -> *mut u8,
) -> *mut u8 {
    // IDA 0x3c9c4c: findServiceProvider ? create<Players>() : 0.
    camera_enum::service_create(provider, instance, has_provider, make_players)
}

#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(RBX::Instance const*)")]
// 0x3c9c64 — __ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub unsafe fn stub_3c9c64(
    provider: *const u8,
    instance: *const u8,
    has_provider: fn(*const u8, *const u8) -> bool,
    make_service: fn() -> *mut u8,
) -> *mut u8 {
    // IDA 0x3c9c64: findServiceProvider ? create<ControllerService>() : 0.
    camera_enum::service_create(provider, instance, has_provider, make_service)
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")]
// 0x3c9d7c — __ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv
// type: int()
pub fn stub_3c9d7c() -> *const std::os::raw::c_char {
    // IDA 0x3c9d7c: static_getCreator + Creator::getClassName ("Camera").
    camera_enum::camera_class_name()
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")]
// 0x3c9d9c — __ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv
// type: int()
pub fn stub_3c9d9c() -> *const std::os::raw::c_char {
    // IDA 0x3c9d9c Thn32: same static path as 0x3c9d7c (adjusted `this` ignored).
    camera_enum::camera_class_name()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// 0x3c9dcc — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED1Ev
// type: int()
pub fn stub_3c9dcc(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3c9dcc D1 thunk -> D2 member teardown.
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// 0x3c9dd0 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED0Ev
// type: void __fastcall(void *)
pub fn stub_3c9dd0(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3c9dd0 D0: D1 teardown; slot stays caller-owned (delete artifact).
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::lookup(char const*)const")]
// 0x3c9e70 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub unsafe fn stub_3c9e70(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    name: *const std::os::raw::c_char,
) -> Option<usize> {
    // IDA 0x3c9e70 CameraType::lookup(name): Name::lookup + convertToValue +
    // convertToItem; miss -> 0/None. `[INFERENCE]` host item index stands in
    // for the IDA Item* address.
    let text = if name.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned()
    };
    desc.lookup_by_name(&text)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x3c9ea0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_3c9ea0(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    variant: &crate::generated_core_watchdog_k::render_settings::Variant,
) -> Option<usize> {
    // IDA 0x3c9ea0 CameraType::lookup(variant): any_cast payload + convertToItem.
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x3c9ec0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
pub fn stub_3c9ec0(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    index: usize,
    out: &mut crate::generated_core_watchdog_k::render_settings::Variant,
) -> bool {
    // IDA 0x3c9ec0 CameraType::convertToValue: table[index] into out-Variant (1/0).
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToString(unsigned long,std::string &)const")]
// 0x3c9ef4 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3c9ef4(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: usize,
    out: &mut String,
) -> bool {
    // IDA 0x3c9ef4 CameraType::convertToString: name slot into out (1/0).
    desc.convert_to_string(value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// 0x3ca038 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED1Ev
// type: int()
pub fn stub_3ca038(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3ca038 CameraMode D1 thunk -> D2 member teardown.
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// 0x3ca03c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED0Ev
// type: void __fastcall(void *)
pub fn stub_3ca03c(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3ca03c CameraMode D0: D1 teardown; slot stays caller-owned.
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::lookup(char const*)const")]
// 0x3ca0dc — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub unsafe fn stub_3ca0dc(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    name: *const std::os::raw::c_char,
) -> Option<usize> {
    // IDA 0x3ca0dc CameraMode::lookup(name): Name::lookup + convertToValue +
    // convertToItem; miss -> 0/None.
    let text = if name.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned()
    };
    desc.lookup_by_name(&text)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0x3ca10c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_3ca10c(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    variant: &crate::generated_core_watchdog_k::render_settings::Variant,
) -> Option<usize> {
    // IDA 0x3ca10c CameraMode::lookup(variant): any_cast payload + convertToItem.
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x3ca12c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
pub fn stub_3ca12c(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    index: usize,
    out: &mut crate::generated_core_watchdog_k::render_settings::Variant,
) -> bool {
    // IDA 0x3ca12c CameraMode::convertToValue: table[index] into out-Variant (1/0).
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToString(unsigned long,std::string &)const")]
// 0x3ca160 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3ca160(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: usize,
    out: &mut String,
) -> bool {
    // IDA 0x3ca160 CameraMode::convertToString: name slot into out (1/0).
    desc.convert_to_string(value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// 0x3ca2a4 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED1Ev
// type: int()
pub fn stub_3ca2a4(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3ca2a4 CameraPanMode D1 thunk -> D2 member teardown.
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// 0x3ca2a8 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED0Ev
// type: void __fastcall(void *)
pub fn stub_3ca2a8(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3ca2a8 CameraPanMode D0: D1 teardown; slot stays caller-owned.
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::lookup(char const*)const")]
// 0x3ca348 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub unsafe fn stub_3ca348(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    name: *const std::os::raw::c_char,
) -> Option<usize> {
    // IDA 0x3ca348 CameraPanMode::lookup(name): Name::lookup + convertToValue +
    // convertToItem; miss -> 0/None.
    let text = if name.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned()
    };
    desc.lookup_by_name(&text)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0x3ca378 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_3ca378(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    variant: &crate::generated_core_watchdog_k::render_settings::Variant,
) -> Option<usize> {
    // IDA 0x3ca378 CameraPanMode::lookup(variant): any_cast payload + convertToItem.
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x3ca398 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
pub fn stub_3ca398(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    index: usize,
    out: &mut crate::generated_core_watchdog_k::render_settings::Variant,
) -> bool {
    // IDA 0x3ca398 CameraPanMode::convertToValue: table[index] into out-Variant (1/0).
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToString(unsigned long,std::string &)const")]
// 0x3ca3cc — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3ca3cc(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: usize,
    out: &mut String,
) -> bool {
    // IDA 0x3ca3cc CameraPanMode::convertToString: name slot into out (1/0).
    desc.convert_to_string(value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToString(RBX::Camera::CameraPanMode const&)const")]
// 0x3ca510 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3ca510(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
    out: &mut String,
) {
    // IDA 0x3ca510 CameraPanMode::convertToString(value): asserts + table read.
    camera_enum::enum_to_string(desc, value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToItem(RBX::Camera::CameraPanMode const&)const")]
// 0x3ca77c — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3ca77c(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
) -> Option<usize> {
    // IDA 0x3ca77c CameraPanMode::convertToItem: asserts + item-table read.
    camera_enum::enum_to_item(desc, value)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraPanMode&)const")]
// 0x3ca938 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub unsafe fn stub_3ca938(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    name: *const std::os::raw::c_char,
    out: *mut i32,
) -> bool {
    // IDA 0x3ca938 CameraPanMode::convertToValue(name): two-map walk, 1/0.
    let text = if name.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned()
    };
    camera_enum::enum_to_value_by_name(desc, &text, &mut *out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// 0x3ca9b4 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3ca9b4(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3ca9b4 CameraPanMode D2: restore off_1240078 + full member teardown.
    desc.vtable = camera_enum::CAMERA_PAN_MODE_VTAB;
    desc.destroy_d2()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToString(RBX::Camera::CameraMode const&)const")]
// 0x3cab88 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3cab88(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
    out: &mut String,
) {
    // IDA 0x3cab88 CameraMode::convertToString(value): asserts + table read.
    camera_enum::enum_to_string(desc, value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToItem(RBX::Camera::CameraMode const&)const")]
// 0x3cadf4 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3cadf4(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
) -> Option<usize> {
    // IDA 0x3cadf4 CameraMode::convertToItem: asserts + item-table read.
    camera_enum::enum_to_item(desc, value)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraMode&)const")]
// 0x3cafb0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub unsafe fn stub_3cafb0(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    name: *const std::os::raw::c_char,
    out: *mut i32,
) -> bool {
    // IDA 0x3cafb0 CameraMode::convertToValue(name): two-map walk, 1/0.
    let text = if name.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned()
    };
    camera_enum::enum_to_value_by_name(desc, &text, &mut *out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// 0x3cb02c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3cb02c(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3cb02c CameraMode D2: restore off_1240048 + full member teardown.
    desc.vtable = camera_enum::CAMERA_MODE_VTAB;
    desc.destroy_d2()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToString(RBX::Camera::CameraType const&)const")]
// 0x3cb200 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3cb200(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
    out: &mut String,
) {
    // IDA 0x3cb200 CameraType::convertToString(value): asserts + table read.
    camera_enum::enum_to_string(desc, value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToItem(RBX::Camera::CameraType const&)const")]
// 0x3cb46c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3cb46c(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    value: i32,
) -> Option<usize> {
    // IDA 0x3cb46c CameraType::convertToItem: asserts + item-table read.
    camera_enum::enum_to_item(desc, value)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToValue(RBX::Name const&,RBX::Camera::CameraType&)const")]
// 0x3cb628 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub unsafe fn stub_3cb628(
    desc: &crate::generated_core_watchdog_k::render_settings::EnumDescData,
    name: *const std::os::raw::c_char,
    out: *mut i32,
) -> bool {
    // IDA 0x3cb628 CameraType::convertToValue(name): two-map walk, 1/0.
    let text = if name.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned()
    };
    camera_enum::enum_to_value_by_name(desc, &text, &mut *out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// 0x3cb6a4 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3cb6a4(desc: &mut crate::generated_core_watchdog_k::render_settings::EnumDescData) {
    // IDA 0x3cb6a4 CameraType D2: restore off_1240018 + full member teardown.
    desc.vtable = camera_enum::CAMERA_TYPE_VTAB;
    desc.destroy_d2()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv")]
// 0x3cb878 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv
// type: void *()
pub fn stub_3cb878() -> &'static camera_enum::Creator {
    // IDA 0x3cb878: wasConstructed assert + &creatorPrivate.
    camera_enum::static_get_creator()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// 0x3cb8ec — __ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_3cb8ec() -> &'static camera_enum::ClassDescriptorInfo {
    // IDA 0x3cb8ec Described<Humanoid>::classDescriptor() singleton.
    camera_enum::humanoid_class_descriptor()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3cbf50 — __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub unsafe fn stub_3cbf50(instance: *mut u8, t: &dyn camera_enum::InstanceTeardown) {
    // IDA 0x3cbf50 Described<Camera> D1 thunk -> Instance::~Instance.
    camera_enum::camera_described_teardown(instance, t)
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3cbf54 — __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub unsafe fn stub_3cbf54(instance: *mut u8, t: &dyn camera_enum::InstanceTeardown) {
    // IDA 0x3cbf54 Described<Camera> D0: D1 + delete artifact.
    camera_enum::camera_described_teardown(instance, t)
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3cbff4 — __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub unsafe fn stub_3cbff4(adj: *const u8, t: &dyn camera_enum::InstanceTeardown) {
    // IDA 0x3cbff4 Thn32: Instance::~Instance(a1 - 32).
    camera_enum::camera_described_teardown_thn32(adj, t)
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3cbffc — __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub unsafe fn stub_3cbffc(adj: *const u8, t: &dyn camera_enum::InstanceTeardown) {
    // IDA 0x3cbffc Thn32 D0: Instance::~Instance(a1 - 32) + delete artifact.
    camera_enum::camera_described_teardown_thn32(adj, t)
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3cc0a0 — __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub unsafe fn stub_3cc0a0(adj: *const u8, t: &dyn camera_enum::InstanceTeardown) {
    // IDA 0x3cc0a0 Thn36: Instance::~Instance(a1 - 36).
    camera_enum::camera_described_teardown_thn36(adj, t)
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3cc0a8 — __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub unsafe fn stub_3cc0a8(adj: *const u8, t: &dyn camera_enum::InstanceTeardown) {
    // IDA 0x3cc0a8 Thn36 D0: Instance::~Instance(a1 - 36) + delete artifact.
    camera_enum::camera_described_teardown_thn36(adj, t)
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::EventDesc(rbx::signal<void ()(bool)> RBX::Camera::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cd164 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
pub unsafe fn stub_3cd164(
    slot: *mut camera_event::EventDescBool,
    member_offset: usize,
    name: *const std::os::raw::c_char,
) -> *mut camera_event::EventDescBool {
    // IDA 0x3cd164: base init + a1[10] = signal member + off_1240418 + bool signature.
    camera_event::EventDescBool::construct(slot, member_offset, name)
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::~EventDesc()")]
// 0x3cd2e8 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cd2e8(slot: &mut camera_enum::EventDescriptor) {
    // IDA 0x3cd2e8 D0: off_122F5A8 restore + list clear (+ delete artifact).
    camera_enum::EventDescriptor::destroy(slot)
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x3cd4f0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3cd4f0(
    desc: &camera_event::EventDescBool,
    source: *const u8,
    args: &[bool],
    sig: &dyn camera_event::BoolSignal,
) {
    // IDA 0x3cd4f0: args.size()==1 assert, derived-36, signal(derived+[40], arg).
    camera_event::fire_event_bool(desc, source, args, sig)
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x3cd57c — __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
pub fn stub_3cd57c(source: *const u8, sig: &dyn camera_event::BoolSignal) {
    // IDA 0x3cd57c: disconnectAll(*([40]) + derived).
    camera_event::disconnect_all_bool(source, sig)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<bool>(bool const&)")]
// 0x3cd590 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IbEEvRKT_
// type: int __fastcall(int, int)
pub fn stub_3cd590(slot: &dyn camera_event::BoolSlot, value: bool) {
    // IDA 0x3cd590: 1-variant vector through the +8 slot; canary discarded.
    camera_event::execute1_bool(slot, value)
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::~EventDesc()")]
// 0x3ce19c — __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3ce19c(slot: &mut camera_enum::EventDescriptor) {
    // IDA 0x3ce19c D0: off_122F5A8 restore + list clear (+ delete artifact).
    camera_enum::EventDescriptor::destroy(slot)
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x3ce454 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3ce454(source: *const u8, args: &[()], sig: &dyn camera_event::VoidSignal) {
    // IDA 0x3ce454: args.size()==0 assert, derived-36, signal(derived+[40]).
    camera_event::fire_event_void(source, args, sig)
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x3ce4c8 — __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
pub fn stub_3ce4c8(source: *const u8, sig: &dyn camera_event::VoidSignal) {
    // IDA 0x3ce4c8: disconnectAll(*([40]) + derived).
    camera_event::disconnect_all_void(source, sig)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::BoundFuncDesc(void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3ce4dc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EEC2EMS2_FvS4_S4_fEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int, int)
pub unsafe fn stub_3ce4dc(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
    n0: *const std::os::raw::c_char,
    n1: *const std::os::raw::c_char,
    n2: *const std::os::raw::c_char,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3ce4dc: base init + off_12405E8 + PAIR64 member + void defaults + declare.
    let a0 = if n0.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n0).to_string_lossy().into_owned() };
    let a1 = if n1.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n1).to_string_lossy().into_owned() };
    let a2 = if n2.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n2).to_string_lossy().into_owned() };
    camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_VOID_CF3_VTAB,
        member_func,
        member_adj,
        "void",
        &[
            (a0.as_str(), "CoordinateFrame"),
            (a1.as_str(), "CoordinateFrame"),
            (a2.as_str(), "float"),
        ],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x3ce6f4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_
// type: int __fastcall(int, int, int, int, int, int, int)
pub unsafe fn stub_3ce6f4(
    desc: *mut camera_enum::BoundFuncDescriptor,
    n0: *const std::os::raw::c_char,
    n1: *const std::os::raw::c_char,
    n2: *const std::os::raw::c_char,
) {
    // IDA 0x3ce6f4: return type + 3 addArgument entries.
    camera_boundfunc::declare_signature(
        desc,
        "void",
        &[(n0, "CoordinateFrame"), (n1, "CoordinateFrame"), (n2, "float")],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()")]
// 0x3ce75c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3ce75c(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3ce75c D0: off_12405E8, delete [14]/[13]/[12], off_1222248, clear (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_CF3_VTAB), 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3ce848 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub unsafe fn stub_3ce848(
    desc: &camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    a: &camera_boundfunc::CoordinateFrame,
    b: &camera_boundfunc::CoordinateFrame,
    c: f32,
    callee: &dyn camera_boundfunc::FnCameraCf3,
) {
    // IDA 0x3ce848: derived-36 + member pair + 3 getArgs + Call3Helper.
    camera_boundfunc::execute_cf3(desc, obj, a, b, c, callee)
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Camera,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),G3D::CoordinateFrame,G3D::CoordinateFrame,float,void>::call(RBX::Camera*,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),RBX::Reflection::Variant &,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float const&)")]
// 0x3ce94c — __ZN3RBX10Reflection11Call3HelperINS_6CameraEMS2_FvN3G3D15CoordinateFrameES4_fES4_S4_fvE4callEPS2_S6_RNS0_7VariantERKS4_SC_RKf
// type: void __fastcall(int, char *, int, int, G3D::Matrix3 *, int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub unsafe fn stub_3ce94c(
    this: *const u8,
    member_func: usize,
    member_adj: usize,
    a: &camera_boundfunc::CoordinateFrame,
    b: &camera_boundfunc::CoordinateFrame,
    c: f32,
    callee: &dyn camera_boundfunc::FnCameraCf3,
) {
    // IDA 0x3ce94c Call3Helper: (adj>>1) adjust + virtual branch + CF copies + call.
    camera_boundfunc::call_cf3(this, member_func, member_adj, a, b, c, callee)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::BoundFuncDesc(bool (RBX::Camera::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cef84 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EEC2EMS2_FbiEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub unsafe fn stub_3cef84(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
    n0: *const std::os::raw::c_char,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3cef84: base init + off_1240628 + PAIR64 member + void default + declare.
    let a0 = if n0.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n0).to_string_lossy().into_owned() };
    camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_BOOL_INT_VTAB,
        member_func,
        member_adj,
        "bool",
        &[(a0.as_str(), "int")],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cf0fc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cf0fc(
    desc: *mut camera_enum::BoundFuncDescriptor,
    n0: *const std::os::raw::c_char,
) {
    // IDA 0x3cf0fc: return type + 1 addArgument entry.
    camera_boundfunc::declare_signature(desc, "bool", &[(n0, "int")])
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::~BoundFuncDesc()")]
// 0x3cf12c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cf12c(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3cf12c D0: off_1240628, delete a1[12], off_1222248, clear (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_BOOL_INT_VTAB), 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cf200 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cf200(
    desc: &camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    arg: i32,
    callee: &dyn camera_boundfunc::FnCameraIntBool,
) -> bool {
    // IDA 0x3cf200: derived-36 + member pair + getArg<int,1> + Call1Helper.
    camera_boundfunc::execute_int_bool(desc, obj, arg, callee)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Camera,bool (RBX::Camera::*)(int),int,bool>::call(RBX::Camera*,bool (RBX::Camera::*)(int),RBX::Reflection::Variant &,int const&)")]
// 0x3cf240 — __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbiEibE4callEPS2_S4_RNS0_7VariantERKi
// type: int __fastcall(int, char *, int, _DWORD *, _DWORD *)
pub unsafe fn stub_3cf240(
    this: *const u8,
    member_func: usize,
    member_adj: usize,
    arg: i32,
    callee: &dyn camera_boundfunc::FnCameraIntBool,
) -> bool {
    // IDA 0x3cf240 Call1Helper: (adj>>1) adjust + virtual branch + call + bool out.
    let adjusted = (this as usize).wrapping_add(member_adj >> 1) as *const u8;
    callee.call(adjusted, arg)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::BoundFuncDesc(void (RBX::Camera::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cf278 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub unsafe fn stub_3cf278(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
    n0: *const std::os::raw::c_char,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3cf278: base init + off_1240648 + PAIR64 member + zeroed +0x30 box
    // + void-default declareSignature (same template as 0x3cef84/0x3cf528/0x3d0184).
    let a0 = if n0.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n0).to_string_lossy().into_owned() };
    camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_VOID_INT_VTAB,
        member_func,
        member_adj,
        "void",
        &[(a0.as_str(), "int")],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cf3f0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cf3f0(
    desc: *mut camera_enum::BoundFuncDescriptor,
    n0: *const std::os::raw::c_char,
) {
    // IDA 0x3cf3f0: void return + Name::declare + int singleton + addArgument.
    camera_boundfunc::declare_signature(desc, "void", &[(n0, "int")])
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::~BoundFuncDesc()")]
// 0x3cf420 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cf420(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3cf420 D0: off_1240648, delete a1[12] (+0x30 box), off_1222248,
    // clear +8 list (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_INT_VTAB), 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cf4f4 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cf4f4(
    desc: &camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    arg: i32,
    callee: &dyn camera_boundfunc::FnCameraIntVoid,
) {
    // IDA 0x3cf4f4: derived-36 + member pair + getArg<int,1> + member tail-call.
    camera_boundfunc::execute_int_void(desc, obj, arg, callee)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::BoundFuncDesc(bool (RBX::Camera::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cf528 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EEC2EMS2_FbfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub unsafe fn stub_3cf528(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
    n0: *const std::os::raw::c_char,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3cf528: base init + off_1240668 + PAIR64 member + zeroed +0x30 box
    // + void-default declareSignature (same template as 0x3cf278).
    let a0 = if n0.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n0).to_string_lossy().into_owned() };
    camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_BOOL_FLOAT_VTAB,
        member_func,
        member_adj,
        "bool",
        &[(a0.as_str(), "float")],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cf6a0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cf6a0(
    desc: *mut camera_enum::BoundFuncDescriptor,
    n0: *const std::os::raw::c_char,
) {
    // IDA 0x3cf6a0: bool return + Name::declare + float singleton + addArgument.
    camera_boundfunc::declare_signature(desc, "bool", &[(n0, "float")])
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::~BoundFuncDesc()")]
// 0x3cf6d0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cf6d0(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3cf6d0 D0: off_1240668, delete a1[12] (+0x30 box), off_1222248,
    // clear +8 list (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_BOOL_FLOAT_VTAB), 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cf7a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cf7a4(
    desc: &camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    arg: f32,
    callee: &dyn camera_boundfunc::FnCameraFloatBool,
) -> bool {
    // IDA 0x3cf7a4: derived-36 + member pair + getArg<float,1> + Call1Helper.
    camera_boundfunc::execute_float_bool(desc, obj, arg, callee)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Camera,bool (RBX::Camera::*)(float),float,bool>::call(RBX::Camera*,bool (RBX::Camera::*)(float),RBX::Reflection::Variant &,float const&)")]
// 0x3cf7e4 — __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbfEfbE4callEPS2_S4_RNS0_7VariantERKf
// type: int __fastcall(int, char *, int, _DWORD *, _DWORD *)
pub unsafe fn stub_3cf7e4(
    this: *const u8,
    member_func: usize,
    member_adj: usize,
    arg: f32,
    callee: &dyn camera_boundfunc::FnCameraFloatBool,
) -> bool {
    // IDA 0x3cf7e4 Call1Helper: (adj>>1) adjust + virtual branch + call + bool out.
    camera_boundfunc::call_float_bool(this, member_func, member_adj, arg, callee)
}
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::BoundFuncDesc(void (RBX::Camera::*)(RBX::Camera::CameraPanMode),char const*,char const*,RBX::Camera::CameraPanMode,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cf9bc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EEC2EMS2_FvS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
pub unsafe fn stub_3cf9bc(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
    n0: *const std::os::raw::c_char,
    default: camera_enum::CameraPanMode,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3cf9bc: base init + off_1240688 + PAIR64 member + operator new(4)
    // default box at +0x30 (*box = arg_4) + CameraPanMode-singleton +
    // typed_holder::singleton declareSignature.
    let a0 = if n0.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n0).to_string_lossy().into_owned() };
    let out = camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_VOID_PANMODE_VTAB,
        member_func,
        member_adj,
        "void",
        &[(a0.as_str(), "CameraPanMode")],
    );
    camera_boundfunc::set_default_panmode(&mut *slot, default);
    out
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cfb68 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cfb68(
    desc: *mut camera_enum::BoundFuncDescriptor,
    n0: *const std::os::raw::c_char,
) {
    // IDA 0x3cfb68: void return + Name::declare + CameraPanMode singleton + addArgument.
    camera_boundfunc::declare_signature(desc, "void", &[(n0, "CameraPanMode")])
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::~BoundFuncDesc()")]
// 0x3cfb98 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cfb98(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3cfb98 D0: off_1240688, delete a1[12] (+0x30 default box),
    // off_1222248, clear +8 list (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_PANMODE_VTAB), 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cfc6c — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3cfc6c(
    desc: &camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    arg: camera_enum::CameraPanMode,
    callee: &dyn camera_boundfunc::FnCameraPanModeVoid,
) {
    // IDA 0x3cfc6c: derived-36 + member pair + getArg<CameraPanMode,1> + member tail-call.
    camera_boundfunc::execute_panmode_void(desc, obj, arg, callee)
}
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode> const>::initSingleton(void)")]
// 0x3cfe84 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE13initSingletonEv
pub fn stub_3cfe84() -> &'static crate::generated_core_watchdog_k::render_settings::EnumDescData {
    // IDA 0x3cfe84: B.W shim tail-calling doGetSingleton (0x3cfe88).
    camera_enum::init_pan_mode_singleton()
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode> const>::doGetSingleton(void)")]
// 0x3cfe88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_3cfe88() -> &'static crate::generated_core_watchdog_k::render_settings::EnumDescData {
    // IDA 0x3cfe88: __cxa_guard acquire/release around EnumDesc<CameraPanMode>::EnumDesc
    // (0x3c4778) into the function-local slot, __cxa_atexit the D1; returns &s.
    camera_enum::pan_mode_singleton()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::BoundFuncDesc(float (RBX::Camera::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cff78 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub unsafe fn stub_3cff78(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3cff78: base init + off_12406B8 + PAIR64 member, then
    // Type::getSingleton<float> stored at +0x1C (the return type); arity 0
    // takes no names and emits no declareSignature call.
    camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_FLOAT_VOID_VTAB,
        member_func,
        member_adj,
        "float",
        &[],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::~BoundFuncDesc()")]
// 0x3d007c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3d007c(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3d007c D0: no derived restore and no arg boxes (arity 0);
    // off_1222248, clear +8 list (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, None, 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3d0130 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3d0130(
    desc: &camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    callee: &dyn camera_boundfunc::FnCameraVoidFloat,
) -> f32 {
    // IDA 0x3d0130: derived-36 + member pair, tail-jump to Call0Helper (0x3d0154).
    camera_boundfunc::execute_float_0(desc, obj, callee)
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Camera,float (RBX::Camera::*)(void),float>::call(RBX::Camera*,float (RBX::Camera::*)(void),RBX::Reflection::Variant &)")]
// 0x3d0154 — __ZN3RBX10Reflection11Call0HelperINS_6CameraEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, __int64 (__fastcall *)(_DWORD), int, _DWORD *)
pub unsafe fn stub_3d0154(
    this: *const u8,
    member_func: usize,
    member_adj: usize,
    callee: &dyn camera_boundfunc::FnCameraVoidFloat,
) -> f32 {
    // IDA 0x3d0154 Call0Helper: (adj>>1) adjust + virtual branch + call + float out.
    camera_boundfunc::call_float_0(this, member_func, member_adj, callee)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::BoundFuncDesc(void (RBX::Camera::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3d0184 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub unsafe fn stub_3d0184(
    slot: *mut camera_enum::BoundFuncDescriptor,
    member_func: usize,
    member_adj: usize,
    n0: *const std::os::raw::c_char,
) -> *mut camera_enum::BoundFuncDescriptor {
    // IDA 0x3d0184: base init + off_12406E8 + PAIR64 member + zeroed +0x30 box
    // + void-default declareSignature (same template as 0x3cf278).
    let a0 = if n0.is_null() { String::new() } else { std::ffi::CStr::from_ptr(n0).to_string_lossy().into_owned() };
    camera_boundfunc::construct(
        slot,
        camera_enum::BOUND_VOID_FLOAT_VTAB,
        member_func,
        member_adj,
        "void",
        &[(a0.as_str(), "float")],
    )
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3d02fc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3d02fc(
    desc: *mut camera_enum::BoundFuncDescriptor,
    n0: *const std::os::raw::c_char,
) {
    // IDA 0x3d02fc: void return + Name::declare + float singleton + addArgument.
    camera_boundfunc::declare_signature(desc, "void", &[(n0, "float")])
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::~BoundFuncDesc()")]
// 0x3d032c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3d032c(slot: &mut camera_enum::BoundFuncDescriptor) {
    // IDA 0x3d032c D0: off_12406E8, delete a1[12] (+0x30 box), off_1222248,
    // clear +8 list (+ delete).
    camera_enum::BoundFuncDescriptor::destroy(slot, Some(camera_enum::BOUND_VOID_FLOAT_VTAB), 1)
}
