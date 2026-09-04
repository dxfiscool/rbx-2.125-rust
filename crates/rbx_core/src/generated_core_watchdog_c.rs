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
pub fn stub_3c2560() -> ! {
    todo!("0x3c2560 __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
// 0x3c2604 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_3c2604() -> ! {
    todo!("0x3c2604 __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED0Ev")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isReadOnly(void)const")]
// 0x3c2634 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10isReadOnlyEv
// type: int __fastcall(int)
pub fn stub_3c2634() -> ! {
    todo!("0x3c2634 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10isReadOnlyEv")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isWriteOnly(void)const")]
// 0x3c2644 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
pub fn stub_3c2644() -> ! {
    todo!("0x3c2644 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x3c2654 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
pub fn stub_3c2654() -> ! {
    todo!("0x3c2654 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x3c267c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_3c267c() -> ! {
    todo!("0x3c267c __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x3c2794 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
pub fn stub_3c2794() -> ! {
    todo!("0x3c2794 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x3c285c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
pub fn stub_3c285c() -> ! {
    todo!("0x3c285c __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x3c2880 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
pub fn stub_3c2880() -> ! {
    todo!("0x3c2880 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x3c2954 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
pub fn stub_3c2954() -> ! {
    todo!("0x3c2954 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c2978 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3c2978() -> ! {
    todo!("0x3c2978 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3c298c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub fn stub_3c298c() -> ! {
    todo!("0x3c298c __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3c2a08 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
pub fn stub_3c2a08() -> ! {
    todo!("0x3c2a08 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3c2a28 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_3c2a28() -> ! {
    todo!("0x3c2a28 __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3c2b08 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
pub fn stub_3c2b08() -> ! {
    todo!("0x3c2b08 __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isReadOnly(void)const")]
// 0x3c2b10 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
pub fn stub_3c2b10() -> ! {
    todo!("0x3c2b10 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isWriteOnly(void)const")]
// 0x3c2b14 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
pub fn stub_3c2b14() -> ! {
    todo!("0x3c2b14 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3c2b18 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_3c2b18() -> ! {
    todo!("0x3c2b18 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// 0x3c2b38 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3c2b38() -> ! {
    todo!("0x3c2b38 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")
}

#[doc(alias = "RBX::Camera::getCameraSubjectInstanceDangerous(void)const")]
// 0x3c39ac — __ZNK3RBX6Camera33getCameraSubjectInstanceDangerousEv
// type: int __fastcall(RBX::Camera *this)
pub fn stub_3c39ac() -> ! {
    todo!("0x3c39ac __ZNK3RBX6Camera33getCameraSubjectInstanceDangerousEv")
}

#[doc(alias = "RBX::Camera::setCameraSubject(RBX::Instance *)")]
// 0x3c39b4 — __ZN3RBX6Camera16setCameraSubjectEPNS_8InstanceE
// type: void __fastcall(shared_count *this, RBX::Instance *)
pub fn stub_3c39b4() -> ! {
    todo!("0x3c39b4 __ZN3RBX6Camera16setCameraSubjectEPNS_8InstanceE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::EnumDesc(void)")]
// 0x3c437c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEEC2Ev
// type: int __fastcall(int)
pub fn stub_3c437c() -> ! {
    todo!("0x3c437c __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEEC2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::EnumDesc(void)")]
// 0x3c45b0 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEEC1Ev
// type: int()
pub fn stub_3c45b0() -> ! {
    todo!("0x3c45b0 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEEC1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::EnumDesc(void)")]
// 0x3c45b4 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_3c45b4() -> ! {
    todo!("0x3c45b4 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEEC2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::EnumDesc(void)")]
// 0x3c4778 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_3c4778() -> ! {
    todo!("0x3c4778 __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEEC2Ev")
}

#[doc(alias = "RBX::Camera::askSetParent(RBX::Instance const*)const")]
// 0x3c4e90 — __ZNK3RBX6Camera12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Camera *__hidden this, const RBX::Instance *)
pub fn stub_3c4e90() -> ! {
    todo!("0x3c4e90 __ZNK3RBX6Camera12askSetParentEPKNS_8InstanceE")
}

#[doc(alias = "RBX::Camera::isPartVisibleFast(RBX::PartInstance const&,G3D::Rect2D const&,RBX::ContactManager const&)const")]
// 0x3c4f8c — __ZNK3RBX6Camera17isPartVisibleFastERKNS_12PartInstanceERKN3G3D6Rect2DERKNS_14ContactManagerE
// type: bool __fastcall(_DWORD *, RBX::PartInstance *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int)
pub fn stub_3c4f8c() -> ! {
    todo!("0x3c4f8c __ZNK3RBX6Camera17isPartVisibleFastERKNS_12PartInstanceERKN3G3D6Rect2DERKNS_14ContactManagerE")
}

#[doc(alias = "RBX::Camera::isPartInFrustum(RBX::PartInstance const&,G3D::Rect2D const&)const")]
// 0x3c50fc — __ZNK3RBX6Camera15isPartInFrustumERKNS_12PartInstanceERKN3G3D6Rect2DE
// type: int __fastcall(int, RBX::PartInstance *)
pub fn stub_3c50fc() -> ! {
    todo!("0x3c50fc __ZNK3RBX6Camera15isPartInFrustumERKNS_12PartInstanceERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::zoomExtents(RBX::ModelInstance const*,RBX::Camera::ZoomType)")]
// 0x3c7590 — __ZN3RBX6Camera11zoomExtentsEPKNS_13ModelInstanceENS0_8ZoomTypeE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_3c7590() -> ! {
    todo!("0x3c7590 __ZN3RBX6Camera11zoomExtentsEPKNS_13ModelInstanceENS0_8ZoomTypeE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::~EnumPropDescriptor()")]
// 0x3c8bec — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8bec() -> ! {
    todo!("0x3c8bec __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()")]
// 0x3c8c14 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8c14() -> ! {
    todo!("0x3c8c14 __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED1Ev")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::~PropDescriptor()")]
// 0x3c8c60 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8c60() -> ! {
    todo!("0x3c8c60 __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED1Ev")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::~RefPropDescriptor()")]
// 0x3c8c84 — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8c84() -> ! {
    todo!("0x3c8c84 __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::~BoundFuncDesc()")]
// 0x3c8cb0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8cb0() -> ! {
    todo!("0x3c8cb0 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::~BoundFuncDesc()")]
// 0x3c8cf0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8cf0() -> ! {
    todo!("0x3c8cf0 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::~BoundFuncDesc()")]
// 0x3c8d24 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8d24() -> ! {
    todo!("0x3c8d24 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::~BoundFuncDesc()")]
// 0x3c8d64 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8d64() -> ! {
    todo!("0x3c8d64 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::~BoundFuncDesc()")]
// 0x3c8da4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8da4() -> ! {
    todo!("0x3c8da4 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::~BoundFuncDesc()")]
// 0x3c8de4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8de4() -> ! {
    todo!("0x3c8de4 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()")]
// 0x3c8e24 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8e24() -> ! {
    todo!("0x3c8e24 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED1Ev")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::~EventDesc()")]
// 0x3c8e78 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8e78() -> ! {
    todo!("0x3c8e78 __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::~EventDesc()")]
// 0x3c8e9c — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3c8e9c() -> ! {
    todo!("0x3c8e9c __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::addPair(RBX::Camera::CameraType,char const*)")]
// 0x3c8ec0 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_3c8ec0() -> ! {
    todo!("0x3c8ec0 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::addPair(RBX::Camera::CameraMode,char const*)")]
// 0x3c9220 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_3c9220() -> ! {
    todo!("0x3c9220 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::addPair(RBX::Camera::CameraPanMode,char const*)")]
// 0x3c9580 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_3c9580() -> ! {
    todo!("0x3c9580 __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Camera::CameraPanMode & RBX::Reflection::Variant::genericConvert<RBX::Camera::CameraPanMode>(void)")]
// 0x3c98e0 — __ZN3RBX10Reflection7Variant14genericConvertINS_6Camera13CameraPanModeEEERT_v
// type: int __fastcall(int)
pub fn stub_3c98e0() -> ! {
    todo!("0x3c98e0 __ZN3RBX10Reflection7Variant14genericConvertINS_6Camera13CameraPanModeEEERT_v")
}

#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
// 0x3c9c4c — __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_3c9c4c() -> ! {
    todo!("0x3c9c4c __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE")
}

#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(RBX::Instance const*)")]
// 0x3c9c64 — __ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_3c9c64() -> ! {
    todo!("0x3c9c64 __ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")]
// 0x3c9d7c — __ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv
// type: int()
pub fn stub_3c9d7c() -> ! {
    todo!("0x3c9d7c __ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")]
// 0x3c9d9c — __ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv
// type: int()
pub fn stub_3c9d9c() -> ! {
    todo!("0x3c9d9c __ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// 0x3c9dcc — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED1Ev
// type: int()
pub fn stub_3c9dcc() -> ! {
    todo!("0x3c9dcc __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// 0x3c9dd0 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED0Ev
// type: void __fastcall(void *)
pub fn stub_3c9dd0() -> ! {
    todo!("0x3c9dd0 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::lookup(char const*)const")]
// 0x3c9e70 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_3c9e70() -> ! {
    todo!("0x3c9e70 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x3c9ea0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_3c9ea0() -> ! {
    todo!("0x3c9ea0 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x3c9ec0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
pub fn stub_3c9ec0() -> ! {
    todo!("0x3c9ec0 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToString(unsigned long,std::string &)const")]
// 0x3c9ef4 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3c9ef4() -> ! {
    todo!("0x3c9ef4 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// 0x3ca038 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED1Ev
// type: int()
pub fn stub_3ca038() -> ! {
    todo!("0x3ca038 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// 0x3ca03c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED0Ev
// type: void __fastcall(void *)
pub fn stub_3ca03c() -> ! {
    todo!("0x3ca03c __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::lookup(char const*)const")]
// 0x3ca0dc — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_3ca0dc() -> ! {
    todo!("0x3ca0dc __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0x3ca10c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_3ca10c() -> ! {
    todo!("0x3ca10c __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x3ca12c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
pub fn stub_3ca12c() -> ! {
    todo!("0x3ca12c __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToString(unsigned long,std::string &)const")]
// 0x3ca160 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3ca160() -> ! {
    todo!("0x3ca160 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// 0x3ca2a4 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED1Ev
// type: int()
pub fn stub_3ca2a4() -> ! {
    todo!("0x3ca2a4 __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// 0x3ca2a8 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED0Ev
// type: void __fastcall(void *)
pub fn stub_3ca2a8() -> ! {
    todo!("0x3ca2a8 __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::lookup(char const*)const")]
// 0x3ca348 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_3ca348() -> ! {
    todo!("0x3ca348 __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0x3ca378 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_3ca378() -> ! {
    todo!("0x3ca378 __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x3ca398 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
pub fn stub_3ca398() -> ! {
    todo!("0x3ca398 __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToString(unsigned long,std::string &)const")]
// 0x3ca3cc — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3ca3cc() -> ! {
    todo!("0x3ca3cc __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToString(RBX::Camera::CameraPanMode const&)const")]
// 0x3ca510 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3ca510() -> ! {
    todo!("0x3ca510 __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToItem(RBX::Camera::CameraPanMode const&)const")]
// 0x3ca77c — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3ca77c() -> ! {
    todo!("0x3ca77c __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraPanMode&)const")]
// 0x3ca938 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_3ca938() -> ! {
    todo!("0x3ca938 __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// 0x3ca9b4 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3ca9b4() -> ! {
    todo!("0x3ca9b4 __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToString(RBX::Camera::CameraMode const&)const")]
// 0x3cab88 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3cab88() -> ! {
    todo!("0x3cab88 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToItem(RBX::Camera::CameraMode const&)const")]
// 0x3cadf4 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3cadf4() -> ! {
    todo!("0x3cadf4 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraMode&)const")]
// 0x3cafb0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_3cafb0() -> ! {
    todo!("0x3cafb0 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// 0x3cb02c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3cb02c() -> ! {
    todo!("0x3cb02c __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToString(RBX::Camera::CameraType const&)const")]
// 0x3cb200 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3cb200() -> ! {
    todo!("0x3cb200 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToItem(RBX::Camera::CameraType const&)const")]
// 0x3cb46c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3cb46c() -> ! {
    todo!("0x3cb46c __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToValue(RBX::Name const&,RBX::Camera::CameraType&)const")]
// 0x3cb628 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_3cb628() -> ! {
    todo!("0x3cb628 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// 0x3cb6a4 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3cb6a4() -> ! {
    todo!("0x3cb6a4 __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED2Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv")]
// 0x3cb878 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv
// type: void *()
pub fn stub_3cb878() -> ! {
    todo!("0x3cb878 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// 0x3cb8ec — __ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_3cb8ec() -> ! {
    todo!("0x3cb8ec __ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3cbf50 — __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3cbf50() -> ! {
    todo!("0x3cbf50 __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3cbf54 — __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3cbf54() -> ! {
    todo!("0x3cbf54 __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3cbff4 — __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3cbff4() -> ! {
    todo!("0x3cbff4 __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3cbffc — __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3cbffc() -> ! {
    todo!("0x3cbffc __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3cc0a0 — __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3cc0a0() -> ! {
    todo!("0x3cc0a0 __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3cc0a8 — __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3cc0a8() -> ! {
    todo!("0x3cc0a8 __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::EventDesc(rbx::signal<void ()(bool)> RBX::Camera::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cd164 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
pub fn stub_3cd164() -> ! {
    todo!("0x3cd164 __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::~EventDesc()")]
// 0x3cd2e8 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cd2e8() -> ! {
    todo!("0x3cd2e8 __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x3cd4f0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3cd4f0() -> ! {
    todo!("0x3cd4f0 __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x3cd57c — __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
pub fn stub_3cd57c() -> ! {
    todo!("0x3cd57c __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<bool>(bool const&)")]
// 0x3cd590 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IbEEvRKT_
// type: int __fastcall(int, int)
pub fn stub_3cd590() -> ! {
    todo!("0x3cd590 __ZN3RBX10Reflection18GenericSlotWrapper8execute1IbEEvRKT_")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::~EventDesc()")]
// 0x3ce19c — __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3ce19c() -> ! {
    todo!("0x3ce19c __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x3ce454 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3ce454() -> ! {
    todo!("0x3ce454 __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x3ce4c8 — __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
pub fn stub_3ce4c8() -> ! {
    todo!("0x3ce4c8 __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::BoundFuncDesc(void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3ce4dc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EEC2EMS2_FvS4_S4_fEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int, int)
pub fn stub_3ce4dc() -> ! {
    todo!("0x3ce4dc __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EEC2EMS2_FvS4_S4_fEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x3ce6f4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_
// type: int __fastcall(int, int, int, int, int, int, int)
pub fn stub_3ce6f4() -> ! {
    todo!("0x3ce6f4 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()")]
// 0x3ce75c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3ce75c() -> ! {
    todo!("0x3ce75c __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3ce848 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3ce848() -> ! {
    todo!("0x3ce848 __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Camera,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),G3D::CoordinateFrame,G3D::CoordinateFrame,float,void>::call(RBX::Camera*,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),RBX::Reflection::Variant &,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float const&)")]
// 0x3ce94c — __ZN3RBX10Reflection11Call3HelperINS_6CameraEMS2_FvN3G3D15CoordinateFrameES4_fES4_S4_fvE4callEPS2_S6_RNS0_7VariantERKS4_SC_RKf
// type: void __fastcall(int, char *, int, int, G3D::Matrix3 *, int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3ce94c() -> ! {
    todo!("0x3ce94c __ZN3RBX10Reflection11Call3HelperINS_6CameraEMS2_FvN3G3D15CoordinateFrameES4_fES4_S4_fvE4callEPS2_S6_RNS0_7VariantERKS4_SC_RKf")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::BoundFuncDesc(bool (RBX::Camera::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cef84 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EEC2EMS2_FbiEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3cef84() -> ! {
    todo!("0x3cef84 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EEC2EMS2_FbiEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cf0fc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3cf0fc() -> ! {
    todo!("0x3cf0fc __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::~BoundFuncDesc()")]
// 0x3cf12c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cf12c() -> ! {
    todo!("0x3cf12c __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cf200 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3cf200() -> ! {
    todo!("0x3cf200 __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Camera,bool (RBX::Camera::*)(int),int,bool>::call(RBX::Camera*,bool (RBX::Camera::*)(int),RBX::Reflection::Variant &,int const&)")]
// 0x3cf240 — __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbiEibE4callEPS2_S4_RNS0_7VariantERKi
// type: int __fastcall(int, char *, int, _DWORD *, _DWORD *)
pub fn stub_3cf240() -> ! {
    todo!("0x3cf240 __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbiEibE4callEPS2_S4_RNS0_7VariantERKi")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::BoundFuncDesc(void (RBX::Camera::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cf278 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3cf278() -> ! {
    todo!("0x3cf278 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cf3f0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3cf3f0() -> ! {
    todo!("0x3cf3f0 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::~BoundFuncDesc()")]
// 0x3cf420 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cf420() -> ! {
    todo!("0x3cf420 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cf4f4 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3cf4f4() -> ! {
    todo!("0x3cf4f4 __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::BoundFuncDesc(bool (RBX::Camera::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cf528 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EEC2EMS2_FbfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3cf528() -> ! {
    todo!("0x3cf528 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EEC2EMS2_FbfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cf6a0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3cf6a0() -> ! {
    todo!("0x3cf6a0 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::~BoundFuncDesc()")]
// 0x3cf6d0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cf6d0() -> ! {
    todo!("0x3cf6d0 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cf7a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3cf7a4() -> ! {
    todo!("0x3cf7a4 __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Camera,bool (RBX::Camera::*)(float),float,bool>::call(RBX::Camera*,bool (RBX::Camera::*)(float),RBX::Reflection::Variant &,float const&)")]
// 0x3cf7e4 — __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbfEfbE4callEPS2_S4_RNS0_7VariantERKf
// type: int __fastcall(int, char *, int, _DWORD *, _DWORD *)
pub fn stub_3cf7e4() -> ! {
    todo!("0x3cf7e4 __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbfEfbE4callEPS2_S4_RNS0_7VariantERKf")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::BoundFuncDesc(void (RBX::Camera::*)(RBX::Camera::CameraPanMode),char const*,char const*,RBX::Camera::CameraPanMode,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cf9bc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EEC2EMS2_FvS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
pub fn stub_3cf9bc() -> ! {
    todo!("0x3cf9bc __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EEC2EMS2_FvS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3cfb68 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3cfb68() -> ! {
    todo!("0x3cfb68 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::~BoundFuncDesc()")]
// 0x3cfb98 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3cfb98() -> ! {
    todo!("0x3cfb98 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3cfc6c — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3cfc6c() -> ! {
    todo!("0x3cfc6c __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode> const>::initSingleton(void)")]
// 0x3cfe84 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE13initSingletonEv
pub fn stub_3cfe84() -> ! {
    todo!("0x3cfe84 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode> const>::doGetSingleton(void)")]
// 0x3cfe88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_3cfe88() -> ! {
    todo!("0x3cfe88 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::BoundFuncDesc(float (RBX::Camera::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3cff78 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_3cff78() -> ! {
    todo!("0x3cff78 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::~BoundFuncDesc()")]
// 0x3d007c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3d007c() -> ! {
    todo!("0x3d007c __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3d0130 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3d0130() -> ! {
    todo!("0x3d0130 __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Camera,float (RBX::Camera::*)(void),float>::call(RBX::Camera*,float (RBX::Camera::*)(void),RBX::Reflection::Variant &)")]
// 0x3d0154 — __ZN3RBX10Reflection11Call0HelperINS_6CameraEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, __int64 (__fastcall *)(_DWORD), int, _DWORD *)
pub fn stub_3d0154() -> ! {
    todo!("0x3d0154 __ZN3RBX10Reflection11Call0HelperINS_6CameraEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::BoundFuncDesc(void (RBX::Camera::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3d0184 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3d0184() -> ! {
    todo!("0x3d0184 __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3d02fc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3d02fc() -> ! {
    todo!("0x3d02fc __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::~BoundFuncDesc()")]
// 0x3d032c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3d032c() -> ! {
    todo!("0x3d032c __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED0Ev")
}
