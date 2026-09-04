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
pub fn stub_25f000() -> ! {
    todo!("0x25f000 __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::GetSetImpl<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x25f004 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_25f004() -> ! {
    todo!("0x25f004 __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::GetSetImpl<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// 0x25f028 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
pub fn stub_25f028() -> ! {
    todo!("0x25f028 __ZNK3RBX10Reflection14PropDescriptorINS_5LightEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")
}

#[doc(alias = "RBX::Reflection::EventDescriptor::EventDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x25f54c — __ZN3RBX10Reflection15EventDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_25f54c() -> ! {
    todo!("0x25f54c __ZN3RBX10Reflection15EventDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::RemoteEventCommon::Attributes::deprecated(RBX::Reflection::RemoteEventCommon::Functionality,RBX::Reflection::MemberDescriptor const*)")]
// 0x25f66c — __ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE
// type: int __fastcall(int result, int, int)
pub fn stub_25f66c() -> ! {
    todo!("0x25f66c __ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE")
}

#[doc(alias = "RBX::Reflection::EventSource::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")]
// 0x25f678 — __ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE
// type: int __fastcall(int, int)
pub fn stub_25f678() -> ! {
    todo!("0x25f678 __ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE")
}

#[doc(alias = "RBX::Reflection::EventSource::raiseEventInvocation(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const*)")]
// 0x25f688 — __ZN3RBX10Reflection11EventSource20raiseEventInvocationERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EEPKNS_13SystemAddressE
// type: void()
pub fn stub_25f688() -> ! {
    todo!("0x25f688 __ZN3RBX10Reflection11EventSource20raiseEventInvocationERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EEPKNS_13SystemAddressE")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declare(RBX::Reflection::EventDescriptor*)")]
// 0x25f690 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
pub fn stub_25f690() -> ! {
    todo!("0x25f690 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE7declareEPS2_")
}

#[doc(alias = "RBX::Reflection::EventDescriptor::~EventDescriptor()")]
// 0x25f810 — __ZN3RBX10Reflection15EventDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::EventDescriptor *__hidden this)
pub fn stub_25f810() -> ! {
    todo!("0x25f810 __ZN3RBX10Reflection15EventDescriptorD1Ev")
}

#[doc(alias = "RBX::Reflection::EventDescriptor::isScriptable(void)const")]
// 0x25f838 — __ZNK3RBX10Reflection15EventDescriptor12isScriptableEv
// type: int __fastcall(RBX::Reflection::EventDescriptor *this)
pub fn stub_25f838() -> ! {
    todo!("0x25f838 __ZNK3RBX10Reflection15EventDescriptor12isScriptableEv")
}

#[doc(alias = "RBX::Reflection::EventDescriptor::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x25f840 — __ZNK3RBX10Reflection15EventDescriptor9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaIS5_EE
// type: int __fastcall(int, int, int)
pub fn stub_25f840() -> ! {
    todo!("0x25f840 __ZNK3RBX10Reflection15EventDescriptor9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaIS5_EE")
}

#[doc(alias = "std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")]
// 0x25f898 — __ZNSt6vectorIPN3RBX10Reflection15EventDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
pub fn stub_25f898() -> ! {
    todo!("0x25f898 __ZNSt6vectorIPN3RBX10Reflection15EventDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declareSub(RBX::Reflection::EventDescriptor*,RBX::Reflection::EventDescriptor*)")]
// 0x25f8d0 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
pub fn stub_25f8d0() -> ! {
    todo!("0x25f8d0 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10declareSubEPS2_S4_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::staticData(void)")]
// 0x25fa50 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv
// type: double *()
pub fn stub_25fa50() -> ! {
    todo!("0x25fa50 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::Collection::~Collection()")]
// 0x25fab8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
pub fn stub_25fab8() -> ! {
    todo!("0x25fab8 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10CollectionD1Ev")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x25fad0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
pub fn stub_25fad0() -> ! {
    todo!("0x25fad0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "std::_List_base<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_clear(void)")]
// 0x260110 — __ZNSt10_List_baseIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
pub fn stub_260110() -> ! {
    todo!("0x260110 __ZNSt10_List_baseIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE8_M_clearEv")
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::~MemberDescriptor()")]
// 0x260140 — __ZN3RBX10Reflection16MemberDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::MemberDescriptor *__hidden this)
pub fn stub_260140() -> ! {
    todo!("0x260140 __ZN3RBX10Reflection16MemberDescriptorD1Ev")
}

#[doc(alias = "RBX::Reflection::FunctionDescriptor::FunctionDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x260274 — __ZN3RBX10Reflection18FunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_260274() -> ! {
    todo!("0x260274 __ZN3RBX10Reflection18FunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::YieldFunctionDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x260394 — __ZN3RBX10Reflection23YieldFunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_260394() -> ! {
    todo!("0x260394 __ZN3RBX10Reflection23YieldFunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declare(RBX::Reflection::FunctionDescriptor*)")]
// 0x2604b8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
pub fn stub_2604b8() -> ! {
    todo!("0x2604b8 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE7declareEPS2_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")]
// 0x260638 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
pub fn stub_260638() -> ! {
    todo!("0x260638 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_")
}

#[doc(alias = "RBX::Reflection::FunctionDescriptor::~FunctionDescriptor()")]
// 0x2607b8 — __ZN3RBX10Reflection18FunctionDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::FunctionDescriptor *__hidden this)
pub fn stub_2607b8() -> ! {
    todo!("0x2607b8 __ZN3RBX10Reflection18FunctionDescriptorD1Ev")
}

#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::~YieldFunctionDescriptor()")]
// 0x2607e0 — __ZN3RBX10Reflection23YieldFunctionDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::YieldFunctionDescriptor *__hidden this)
pub fn stub_2607e0() -> ! {
    todo!("0x2607e0 __ZN3RBX10Reflection23YieldFunctionDescriptorD1Ev")
}

#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")]
// 0x260808 — __ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
pub fn stub_260808() -> ! {
    todo!("0x260808 __ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")]
// 0x260840 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
pub fn stub_260840() -> ! {
    todo!("0x260840 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")]
// 0x2609c0 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv
// type: double *()
pub fn stub_2609c0() -> ! {
    todo!("0x2609c0 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::Collection::~Collection()")]
// 0x260a28 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
pub fn stub_260a28() -> ! {
    todo!("0x260a28 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10CollectionD1Ev")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x260a40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
pub fn stub_260a40() -> ! {
    todo!("0x260a40 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x260bc8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
pub fn stub_260bc8() -> ! {
    todo!("0x260bc8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x260d48 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
pub fn stub_260d48() -> ! {
    todo!("0x260d48 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::~MemberDescriptor()")]
// 0x260f78 — __ZN3RBX10Reflection16MemberDescriptorD0Ev
// type: void __fastcall(RBX::Reflection::MemberDescriptor *__hidden this)
pub fn stub_260f78() -> ! {
    todo!("0x260f78 __ZN3RBX10Reflection16MemberDescriptorD0Ev")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::allClasses(void)")]
// 0x2610ac — __ZN3RBX10Reflection15ClassDescriptor10allClassesEv
// type: _DWORD __fastcall(RBX::Reflection::ClassDescriptor *__hidden this)
pub fn stub_2610ac() -> ! {
    todo!("0x2610ac __ZN3RBX10Reflection15ClassDescriptor10allClassesEv")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(void)")]
// 0x261138 — __ZN3RBX10Reflection15ClassDescriptorC1Ev
// type: int __fastcall(RBX::Reflection::ClassDescriptor *this)
pub fn stub_261138() -> ! {
    todo!("0x261138 __ZN3RBX10Reflection15ClassDescriptorC1Ev")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(void)")]
// 0x26113c — __ZN3RBX10Reflection15ClassDescriptorC2Ev
// type: RBX::Reflection::ClassDescriptor *__fastcall(RBX::Reflection::ClassDescriptor *this)
pub fn stub_26113c() -> ! {
    todo!("0x26113c __ZN3RBX10Reflection15ClassDescriptorC2Ev")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(RBX::Reflection::ClassDescriptor&,char const*,RBX::Reflection::ClassDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x261300 — __ZN3RBX10Reflection15ClassDescriptorC1ERS1_PKcNS1_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_261300() -> ! {
    todo!("0x261300 __ZN3RBX10Reflection15ClassDescriptorC1ERS1_PKcNS1_10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(RBX::Reflection::ClassDescriptor&,char const*,RBX::Reflection::ClassDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x26131c — __ZN3RBX10Reflection15ClassDescriptorC2ERS1_PKcNS1_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_26131c() -> ! {
    todo!("0x26131c __ZN3RBX10Reflection15ClassDescriptorC2ERS1_PKcNS1_10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::operator==(RBX::Reflection::ClassDescriptor const&)const")]
// 0x2616c0 — __ZNK3RBX10Reflection15ClassDescriptoreqERKS1_
// type: bool __fastcall(int, int)
pub fn stub_2616c0() -> ! {
    todo!("0x2616c0 __ZNK3RBX10Reflection15ClassDescriptoreqERKS1_")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::operator!=(RBX::Reflection::ClassDescriptor const&)const")]
// 0x2616cc — __ZNK3RBX10Reflection15ClassDescriptorneERKS1_
// type: bool __fastcall(int, int)
pub fn stub_2616cc() -> ! {
    todo!("0x2616cc __ZNK3RBX10Reflection15ClassDescriptorneERKS1_")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::isA(RBX::Reflection::ClassDescriptor const&)const")]
// 0x2616d8 — __ZNK3RBX10Reflection15ClassDescriptor3isAERKS1_
// type: int __fastcall(RBX::Reflection::ClassDescriptor *this, const ClassDescriptor *)
pub fn stub_2616d8() -> ! {
    todo!("0x2616d8 __ZNK3RBX10Reflection15ClassDescriptor3isAERKS1_")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::isA(char const*)const")]
// 0x2616f0 — __ZNK3RBX10Reflection15ClassDescriptor3isAEPKc
// type: int __fastcall(RBX::Reflection::ClassDescriptor *this, const char *)
pub fn stub_2616f0() -> ! {
    todo!("0x2616f0 __ZNK3RBX10Reflection15ClassDescriptor3isAEPKc")
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::isMemberOf(RBX::Reflection::DescribedBase const*)const")]
// 0x261718 — __ZNK3RBX10Reflection16MemberDescriptor10isMemberOfEPKNS0_13DescribedBaseE
// type: int __fastcall(RBX::Reflection::MemberDescriptor *this, const RBX::Reflection::DescribedBase *, int)
pub fn stub_261718() -> ! {
    todo!("0x261718 __ZNK3RBX10Reflection16MemberDescriptor10isMemberOfEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")]
// 0x261798 — __ZN3RBX10Reflection10DescriptorC2EPKcNS1_10AttributesE
// type: int __fastcall(int, const char *const *, char, int)
pub fn stub_261798() -> ! {
    todo!("0x261798 __ZN3RBX10Reflection10DescriptorC2EPKcNS1_10AttributesE")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")]
// 0x261830 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEEC2EPS3_
// type: _DWORD *__fastcall(_DWORD *, int)
pub fn stub_261830() -> ! {
    todo!("0x261830 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEEC2EPS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")]
// 0x261948 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
pub fn stub_261948() -> ! {
    todo!("0x261948 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEEC2EPS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")]
// 0x261a60 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
pub fn stub_261a60() -> ! {
    todo!("0x261a60 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEEC2EPS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")]
// 0x261b78 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
pub fn stub_261b78() -> ! {
    todo!("0x261b78 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")]
// 0x261c90 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEEC2EPS3_
// type: _DWORD *__fastcall(_DWORD *, int)
pub fn stub_261c90() -> ! {
    todo!("0x261c90 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEEC2EPS3_")
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor **,std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>>,RBX::Reflection::ClassDescriptor * const&)")]
// 0x261da8 — __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
pub fn stub_261da8() -> ! {
    todo!("0x261da8 __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::~vector()")]
// 0x261de0 — __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EED1Ev
// type: void **__fastcall(void **)
pub fn stub_261de0() -> ! {
    todo!("0x261de0 __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EED1Ev")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
// 0x2623e8 — __ZN3RBX10Reflection15ClassDescriptorD2Ev
// type: void __fastcall(RBX::Reflection::ClassDescriptor *__hidden this)
pub fn stub_2623e8() -> ! {
    todo!("0x2623e8 __ZN3RBX10Reflection15ClassDescriptorD2Ev")
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor **,std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>>,RBX::Reflection::ClassDescriptor * const&)")]
// 0x2624b4 — __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
pub fn stub_2624b4() -> ! {
    todo!("0x2624b4 __ZNSt6vectorIPN3RBX10Reflection15ClassDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_allocate(unsigned long)")]
// 0x262594 — __ZNSt12_Vector_baseIPN3RBX10Reflection15ClassDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_262594() -> ! {
    todo!("0x262594 __ZNSt12_Vector_baseIPN3RBX10Reflection15ClassDescriptorESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")]
// 0x2625ac — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int result, int *)
pub fn stub_2625ac() -> ! {
    todo!("0x2625ac __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE12mergeMembersEPKS3_")
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> * const&)")]
// 0x2625d4 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_2625d4() -> ! {
    todo!("0x2625d4 __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::Collection::~Collection()")]
// 0x262600 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
pub fn stub_262600() -> ! {
    todo!("0x262600 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10CollectionD1Ev")
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> * const&)")]
// 0x262618 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: char *__fastcall(int, char *__src, _DWORD *)
pub fn stub_262618() -> ! {
    todo!("0x262618 __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::_M_allocate(unsigned long)")]
// 0x2626f8 — __ZNSt12_Vector_baseIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_2626f8() -> ! {
    todo!("0x2626f8 __ZNSt12_Vector_baseIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")]
// 0x262710 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
pub fn stub_262710() -> ! {
    todo!("0x262710 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE7declareEPS2_")
}

#[doc(alias = "std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::CallbackDescriptor **,std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>>,RBX::Reflection::CallbackDescriptor * const&)")]
// 0x262890 — __ZNSt6vectorIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
pub fn stub_262890() -> ! {
    todo!("0x262890 __ZNSt6vectorIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")]
// 0x2628c8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
pub fn stub_2628c8() -> ! {
    todo!("0x2628c8 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10declareSubEPS2_S4_")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::initStaticData(void)")]
// 0x262a44 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE14initStaticDataEv
pub fn stub_262a44() -> ! {
    todo!("0x262a44 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE14initStaticDataEv")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")]
// 0x262a48 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv
// type: double *()
pub fn stub_262a48() -> ! {
    todo!("0x262a48 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x262ab0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
pub fn stub_262ab0() -> ! {
    todo!("0x262ab0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x262c34 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
pub fn stub_262c34() -> ! {
    todo!("0x262c34 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x262c88 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
pub fn stub_262c88() -> ! {
    todo!("0x262c88 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x262db0 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
pub fn stub_262db0() -> ! {
    todo!("0x262db0 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x262e40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
pub fn stub_262e40() -> ! {
    todo!("0x262e40 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x262e6c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_262e6c() -> ! {
    todo!("0x262e6c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)")]
// 0x262ec4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv
// type: int __fastcall(int)
pub fn stub_262ec4() -> ! {
    todo!("0x262ec4 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x262efc — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, const char **)
pub fn stub_262efc() -> ! {
    todo!("0x262efc __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")
}

#[doc(alias = "RBX::Reflection::StringHashPredicate::operator()(char const*)const")]
// 0x262f6c — __ZNK3RBX10Reflection19StringHashPredicateclEPKc
// type: unsigned int __fastcall(int, char *__s)
pub fn stub_262f6c() -> ! {
    todo!("0x262f6c __ZNK3RBX10Reflection19StringHashPredicateclEPKc")
}

#[doc(alias = "std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::CallbackDescriptor **,std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>>,RBX::Reflection::CallbackDescriptor * const&)")]
// 0x262fa4 — __ZNSt6vectorIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
pub fn stub_262fa4() -> ! {
    todo!("0x262fa4 __ZNSt6vectorIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_allocate(unsigned long)")]
// 0x263084 — __ZNSt12_Vector_baseIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_263084() -> ! {
    todo!("0x263084 __ZNSt12_Vector_baseIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)")]
// 0x26309c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// type: int __fastcall(int result, unsigned int)
pub fn stub_26309c() -> ! {
    todo!("0x26309c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")]
// 0x263108 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int result, int *)
pub fn stub_263108() -> ! {
    todo!("0x263108 __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_")
}
